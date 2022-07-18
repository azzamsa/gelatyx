use std::fs;

use ansi_term::Colour::Red;
use regex::{Captures, Regex};
use stylua_lib::{format_code, Config as LuaConfig, OutputVerification};

use crate::{config::Config, error, Error};

pub fn load_config(path: &str) -> error::Result<LuaConfig> {
    let contents = fs::read_to_string(path)?;
    toml::from_str(&contents).map_err(|_| Error::Msg("Config file not in correct format".into()))
}

pub fn format_lua(content: &str, config: &Config) -> Result<String, Error> {
    let re = Regex::new(
        r"(?xms)
           (?P<before>^```\s*lua\n)
           (?P<code>.*?)
           (?P<after>^```$)
           ",
    )?;

    let language_config = match config.language_config {
        Some(config_) => load_config(config_)?,
        None => LuaConfig::default(),
    };

    let new_content = re.replace_all(content, |capture: &Captures<'_>| {
        let code = &capture["code"];
        let new_code_or_old = format_code(code, language_config, None, OutputVerification::None)
            .unwrap_or_else(|e| {
                eprintln!("{}", Red.paint(e.to_string()));
                code.into()
            });
        let new_code_block = format!(
            "{}{}{}",
            &capture["before"], new_code_or_old, &capture["after"]
        );
        new_code_block
    });

    Ok(new_content.to_string())
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
