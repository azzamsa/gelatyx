use std::io::{self, Write};

pub fn stdout(input: &str) {
    writeln!(io::stdout(), "{input}").ok();
}

pub fn stderr(input: &str) {
    writeln!(io::stderr(), "{input}").ok();
}

#[macro_export]
macro_rules! paint {
    ($input:expr,$style:expr) => {{
        $input
            .if_supports_color(Stdout, |s| s.style($style))
            .to_string()
    }};
}

#[macro_export]
macro_rules! epaint {
    ($input:expr,$style:expr) => {{
        $input
            .if_supports_color(Stderr, |s| s.style($style))
            .to_string()
    }};
}
