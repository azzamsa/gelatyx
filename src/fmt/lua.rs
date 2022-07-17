use std::str::FromStr;

use ansi_term::Colour::Red;
use regex::{Captures, Regex};
#[cfg(feature = "lua")]
use stylua_lib::{format_code, Config, OutputVerification};

use crate::Error;

/// Language choices
#[derive(Debug)]
pub enum Lang {
    Lua,
}

impl FromStr for Lang {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "lua" => Ok(Self::Lua),
            _ => Err("language not supported"),
        }
    }
}

#[cfg(feature = "lua")]
pub fn format_lua(content: &str) -> Result<String, Error> {
    let re = Regex::new(
        r"(?xms)
           (?P<before>^```lua\n)
           (?P<code>.*?)
           (?P<after>^```$)
           ",
    )?;

    let new_content = re.replace_all(content, |capture: &Captures<'_>| {
        let code = &capture["code"];
        let new_code_or_old = format_code(code, Config::default(), None, OutputVerification::None)
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
    use crate::error::Result;

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

"#;

        assert_eq!(output, format_lua(input)?);

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

        assert_eq!(output, format_lua(input)?);

        Ok(())
    }
}
