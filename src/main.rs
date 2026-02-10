use crossterm::{
    cursor::{MoveToColumn, MoveUp},
    execute,
    terminal::{Clear, ClearType},
};
use include_dir::{Dir, include_dir};
use serde::{Deserialize, Serialize};
use std::io::stdout;
use std::io::{self};
use std::path::PathBuf;
use std::{collections::HashMap, fs};

static PROJECT_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/typstFiles");

#[derive(Deserialize, Serialize)]
pub struct Config {
    default: Default,
    extra: HashMap<String, PathBuf>,
}

#[derive(Deserialize, Serialize)]
pub struct Default {
    template: PathBuf,
    common: PathBuf,
    references: PathBuf,
    main: PathBuf,
}

fn main() {
    execute!(stdout(), Clear(ClearType::All)).unwrap();
    print_name();
    // get the right config dir for the platform
    let mut config_dir = match get_config_dir() {
        Some(config_dir) => config_dir,
        None => panic!("No config dir found"),
    };

    // create dir for typinit in the config dir
    config_dir.push(r"typinit");
    let config = match config_dir.try_exists() {
        Ok(true) => read_config(&mut config_dir.join(r"config.toml")),
        Ok(false) => default_config(),
        Err(e) => panic!("Failed to check for config dir. Error: {e}"),
    };

    println!("1. New Typst project in current directory");
    println!("2. Change template paths");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let input: isize = input.trim().parse().expect("Invalid input");
    config_dir.push(r"config.toml");
    match input {
        1 => new_project(config),
        2 => setup(&config_dir),
        _ => panic!("Non-valid Input!"),
    }

    let config = read_config(&mut config_dir);
    write_config(&config, &config_dir);
}

fn new_project(config: Config) {
    println!("Enter the name of project folder: ");
    let mut folder_name = String::new();

    // create project dir in the current dir
    io::stdin()
        .read_line(&mut folder_name)
        .expect("Failed to read Input");

    trim_newline(&mut folder_name);

    match fs::create_dir(&folder_name) {
        Ok(_) => println!("Created Typst project directory with name {}", &folder_name),
        Err(e) => println!("Failed to create project directory, {e}"),
    };

    let project_folder = PathBuf::from(&folder_name);
    copy_files(config, project_folder);
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

    // get config dir
    let mut config_dir: PathBuf = match get_config_dir() {
        Some(config_dir) => config_dir,
        None => panic!("A config directory was expected but not found"),
    };

    // create typinit dir
    config_dir.push(r"typinit");
    let config_dir2 = config_dir.clone();
    if let Err(e) = fs::create_dir(&config_dir) {
        eprintln!("Config dir already exists: {e}");
    }

    // create config.toml file
    config_dir.push(r"config.toml");
    if let Err(e) = fs::File::create(&config_dir) {
        eprintln!("Error creating config.toml file: {e}");
    }

    // populate config.toml
    let config_toml = format!(
        r#"[default]
template = "{}/template.typ"
common = "{}/common.typ"
references = "{}/references.bib"
main = "{}/main.typ"

[extra]

"#,
        &config_dir2.display(),
        &config_dir2.display(),
        &config_dir2.display(),
        &config_dir2.display()
    );
    if let Err(e) = fs::write(&config_dir, config_toml.as_bytes()) {
        eprintln!("Could not write default config to file: {e}");
    }

    // write default typst files to config dir
    write_default_files();

    // read config file into the struct
    read_config(&mut config_dir)
}

/// Writes the default setup files to the config dir.
/// Gets the paths from the
fn write_default_files() {
    // get right config dir
    let mut config_dir: PathBuf = match get_config_dir() {
        Some(config_dir) => config_dir,
        None => panic!("A config directory was expected but not found"),
    };

    // write typst files from the project to the config dir
    config_dir.push(r"typinit");
    for file in PROJECT_DIR.files() {
        let path = config_dir.join(file.path());
        if let Err(e) = fs::write(path, file.contents()) {
            eprintln!("Something went wrong moving the typst files: {e}");
        }
    }
}

