use anyhow::Result;
use colored::*;
use humansize::{BINARY, format_size};
use std::collections::HashMap;
use std::path::Path;
use terminal_size::{Width, terminal_size};
use walkdir::WalkDir;

// Show detailed statistics about a directory
pub fn show_stats(path: &str, recursive: bool, hidden: bool, verbose: bool) -> Result<()> {
    let source_path = Path::new(path);

    println!(
        "\n{} {}\n",
        "Analyzing directory:".bold(),
        source_path.display().to_string().cyan()
    );

    if verbose {
        eprintln!("Recursive: {}, Hidden files: {}", recursive, hidden);
    }

    // Set up directory walker
    let mut walker = WalkDir::new(source_path);

    if !recursive {
        walker = walker.max_depth(1);
    }

    let mut total_files = 0u64;
    let mut total_dirs = 0u64;
    let mut total_size = 0u64;
    let mut extension_stats: HashMap<String, (u64, u64)> = HashMap::new(); // (count, size)
    let mut size_distribution: HashMap<&str, u64> = HashMap::new();

    for entry in walker.into_iter().filter_map(|e| e.ok()) {
        let file_type = entry.file_type();

        if !hidden {
            let file_name = entry.file_name().to_string_lossy();
            if file_name.starts_with('.') && entry.depth() > 0 {
                continue;
            }
        }

        if file_type.is_dir() {
            total_dirs += 1;
        } else if file_type.is_file() {
            total_files += 1;

            if let Ok(metadata) = entry.metadata() {
                let size = metadata.len();
                total_size += size;

                // Extension statistics
                let ext = entry
                    .path()
                    .extension()
                    .and_then(|s| s.to_str())
                    .unwrap_or("(no extension)")
                    .to_lowercase();

                let stat = extension_stats.entry(ext).or_insert((0, 0));
                stat.0 += 1;
                stat.1 += size;

                // Size distribution
                let category = match size {
                    0..=1024 => "0-1KB",
                    1025..=102400 => "1KB-100KB",
                    102401..=1048576 => "100KB-1MB",
                    1048577..=10485760 => "1MB-10MB",
                    10485761..=104857600 => "10MB-100MB",
                    _ => "100MB+",
                };
                *size_distribution.entry(category).or_insert(0) += 1;
            }
        }
    }

    // Display general statistics
    println!("{}", "General Statistics:".green().bold());
    println!("  Total Files:       {}", total_files.to_string().cyan());
    println!("  Total Directories: {}", total_dirs.to_string().cyan());
    println!(
        "  Total Size:        {}\n",
        format_size(total_size, BINARY).cyan()
    );

    // Show size breakdown
    println!("{}", "Size Distribution:".green().bold());
    for (range, count) in [
        ("0-1KB", size_distribution.get("0-1KB").unwrap_or(&0)),
        (
            "1KB-100KB",
            size_distribution.get("1KB-100KB").unwrap_or(&0),
        ),
        (
            "100KB-1MB",
            size_distribution.get("100KB-1MB").unwrap_or(&0),
        ),
        ("1MB-10MB", size_distribution.get("1MB-10MB").unwrap_or(&0)),
        (
            "10MB-100MB",
            size_distribution.get("10MB-100MB").unwrap_or(&0),
        ),
        ("100MB+", size_distribution.get("100MB+").unwrap_or(&0)),
    ] {
        let bar = create_bar(*count, total_files, 30);
        println!("  {:12} {:>6} {}", range, count, bar);
    }

    // Display top file types
    let mut ext_vec: Vec<_> = extension_stats.iter().collect();
    ext_vec.sort_by(|a, b| b.1.0.cmp(&a.1.0)); // Sort by count descending

    println!("\n{}", "Top File Types:".green().bold());
    println!("  {:<20} {:>10} {:>15}", "Extension", "Count", "Total Size");
    println!("  {}", "─".repeat(50).bright_black());

    for (ext, (count, size)) in ext_vec.iter().take(15) {
        let ext_display = if ext.is_empty() || *ext == "(no extension)" {
            "(no extension)".bright_black()
        } else {
            format!(".{}", ext).normal()
        };

        println!(
            "  {:<20} {:>10} {:>15}",
            ext_display,
            count.to_string().cyan(),
            format_size(*size, BINARY).yellow()
        );
    }

    // Display largest files
    println!("\n{}", "Finding largest files...".green().bold());
    let mut file_sizes: Vec<_> = WalkDir::new(source_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| {
            let metadata = e.metadata().ok()?;
            Some((e.path().to_path_buf(), metadata.len()))
        })
        .collect();

    file_sizes.sort_by(|a, b| b.1.cmp(&a.1));

    // Get terminal width for dynamic column sizing
    let term_width = terminal_size()
        .map(|(Width(w), _)| w as usize)
        .unwrap_or(80);

    // Reserve 20 chars for size column and spacing
    let path_width = term_width.saturating_sub(20).max(40);

    println!("  {:<width$} {:>15}", "File", "Size", width = path_width);
    println!(
        "  {}",
        "─".repeat(term_width.saturating_sub(2)).bright_black()
    );

    for (path, size) in file_sizes.iter().take(10) {
        let display_path = path
            .strip_prefix(source_path)
            .unwrap_or(path)
            .display()
            .to_string();
        println!(
            "  {:<width$} {:>15}",
            display_path,
            format_size(*size, BINARY).yellow(),
            width = path_width
        );
    }

    println!();
    Ok(())
}

