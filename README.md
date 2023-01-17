<div align="center">
  <h1>Gelatyx</h1>

<img src='docs/bird.svg' width=80px />

Format codebase in documentation.

<a href="https://github.com/azzamsa/gelatyx/workflows/ci.yml">
    <img src="https://github.com/azzamsa/gelatyx/workflows/ci/badge.svg" alt="Build status" />
  </a>

<a href="https://crates.io/crates/gelatyx">
    <img src="https://img.shields.io/crates/v/gelatyx.svg">
  </a>

<a href=" https://docs.rs/gelatyx/">
    <img src="https://docs.rs/gelatyx/badge.svg">
  </a>

<a href="https://azzamsa.com/support/">
    <img alt="Sponsor me" src="https://img.shields.io/badge/Sponsor%20Me-%F0%9F%92%96-ff69b4">
  </a>

<p><p/>

![demo](https://user-images.githubusercontent.com/17734314/212819885-463cf1d9-a66a-4195-8d5e-f5fdfda5edcb.gif)

</div>

---

## Features

- Format language code block inside documentation files.
- Check mode. Ask Gelatyx if the documentation has been formatted.
- Glob support.
- Configure the formatter via a configuration file.
- Fancy error message and colorful output.
- Cross-platform and single binary.
- Supported languages
  - Lua via [StyLua](https://github.com/JohnnyMorganz/StyLua)

## Usage

```bash
$ gelatyx --language lua input.md                                 Format a file with lua formatter
$ gelatyx --language lua *.md                                     Format multiple files ...
$ gelatyx --language lua input.md --check                         Check if the docs has been formatted.
$ gelatyx --language lua input.md --language-config config.toml   Configure the formatter.
```

## Integration with other formatter

If you like `gelatyx` to support your favorite formatter, feel free to open [new issue](https://github.com/azzamsa/gelatyx/issues/new).

## Installation

### From binaries

The [release page](https://github.com/azzamsa/gelatyx/releases) includes
pre-compiled binaries for GNU/Linux, macOS, and Windows.

### From source

Using [cargo-binstall](https://github.com/cargo-bins/cargo-binstall)

```bash
$ cargo binstall gelatyx
```

Using Rust's package manager [cargo](https://github.com/rust-lang/cargo):

```bash
$ cargo install gelatyx
```

## Development

```bash
git clone https://github.com/azzamsa/gelatyx

# Build
cd gelatyx
cargo build

# Run unit tests and integration tests
cargo test

# Install
cargo install --path .
```

## Contributing

To learn more read [the development guide](docs/dev/README.md)

## Origin of the name

The name Gelatyx is a played version of [Gelatik](https://id.wikipedia.org/wiki/Gelatik). A beautiful bird from Indonesia.

## Credits

- [Anthony Sottile's blacken-docs](https://github.com/asottile/blacken-docs)
- [Noto Emoji](https://github.com/googlefonts/noto-emoji)
