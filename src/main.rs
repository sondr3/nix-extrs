use std::collections::HashMap;
use std::str::FromStr;

type Error = Box<dyn std::error::Error>;

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
            "nixos-unstable" => Ok(NixOS::UNSTABLE),
            "nixpkgs-unstable" => Ok(NixOS::NIXPKGS),
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
    fn get_packages(mut self) -> Self {
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

fn main() -> Result<(), Error> {
    let config = Config::default().get_packages();

    println!("{:?}", &config);

    Ok(())
}