/// Function to set the appropriate paths for the needed files.
fn setup(path: &PathBuf) {
    execute!(stdout(), Clear(ClearType::All)).unwrap();
    print_name();

    let mut config_str: Vec<String> = Vec::new();
    let mut extra_hashmap: HashMap<String, PathBuf> = HashMap::new();

    let paths = ["template.typ", "common.typ", "references.bib", "main.typ"];
    for file in paths {
        println!("Enter the absolute path to your {file} file: ");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read Input");

        trim_newline(&mut input);
        config_str.push(input);
        execute!(
            stdout(),
            MoveUp(2),
            MoveToColumn(0),
            Clear(ClearType::FromCursorDown)
        )
        .unwrap();
    }
    println!("Do you want to add more files? y/n");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let nr_file: isize = if input.trim() == "yes" || input.trim() == "y" || input.trim() == "Yes" {
        println!("How many files do you want to add?");
        let mut file_in = String::new();
        io::stdin()
            .read_line(&mut file_in)
            .expect("Failed to read input");
        file_in.trim().parse().expect("Invalid input")
    } else if input.trim() == "no" || input.trim() == "n" || input.trim() == "No" {
        println!("No extra files added");
        0
    } else {
        println!("Invalid input. No extra files added.");
        0
    };

    if nr_file > 0 {
        for i in 1..nr_file + 1 {
            println!("Enter the name of your extra file {i}, including file type: ");

            let mut name = String::new();
            io::stdin()
                .read_line(&mut name)
                .expect("Failed to read Input");

            println!("Enter the absolute path to your extra file {i}: ");
            let mut path = String::new();
            io::stdin()
                .read_line(&mut path)
                .expect("Failed to read Input");

            trim_newline(&mut name);
            trim_newline(&mut path);

            let path = PathBuf::from(path);

            extra_hashmap.insert(name, path);

            execute!(
                stdout(),
                MoveUp(2),
                MoveToColumn(0),
                Clear(ClearType::FromCursorDown)
            )
            .unwrap();
        }
    }

    let default = Default {
        template: PathBuf::from(&config_str[0]),
        common: PathBuf::from(&config_str[1]),
        references: PathBuf::from(&config_str[2]),
        main: PathBuf::from(&config_str[3]),
    };

    let config = Config {
        default,
        extra: extra_hashmap,
    };

    write_config(&config, path);
}

/// reads the config.toml file an populates the config struct
fn read_config(path: &mut PathBuf) -> Config {
    let config_content = match fs::read_to_string(path) {
        Ok(config_content) => config_content,
        Err(e) => panic!("Error reading config file: {e}"),
    };
    match toml::from_str(&config_content) {
        Ok(config) => config,
        Err(e) => panic!("Error deserializing config: {e}"),
    }
}

/// writes the passed config struct to the config.toml file
fn write_config(config: &Config, path: &PathBuf) {
    if let Ok(config_toml) = toml::to_string(&config) {
        if let Err(e) = fs::write(path, config_toml.as_bytes()) {
            eprintln!("Failed to write to config.toml file: {e}");
        };
    } else {
        eprintln!("Error writing config to config.toml:")
    };
}

/// Copies the template files from the location specified in
/// the config file(struct) to the created project folder.
fn copy_files(config: Config, path: PathBuf) {
    if let Err(e) = fs::copy(config.default.template, path.join("template.typ")) {
        eprintln!("Failed to copy your template: {e}");
    }
    if let Err(e) = fs::copy(config.default.common, path.join("common.typ")) {
        eprintln!("Failed to copy your common file: {e}");
    }
    if let Err(e) = fs::copy(config.default.references, path.join("references.bib")) {
        eprintln!("Failed to copy your reference file: {e}");
    }
    if let Err(e) = fs::copy(config.default.main, path.join("main.typ")) {
        eprintln!("Failed to copy your main file: {e}");
    }

    for (name, file_path) in config.extra {
        if let Err(e) = fs::copy(file_path, path.join(&name)) {
            eprintln!("Failed to copy your {} file: {e}", &name);
        }
    }
}

fn get_config_dir() -> Option<PathBuf> {
    dirs::config_dir()
}

fn print_name() {
    println!(
        r"
  _______          _       _ _   
 |__   __|        (_)     (_) |  
    | |_   _ _ __  _ _ __  _| |_ 
    | | | | | '_ \| | '_ \| | __|
    | | |_| | |_) | | | | | | |_ 
    |_|\__, | .__/|_|_| |_|_|\__|
        __/ | |                  
       |___/|_|                      
       ",
    )
}
