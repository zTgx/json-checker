# [json-checker](https://github.com/zTgx/json-checker.git)  [![Build Status](https://travis-ci.org/zTgx/json-checker.svg?branch=master)](https://travis-ci.org/zTgx/json-checker) 

A wrapper around [JSON-c](https://github.com/douglascrockford/JSON-c.git), a light-weight json checker by Douglas Crockford . 

# Usage
Add dependencies
```
[dependencies]
json-checker = "0.1.0"
```

```rust
extern crate json_checker;
use json_checker::*;

extern crate ncurses;
use ncurses::*;

fn main() {
    let mut checker = JsonChecker::new(20);

    initscr();
    raw();

    keypad(stdscr(), true);

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
```
