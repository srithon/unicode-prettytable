Rust table formatting library using Unicode Box-drawing characters.

[![Crates.io](https://img.shields.io/crates/v/unicode-prettytable)](https://crates.io/crates/unicode-prettytable)
[![docs.rs](https://img.shields.io/docsrs/unicode-prettytable)](https://docs.rs/unicode-prettytable)

**NOTE**: the version that was previously `1.0.0` has become `0.3.0`.

## Example usage

```rust
use unicode_prettytable::*;

fn main() -> Result<(), String> {
    let input = vec![
        vec!["oh hello there", "hello there", "hello"],
        vec!["hello there", "oh hello there", "hello"],
        vec!["oh hello there", "hello", "hello there"],
        vec!["oh hello there", "hello there", "hello"],
        vec!["oh hello there", "hello there", "hello"],
    ];

    // uses double bar characters for header and centers text within columns
    let table1 = TableBuilder::default()
        .header(
            HeaderBuilder::default()
                .double_bar(true)
                .centered_text(true)
                .build()?
        )
        .rows(&input)
        .build()?;

    // does not use double bar characters for header, but centers header text within columns
    let table2 = TableBuilder::default()
        .header(
            HeaderBuilder::default()
                .double_bar(false)
                .centered_text(true)
                .build()?
        )
        .rows(&input)
        .build()?;

    // uses default header settings
    let table3 = TableBuilder::default()
        .rows(&input)
        .build()?;

    println!("{}\n", table1);
    println!("{}\n", table2);
    println!("{}", table3);

    Ok(())
}
```

The code above outputs
```
╒══════════════════╤═══════════════╤═══════════╕
│  oh hello there  │  hello there  │   hello   │
╞══════════════════╪═══════════════╪═══════════╡
│hello there       │oh hello there │hello      │
├──────────────────┼───────────────┼───────────┤
│oh hello there    │hello          │hello there│
├──────────────────┼───────────────┼───────────┤
│oh hello there    │hello there    │hello      │
├──────────────────┼───────────────┼───────────┤
│oh hello there    │hello there    │hello      │
└──────────────────┴───────────────┴───────────┘

┌──────────────────┬───────────────┬───────────┐
│  oh hello there  │  hello there  │   hello   │
├──────────────────┼───────────────┼───────────┤
│hello there       │oh hello there │hello      │
├──────────────────┼───────────────┼───────────┤
│oh hello there    │hello          │hello there│
├──────────────────┼───────────────┼───────────┤
│oh hello there    │hello there    │hello      │
├──────────────────┼───────────────┼───────────┤
│oh hello there    │hello there    │hello      │
└──────────────────┴───────────────┴───────────┘

┌──────────────┬──────────────┬───────────┐
│oh hello there│hello there   │hello      │
├──────────────┼──────────────┼───────────┤
│hello there   │oh hello there│hello      │
├──────────────┼──────────────┼───────────┤
│oh hello there│hello         │hello there│
├──────────────┼──────────────┼───────────┤
│oh hello there│hello there   │hello      │
├──────────────┼──────────────┼───────────┤
│oh hello there│hello there   │hello      │
└──────────────┴──────────────┴───────────┘
```

## License
[License]: #license

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0), or
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### License of your contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
