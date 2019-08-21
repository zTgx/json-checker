extern crate json_checker;
use json_checker::*;

extern crate ncurses;
use ncurses::*;

fn main() {
    let mut checker = JsonChecker::new(20);

    initscr();
    raw();

    keypad(stdscr(), true);
    // noecho();

    printw("Enter a json string: ");

    loop {
        let next_char = getch();
        if next_char == 0xa {
            endwin();
            break;
        }

        if checker.check_char(next_char) == 0 {
            endwin();
            panic!("JSON_checker_end: syntax error\n");
        }
    }

    if checker.done() == 0 {
        panic!("JSON_checker_end: syntax error\n");
    } else {
        println!("well-formed JSON text!")
    }
}