// Create a simple ASCII bar chart
fn create_bar(value: u64, max: u64, width: usize) -> String {
    if max == 0 {
        return String::new();
    }

    let filled = ((value as f64 / max as f64) * width as f64) as usize;
    let empty = width.saturating_sub(filled);

    format!(
        "{}{}",
        "█".repeat(filled).cyan(),
        "░".repeat(empty).bright_black()
    )
}

/// List files in a directory (ls command)
pub fn list_files(
    path: &str,
    show_all: bool,
    long_format: bool,
    recursive: bool,
    human_readable: bool,
    explain_perms: bool,
) -> Result<()> {
    use std::path::Path;

    let source_path = Path::new(path);

    if !source_path.exists() {
        anyhow::bail!("Path does not exist: {}", path);
    }

    if recursive {
        list_recursive(source_path, show_all, long_format, human_readable, explain_perms, 0)?;
    } else {
        list_directory(source_path, show_all, long_format, human_readable, explain_perms)?;
    }

    Ok(())
}

fn list_directory(
    path: &Path,
    show_all: bool,
    long_format: bool,
    human_readable: bool,
    explain_perms: bool,
) -> Result<()> {
    use std::fs;

    if path.is_file() {
        if long_format {
            print_long_entry(path, human_readable, explain_perms)?;
        } else {
            println!("{}", path.display());
        }
        return Ok(());
    }

    let mut entries: Vec<_> = fs::read_dir(path)?
        .filter_map(|e| e.ok())
        .collect();

    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();

        // Skip hidden files unless -a flag
        if !show_all && name.starts_with('.') {
            continue;
        }

        if long_format {
            print_long_entry(&entry.path(), human_readable, explain_perms)?;
        } else {
            let metadata = entry.metadata()?;
            if metadata.is_dir() {
                println!("{}/", name.cyan().bold());
            } else if metadata.is_symlink() {
                println!("{}", name.purple());
            } else {
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    let mode = metadata.permissions().mode();
                    if mode & 0o111 != 0 {
                        println!("{}", name.green().bold());
                    } else {
                        println!("{}", name);
                    }
                }
                #[cfg(not(unix))]
                {
                    println!("{}", name);
                }
            }
        }
    }

    Ok(())
}

fn list_recursive(
    path: &Path,
    show_all: bool,
    long_format: bool,
    human_readable: bool,
    explain_perms: bool,
    depth: usize,
) -> Result<()> {
    use std::fs;

    let indent = "  ".repeat(depth);

    if path.is_file() {
        if long_format {
            print!("{}", indent);
            print_long_entry(path, human_readable, explain_perms)?;
        } else {
            println!("{}{}", indent, path.file_name().unwrap().to_string_lossy());
        }
        return Ok(());
    }

    if depth == 0 {
        println!("{}:", path.display().to_string().cyan().bold());
    } else {
        println!(
            "{}{}:",
            indent,
            path.file_name().unwrap().to_string_lossy().cyan().bold()
        );
    }

    let mut entries: Vec<_> = fs::read_dir(path)?
        .filter_map(|e| e.ok())
        .collect();

    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();

        // Skip hidden files unless -a flag
        if !show_all && name.starts_with('.') {
            continue;
        }

        let metadata = entry.metadata()?;

        if long_format {
            print!("{}", indent);
            print_long_entry(&entry.path(), human_readable, explain_perms)?;
        } else {
            if metadata.is_dir() {
                println!("{}{}/", indent, name.cyan().bold());
            } else if metadata.is_symlink() {
                println!("{}{}", indent, name.purple());
            } else {
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    let mode = metadata.permissions().mode();
                    if mode & 0o111 != 0 {
                        println!("{}{}", indent, name.green().bold());
                    } else {
                        println!("{}{}", indent, name);
                    }
                }
                #[cfg(not(unix))]
                {
                    println!("{}{}", indent, name);
                }
            }
        }

        if metadata.is_dir() {
            list_recursive(&entry.path(), show_all, long_format, human_readable, explain_perms, depth + 1)?;
        }
    }

    Ok(())
}

