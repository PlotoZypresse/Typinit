# Typinit

Typinit is a tool to instantly create a Typst project, in form of a folder containing your default project files, in your current directory. It creates a folder with a specified name and copies the _template.typ, common.typ, references.bib and main.typ_ files into the created directory, additionally you can specify further files that also are added to the project directory. By default it uses the files delivered with Typinit. You can specify the path to your desired files by either changing the paths in the config.toml file in _.config/typinit/_ (or platform appropriate) or by using the prompt when running Typinit.

## Download

```
git clone https://github.com/PlotoZypresse/Typinit.git
cargo install --path .
```

In the project directory then run

```
cargo install --path .
```

## The Config file

Below is an example of the config.toml file that is created when running _typinit_ for the first time.

``` toml
[default]
template = "/absolute/path/to/template.typ"
common = "/absolute/path/to/common.typ"
references = "/absolute/path/to/references.bib"
main = "/absolute/path/to/main.typ"

[extra]
extra.txt = "path/to/additional/file.txt"
```

The directory containing the config.toml file is the platforms _./config_ folder. It can be changed either directly or through running _typinit_ in the command line.

### Improvements

Working on ideas.
