Rust table formatting library using Unicode Box-drawing characters.

![Crates.io](https://img.shields.io/crates/v/unicode-prettytable)
![docs.rs](https://img.shields.io/docsrs/unicode-prettytable)

## Example usage

```rust
use unicode_prettytable::table_to_string;

fn main() {
    let input = vec![
        vec!["oh hello there", "hello there", "hello"],
        vec!["hello there", "oh hello there", "hello"],
        vec!["oh hello there", "hello", "hello there"],
        vec!["oh hello there", "hello there", "hello"],
        vec!["oh hello there", "hello there", "hello"],
    ];

    // treats first row as fancy header with each header centered within the column
    println!("{}\n", table_to_string(input.clone(), true, true));

    // does not treat first row as fancy header and does not center header text
    println!("{}", table_to_string(input, false, false));
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
