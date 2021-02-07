Rust table formatting library using Unicode Box-drawing characters.

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