fn print_long_entry(path: &Path, human_readable: bool, explain_perms: bool) -> Result<()> {
    use chrono::{DateTime, Local};
    use std::fs;

    let metadata = fs::metadata(path)?;
    let file_name = path.file_name().unwrap().to_string_lossy();

    // Permissions
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = metadata.permissions().mode();
        let perm_str = format_permissions(mode);
        if explain_perms {
            let perm_explain = explain_permissions(mode);
            print!("{} {} ", perm_str, perm_explain.bright_black());
        } else {
            print!("{} ", perm_str);
        }
    }
    #[cfg(not(unix))]
    {
        if explain_perms {
            if metadata.is_dir() {
                print!("drwxr-xr-x (owner:rwx, group:r-x, other:r-x) ");
            } else {
                print!("-rw-r--r-- (owner:rw-, group:r--, other:r--) ");
            }
        } else {
            if metadata.is_dir() {
                print!("drwxr-xr-x ");
            } else {
                print!("-rw-r--r-- ");
            }
        }
    }

    // Size
    let size = metadata.len();
    if human_readable {
        print!("{:>8} ", format_size(size, BINARY).cyan());
    } else {
        print!("{:>10} ", size.to_string().cyan());
    }

    // Modified time
    if let Ok(modified) = metadata.modified() {
        let datetime: DateTime<Local> = modified.into();
        print!("{} ", datetime.format("%b %d %H:%M").to_string().yellow());
    } else {
        print!("            ");
    }

    // Name
    if metadata.is_dir() {
        println!("{}/", file_name.cyan().bold());
    } else if metadata.is_symlink() {
        println!("{}", file_name.purple());
    } else {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = metadata.permissions().mode();
            if mode & 0o111 != 0 {
                println!("{}", file_name.green().bold());
            } else {
                println!("{}", file_name);
            }
        }
        #[cfg(not(unix))]
        {
            println!("{}", file_name);
        }
    }

    Ok(())
}

#[cfg(unix)]
fn format_permissions(mode: u32) -> String {
    use std::os::unix::fs::PermissionsExt;
    
    let file_type = match mode & 0o170000 {
        0o040000 => 'd',
        0o120000 => 'l',
        _ => '-',
    };

    let user = format!(
        "{}{}{}",
        if mode & 0o400 != 0 { 'r' } else { '-' },
        if mode & 0o200 != 0 { 'w' } else { '-' },
        if mode & 0o100 != 0 { 'x' } else { '-' }
    );

    let group = format!(
        "{}{}{}",
        if mode & 0o040 != 0 { 'r' } else { '-' },
        if mode & 0o020 != 0 { 'w' } else { '-' },
        if mode & 0o010 != 0 { 'x' } else { '-' }
    );

    let other = format!(
        "{}{}{}",
        if mode & 0o004 != 0 { 'r' } else { '-' },
        if mode & 0o002 != 0 { 'w' } else { '-' },
        if mode & 0o001 != 0 { 'x' } else { '-' }
    );

    format!("{}{}{}{}", file_type, user, group, other)
}

#[cfg(unix)]
fn explain_permissions(mode: u32) -> String {
    let user = format!(
        "{}{}{}",
        if mode & 0o400 != 0 { 'r' } else { '-' },
        if mode & 0o200 != 0 { 'w' } else { '-' },
        if mode & 0o100 != 0 { 'x' } else { '-' }
    );

    let group = format!(
        "{}{}{}",
        if mode & 0o040 != 0 { 'r' } else { '-' },
        if mode & 0o020 != 0 { 'w' } else { '-' },
        if mode & 0o010 != 0 { 'x' } else { '-' }
    );

    let other = format!(
        "{}{}{}",
        if mode & 0o004 != 0 { 'r' } else { '-' },
        if mode & 0o002 != 0 { 'w' } else { '-' },
        if mode & 0o001 != 0 { 'x' } else { '-' }
    );

    format!("(owner:{}, group:{}, other:{})", user, group, other)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_bar() {
        let bar = create_bar(50, 100, 10);
        assert_eq!(bar.chars().filter(|c| *c == '█').count(), 5);
    }
}
