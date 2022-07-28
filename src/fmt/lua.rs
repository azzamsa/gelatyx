use std::{fs, path::Path};

use ansi_term::Colour::Red;
use full_moon::{ast::AstError::UnexpectedToken, Error as FullMoonError};
use regex::{Captures, Regex};
use stylua_lib::{format_code, Config as LuaConfig, Error::ParseError, OutputVerification};

use super::FormatCode;
use crate::{config::Config, error, Error};
pub fn load_config(path: &str) -> error::Result<LuaConfig> {
    let contents = fs::read_to_string(path)?;
    toml::from_str(&contents).map_err(|_| Error::Msg("Config file not in correct format".into()))
}
fn handle_error(file: &Path, capture: &str, error: stylua_lib::Error) {
    let (start_line, end_line) = match &error {
        ParseError(FullMoonError::AstError(UnexpectedToken {
            token,
            additional: _,
        })) => (
            Some(token.start_position().line()),
            Some(token.end_position().line()),
        ),
        _ => (None, None),
    };

    let code_to_print = match (start_line, end_line) {
        (Some(start), Some(end)) => {
            let mut lines = Vec::new();
            // line index starts from 0
            let start = start - 1;
            for line in start..=end {
                lines.push(capture.lines().nth(line));
            }
            Some(lines)
        }
        (_, _) => None,
    };
    eprintln!(
        "\nFailed to format {}\n{}\n",
        file.display(),
        Red.paint(&error.to_string()),
    );
    if let Some(code) = code_to_print {
        for line in code.into_iter().flatten() {
            println!("{}", line);
        }
        println!();
    };
}

pub fn format_lua(content: &str, config: &Config, file: &Path) -> Result<FormatCode, Error> {
    let re = Regex::new(
        r"(?xms)
           (?P<before>^```\s*lua\n)
           (?P<code>.*?)
           (?P<after>^```$)
           ",
    )?;

    let mut is_parse_failed = false;
    let language_config = match config.language_config {
        Some(config_) => load_config(config_)?,
        None => LuaConfig::default(),
    };

    let new_content = re.replace_all(content, |capture: &Captures<'_>| {
        let code = &capture["code"];
        let new_code_or_old = format_code(code, language_config, None, OutputVerification::None)
            .unwrap_or_else(|e| {
                handle_error(file, code, e);
                is_parse_failed = true;
                code.into()
            });
        let new_code_block = format!(
            "{}{}{}",
            &capture["before"], new_code_or_old, &capture["after"]
        );
        new_code_block
    });

    let result = FormatCode {
        content: new_content.to_string(),
        is_parse_failed,
    };

    Ok(result)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;
    use crate::{config::Mode, error::Result};

    fn dummy_config() -> Config<'static> {
        Config {
            language: "lua",
            files: [Path::new("")].to_vec(),
            colored_output: true,
            mode: Mode::Format,
            language_config: None,
        }
    }

    #[test]
    fn compex() -> Result<()> {
        let input = r#"
# Document Title

first line

`local foo=require("bar")`

second line

```lua
local foo=require("bar")
return {first}
```

third line

```
I am text
```

multiple lines,
multiple lines.

```python
return "python"
```


```lua
return {second}
```

```lua
return {third}
```

empty code block

```lua
```

```
```

``` lua
return {whitespace}
```

"#;

        let output = r#"
# Document Title

first line

`local foo=require("bar")`

second line

```lua
local foo = require("bar")
return { first }
```

third line

```
I am text
```

multiple lines,
multiple lines.

```python
return "python"
```


```lua
return { second }
```

```lua
return { third }
```

empty code block

```lua
```

```
```

``` lua
return { whitespace }
```

"#;
        let config = dummy_config();
        assert_eq!(output, format_lua(input, &config)?);

        Ok(())
    }

    #[test]
    fn one_line() -> Result<()> {
        let input = r#"

# Document Title

first line

`local foo=require("bar")`

second line
"#;

        let output = r#"

# Document Title

first line

`local foo=require("bar")`

second line
"#;

        let config = dummy_config();
        assert_eq!(output, format_lua(input, &config)?);

        Ok(())
    }
}
