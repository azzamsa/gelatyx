use clap::{crate_version, Arg, Command};

pub fn build() -> Command<'static> {
    let app = Command::new("Gelatyx 🦤 Format codebease inside the docs")
        .arg_required_else_help(true)
        .version(crate_version!())
        .arg(
            Arg::new("lang")
                .required(true)
                .possible_values(&["lua"])
                .help("Language used in code block"),
        )
        .arg(
            Arg::new("file")
                .required(true)
                .short('f')
                .long("file")
                .takes_value(true)
                .multiple_values(true)
                .empty_values(false)
                .help("File(s) to format."),
        );

    app
}
