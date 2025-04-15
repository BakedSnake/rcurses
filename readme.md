## rcurses :: ncurses bindings for rust

Useful libncurses bindings to be used in rust.

For more info on how to use ncurses please consider reading the man pages.

### Install with cargo

```bash
cargo add --git https://github.com/bakedsnake/rcurses
```

### Usage

```rust
use rcurses::*;
use libc::{setlocale, LC_ALL};
use std::ffi::CString;

fn main() {
    let locale = CString::new("en_US.UTF-8").unwrap();
    let str = CString::new("Hello, world!").unwrap();

    unsafe {
        setlocale(LC_ALL, locale.as_ptr());
        let win = initscr();
        cbreak();
        noecho();
        noqiflush();
        keypad(win, true);

        // Print "Hello, world!" and wait for input
        mvprintw(10, 10, str.as_ptr());
        wgetch(win);

        nocbreak();
        endwin();
    }
}

```

### TODO

More bindings will be added.
