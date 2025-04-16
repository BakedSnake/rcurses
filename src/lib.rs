#![allow(non_camel_case_types)]
use libc::{c_void, c_int, c_short, c_char};

// VT100 Symbols
pub const ACS_ULCORNER:         c_int = ncurses_acs('l');
pub const ACS_LLCORNER:         c_int = ncurses_acs('m');
pub const ACS_URCORNER:         c_int = ncurses_acs('k');
pub const ACS_LRCORNER:         c_int = ncurses_acs('j');
pub const ACS_LTEE:             c_int = ncurses_acs('t');
pub const ACS_RTEE:             c_int = ncurses_acs('u');
pub const ACS_BTEE:             c_int = ncurses_acs('v');
pub const ACS_TTEE:             c_int = ncurses_acs('w');
pub const ACS_HLINE:            c_int = ncurses_acs('q');
pub const ACS_VLINE:            c_int = ncurses_acs('x');
pub const ACS_PLUS:             c_int = ncurses_acs('n');
pub const ACS_S1:               c_int = ncurses_acs('o');
pub const ACS_S9:               c_int = ncurses_acs('s');
pub const ACS_DIAMOND:          c_int = ncurses_acs('`');
pub const ACS_CKBOARD:          c_int = ncurses_acs('a');
pub const ACS_DEGREE:           c_int = ncurses_acs('f');
pub const ACS_PLMINUS:          c_int = ncurses_acs('g');
pub const ACS_BULLET:           c_int = ncurses_acs('~');
// 5410v1 symbols
pub const ACS_LARROW:           c_int = ncurses_acs(',');
pub const ACS_RARROW:           c_int = ncurses_acs('+');
pub const ACS_DARROW:           c_int = ncurses_acs('.');
pub const ACS_UARROW:           c_int = ncurses_acs('-');
pub const ACS_BOARD:            c_int = ncurses_acs('h');
pub const ACS_LANTERN:          c_int = ncurses_acs('i');
pub const ACS_BLOCK:            c_int = ncurses_acs('0');

// Attributes
pub const NCURSES_ATTR_SHIFT:   c_int = 8;

pub const A_COLOR:              c_int = ncurses_bits((1 << 8) - 1, 0);
pub const A_NORMAL:             c_int = ncurses_bits(1, 1);
pub const A_STANDOUT:           c_int = ncurses_bits(1, 8);
pub const A_UNDERLINE:          c_int = ncurses_bits(1, 9);
pub const A_REVERSE:            c_int = ncurses_bits(1, 10);
pub const A_BLINK:              c_int = ncurses_bits(1, 11);
pub const A_DIM:                c_int = ncurses_bits(1, 12);
pub const A_BOLD:               c_int = ncurses_bits(1, 13);
pub const A_ALTCHARSET:         c_int = ncurses_bits(1, 14);
pub const A_INVIS:              c_int = ncurses_bits(1, 15);
pub const A_PROTECT:            c_int = ncurses_bits(1, 16);
pub const A_ITALIC:             c_int = ncurses_bits(1, 23);

// Colors
pub const COLOR_BLACK:          c_short = 0;
pub const COLOR_RED:            c_short = 1;
pub const COLOR_GREEN:          c_short = 2;
pub const COLOR_YELLOW:         c_short = 3;
pub const COLOR_BLUE:           c_short = 4;
pub const COLOR_MAGENTA:        c_short = 5;
pub const COLOR_CYAN:           c_short = 6;
pub const COLOR_WHITE:          c_short = 7;

// Ncurses Semantic Types
pub type bf     = bool;
pub type c      = c_char;
pub type ch     = c_int;
pub type win    = c_void;
pub type chtype = c_int;

