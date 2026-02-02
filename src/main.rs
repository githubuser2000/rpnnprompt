mod autocomplete;
mod csv_parser;
mod csv_data;
mod ui;

use anyhow::Result;

fn main() -> Result<()> {
    ui::run()
}
