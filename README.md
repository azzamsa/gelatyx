<div align="center">
<h1>Gelatyx</h1>
<img src='docs/bird.svg' width=80px/>

Format codebase in documentation 🦤

<a href="https://github.com/azzamsa/gelatyx/workflows/ci.yml">
    <img src="https://github.com/azzamsa/gelatyx/workflows/ci/badge.svg" alt="Build status" />
</a>

</div>

---

## Features

- Format language code block inside documentation files
- Supported languages
  - Lua via [StyLua](https://github.com/JohnnyMorganz/StyLua)

## Usage

## Usage Examples

``` bash
$ gelatyx lua --file input.md       Format a file with lua formatter
$ gelatyx lua --file *.md           Format multiple files ...
```

### Command-line options

``` bash
Gelatyx 🦤 Format codebease inside the docs

USAGE:
    gelatyx --file <file>... <lang>

ARGS:
    <lang>    Language used in code block [possible values: lua]

OPTIONS:
    -f, --file <file>...    File(s) to format.
    -h, --help              Print help information
    -V, --version           Print version information
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
