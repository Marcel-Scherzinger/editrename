use std::{fs::DirEntry, path::PathBuf};

use itertools::Itertools;

mod cli;

fn main() {
    use clap::Parser;
    let args = cli::Cli::parse();
    let cli::Cli {
        directory,
        with_extension,
        sequential_delay,
    } = args;

    let _ = dotenvy::dotenv();
    env_logger::init();

    if let Some(d) = directory.as_ref() {
        std::env::set_current_dir(d).unwrap();
    }
    let directory = directory.unwrap_or_else(|| std::env::current_dir().unwrap());

    let file_names = std::fs::read_dir(directory).unwrap();

    let mut files: Vec<_> = file_names
        .into_iter()
        .flatten()
        .filter(|entry| entry.path().is_file())
        .map(|entry| {
            let with_ext = entry
                .path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            let without_ext = entry
                .path()
                .with_extension("")
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            (with_ext, without_ext, entry)
        })
        .collect();

    if files.is_empty() {
        log::error!("No files found");
        std::process::exit(1);
    }

    files.sort_by(|(a, _, _), (b, _, _)| alphanumeric_sort::compare_str(a, b));

    loop {
        let edited = edit::edit(
            files
                .iter()
                .map(|(with, without, _)| if with_extension { with } else { without })
                .join("\n"),
        )
        .unwrap();
        let trimmed = edited.trim_end_matches("\n");

        let any_changes = handle_edited(with_extension, trimmed, &files);
        if !any_changes {
            println!("Nothing changed");
            std::process::exit(3);
        }

        use inquire::Confirm;

        let apply_changes = Confirm::new("Apply changes?")
            .with_default(false)
            .with_help_message("If the shown changes shoud be applied")
            .prompt_skippable();

        use inquire::error::InquireError as IE;
        match apply_changes {
            Ok(None) | Err(IE::OperationCanceled | IE::OperationInterrupted) => {
                println!("\nRenaming was stopped");
                std::process::exit(10);
            }
            Ok(Some(true)) => {
                let longest_name_len = files
                    .iter()
                    .map(|(name, _, _)| name.chars().count())
                    .max()
                    .unwrap();

                for ((previous, _, file), tried) in files.iter().zip(trimmed.lines()) {
                    let spaces = " ".repeat(longest_name_len - previous.chars().count());

                    use colored::Colorize;
                    let fmt_prev = format!("{previous:?}").red();
                    let fmt_tried = format!("{tried:?}").green();

                    let tried = if !with_extension && let Some(ext) = file.path().extension() {
                        PathBuf::new().with_file_name(tried).with_extension(ext)
                    } else {
                        PathBuf::new().with_file_name(tried)
                    }
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();

                    if *previous != tried {
                        println!(
                            "[{}]    {fmt_prev}{spaces} -> {fmt_tried}",
                            "rename".yellow()
                        );
                        std::fs::rename(previous, tried).unwrap();
                        if let Some(delay) = sequential_delay
                            && delay >= 0.0
                        {
                            std::thread::sleep(std::time::Duration::from_secs_f64(delay));
                        }
                    }
                }

                break;
            }
            Ok(Some(false)) => continue,
            Err(_) => panic!("Error with questionnaire, try again later"),
        }
    }
}

fn handle_edited(
    with_extension: bool,
    trimmed: &str,
    files: &[(String, String, DirEntry)],
) -> bool {
    if trimmed.lines().count() != files.len() {
        log::error!(
            "Input has different number of lines than number of files: {} != {}",
            trimmed.lines().count(),
            files.len()
        );
        std::process::exit(1);
    }
    let longest_name_len = files
        .iter()
        .map(|(with, _without, _)| with.chars().count())
        .max()
        .unwrap();

    let mut any_changes = false;
    for ((previous, _, file), tried) in files.iter().zip(trimmed.lines()) {
        let spaces = " ".repeat(longest_name_len - previous.chars().count());

        use colored::Colorize;
        let fmt_prev = format!("{previous:?}").red();

        let fmt_tried = if !with_extension && let Some(ext) = file.path().extension() {
            PathBuf::new().with_file_name(tried).with_extension(ext)
        } else {
            PathBuf::new().with_file_name(tried)
        }
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
        let fmt_tried = format!("{fmt_tried:?}").green();

        if previous == tried {
            println!("{}", format!("[unchanged] {previous:?}").bright_black());
        } else {
            any_changes = true;
            println!(
                "[{}]    {fmt_prev}{spaces} -> {fmt_tried}",
                "rename".yellow()
            );
        }
    }
    any_changes
}
