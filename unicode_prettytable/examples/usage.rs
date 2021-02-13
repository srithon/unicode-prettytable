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

