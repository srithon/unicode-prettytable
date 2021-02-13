use unicode_prettytable::*;

fn main() -> Result<(), String> {
    let header = vec![
        "name",
        "age",
        "height (in)",
        "weight (lb)"
    ];

    let rows = vec![
        vec!["Dave", "14", "72", "164"],
        vec!["Joshua", "24", "65", "141"],
        vec!["Stacie", "22", "61", "124"],
        vec!["Catherine", "30", "68", "130"],
        vec!["Jack", "8", "58", "78"],
    ];

    let table1 = TableBuilder::default()
        .header(
            HeaderBuilder::default()
                .double_bar(true)
                .centered_text(true)
                .columns(&header)
                .build()?
        )
        .rows(&rows)
        .build()?;

    println!("{}", table1);

    Ok(())
}
