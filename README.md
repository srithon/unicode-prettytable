Rust table formatting library using Unicode Box-drawing characters.

## Example usage

```rust
use unicode_prettytable::table;

fn main() {
    let input = vec![
        vec!["oh hello there", "hello there", "hello"],
        vec!["hello there", "oh hello there", "hello"],
        vec!["oh hello there", "hello", "hello there"],
        vec!["oh hello there", "hello there", "hello"],
        vec!["oh hello there", "hello there", "hello"],
    ];

    println!("{}", table::table_to_string(input));
}
```

The code above outputs
```
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

