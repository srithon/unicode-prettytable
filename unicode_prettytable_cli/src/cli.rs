use clap::*;

pub fn create_cli() -> App<'static, 'static> {
    clap_app!(
        myapp =>
        (name: crate_name!())
        (version: crate_version!())
        (about: crate_description!())
        (@arg CENTER_HEADERS: --("center-headers") "Centers header text within each column")
        (@arg DOUBLE_BAR_HEADERS: --("double-bar-headers") "Uses double bar Unicode characters surrounding the header")
        (@arg FILE_FORMAT: +required -f --format <FORMAT> possible_value[json csv] "Data format of file")
        (@arg INPUT_FILE: +required "File to read data from")
    )
}
