// https://www.unicode.org/charts/PDF/U2500.pdf
const HORIZONTAL: char = '\u{2500}'; // ─
const VERTICAL: char = '\u{2502}'; // │

const TOP_BRACE: char = '\u{252C}'; // ┬
const BOTTOM_BRACE: char = '\u{2534}'; // ┴
const LEFT_BRACE: char = '\u{251c}'; // ├
const RIGHT_BRACE: char = '\u{2524}'; // ┤
const MIDDLE_BRACE: char = '\u{253C}'; // ┤

const TOP_LEFT_CORNER: char = '\u{250c}'; // ┌
const TOP_RIGHT_CORNER: char = '\u{2510}'; // ┐
const BOTTOM_RIGHT_CORNER: char = '\u{2518}'; // ┘
const BOTTOM_LEFT_CORNER: char = '\u{2514}'; // └

pub fn table_to_string<'a, T: 'a>(input: Vec<Vec<T>>) -> String
where T: AsRef<str>,
      &'a T: AsRef<str> {
    let num_columns = {
        if let Some(row) = input.get(0) {
            row.len()
        }
        else {
            return String::new();
        }
    };

    let column_widths = {
        input
        .iter()
        .map(|row| row
            .into_iter()
            .map(|entry| entry.as_ref().len())
        )
        .fold(vec![0; num_columns], |mut column_widths, row| {
            for (a, b) in column_widths.iter_mut().zip(row) {
                // if b is bigger than the current column width, set the new column width to b
                *a = b.max(*a);
            }

            column_widths
        })
    };

    let total_width_per_row = {
        // one separator to the left of each one, as well as one separator on the very right
        let num_separators_per_row = num_columns + 1;

        let base_width_per_row: usize = column_widths.iter().sum();

        base_width_per_row + num_separators_per_row
    };

    let string_length = {
        let total_lines = {
            let num_rows = input.len();

            // separator above each row PLUS one under the last row
            let num_separator_lines = num_rows + 1;

            num_separator_lines + num_rows
        };

        // no newline after the last line
        let num_newlines = total_lines - 1;

        (total_width_per_row * total_lines) + num_newlines
    };

    let mut string = String::with_capacity(string_length);

    println!("String Length: {}", string_length);

    // TODO
    string
}
