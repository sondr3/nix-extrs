use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
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

#[derive(Debug, Eq, PartialEq, Hash)]
enum NixOS {
    STABLE,
    UNSTABLE,
    NIXPKGS,
}

impl FromStr for NixOS {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "unstable" => Ok(NixOS::UNSTABLE),
            "nixos-unstable" => Ok(NixOS::UNSTABLE),
            "nixpkgs-unstable" => Ok(NixOS::NIXPKGS),
            "nixpkgs" => Ok(NixOS::NIXPKGS),
            "stable" => Ok(NixOS::STABLE),
            s if s.starts_with("nixos-") => Ok(NixOS::STABLE),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Config {
    url: String,
    package_path: String,
    packages: HashMap<NixOS, String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            url: "https://nixos.org/nixpkgs/".into(),
            package_path: "packages-channels.json".into(),
            packages: HashMap::new(),
        }
    }
}

impl Config {
    fn get_package_list(mut self) -> Self {
        let resp: Vec<String> = reqwest::get(&format!("{}{}", self.url, self.package_path))
            .expect("Could not fetch packages.")
            .json()
            .expect("Could not deserialize JSON response.");

        for version in resp {
            let nix = NixOS::from_str(&version).expect("Could not parse NixOS version");
            self.packages.insert(nix, version);
        }

        self
    }
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
    license: License,
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
    fn new() -> Self {
        Packages {
            commit: "".into(),
            packages: HashMap::new(),
        }
    }
}

fn main() -> Result<(), Error> {
    let config = Config::default().get_package_list();
    let cli = CLI::from_args();

    println!("{:?}", cli);
    println!("{:?}", &config);

    Ok(())
}
