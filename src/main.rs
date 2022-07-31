use cli_table::{format::Justify, Cell, Style, Table};
use console::{style, Term};
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn do_stuff() -> io::Result<()> {
    let term = Term::stdout();
    term.set_title("Counting...");
    term.write_line("Going to do some counting now")?;
    term.hide_cursor()?;

    for x in 0..10 {
        let table_options = vec![
            vec![
                "Tom".cell(),
                format!(" {}/11", style(x + 1).cyan())
                    .cell()
                    .justify(Justify::Right),
            ],
            vec![
                "Jerry".cell(),
                format!(" {}/12", style(x + 1).cyan())
                    .cell()
                    .justify(Justify::Right),
            ],
            vec![
                "Scooby Doo".cell(),
                format!(" {}/10", style(x + 1).cyan())
                    .cell()
                    .justify(Justify::Right),
            ],
        ];
        let header_size = 4;
        let row_size = 2;
        let move_cursor_up = table_options.len() * row_size + header_size;

        if x != 0 {
            term.move_cursor_up(move_cursor_up)?;
        }

        let table = table_options
            .table()
            .title(vec!["Name".cell().bold(true), "Progress".cell().bold(true)])
            .bold(true);

        let table_display = table.display().unwrap();

        // term.write_line(&format!("Counting {}/10", style(x + 1).cyan()))?;
        match term.write_line(&format!("{}", table_display)) {
            Err(e) => return Err(e),
            Ok(t) => t,
        };

        thread::sleep(Duration::from_millis(750));
    }
    term.show_cursor()?;
    term.clear_last_lines(1)?;
    term.write_line("Done counting!")?;
    writeln!(&term, "Hello World!")?;

    write!(&term, "To edit: ")?;
    let res = term.read_line_initial_text("default")?;
    writeln!(&term, "\n{}", res)?;

    Ok(())
}

fn main() {
    do_stuff().unwrap();
}
