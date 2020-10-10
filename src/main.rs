use ascii_table::{Align, AsciiTable, Column};
use termion::{clear, color, cursor};

use std::fmt::Display;
use std::{thread, time};
fn main() {
    let mut ascii_table = AsciiTable::default();
    ascii_table.max_width = 50;

    let mut column = Column::default();
    column.header = "Header1".into();
    column.align = Align::Left;
    ascii_table.columns.insert(0, column);

    let mut column = Column::default();
    column.header = "Header2".into();
    column.align = Align::Center;
    ascii_table.columns.insert(1, column);

    let mut column = Column::default();
    column.header = "Header3".into();
    column.align = Align::Right;
    ascii_table.columns.insert(2, column);
    let mut i = 0;
    while (true) {
        let data: Vec<Vec<&dyn Display>> = vec![
            vec![&i, &"Good morning", &"When will 2020 end?"],
        ];

        let s = ascii_table.format(data.clone());

        println!(
            "\n{}{}{}{}",
            cursor::Hide,
            clear::All,
            cursor::Goto(1, 1),
            s
        );
        println!("Hello there");
        i = i+1;
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
}
