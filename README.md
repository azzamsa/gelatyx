<div align="center">
  <h1>Gelatyx</h1>

  <img src='docs/bird.svg' width=80px />

  Format codebase in documentation 🦤

  <a href="https://github.com/azzamsa/gelatyx/workflows/ci.yml">
    <img src="https://github.com/azzamsa/gelatyx/workflows/ci/badge.svg" alt="Build status" />
  </a>

  <a href="https://crates.io/crates/gelatyx">
    <img src="https://img.shields.io/crates/v/gelatyx.svg">
  </a>

  <a href="https://docs.rs/gelatyx/">
    <img src="https://docs.rs/gelatyx/badge.svg">
  </a>

  <a href="https://azzamsa.com/support/">
    <img alt="Sponsor me" src="https://img.shields.io/badge/Sponsor%20Me-%F0%9F%92%96-ff69b4">
  </a>

</div>

---

## Features

- Format language code block inside documentation files
- Check mode. Ask Gelatyx is the documentation has been formatted.
- Glob support.
- Configure the formatter via configuration file.
- Supported languages
  - Lua via [StyLua](https://github.com/JohnnyMorganz/StyLua)

## Usage

## Usage Examples

``` bash
$ gelatyx lua --file input.md                                 Format a file with lua formatter
$ gelatyx lua --file *.md                                     Format multiple files ...
$ gelatyx lua --file input.md --check                         Check if the docs has been formatted.
$ gelatyx lua --file input.md --language-config config.toml   Configure the formatter.
```

### Command-line options

``` bash
gelatyx [version] 
Gelatyx 🦤.

            Format codebease inside the docs

USAGE:
    gelatyx [OPTIONS] --file <file>... <LANGUAGE>

ARGS:
    <LANGUAGE>    Language used in code block. [possible values: lua]

OPTIONS:
    -f, --file <file>...    File(s) to format.
        --color <color>     When to use colors (*auto*, never, always). [default: auto] [possible
                            values: auto, never, always]
        --check             Check if the docs has been formatted.
        --language-config <language-config> Specify an alternate configuration file
    -h, --help              Print help information
    -V, --version           Print version information

Note: `gelatyx -h` prints a short and concise overview while `gelatyx --help` gives all details.
```


If you like `gelatyx` to support your favorite language, feel free to open new issue.

## Installation

### From binaries

The [release page](https://github.com/azzamsa/gelatyx/releases) includes
pre-compiled binaries for GNU/Linux, macOS and Windows.

### From source

Using Rust's package manager [cargo](https://github.com/rust-lang/cargo):

``` bash
$ cargo install gelatyx
```


## Development

``` bash
$ clone the repository 

$ # Run unit tests and integration tests
$ cargo test

$ # Install
$ cargo install --path .
```

## Origin of the name

The name Gelatyx is a played version of [Gelatik](https://id.wikipedia.org/wiki/Gelatik). A beautiful bird from Indonesia.

## Credits

- [Anthony Sottile's blacken-docs](https://github.com/asottile/blacken-docs) 
- [David Peter 's bat](https://github.com/sharkdp/bat) 
- [Noto Emoji](https://github.com/googlefonts/noto-emoji) 
