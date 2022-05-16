# unicode-prettytable

`unicode-prettytable` is a toy project for creating tables in Rust code with Unicode box characters.
It was created primarily for the purpose of getting a crate on [crates.io](crates.io) and using it in another project.

With that in mind, this crate is _not_ meant to be used in production; for viable alternatives, please see the following crates:
1. [comfy-table](Nukesor/comfy-table) 
2. [prettytable-rs](https://github.com/phsym/prettytable-rs) 

## Limitations

As illustrated in [this issue](https://github.com/srithon/unicode-prettytable/issues/2), the current implementation does not support non-ASCII text within table fields. 
