use crate::util::StringBuffer;

use std::fmt;
use derive_builder::Builder;

// regular derive(Default) requires T: Default for Option<T>
use smart_default::SmartDefault;

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

pub struct Header {
#[derive(Builder, Clone, SmartDefault)]
    /// Whether to use double bar Unicode characters surrounding the header
    double_bar: bool,
    /// Whether to center the header text within each column
    centered_text: bool,
}

#[derive(Builder)]
pub struct Table<'a, T>
where
    T: AsRef<str>,
    &'a T: AsRef<str>,
{
    /// If you provide header settings, the first row will be treated as headers
    #[builder(default)]
    header: Header,
    /// Rows holding the data
    rows: &'a Vec<Vec<T>>,
}

impl <'a, T> Table<'a, T>
where
    T: AsRef<str>,
    &'a T: AsRef<str>
{
    /// Creates a list of byte vectors corresponding to each string of horizontal separators
    fn generate_horizontal_separators(column_widths: &Vec<usize>, horizontal_char: &str) -> Vec<Vec<u8>> {
        column_widths
            .iter()
            .map(|&length| horizontal_char.repeat(length).into_bytes())
            .collect::<Vec<_>>()
    }

    /// Given a 2D input, returns the minimum width of each column in a vector
    fn get_column_widths(&self) -> Vec<usize>
    where
        T: AsRef<str>,
        &'a T: AsRef<str>
    {
        let header_widths: Vec<usize> = {
            if let Some(row) = self.rows.get(0) {
                let header_padding = {
                    if self.header.centered_text {
                        2
                    }
                    else {
                        0
                    }
                };

                row.iter().map(|entry| entry.as_ref().len() + header_padding).collect()
            }
            else {
                return Vec::new();
            }
        };

        let mut widths = self.rows
            .iter()
            .map(|row| row
                .into_iter()
                .map(|entry| entry.as_ref().len())
            )
            .skip(1)
            .fold(header_widths.clone(), |mut column_widths: Vec<usize>, row| {
                for (a, b) in column_widths.iter_mut().zip(row) {
                    // if b is bigger than the current column width, set the new column width to b
                    *a = b.max(*a);
                }

                column_widths
            });

        if self.header.centered_text {
            // make sure that header has an even number of spaces on either side
            for (col_width, header_width) in widths.iter_mut().zip(header_widths.into_iter()) {
                let num_spaces = *col_width - header_width;

                // let there be a space on either side
                if num_spaces == 0 {
                    *col_width += 2;
                }
                // make it an even number of spaces
                else if num_spaces % 2 == 1 {
                    *col_width += 1;
                }
            }
        }

        widths
    }
}

impl <'a, T> fmt::Display for Table<'a, T>
where
    T: AsRef<str>,
    &'a T: AsRef<str>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let column_widths = self.get_column_widths();

        let total_width_per_row = {
            // one separator to the left of each one, as well as one separator on the very right
            let num_separators_per_row = column_widths.len() + 1;

            let base_width_per_row: usize = column_widths.iter().sum();

            base_width_per_row + num_separators_per_row
        };

        let string_length = {
            let total_lines = {
                let num_rows = self.rows.len();

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

        let standard_horizontal_separators = Self::generate_horizontal_separators(&column_widths, HORIZONTAL);
        let header_horizontal_separators = Self::generate_horizontal_separators(&column_widths, HORIZONTAL_HEADER);

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

        let mut input_iterator = self.rows.iter().peekable();

        macro_rules! push_data_row {
            ($col_formatter:expr) => {
                if let Some(row) = input_iterator.next() {
                    buffer.push_chars(VERTICAL);
                    for (col_index, col) in row.into_iter().enumerate() {
                        let base = col.as_ref();

                        buffer.push_chars_fixed_width($col_formatter(base, column_widths[col_index]).as_ref(), column_widths[col_index]);
                        buffer.push_chars(VERTICAL);
                    }

                    buffer.push_chars("\n");
                }
            }
        }

        let standard_separator = create_sep_row(&standard_horizontal_separators, LEFT_BRACE, MIDDLE_BRACE, RIGHT_BRACE, true);

        macro_rules! push_header_data_row {
            () => (
                if self.header.centered_text {
                    // align header text in the center of each column
                    push_data_row!(|base_str, width| format!("{:^width$}", base_str, width = width));
                }
                else {
                    push_data_row!(|base_str, _| base_str);
                }
            )
        }

        if self.header.double_bar {
            let header_top = create_sep_row(&header_horizontal_separators, TOP_LEFT_CORNER_HEADER, TOP_BRACE_HEADER, TOP_RIGHT_CORNER_HEADER, true);
            buffer.push_bytes(&header_top);

            push_header_data_row!();

            let header_bottom = create_sep_row(&header_horizontal_separators, LEFT_BRACE_HEADER, MIDDLE_BRACE_HEADER, RIGHT_BRACE_HEADER, true);
            buffer.push_bytes(&header_bottom);

            // TODO
            // handle case where there are no remaining rows
            // currently, it will output an empty body
        }
        else {
            let header = create_sep_row(&standard_horizontal_separators, TOP_LEFT_CORNER, TOP_BRACE, TOP_RIGHT_CORNER, true);
            // add header
            buffer.push_bytes(&header);
            push_header_data_row!();

            // only put the separator if there are rows under
            if input_iterator.peek().is_some() {
                buffer.push_bytes(&standard_separator);
            }
        }

        let footer = create_sep_row(&standard_horizontal_separators, BOTTOM_LEFT_CORNER, BOTTOM_BRACE, BOTTOM_RIGHT_CORNER, false);

        loop {
            push_data_row!(|base_str, _| base_str);

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

        write!(f, "{}", buffer.to_string())
    }
}
