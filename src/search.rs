use anyhow::{Context, Result};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;
use std::process::Command;
use std::time::SystemTime;
use walkdir::{DirEntry, WalkDir};

// Holds all the search parameters
pub struct SearchCommand {
    pub pattern: String,
    pub path: String,
    pub ignore_case: bool,
    pub regex: bool,
    pub file_type: Option<String>,
    pub min_size: Option<String>,
    pub max_size: Option<String>,
    pub modified_days: Option<u64>,
    pub recursive: bool,
    pub max_depth: Option<usize>,
    pub hidden: bool,
    pub output: String,
    pub exec: Option<String>,
    pub verbose: bool,
    pub quiet: bool,
    pub follow_links: bool,
}

impl SearchCommand {
    pub fn execute(&self) -> Result<()> {
        let start = std::time::Instant::now();

        // Show what we're searching for if verbose mode
        if self.verbose && !self.quiet {
            eprintln!("Searching for: {}", self.pattern.bright_yellow());
            eprintln!("Path: {}", self.path.cyan());
            eprintln!(
                "Options: case-insensitive={}, regex={}, hidden={}, follow-links={}",
                self.ignore_case, self.regex, self.hidden, self.follow_links
            );
        }

        // Convert pattern to regex (either from regex flag or glob pattern)
        let pattern = if self.regex {
            self.compile_regex()?
        } else {
            self.compile_glob_to_regex()?
        };

        // Parse size filters
        let min_size = self.min_size.as_ref().map(|s| parse_size(s)).transpose()?;
        let max_size = self.max_size.as_ref().map(|s| parse_size(s)).transpose()?;

        // Setup walker
        let mut walker = WalkDir::new(&self.path);

        if self.follow_links {
            walker = walker.follow_links(true);
        }

        if !self.recursive {
            walker = walker.max_depth(1);
        } else if let Some(depth) = self.max_depth {
            walker = walker.max_depth(depth);
        }

        // Collect entries
        let entries: Vec<DirEntry> = walker
            .into_iter()
            .filter_entry(|e| self.should_process_entry(e))
            .filter_map(|e| e.ok())
            .collect();

        let progress = if !self.quiet {
            let pb = ProgressBar::new(entries.len() as u64);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
                    .unwrap()
                    .progress_chars("=>-"),
            );
            Some(pb)
        } else {
            None
        };

        // Walk through directories and collect matching files
        let mut results: Vec<DirEntry> = entries
            .into_iter()
            .filter(|entry| {
                if let Some(ref pb) = progress {
                    pb.inc(1);
                }
                self.matches_criteria(entry, &pattern, min_size, max_size)
            })
            .collect();

        if let Some(pb) = progress {
            pb.finish_with_message("Search complete");
        }

        // Sort results by path for consistent output
        results.sort_by(|a, b| a.path().cmp(b.path()));

        // Display results
        self.display_results(&results)?;

        // Execute command if specified
        if let Some(cmd) = &self.exec {
            self.execute_on_files(&results, cmd)?;
        }

        if !self.quiet {
            let duration = start.elapsed();
            eprintln!(
                "\n{} {} files in {:.2?}",
                "Found".green().bold(),
                results.len(),
                duration
            );
        }

