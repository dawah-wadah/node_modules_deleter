use std::env;
use std::fs;
use walkdir::WalkDir;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use atty::Stream;

fn get_directory_size(path: &std::path::Path) -> u64 {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .map(|e| e.metadata().unwrap().len())
        .sum()
}

fn format_bytes(bytes: u64) -> String {
    const BYTE: u64 = 1;
    const KIB: u64 = 1024 * BYTE;
    const MIB: u64 = 1024 * KIB;
    const GIB: u64 = 1024 * MIB;
    const TIB: u64 = 1024 * GIB;

    if bytes < KIB {
        format!("{} B", bytes)
    } else if bytes < MIB {
        format!("{:.2} KiB", bytes as f64 / KIB as f64)
    } else if bytes < GIB {
        format!("{:.2} MiB", bytes as f64 / MIB as f64)
    } else if bytes < TIB {
        format!("{:.2} GiB", bytes as f64 / GIB as f64)
    } else {
        format!("{:.2} TiB", bytes as f64 / TIB as f64)
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <root_directory_path> [--dry-run]", args[0]);
        return Ok(());
    }

    let path = std::path::Path::new(&args[1]);
    let dry_run = args.contains(&"--dry-run".to_string());

    // Create a ColorChoice based on whether standard output is a terminal or not.
    let color_choice = if atty::is(Stream::Stdout) {
        ColorChoice::Auto
    } else {
        ColorChoice::Never
    };

    // Create a StandardStream for standard output.
    let mut stdout = StandardStream::stdout(color_choice);

    // Define the color for the numeric values as magenta (purple).
    let mut color_spec = ColorSpec::new();
    color_spec.set_fg(Some(Color::Magenta));

    let mut total_size_reclaimed = 0u64;
    let mut count = 0;

    for entry in WalkDir::new(path).min_depth(1).into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_dir() {
            let dir_name = entry.file_name().to_string_lossy(); // Convert to a string
            
            if dir_name == "node_modules" {
                let dir_size = get_directory_size(entry.path());

                stdout.set_color(&color_spec)?; // Set color before printing the size.
                if dry_run {
                    print!("Will delete directory (size: {}): ", format_bytes(dir_size));
                } else {
                    print!("Deleted directory (size: {}): ", format_bytes(dir_size));
                }
                stdout.reset()?; // Reset color after printing the size.
                println!("{:?}", entry.path());

                total_size_reclaimed += dir_size;
                count += 1;

                if !dry_run {
                    fs::remove_dir_all(entry.path())?; // Actually delete the directory
                }
            }
        }
    }

    if dry_run {
        print!("\nTotal `node_modules` directories to be deleted: ");
        stdout.set_color(&color_spec)?; // Set color before printing the count.
        print!("{}", count);
        stdout.reset()?; // Reset color after printing the count.

        print!("\nTotal space to be reclaimed: ");
        stdout.set_color(&color_spec)?; // Set color before printing the size.
        println!("{}", format_bytes(total_size_reclaimed));
        stdout.reset()?; // Reset color after printing the size.
    } else {
        print!("\nTotal `node_modules` directories deleted: ");
        stdout.set_color(&color_spec)?; // Set color before printing the count.
        print!("{}", count);
        stdout.reset()?; // Reset color after printing the count.

        print!("\nTotal space reclaimed: ");
        stdout.set_color(&color_spec)?; // Set color before printing the size.
        println!("{}", format_bytes(total_size_reclaimed));
        stdout.reset()?; // Reset color after printing the size.
    }

    Ok(())
}