#[link(name = "ncurses")]
unsafe extern "C" {
    /// Initialize screen.
    pub fn initscr() -> *mut win;

    /// echo and noecho determine whether characters typed by the user are written to
    /// the curses window by the input character reading function as they are  typed.
    pub fn noecho() -> c_int;

    /// Enable non-blocking intput.
    pub fn nodelay(w: *mut win, b: bf) -> c_int;

    /// qiflush  and noqiflush configure the terminal driver's treatment of its input
    /// and output queues when it handles the interrupt, suspend, or quit  characters
    /// under  the  canonical (“cooked”) or cbreak line disciplines on POSIX systems.
    pub fn noqiflush() -> c_void;

    /// Adjusts the cursor visibility to "invisible", "visible", "very visible",   as
    /// it's argument is 0, 1 or 2, respectively.
    /// It returns the previous visibility if the requested one is supported, and ERR
    /// otherwise.
    pub fn curs_set(i: c_int) -> c_int;

    /// keypad  enables recognition of a terminal's function keys.
    pub fn keypad(w: *mut win, b: bf) -> c_int;

    /// The refresh and wrefresh routines (or  wnoutrefresh  and  doupdate)  must  be
    /// called to get actual output to the terminal, as other routines merely manipu‐
    /// late  data  structures.
    pub fn refresh() -> c_int;

    /// The refresh and wrefresh routines (or  wnoutrefresh  and  doupdate)  must  be
    /// called to get actual output to the terminal, as other routines merely manipu‐
    /// late  data  structures.
    pub fn wrefresh(w: *mut win) -> c_int;

    /// Initializes two global variables, COLORS and COLOR_PAIRS (respectively defin-
    /// ining the maximum number of colors and color pairs the terminal can support).
    pub fn start_color() -> c_int;

    /// Use terminal's default colors.
    pub fn use_default_colors() -> c_int;

    /// Initialize color pair.
    pub fn init_pair(s: c_short, fg: c_short, bg: c_short) -> c_int;

    /// Set attributes on.
    pub fn attron(a: chtype) -> c_int;

    /// Set attributes off.
    pub fn attroff(a: chtype) -> c_int;

    /// Get window attributes.
    pub fn getattrs(w: *mut win) -> c_int;

    /// Get window width.
    pub fn getmaxx(w: *mut win) -> c_int;

    /// Get window height.
    pub fn getmaxy(w: *mut win) -> c_int;

    /// Set window attribute color.
    pub fn wcolor_set(w: *mut win, p: c_short) -> c_int;

    /// Move to and print.
    pub fn mvprintw(y: i8, x: i8, s: *const c_char) -> c_int;

    /// Get cursor coordinate x.
    pub fn getcurx(w: *mut win) -> c_int;

    /// Get cursor coordinate y.
    pub fn getcury(w: *mut win) -> c_int;

    /// Curor routines.
    /// Move cursor to a specific position.
    pub fn mvcur(ol: c_int, oc: c_int, nl: c_int, nc: c_int) -> c_int;

    /// get (or push back) characters from curses terminal keyboard buffer.
    pub fn getch() -> c_int;

    /// Creates and returns a pointer to a new window with the given number of lines and columns.
    pub fn newwin(nl: c_int, nc: c_int, by: c_int, bx: c_int) -> *mut win;

    /// Deletes a window.
    pub fn delwin(w: *mut win) -> c_int;

    /// - resets colors to correspond with the default color pair 0.
    /// - moves the cursor to the lower left-hand corner of the screen.
    /// - clears the remainder of the line so that it uses the default colors.
    /// - sets the cursor to normal visibility/
    /// - if  applicable,  stops  cursor-addressing  mode  using  the  exit_ca_mode
    /// (rmcup) terminal capability.
    /// - restores terminal modes.
    pub fn endwin() -> c_int;

    /// Add a curses character to a window and advance the cursor.
    pub fn addch(c: chtype) -> c_int;

    /// Add a curses character to a window and advance the cursor at a specific place.
    pub fn mvaddch(y: c_int, x: c_int, c: chtype) -> c_int;

    /// Set window attributes on.
    pub fn wattron(w: *mut win, a: c_int) -> c_int;

    /// Set window attributes off.
    pub fn wattroff(w: *mut win, a: c_int) -> c_int;

    /// Set a window attribute.
    pub fn wattrset(w: *mut win, a: c_int) -> c_int;

    /// Change attribute of a given number of characters.
    pub fn mvwchgat(w: *mut win, y: c_int, x: c_int, n: c_int, a: c_int, p: c_short) -> c_int;

    /// Set window colors.
    pub fn wbkgd(w: *mut win, bg: chtype) -> c_int;

    /// Draw window borders.
    pub fn wborder(w: *mut win, ls: chtype, rs: chtype, ts: chtype,
        bs: chtype, tl: chtype, tr: chtype, bl: chtype, br: chtype) -> c_int;

    /// Add a curses character to a window and advance the cursor.
    pub fn waddch(w: *mut win, c: chtype) -> c_int;

    /// Write formatted output to a curses window.
    pub fn mvwprintw(w: *mut win, y: c_int, x: c_int, c: *const c_char) -> c_int;

    /// Add a curses character to a window and advance the cursor at a specific place.
    pub fn mvwaddch(w: *mut win, y: c_int, x: c_int, c: chtype) -> c_int;

    /// Clear window.
    pub fn wclear(w: *mut win) -> c_int;

    /// get (or push back) characters from curses terminal keyboard buffer.
    pub fn wgetch(w: *mut win) -> c_int;

    /// get (or push back) characters from curses terminal keyboard buffer.
    /// specifying cursor position.
    pub fn mvwgetch(w: *mut win, y: c_int, x: c_int) -> c_int;
}

#[link(name = "tinfo")]
unsafe extern "C" {
    /// nocbreak restores canonical (“cooked”) mode.
    pub fn nocbreak() -> c_int;

    /// cbreak configures the terminal in cbreak mode,
    /// which disables line buffering and erase and kill character processing —  the  interrupt,  quit,
    /// suspend,  and  flow  control characters are unaffected — and makes characters typed by the user
    /// immediately available to the program.
    pub fn cbreak() -> c_int;
}

pub const fn ncurses_bits(mask: c_int, shift: c_int) -> c_int {
    (mask << (shift + NCURSES_ATTR_SHIFT)) as c_int
}

pub const fn ncurses_acs(ch: char) -> c_int {
    (ch as c_int) | A_ALTCHARSET
}

pub const fn color_pair(n: c_int) -> c_int {
    ncurses_bits(n, 0)
}
