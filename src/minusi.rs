use std::env;
use std::fs;
use std::process::Command;

fn read_dependencies(pkgbuild_path: &str) -> Option<Vec<String>> {
    let contents = match fs::read_to_string(pkgbuild_path) {
        Ok(content) => content,
        Err(_) => return None,
    };

    let mut dependencies = vec![];
    for line in contents.lines() {
        if line.starts_with("depends=(") {
            let deps_line = line.trim_start_matches("depends=(").trim_end_matches(")");
            dependencies = deps_line
                .split_whitespace()
                .map(|dep| dep.trim_matches('\'').to_string())
                .collect();
            break;
        }
    }

    Some(dependencies)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 || args[1] != "minusi" || args[2] != "install" {
        println!("Usage: minusi install <package-name>");
        return;
    }

    let package_name = &args[3];

    let _ = fs::create_dir_all("/opt/aured/pkgs");

    let clone_command = format!("git clone https://aur.archlinux.org/{}.git /opt/aured/pkgs/{}", package_name, package_name);
    let clone_result = Command::new("sh").arg("-c").arg(&clone_command).output();

    if let Ok(output) = clone_result {
        if output.status.success() {
            println!("Package cloned successfully!");
            
            let pkgbuild_path = format!("/opt/aured/pkgs/{}/PKGBUILD", package_name);
            if let Some(dependencies) = read_dependencies(&pkgbuild_path) {
                // Install dependencies with pacman
                let install_deps_command = format!("pacman -S --needed {}", dependencies.join(" "));
                let install_deps_result = Command::new("sh").arg("-c").arg(&install_deps_command).output();

                if let Ok(install_output) = install_deps_result {
                    if install_output.status.success() {
                        println!("Dependencies installed successfully!");
                    } else {
                        eprintln!("Failed to install dependencies: {:?}", install_output.stderr);
                    }
                } else {
                    eprintln!("Error installing dependencies");
                }
            } else {
                println!("No dependencies found in PKGBUILD");
            }

            let build_command = Command::new("makepkg")
                .args(&["-si"])
                .current_dir(format!("/opt/aured/pkgs/{}", package_name))
                .output();

            if let Ok(build_output) = build_command {
                if build_output.status.success() {
                    println!("Package installed successfully!");
                    let _ = fs::remove_dir_all(format!("/opt/aured/pkgs/{}", package_name));
                } else {
                    eprintln!("Failed to build package: {:?}", build_output.stderr);
                }
            } else {
                eprintln!("Error executing makepkg");
            }
        } else {
            eprintln!("Failed to clone package from AUR. Package may not exist.");
        }
    } else {
        eprintln!("Error cloning package from AUR");
    }
}
