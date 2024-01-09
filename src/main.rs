use std::env;
use std::process::Command;
use reqwest::blocking::get;
use select::document::Document;
use select::predicate::Name;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: aured <command> <package>");
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "-i" => install_package(&args),
        "-s" => search_package(&args),
        "-r" => remove_package(&args),
        "-su" => update_repository(),
        _ => println!("Invalid command. Use -i, -s, -r, or -su."),
    }
}

fn install_package(args: &[String]) {
    if args.len() < 3 {
        println!("Usage: aured -i <package>");
        return;
    }

    let package = &args[2];
    println!("Installing package {}", package);
    
    // Execute minusi install <package>
    let minusi_command = format!("minusi install {}", package);
    let minusi_result = Command::new("sh").arg("-c").arg(&minusi_command).output();
    
    if let Ok(output) = minusi_result {
        if output.status.success() {
            println!("Package installed successfully!");
        } else {
            eprintln!("Failed to install package: {:?}", output.stderr);
        }
    } else {
        eprintln!("Error executing minusi command");
    }
}

fn search_package(args: &[String]) {
    if args.len() != 3 || args[1] != "-s" {
        println!("Usage: aured -s <package>");
        return;
    }

    let package_name = &args[2];
    println!("Searching for package '{}'", package_name);
    
    let url = format!("https://aur.archlinux.org/packages/?K={}", package_name);

    match get(&url) {
        Ok(response) => {
            if let Ok(body) = response.text() {
                let document = Document::from(body.as_str());

                // Extract relevant details from the HTML
                for node in document.find(Name("tr")) {
                    let name = node.find(Name("td"))
                        .nth(0)
                        .and_then(|n| n.find(Name("a")).next())
                        .map(|n| n.text());

                    if let Some(package_name) = name {
                        println!("Package found: {}", package_name);
                        // Extract other relevant details here and display to the user
                    }
                }
            } else {
                println!("Failed to fetch search results.");
            }
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}

use std::io::{self, Write};

fn confirm_removal() -> bool {
    print!("Are you sure to remove the package? (y/n): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().to_lowercase();

    match input.as_str() {
        "y" => true,
        "n" => false,
        _ => {
            println!("Invalid answer. Please enter 'y' or 'n'.");
            confirm_removal()
        }
    }
}

fn remove_package(args: &[String]) {
    if args.len() < 3 {
        println!("Usage: aured -r <package>");
        return;
    }

    let package = &args[2];
    println!("Removing package {}", package);

    if confirm_removal() {
        let remove_command = format!("pacman -Rs {}", package); // Using -Rs to remove the package and its dependencies
        let remove_result = std::process::Command::new("sh").arg("-c").arg(&remove_command).output();

        if let Ok(output) = remove_result {
            if output.status.success() {
                println!("Package removed successfully!");
            } else {
                eprintln!("Failed to remove package: {:?}", output.stderr);
            }
        } else {
            eprintln!("Error executing pacman command");
        }
    } else {
        println!("Operation cancelled.");
    }
}



fn update_repository() {
    println!("Updating repository");

    let update_command = "pacman -Syy"; // Update the system's package databases
    let update_result = std::process::Command::new("sh").arg("-c").arg(update_command).output();

    if let Ok(output) = update_result {
        if output.status.success() {
            println!("Repository updated successfully!");
        } else {
            eprintln!("Failed to update repository: {:?}", output.stderr);
        }
    } else {
        eprintln!("Error executing update command");
    }
}

/*
aaah, i never write code like this....
the variable name...
is not shorted

*/

