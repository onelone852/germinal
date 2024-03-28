# Germinal

A gleam (not pure-gleam), terminal manipulation library based on [crossterm](https://github.com/crossterm-rs/crossterm) and [rustler](https://github.com/rusterlium/rustler). Currently, the functionality is not complete and unsafe to use. This library can only be run in erlang VM, but the support of Javascript would be considered in the future.

To see the source code, see [Github](https://github.com/onelone852/germinal).

[![Package Version](https://img.shields.io/hexpm/v/germinal)](https://hex.pm/packages/germinal)
[![Hex Docs](https://img.shields.io/badge/hex-docs-ffaff3)](https://hexdocs.pm/germinal/)

```sh
gleam add germinal
```

```gleam
import germinal

pub fn main() {
    use _ <- result.try(germinal.enable_raw_mode())
    use res <- result.try(germinal.read())
    io.debug(res)
    use _ <- result.try(germinal.disable_raw_mode())
    Ok(Nil)
}
```

Further documentation can be found at <https://hexdocs.pm/germinal>.

## Development

```sh
gleam run   # Run the project
gleam test  # Run the tests
gleam shell # Run an Erlang shell
```
