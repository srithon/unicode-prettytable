use crate::util::StringBuffer;

// https://www.unicode.org/charts/PDF/U2500.pdf
const VERTICAL: &str = "\u{2502}"; // │

const HORIZONTAL: &str = "\u{2500}"; // ─

const TOP_BRACE: &str = "\u{252C}"; // ┬
const BOTTOM_BRACE: &str = "\u{2534}"; // ┴
const LEFT_BRACE: &str = "\u{251c}"; // ├
const RIGHT_BRACE: &str = "\u{2524}"; // ┤
const MIDDLE_BRACE: &str = "\u{253C}"; // ┤

const TOP_LEFT_CORNER: &str = "\u{250c}"; // ┌
const TOP_RIGHT_CORNER: &str = "\u{2510}"; // ┐
const BOTTOM_RIGHT_CORNER: &str = "\u{2518}"; // ┘
const BOTTOM_LEFT_CORNER: &str = "\u{2514}"; // └

fn get_column_widths<'a, T: 'a>(input: &Vec<Vec<T>>) -> Vec<usize>
where
    T: AsRef<str>,
    &'a T: AsRef<str>
{
    let num_columns = {
        if let Some(row) = input.get(0) {
            row.len()
        }
        else {
            return Vec::new();
        }
    };

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
}

pub fn table_to_string<'a, T: 'a>(input: Vec<Vec<T>>) -> String
where
    T: AsRef<str>,
    &'a T: AsRef<str>,
{
    let column_widths = get_column_widths(&input);

    let total_width_per_row = {
        // one separator to the left of each one, as well as one separator on the very right
        let num_separators_per_row = column_widths.len() + 1;

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

    // fill string with spaces
    let mut buffer = StringBuffer::with_capacity_fill(string_length, ' ');

    let horizontal_separators = {
        let horizontal_separator: String = HORIZONTAL.to_string();
        column_widths
            .iter()
            .map(|&length| (horizontal_separator.repeat(length).into_bytes(), length))
            .collect::<Vec<_>>()
    };

    macro_rules! push_sep_row {
        ($left_char:expr, $middle_char:expr, $right_char:expr, $newline:expr) => {{
            buffer.push_chars($left_char);

            let (last_sep, seps) = horizontal_separators.split_last().unwrap();

            // TODO remove length
            for (sep, _) in seps {
                buffer.push_bytes(sep);
                buffer.push_chars($middle_char);
            }

            buffer.push_bytes(&last_sep.0);
            buffer.push_chars($right_char);

            if $newline {
                buffer.push_chars("\n")
            }
        }}
    }

    let mut input_iterator = input.into_iter().peekable();

    // add header
    push_sep_row!(TOP_LEFT_CORNER, TOP_BRACE, TOP_RIGHT_CORNER, true);

    loop {
        if let Some(row) = input_iterator.next() {
            buffer.push_chars(VERTICAL);
            for (col_index, col) in row.into_iter().enumerate() {
                // NOTE
                // this is why we wanted a byte buffer
                // so we dont have to rely on the slow chars() implementation
                buffer.push_chars_fixed_width(col.as_ref(), column_widths[col_index]);
                buffer.push_chars(VERTICAL);
                // buffer.push_single_char(" ");
            }

            buffer.push_chars("\n");
        }

        // only create a standard separator row if there are more data rows
        if input_iterator.peek().is_some() {
            // TODO create a constant for this separator row
            push_sep_row!(LEFT_BRACE, MIDDLE_BRACE, RIGHT_BRACE, true);
        }
        else {
            // break out if there are no more data rows
            break;
        }
    }

    // add footer
    push_sep_row!(BOTTOM_LEFT_CORNER, BOTTOM_BRACE, BOTTOM_RIGHT_CORNER, false);

    buffer.to_string()
}
