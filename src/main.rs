type Error = Box<dyn std::error::Error>;

#[derive(Debug)]
struct Config {
    url: String,
    package_path: String,
    packages: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            url: "https://nixos.org/nixpkgs/".into(),
            package_path: "packages-channels.json".into(),
            packages: vec![],
        }
    }
}

impl Config {
    fn get_packages(mut self) -> Self {
        let resp: Vec<String> = reqwest::get(&format!("{}{}", self.url, self.package_path))
            .expect("Could not fetch packages.")
            .json()
            .expect("Could not deserialize JSON response.");

        self.packages = resp;
        self
    }
}

fn main() -> Result<(), Error> {
    let config = Config::default().get_packages();

    println!("{:?}", &config);

    Ok(())
}
