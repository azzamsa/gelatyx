use std::{fs, path::PathBuf};

use regex::{Captures, Regex};
use stylua_lib::{Config as LuaConfig, Error as StyluaError, OutputVerification, format_code};

use crate::{Error, config::Config};

use super::{FormatResult, SyntaxError};

pub fn format_lua(content: &str, config: &Config) -> Result<FormatResult, Error> {
    let mut format_result = FormatResult::Unchanged;
    let re = Regex::new(
        r"(?xms)
           (?P<before>^```\s*lua\n)
           (?P<code>.*?)
           (?P<after>^```$)
           ",
    )?;

    let language_config = match &config.language_config {
        Some(path) => load_custom_config(path.clone())?,
        None => LuaConfig::default(),
    };

    let new_content = re.replace_all(content, |capture: &Captures<'_>| {
        let code = &capture["code"];
        let new_code_or_old: Option<String> =
            match format_code(code, language_config, None, OutputVerification::None) {
                Ok(c) => Some(c),
                Err(error) => {
                    format_result = parse_error(code, error);
                    None
                }
            };
        let new_code_or_old = new_code_or_old.unwrap_or_else(|| code.into());
        let new_code_block = format!(
            "{}{}{}",
            &capture["before"], new_code_or_old, &capture["after"]
        );
        new_code_block
    });

    if content != new_content {
        format_result = FormatResult::Formatted(new_content.to_string());
    }

    Ok(format_result)
}

pub fn load_custom_config(path: PathBuf) -> Result<LuaConfig, Error> {
    let content = fs::read_to_string(&path).map_err(|_| Error::ConfigNotFound { path })?;
    toml::from_str(&content).map_err(|e| Error::InvalidConfig {
        message: e.to_string(),
    })
}

fn parse_error(code_block: &str, error: stylua_lib::Error) -> FormatResult {
    let mut syntax_errors: Vec<SyntaxError> = Vec::new();

    match &error {
        StyluaError::ParseError(errors) => {
            for e in errors {
                let error = SyntaxError {
                    position: e.range(),
                    code_block: code_block.to_string(),
                    message: e.to_string(),
                    summary: e.error_message().to_string(),
                };
                syntax_errors.push(error);
            }
        }
        StyluaError::VerificationAstError(_) => {}
        StyluaError::VerificationAstDifference => {}
    }

    FormatResult::InvalidSyntax(syntax_errors)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_config() -> Config {
        Config::default()
    }

    #[test]
    fn complex() -> Result<(), Error> {
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

        let expected = r#"
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
        let format_result = format_lua(input, &config)?;
        if let FormatResult::Formatted(result) = format_result {
            assert_eq!(expected, result);
        }

        Ok(())
    }

    #[test]
    fn one_line() -> Result<(), Error> {
        let input = r#"

# Document Title

first line

`local foo=require("bar")`

second line
"#;

        let expected = r#"

# Document Title

first line

`local foo=require("bar")`

second line
"#;

        let config = dummy_config();
        let format_result = format_lua(input, &config)?;
        if let FormatResult::Formatted(result) = format_result {
            assert_eq!(expected, result);
        }

        Ok(())
    }
}
