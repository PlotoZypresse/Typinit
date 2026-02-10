# Typinit

Typeinit is a tool to instantly create a typst project in your current directory. It creates a folder with a name you specify and copies the _template.typ, common.typ, references.bib and main.typ_ files into the created directory. By default it uses the files delivered with Typinit. You can specify the path to your desired files by either changing the paths in the config.toml file in _.config/typinit/_ (or platform appropriate) or by using the prompt when running Typinit.

## Download

```
git clone https://github.com/PlotoZypresse/Typinit.git
cargo install --path .
```

In the project directory then run

```
cargo install --path .
```

### Improvements

I want to add the ability to specify arbitrary many files so its easier for users to use their own setups.
