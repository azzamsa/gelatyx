use std::{fs, path::PathBuf};

use ansi_term::Colour::Red;
use regex::{Captures, Regex};
use stylua_lib::{format_code, Config as LuaConfig, OutputVerification};

use crate::{config::Config, Error};

pub fn load_custom_config(path: PathBuf) -> Result<LuaConfig, Error> {
    let content = fs::read_to_string(&path).map_err(|_| Error::ConfigNotFound { path })?;
    toml::from_str(&content).map_err(|e| Error::InvalidConfig {
        message: e.to_string(),
    })
}

pub fn format_lua(content: &str, config: &Config) -> Result<String, Error> {
    let re = Regex::new(
        r"(?xms)
           (?P<before>^```\s*lua\n)
           (?P<code>.*?)
           (?P<after>^```$)
           ",
    )?;

    let language_config = match &config.language_config {
        Some(path) => load_custom_config(path.to_path_buf())?,
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
    use super::*;

    fn dummy_config() -> Config {
        Config::default()
    }

    #[test]
    fn compex() -> Result<(), Error> {
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
    fn one_line() -> Result<(), Error> {
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
