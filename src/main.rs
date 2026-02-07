use include_dir::{Dir, include_dir};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self};
use std::path::PathBuf;

static PROJECT_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/typstFiles");

#[derive(Deserialize)]
pub struct Config {
    template: PathBuf,
    common: PathBuf,
    references: PathBuf,
    main: PathBuf,
}

fn main() {
    let mut config_dir = match get_config_dir() {
        Some(config_dir) => config_dir,
        None => panic!("No config dir found"),
    };
    println!("{}", config_dir.display());

    config_dir.push(r"typinit");
    let config = match config_dir.try_exists() {
        Ok(true) => read_config(&mut config_dir.join(r"config.toml")),
        Ok(false) => default_config(),
        Err(e) => panic!("Failed to check for config dir. Error: {e}"),
    };

    println!("Template: {}", config.template.display());
    println!("Common: {}", config.common.display());
    println!("References: {}", config.references.display());
    println!("Main: {}", config.main.display());

    println!("Enter the name of project folder: ");
    let mut folder_name = String::new();

    io::stdin()
        .read_line(&mut folder_name)
        .expect("Failed to read Input");

    trim_newline(&mut folder_name);

    match fs::create_dir(&folder_name) {
        Ok(_) => println!("Created Typst project directory with name {}", &folder_name),
        Err(e) => println!("Failed to create project directory, {e}"),
    }
}

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

fn default_config() -> Config {
    // create ~/.config if not found and the projects subfolder.
    // generate the standard toml config file and the default typst files.
    //
    //let mut config_dir = get_config_dir();
    let mut config_dir: PathBuf = match get_config_dir() {
        Some(config_dir) => config_dir,
        None => panic!("A config directory was expected but not found"),
    };
    config_dir.push(r"typinit");
    let config_dir2 = config_dir.clone();
    match fs::create_dir(&config_dir) {
        Ok(_) => println!("Created config dir in the appropriate config dir for the machine"),
        Err(e) => println!("Config dir already exists: {e}"),
    }
    config_dir.push(r"config.toml");
    match fs::File::create(&config_dir) {
        Ok(_) => println!("created config file"),
        Err(e) => println!("Error creating config file: {e}"),
    }

    let config_toml = format!(
        r#"
template = "{}/template.typ"
common = "{}/common.typ"
references = "{}/references"
main = "{}/main"
"#,
        &config_dir2.display(),
        &config_dir2.display(),
        &config_dir2.display(),
        &config_dir2.display()
    );

    match fs::write(&config_dir, config_toml.as_bytes()) {
        Ok(_) => println!("Default Config written to file"),
        Err(e) => println!("Could not write default config to file: {e}"),
    }

    // write default typs files
    write_default_files();

    read_config(&mut config_dir)
}

/// Writes the default setup files to the config dir.
/// Gets the paths from the
fn write_default_files() {
    let mut config_dir: PathBuf = match get_config_dir() {
        Some(config_dir) => config_dir,
        None => panic!("A config directory was expected but not found"),
    };
    config_dir.push(r"typinit");
    for file in PROJECT_DIR.files() {
        let path = config_dir.join(file.path());
        match fs::write(path, file.contents()) {
            Ok(_) => println!("Typst files moved"),
            Err(e) => println!("Something went wrong moving the typst files: {e}"),
        };
    }
}

/// Funciton to set the appropriate paths for the needed files.
fn setup() {
    println!("Setup Funciton - TODO");
}

/// reads the config file an populates the config struct
fn read_config(path: &mut PathBuf) -> Config {
    //path.join(r"config.toml");
    let config_content = match fs::read_to_string(&path) {
        Ok(config_content) => config_content,
        Err(e) => panic!("Error reading config file: {e}"),
    };
    let config = match toml::from_str(&config_content) {
        Ok(config) => config,
        Err(e) => panic!("Error deserializing config: {e}"),
    };
    return config;
}

/// Copies the template files from the location specified in
/// the config file(struct) to the created project folder.
fn copy_files() {
    println!("Copy Files - TODO");
}

fn get_config_dir() -> Option<PathBuf> {
    dirs::config_dir()
}
