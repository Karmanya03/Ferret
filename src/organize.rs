use anyhow::{Context, Result, anyhow};
use chrono::{DateTime, Datelike, Local};
use colored::*;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

// Configuration for the organize command
pub struct OrganizeCommand {
    pub path: String,
    pub method: String,
    pub output: Option<String>,
    pub dry_run: bool,
    pub copy: bool,
    pub recursive: bool,
    pub hidden: bool,
    pub verbose: bool,
}

impl OrganizeCommand {
    pub fn execute(&self) -> Result<()> {
        let source_path = Path::new(&self.path);

        // Basic validation
        if !source_path.exists() {
            return Err(anyhow!("Path does not exist: {}", self.path));
        }
        // Determine output directory

        let output_base = self
            .output
            .as_ref()
            .map(PathBuf::from)
            .unwrap_or_else(|| source_path.join("organized"));

        if self.dry_run {
            println!(
                "{}",
                "DRY RUN MODE - No files will be moved/copied"
                    .yellow()
                    .bold()
            );
        }

        if self.verbose {
            eprintln!("Source: {}", source_path.display().to_string().cyan());
            eprintln!("Output: {}", output_base.display().to_string().cyan());
            eprintln!("Method: {}", self.method.green());
            eprintln!("Mode: {}", if self.copy { "copy" } else { "move" });
        }

        // Build the list of files to organize
        let mut walker = WalkDir::new(source_path);

        if !self.recursive {
            walker = walker.max_depth(1);
        }

        let entries: Vec<DirEntry> = walker
            .into_iter()
            .filter_entry(|e| self.should_process_entry(e))
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .collect();

        println!(
            "{} {} files to organize...\n",
            "Found".green().bold(),
            entries.len()
        );

        // Organize based on method
        match self.method.as_str() {
            "type" | "extension" => self.organize_by_type(&entries, &output_base)?,
            "date" => self.organize_by_date(&entries, &output_base)?,
            "size" => self.organize_by_size(&entries, &output_base)?,
            _ => return Err(anyhow!("Unknown organization method: {}", self.method)),
        }

        if !self.dry_run {
            println!("\n{} Files organized successfully!", "✓".green().bold());
        }

        Ok(())
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

    fn organize_by_type(&self, entries: &[DirEntry], output_base: &Path) -> Result<()> {
        let mut stats: HashMap<String, usize> = HashMap::new();

        for entry in entries {
            let path = entry.path();
            let category = self.categorize_file(path);

            let dest_dir = output_base.join(&category);
            let dest_path = dest_dir.join(path.file_name().unwrap());

            *stats.entry(category.clone()).or_insert(0) += 1;

            self.move_or_copy_file(path, &dest_path, &dest_dir)?;
        }

        // Print statistics
        println!("\n{}", "Organization Summary:".bold());
        for (category, count) in stats.iter() {
            println!("  {} {} files", category.cyan(), count);
        }

        Ok(())
    }

    // Organize files by modification date (year/month structure)
    fn organize_by_date(&self, entries: &[DirEntry], output_base: &Path) -> Result<()> {
        let mut stats: HashMap<String, usize> = HashMap::new();

        for entry in entries {
            let path = entry.path();
            let metadata = fs::metadata(path)?;

            if let Ok(modified) = metadata.modified() {
                let datetime: DateTime<Local> = modified.into();
                let year = datetime.year();
                let month = datetime.month();

                let date_folder = format!("{}/{:02}-{}", year, month, datetime.format("%B"));
                let dest_dir = output_base.join(&date_folder);
                let dest_path = dest_dir.join(path.file_name().unwrap());

                *stats.entry(date_folder.clone()).or_insert(0) += 1;

                self.move_or_copy_file(path, &dest_path, &dest_dir)?;
            }
        }

        // Print statistics
        println!("\n{}", "Organization Summary:".bold());
        for (date, count) in stats.iter() {
            println!("  {} {} files", date.cyan(), count);
        }

        Ok(())
    }

    // Organize files by size categories
    fn organize_by_size(&self, entries: &[DirEntry], output_base: &Path) -> Result<()> {
        let mut stats: HashMap<String, usize> = HashMap::new();

        for entry in entries {
            let path = entry.path();
            let metadata = fs::metadata(path)?;
            let size = metadata.len();

            let category = match size {
                0..=1024 => "tiny (0-1KB)",
                1025..=102400 => "small (1KB-100KB)",
                102401..=1048576 => "medium (100KB-1MB)",
                1048577..=10485760 => "large (1MB-10MB)",
                10485761..=104857600 => "huge (10MB-100MB)",
                _ => "gigantic (100MB+)",
            };

            let dest_dir = output_base.join(category);
            let dest_path = dest_dir.join(path.file_name().unwrap());

            *stats.entry(category.to_string()).or_insert(0) += 1;

            self.move_or_copy_file(path, &dest_path, &dest_dir)?;
        }

        // Print statistics
        println!("\n{}", "Organization Summary:".bold());
        for (category, count) in stats.iter() {
            println!("  {} {} files", category.cyan(), count);
        }

        Ok(())
    }
    // Figure out what category a file belongs to based on extension

    fn categorize_file(&self, path: &Path) -> String {
        let extension = path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();

        match extension.as_str() {
            // Documents
            "pdf" | "doc" | "docx" | "txt" | "rtf" | "odt" | "tex" | "md" => {
                "documents".to_string()
            }

            // Images
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "svg" | "webp" | "ico" | "tiff" => {
                "images".to_string()
            }

            // Videos
            "mp4" | "avi" | "mkv" | "mov" | "wmv" | "flv" | "webm" | "m4v" => "videos".to_string(),

            // Audio
            "mp3" | "wav" | "flac" | "aac" | "ogg" | "wma" | "m4a" | "opus" => "audio".to_string(),

            // Archives
            "zip" | "rar" | "7z" | "tar" | "gz" | "bz2" | "xz" | "iso" => "archives".to_string(),

            // Code
            "rs" | "py" | "js" | "ts" | "java" | "c" | "cpp" | "h" | "hpp" | "go" | "rb"
            | "php" | "swift" | "kt" | "cs" | "sh" | "bash" | "zsh" => "code".to_string(),

            // Web
            "html" | "css" | "scss" | "sass" | "less" | "json" | "xml" | "yaml" | "yml" => {
                "web".to_string()
            }

            // Spreadsheets
            "xls" | "xlsx" | "csv" | "ods" => "spreadsheets".to_string(),

            // Presentations
            "ppt" | "pptx" | "odp" | "key" => "presentations".to_string(),

            // Executables & Binaries
            "exe" | "msi" | "app" | "deb" | "rpm" | "apk" | "dmg" => "executables".to_string(),

            // Databases
            "db" | "sqlite" | "sql" | "mdb" => "databases".to_string(),

            // Fonts
            "ttf" | "otf" | "woff" | "woff2" => "fonts".to_string(),

            // Other
            "" => "no-extension".to_string(),
            _ => format!("{}-files", extension),
            // Either move or copy a file (depending on settings)
        }
    }

    fn move_or_copy_file(&self, source: &Path, dest: &Path, dest_dir: &Path) -> Result<()> {
        if self.dry_run {
            let action = if self.copy { "Copy" } else { "Move" };
            println!(
                "{} {} {} {}",
                action.yellow(),
                source.display(),
                "→".bright_black(),
                dest.display()
            );
            return Ok(());
        }

        // Create the destination folder
        fs::create_dir_all(dest_dir)
            .with_context(|| format!("Failed to create directory: {}", dest_dir.display()))?;

        // Handle name conflicts (adds _1, _2, etc if file exists)
        let final_dest = self.resolve_conflict(dest)?;

        if self.copy {
            fs::copy(source, &final_dest).with_context(|| {
                format!(
                    "Failed to copy {} to {}",
                    source.display(),
                    final_dest.display()
                )
            })?;
            println!(
                "{} {} {} {}",
                "Copied".green(),
                source.display(),
                "→".bright_black(),
                final_dest.display()
            );
        } else {
            fs::rename(source, &final_dest).with_context(|| {
                format!(
                    "Failed to move {} to {}",
                    source.display(),
                    final_dest.display()
                )
            })?;
            println!(
                "{} {} {} {}",
                "Moved".blue(),
                source.display(),
                "→".bright_black(),
                final_dest.display()
            );
        }

        // Add numbers to filename if it already exists (_1, _2, etc)
        Ok(())
    }

    fn resolve_conflict(&self, path: &Path) -> Result<PathBuf> {
        if !path.exists() {
            return Ok(path.to_path_buf());
        }

        let parent = path.parent().unwrap();
        let stem = path.file_stem().unwrap().to_string_lossy();
        let ext = path
            .extension()
            .map(|s| format!(".{}", s.to_string_lossy()))
            .unwrap_or_default();

        for i in 1..1000 {
            let new_name = format!("{}_{}{}", stem, i, ext);
            let new_path = parent.join(new_name);

            if !new_path.exists() {
                return Ok(new_path);
            }
        }

        Err(anyhow!(
            "Could not resolve filename conflict for {}",
            path.display()
        ))
    }
}
