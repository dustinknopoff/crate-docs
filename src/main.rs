use std::collections::HashSet;
use std::fs::File;

use cargo_toml::Manifest;

fn main() {
    use cargo_lock::Lockfile;
    let lockfile = Lockfile::load("Cargo.lock").unwrap();
    let manifest = Manifest::from_path("Cargo.toml").unwrap();
    let keys_to_find: HashSet<&String> = {
        let mut result: HashSet<&String> = manifest.dependencies.keys().collect();
        result.remove(&"csv".to_string());
        result.remove(&"cargo-lock".to_string());
        result.remove(&"cargo_toml".to_string());
        result
    };

    let pkgs: Vec<_> = lockfile
        .packages
        .iter()
        .filter(|pkg| keys_to_find.contains(&pkg.name.as_str().to_string()))
        .map(|pkg| (pkg.name.as_str(), pkg.version.to_string()))
        .collect();
    let file = File::create("crates.txt").unwrap();
    let mut writer = csv::Writer::from_writer(file);
    for (name, version) in pkgs.iter() {
        writer.write_record(&[name, version.as_str()]).unwrap();
    }
    writer.flush().unwrap();
}
