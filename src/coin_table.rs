use cli_table::{format::Justify, Cell, CellStruct};
use console::style;

pub fn get_percentage_cell(value: f64) -> CellStruct {
    let mut cell_value = format!("{:.2}", value);
    cell_value.push_str("%");

    if value < 0.0 {
        return format!("{}", style(cell_value).red())
            .cell()
            .justify(Justify::Center);
    }

    let positive_value = format!("+{}", cell_value);
    return format!("{}", style(positive_value).green())
        .cell()
        .justify(Justify::Center);
}

pub fn get_currency_cell(value: f64) -> CellStruct {
    let cell_value = format!("{:.4}", value);

    if value < 0.0 {
        return format!("{}", style(cell_value).red())
            .cell()
            .justify(Justify::Center);
    }

    return format!("{}", style(cell_value).green())
        .cell()
        .justify(Justify::Center);
}
