use anyhow::Result;
use clap::{Parser, Subcommand};

// Import our custom modules
mod organize;
mod search;
mod utils;

use organize::OrganizeCommand;
use search::SearchCommand;

// Main CLI structure
#[derive(Parser)]
#[command(name = "fr")]
#[command(author = "Ferret")]
#[command(version = "1.1.2")]
#[command(about = "Ferret - Fast file finder and organizer for Linux/Unix systems", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

// Available commands
#[derive(Subcommand)]
enum Commands {
    /// Find files with advanced filters and pattern matching
    Find {
        /// Pattern to search for (supports glob patterns)
        pattern: String,

        /// Directory to search in (default: current directory)
        #[arg(short, long, default_value = ".")]
        path: String,

        /// Case-insensitive search (can combine: -irH)
        #[arg(short, long)]
        ignore_case: bool,

        /// Use regex pattern matching (can combine: -irH)
        #[arg(short, long)]
        regex: bool,

        /// File type filter (file, dir, symlink)
        #[arg(short = 't', long)]
        file_type: Option<String>,

        /// Minimum file size (e.g., 1M, 500K, 2G)
        #[arg(long)]
        min_size: Option<String>,

        /// Maximum file size (e.g., 1M, 500K, 2G)
        #[arg(long)]
        max_size: Option<String>,

        /// Modified within last N days
        #[arg(short = 'm', long)]
        modified_days: Option<u64>,

        /// Search recursively (default: true)
        #[arg(short = 'R', long, default_value = "true")]
        recursive: bool,

        /// Maximum depth for recursive search
        #[arg(short = 'd', long)]
        max_depth: Option<usize>,

        /// Show hidden files (can combine: -iH or -irH)
        #[arg(short = 'H', long)]
        hidden: bool,

        /// Output format (default, json, detailed)
        #[arg(short = 'o', long, default_value = "default")]
        output: String,

        /// Execute command on found files
        #[arg(short = 'x', long)]
        exec: Option<String>,

        /// Verbose output (can combine: -vH or -viH)
        #[arg(short = 'v', long)]
        verbose: bool,

        /// Quiet mode - only show file paths
        #[arg(short = 'q', long)]
        quiet: bool,

        /// Follow symbolic links (can combine: -iHl)
        #[arg(short = 'l', long)]
        follow_links: bool,
    },

    /// Organize files by type, date, or custom rules
    Organize {
        /// Directory to organize
        #[arg(default_value = ".")]
        path: String,

        /// Organization method (type, date, size, extension)
        #[arg(short, long, default_value = "type")]
        method: String,

        /// Output directory for organized files
        #[arg(short, long)]
        output: Option<String>,

        /// Dry run - show what would be done without moving files (can combine: -nrv)
        #[arg(short = 'n', long)]
        dry_run: bool,

        /// Copy files instead of moving (can combine: -crv)
        #[arg(short, long)]
        copy: bool,

        /// Organize recursively (can combine: -rn or -rc)
        #[arg(short, long)]
        recursive: bool,

        /// Include hidden files (can combine: -rH or -nrH)
        #[arg(short = 'H', long)]
        hidden: bool,

        /// Verbose output (can combine: -rvH)
        #[arg(short = 'v', long)]
        verbose: bool,
    },

    /// Get statistics about files in a directory
    Stats {
        /// Directory to analyze
        #[arg(default_value = ".")]
        path: String,

        /// Analyze recursively (can combine: -rH or -rv)
        #[arg(short, long)]
        recursive: bool,

        /// Include hidden files (can combine: -rH)
        #[arg(short = 'H', long)]
        hidden: bool,

        /// Verbose output (can combine: -rvH)
        #[arg(short = 'v', long)]
        verbose: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Route to the appropriate command handler
    match cli.command {
        Commands::Find {
            pattern,
            path,
            ignore_case,
            regex,
            file_type,
            min_size,
            max_size,
            modified_days,
            recursive,
            max_depth,
            hidden,
            output,
            exec,
            verbose,
            quiet,
            follow_links,
        } => {
            let search_cmd = SearchCommand {
                pattern,
                path,
                ignore_case,
                regex,
                file_type,
                min_size,
                max_size,
                modified_days,
                recursive,
                max_depth,
                hidden,
                output,
                exec,
                verbose,
                quiet,
                follow_links,
            };
            search_cmd.execute()?;
        }

        Commands::Organize {
            path,
            method,
            output,
            dry_run,
            copy,
            recursive,
            hidden,
            verbose,
        } => {
            let organize_cmd = OrganizeCommand {
                path,
                method,
                output,
                dry_run,
                copy,
                recursive,
                hidden,
                verbose,
            };
            organize_cmd.execute()?;
        }

        Commands::Stats {
            path,
            recursive,
            hidden,
            verbose,
        } => {
            utils::show_stats(&path, recursive, hidden, verbose)?;
        }
    }

    Ok(())
}
