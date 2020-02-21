use colored::*;
use directories::ProjectDirs;
use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use structopt::{clap::AppSettings, StructOpt};

type Error = Box<dyn std::error::Error>;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "nix-search",
    about = "A better nix-search",
    global_settings = &[AppSettings::ColoredHelp, AppSettings::ArgRequiredElseHelp]
)]
struct CLI {
    /// Print verbose output.
    #[structopt(short, long)]
    verbose: bool,
    /// Update packages
    #[structopt(short, long)]
    update: bool,
    /// Package name to search for.
    package: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Packages {
    commit: String,
    packages: HashMap<String, Package>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Package {
    name: String,
    pname: String,
    version: String,
    system: String,
    meta: Meta,
}

#[derive(Debug, Deserialize, Serialize)]
struct Meta {
    available: bool,
    description: String,
    homepage: String,
    license: Vec<License>,
    maintainers: Vec<Maintainer>,
    position: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct License {
    free: Option<bool>,
    short_name: String,
    full_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Maintainer {
    email: String,
    github: String,
    name: String,
}

#[derive(Debug)]
struct NixSearch {
    cache_dir: PathBuf,
    package_cache: PathBuf,
}

impl NixSearch {
    fn new() -> Self {
        let proj_dir = ProjectDirs::from("com", "sondr3", "nix-search")
            .expect("Could not find project directory.");

        let cache_dir: PathBuf = proj_dir.cache_dir().into();
        let package_cache: PathBuf = [cache_dir.to_str().unwrap(), "pkgs.json"].iter().collect();

        NixSearch {
            cache_dir,
            package_cache,
        }
    }

    fn create(&self) {
        self.create_cache().unwrap();
        self.build_index();
    }

    fn cache_exists(&self) -> bool {
        self.cache_dir.exists() && self.package_cache.exists()
    }

    fn create_cache(&self) -> std::io::Result<()> {
        if !self.cache_exists() {
            std::fs::create_dir(&self.cache_dir)?;
        }

        Ok(())
    }

    fn build_index(&self) {
        let spinner = ProgressBar::new_spinner();
        spinner.enable_steady_tick(80);
        spinner.set_style(
            ProgressStyle::default_spinner()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏", ""])
                .template("{spinner} {msg:.green}"),
        );
        spinner.set_message("Building package index...");

        let mut file = File::create(&self.package_cache).unwrap();
        let command = Command::new("nix-env")
            .args(&["-f", "<nixpkgs>", "-qa", "--json"])
            .output()
            .unwrap();

        if !&command.status.success() {
            panic!("ohno");
        }

        file.write_all(&command.stdout).unwrap();
        spinner.finish_with_message("Updated!");
    }
}

fn main() -> Result<(), Error> {
    let cli = CLI::from_args();

    let search = NixSearch::new();
    if cli.update {
        search.build_index();
    } else if !search.cache_exists() {
        eprintln!("Cache directory missing, attempting to build index...");
        search.create();
    }

    Ok(())
}
