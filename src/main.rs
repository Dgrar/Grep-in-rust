use colored::*;
use std::{env, fs};

#[derive(Debug)]
struct Options {
    mode: String,
    pattern: String,
    files: Vec<String>,
    invert: bool,
    ignore_case: bool,
}

fn main() {
    println!(
        "{}",
        "────────────────────────────────────────────────────────────────────────────────────────"
            .cyan()
    );

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "--help" {
        print_help();
        return;
    }

    let mut opts = Options {
        mode: "-e".to_string(),
        pattern: String::new(),
        files: Vec::new(),
        invert: false,
        ignore_case: false,
    };

    let mut i = 1;
    let mut pattern_found = false;

    while i < args.len() {
        match args[i].as_str() {
            "-e" | "-c" | "-h" => {
                opts.mode = args[i].clone();
                i += 1;
            }
            "-v" => {
                opts.invert = true;
                i += 1;
            }
            "-i" => {
                opts.ignore_case = true;
                i += 1;
            }
            "--help" => {
                print_help();
                return;
            }
            arg if arg.starts_with("-") => {
                eprintln!("{}", format!("Unknown flag: {}", arg).red().bold());
                return;
            }
            _ => {
                if !pattern_found {
                    opts.pattern = args[i].clone();
                    pattern_found = true;
                    i += 1;
                } else {
                    opts.files = args[i..].to_vec();
                    break;
                }
            }
        }
    }

    if !pattern_found {
        eprintln!("{}", "Missing pattern".red().bold());
        return;
    }

    if opts.files.is_empty() {
        eprintln!("{}", "No files selected".red().bold());
        return;
    }

    for file_str in &opts.files {
        let file_text = match fs::read_to_string(file_str) {
            Ok(content) => content,
            Err(_) => {
                eprintln!(
                    "{} {}",
                    "Error:".red().bold(),
                    format!("File '{}' does not exist", file_str).yellow()
                );
                continue;
            }
        };

        println!("{} {}", "| File:".cyan().bold(), file_str.green().bold());
        println!(
            "{}",
            "────────────────────────────────────────────────────────────────────────────────────────"
                .cyan()
        );

        match opts.mode.as_str() {
            "-e" => show_lines(&file_text, &opts),
            "-c" => show_count(&file_text, &opts, file_str),
            "-h" => show_exists(&file_text, &opts, file_str),
            _ => {}
        }
    }

    println!(
        "{}",
        "────────────────────────────────────────────────────────────────────────────────────────"
            .cyan()
    );
}

fn show_lines(content: &str, opts: &Options) {
    let mut count = 0;

    for (i, line) in content.lines().enumerate() {
        let should_show = check_if_contains(opts, line);

        if should_show {
            let highlighted = if !opts.invert {
                highlight_line(line, opts)
            } else {
                line.to_string()
            };
            println!("{} {}", format!("⎮ {}:", i + 1).cyan(), highlighted);
            count += 1;
        }
    }

    if count == 0 {
        println!("{}", "⎮ No matches found".yellow());
    }
}

fn highlight_line(text: &str, opts: &Options) -> String {
    if opts.ignore_case {
        highlight_case_insensitive(text, &opts.pattern)
    } else {
        text.replace(&opts.pattern, &opts.pattern.yellow().bold().to_string())
    }
}

fn highlight_case_insensitive(text: &str, pattern: &str) -> String {
    let mut result = String::new();
    let text_lower = text.to_lowercase();
    let pattern_lower = pattern.to_lowercase();
    let mut last_end = 0;

    for (pos, _) in text_lower.match_indices(&pattern_lower) {
        result.push_str(&text[last_end..pos]);
        let match_end = pos + pattern_lower.len();
        result.push_str(&text[pos..match_end].yellow().bold().to_string());
        last_end = match_end;
    }
    result.push_str(&text[last_end..]);
    result
}

fn show_count(content: &str, opts: &Options, file: &str) {
    let count = content
        .lines()
        .filter(|line| check_if_contains(opts, line))
        .count();

    let msg = if opts.invert {
        format!(
            "⎮ {} lines that do NOT contain '{}' in '{}'",
            count, opts.pattern, file
        )
    } else {
        format!(
            "⎮ {} occurrences of '{}' in '{}'",
            count, opts.pattern, file
        )
    };

    println!("{}", msg.green());
}

fn check_if_contains(opts: &Options, line: &str) -> bool {
    let contains = if opts.ignore_case {
        line.to_lowercase().contains(&opts.pattern.to_lowercase())
    } else {
        line.contains(&opts.pattern)
    };

    if opts.invert {
        !contains
    } else {
        contains
    }
}

fn show_exists(content: &str, opts: &Options, file: &str) {
    let exists = content.lines().any(|line| check_if_contains(opts, line));

    if exists {
        let msg = if opts.invert {
            format!("Lines without '{}' exist in '{}'", opts.pattern, file)
        } else {
            format!("'{}' exists in '{}'", opts.pattern, file)
        };
        println!("{} {}", "⎮ Result:".cyan().bold(), msg.green().bold());
    } else {
        let msg = if opts.invert {
            format!("All lines contain '{}'", opts.pattern)
        } else {
            format!("'{}' does NOT exist in '{}'", opts.pattern, file)
        };
        println!("{} {}", "⎮ Result:".cyan().bold(), msg.red().bold());
    }
}

fn print_help() {
    println!("{}", "grep-rs - Text search tool".cyan().bold());
    println!();
    println!("{}", "USAGE:".yellow().bold());
    println!("  grep-rs [FLAGS] <PATTERN> <FILES...>");
    println!("  grep-rs <FILES...> [FLAGS] <PATTERN>");
    println!();
    println!("{}", "FLAGS:".yellow().bold());
    println!("  {} {}", "-e".green(), "Show matching lines (default)");
    println!("  {} {}", "-c".green(), "Count matching lines");
    println!("  {} {}", "-h".green(), "Check if pattern exists");
    println!(
        "  {} {}",
        "-v".magenta().bold(),
        "Invert search (non-matching)"
    );
    println!("  {} {}", "-i".green(), "Case-insensitive search");
    println!("  {} {}", "--help".cyan(), "Show this help");
    println!();
    println!("{}", "EXAMPLES:".yellow().bold());
    println!("  grep-rs -e \"error\" file.txt");
    println!("  grep-rs file.txt -i \"ERROR\"");
    println!("  grep-rs -c \"pattern\" file.txt");
    println!("  grep-rs -v -e \"debug\" file.txt");
}