        Ok(())
    }

    fn compile_regex(&self) -> Result<Regex> {
        let pattern = if self.ignore_case {
            format!("(?i){}", self.pattern)
        } else {
            self.pattern.clone()
        };

        Regex::new(&pattern).with_context(|| format!("Invalid regex pattern: {}", self.pattern))
    }

    fn compile_glob_to_regex(&self) -> Result<Regex> {
        let pattern = glob_to_regex(&self.pattern);
        let pattern = if self.ignore_case {
            format!("(?i){}", pattern)
        } else {
            pattern
        };

        Regex::new(&pattern).with_context(|| format!("Invalid pattern: {}", self.pattern))
    }

    fn should_process_entry(&self, entry: &DirEntry) -> bool {
        if !self.hidden {
            let file_name = entry.file_name().to_string_lossy();
            if file_name.starts_with('.') && entry.depth() > 0 {
                return false;
            }
        }
        true
    }

    fn matches_criteria(
        &self,
        entry: &DirEntry,
        pattern: &Regex,
        min_size: Option<u64>,
        max_size: Option<u64>,
    ) -> bool {
        let file_name = entry.file_name().to_string_lossy();

        // Pattern matching
        if !pattern.is_match(&file_name) {
            return false;
        }

        // File type filter
        if let Some(ref ftype) = self.file_type {
            match ftype.as_str() {
                "file" | "f" => {
                    if !entry.file_type().is_file() {
                        return false;
                    }
                }
                "dir" | "d" => {
                    if !entry.file_type().is_dir() {
                        return false;
                    }
                }
                "symlink" | "l" => {
                    if !entry.file_type().is_symlink() {
                        return false;
                    }
                }
                _ => {}
            }
        }

        // Size filters
        if let Ok(metadata) = entry.metadata() {
            let size = metadata.len();

            if let Some(min) = min_size
                && size < min
            {
                return false;
            }

            if let Some(max) = max_size
                && size > max
            {
                return false;
            }

            // Modified time filter
            if let Some(days) = self.modified_days
                && let Ok(modified) = metadata.modified()
                && let Ok(duration) = SystemTime::now().duration_since(modified)
                && duration.as_secs() > days * 86400
            {
                return false;
            }
        }

        true
    }

    fn display_results(&self, results: &[DirEntry]) -> Result<()> {
        match self.output.as_str() {
            "json" => self.display_json(results)?,
            "detailed" => self.display_detailed(results)?,
            _ => self.display_default(results)?,
        }
        Ok(())
    }

    fn display_default(&self, results: &[DirEntry]) -> Result<()> {
        for entry in results {
            let display_path = entry
                .path()
                .strip_prefix(&self.path)
                .unwrap_or(entry.path())
                .display();

            if entry.file_type().is_dir() {
                println!("{}", format!("{}/", display_path).blue().bold());
            } else if entry.file_type().is_symlink() {
                println!("{}", format!("{}", display_path).cyan());
            } else {
                println!("{}", display_path);
            }
        }
        Ok(())
    }

    fn display_detailed(&self, results: &[DirEntry]) -> Result<()> {
        use chrono::{DateTime, Local};
        use humansize::{BINARY, format_size};

        for entry in results {
            let path = entry.path();
            let display_path = path.strip_prefix(&self.path).unwrap_or(path).display();

            if let Ok(metadata) = entry.metadata() {
                let size = format_size(metadata.len(), BINARY);
                let modified = metadata
                    .modified()
                    .ok()
                    .and_then(|t| {
                        DateTime::<Local>::from(t)
                            .format("%Y-%m-%d %H:%M")
                            .to_string()
                            .into()
                    })
                    .unwrap_or_else(|| "Unknown".to_string());

                let type_str = if entry.file_type().is_dir() {
                    "DIR ".blue().bold()
                } else if entry.file_type().is_symlink() {
                    "LINK".cyan()
                } else {
                    "FILE".normal()
                };

                println!(
                    "{} {:>10} {} {}",
                    type_str,
                    size,
                    modified.bright_black(),
                    display_path
                );
            }
        }
        Ok(())
    }

    fn display_json(&self, results: &[DirEntry]) -> Result<()> {
        use serde_json::json;

        let json_results: Vec<_> = results
            .iter()
            .filter_map(|entry| {
                let path = entry.path();
                let metadata = entry.metadata().ok()?;

                Some(json!({
                    "path": path.to_string_lossy(),
                    "name": entry.file_name().to_string_lossy(),
                    "type": if entry.file_type().is_dir() { "directory" }
                            else if entry.file_type().is_symlink() { "symlink" }
                            else { "file" },
                    "size": metadata.len(),
                    "modified": metadata.modified().ok()
                        .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok())
                        .map(|d| d.as_secs()),
                }))
            })
            .collect();

        println!("{}", serde_json::to_string_pretty(&json_results)?);
        Ok(())
    }

    fn execute_on_files(&self, results: &[DirEntry], cmd: &str) -> Result<()> {
        println!(
            "\n{} on {} files...",
            "Executing command".yellow().bold(),
            results.len()
        );

        for entry in results {
            let path = entry.path();
            let output = Command::new("sh")
                .arg("-c")
                .arg(cmd.replace("{}", &path.to_string_lossy()))
                .output()
                .with_context(|| format!("Failed to execute command on {}", path.display()))?;

            if !output.status.success() {
                eprintln!(
                    "{} for {}: {}",
                    "Error".red(),
                    path.display(),
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }

        Ok(())
    }
}

fn glob_to_regex(pattern: &str) -> String {
    let mut regex = String::from("^");

    for ch in pattern.chars() {
        match ch {
            '*' => regex.push_str(".*"),
            '?' => regex.push('.'),
            '.' => regex.push_str("\\."),
            '+' => regex.push_str("\\+"),
            '(' | ')' | '[' | ']' | '{' | '}' | '^' | '$' | '|' | '\\' => {
                regex.push('\\');
                regex.push(ch);
            }
            _ => regex.push(ch),
        }
    }

    regex.push('$');
    regex
}

fn parse_size(size_str: &str) -> Result<u64> {
    let size_str = size_str.trim().to_uppercase();
    let (num_str, multiplier) = if let Some(stripped) = size_str.strip_suffix('K') {
        (stripped, 1024u64)
    } else if let Some(stripped) = size_str.strip_suffix('M') {
        (stripped, 1024 * 1024)
    } else if let Some(stripped) = size_str.strip_suffix('G') {
        (stripped, 1024 * 1024 * 1024)
    } else if let Some(stripped) = size_str.strip_suffix('T') {
        (stripped, 1024 * 1024 * 1024 * 1024)
    } else {
        (size_str.as_str(), 1)
    };

    let num: u64 = num_str
        .parse()
        .with_context(|| format!("Invalid size format: {}", size_str))?;

    Ok(num * multiplier)
}
