use crate::util::StringBuffer;

// https://www.unicode.org/charts/PDF/U2500.pdf
const VERTICAL: &str = "\u{2502}"; // │

const HORIZONTAL: &str = "\u{2500}"; // ─
const HORIZONTAL_HEADER: &str = "\u{2550}"; // ═

const TOP_BRACE: &str = "\u{252C}"; // ┬
const TOP_BRACE_HEADER: &str = "\u{2564}"; // ╤

const BOTTOM_BRACE: &str = "\u{2534}"; // ┴
const LEFT_BRACE: &str = "\u{251c}"; // ├
const LEFT_BRACE_HEADER: &str = "\u{255e}"; // ╞
const RIGHT_BRACE: &str = "\u{2524}"; // ┤
const RIGHT_BRACE_HEADER: &str = "\u{2561}"; // ╡
const MIDDLE_BRACE: &str = "\u{253C}"; // ┼
const MIDDLE_BRACE_HEADER: &str = "\u{256a}"; // ╪

const TOP_LEFT_CORNER: &str = "\u{250c}"; // ┌
const TOP_RIGHT_CORNER: &str = "\u{2510}"; // ┐

const TOP_LEFT_CORNER_HEADER: &str = "\u{2552}"; // ╒
const TOP_RIGHT_CORNER_HEADER: &str = "\u{2555}"; // ╕

const BOTTOM_RIGHT_CORNER: &str = "\u{2518}"; // ┘
const BOTTOM_LEFT_CORNER: &str = "\u{2514}"; // └

/// Creates a list of byte vectors corresponding to each string of horizontal separators
fn generate_horizontal_separators(column_widths: &Vec<usize>, horizontal_char: &str) -> Vec<Vec<u8>> {
    column_widths
        .iter()
        .map(|&length| horizontal_char.repeat(length).into_bytes())
        .collect::<Vec<_>>()
}

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

pub fn table_to_string<'a, T: 'a>(input: Vec<Vec<T>>, first_row_header: bool) -> String
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

    let standard_horizontal_separators = generate_horizontal_separators(&column_widths, HORIZONTAL);
    let header_horizontal_separators = generate_horizontal_separators(&column_widths, HORIZONTAL_HEADER);

    let create_sep_row = |horizontal_separators: &Vec<Vec<u8>>, left_char, middle_char, right_char, newline| {
        let mut sep_buffer = {
            // allocate one extra byte if there is a newline at the end
            let newline_increment = if newline { 1 } else { 0 };
            let capacity = total_width_per_row + newline_increment;
            StringBuffer::with_capacity(capacity)
        };

        sep_buffer.push_chars(left_char);

        let (last_sep, seps) = horizontal_separators.split_last().unwrap();

        for sep in seps {
            sep_buffer.push_bytes(sep);
            sep_buffer.push_chars(middle_char);
        }

        sep_buffer.push_bytes(&last_sep);
        sep_buffer.push_chars(right_char);

        if newline {
            sep_buffer.push_chars("\n")
        }

        sep_buffer.into_buffer()
    };

    let mut input_iterator = input.into_iter().peekable();

    macro_rules! push_data_row {
        () => {
            if let Some(row) = input_iterator.next() {
                buffer.push_chars(VERTICAL);
                for (col_index, col) in row.into_iter().enumerate() {
                    buffer.push_chars_fixed_width(col.as_ref(), column_widths[col_index]);
                    buffer.push_chars(VERTICAL);
                }

                buffer.push_chars("\n");
            }
        }
    }

    if first_row_header {
        let header_top = create_sep_row(&header_horizontal_separators, TOP_LEFT_CORNER_HEADER, TOP_BRACE_HEADER, TOP_RIGHT_CORNER_HEADER, true);
        buffer.push_bytes(&header_top);

        push_data_row!();

        let header_bottom = create_sep_row(&header_horizontal_separators, LEFT_BRACE_HEADER, MIDDLE_BRACE_HEADER, RIGHT_BRACE_HEADER, true);
        buffer.push_bytes(&header_bottom);
    }
    else {
        let header = create_sep_row(&standard_horizontal_separators, TOP_LEFT_CORNER, TOP_BRACE, TOP_RIGHT_CORNER, true);
        // add header
        buffer.push_bytes(&header);
    }

    let standard_separator = create_sep_row(&standard_horizontal_separators, LEFT_BRACE, MIDDLE_BRACE, RIGHT_BRACE, true);
    let footer = create_sep_row(&standard_horizontal_separators, BOTTOM_LEFT_CORNER, BOTTOM_BRACE, BOTTOM_RIGHT_CORNER, false);

    loop {
        push_data_row!();

        // only create a standard separator row if there are more data rows
        if input_iterator.peek().is_some() {
            buffer.push_bytes(&standard_separator);
        }
        else {
            // break out if there are no more data rows
            break;
        }
    }

    // add footer
    buffer.push_bytes(&footer);

    buffer.to_string()
}
