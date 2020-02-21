use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
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
    #[structopt(short, long)]
    /// Print verbose output.
    verbose: bool,
    /// Package name to search for.
    package: String,
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

impl Packages {
    fn build_index() {
        let spinner = ProgressBar::new_spinner();
        spinner.enable_steady_tick(80);
        spinner.set_style(
            ProgressStyle::default_spinner()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏", ""])
                .template("{spinner} {msg:.green}"),
        );
        spinner.set_message("Building package index...");

        let mut file = File::create("pkgs.json").unwrap();
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
    println!("{:?}", cli);

    Packages::build_index();

    Ok(())
}
