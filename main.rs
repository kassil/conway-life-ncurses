// Copyright (c) 2025 Kevin Kassil
// Conway's Game of Life

extern crate ncurses;

use ncurses::*;
use rand::random;
use std::cmp::{max, min};
use std::time;

fn count_live_neighbours(win: WINDOW, y_ctr: i32, x_ctr: i32) -> i32 {
    let mut n_live: i32 = 0;

    for y in max(0, y_ctr - 1)..=min(y_ctr + 1, getmaxy(win) - 1) {
        for x in max(0, x_ctr - 1)..=min(x_ctr + 1, getmaxx(win) - 1) {
            if x != x_ctr || y != y_ctr {
                // don't count yourself
                let cell = canvas_get_cell(win, y, x);
                if cell != ' ' {
                    n_live += 1;
                }
            }
        }
    }
    n_live
}

fn canvas_get_cell(win: WINDOW, y: i32, x: i32) -> char {
    char::from_u32(mvwinch(win, y, x) & 0xFF).unwrap_or(' ')
}

fn canvas_put_cell(win: WINDOW, y: i32, x: i32, cell_state: char) {
    mvwaddch(win, y, x, cell_state as u32);
}

fn main() {
    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    let win = stdscr();
    keypad(win, true);
    scrollok(win, true);
    nodelay(win, true);
    let win2 = newwin(getmaxy(win), getmaxx(win), 0, 0);
    keypad(win2, true);
    scrollok(win2, true);
    nodelay(win2, true);
    start_color();
    init_pair(1, COLOR_WHITE, COLOR_BLUE);      // Regular files
    init_pair(2, COLOR_YELLOW, COLOR_BLUE);     // Directories

    let mut delay_ms = 100;

    let initial_mode = 1;
    match initial_mode {
        0 => {
            for j in -3..4 {
                for i in -3..4 {
                    if i!=0 && j!=0 {
                        let y = getmaxy(win)/2 + j;
                        let x = getmaxx(win)/2 + i;
                        canvas_put_cell(win, y, x, 'O');
                        canvas_put_cell(win2, y, x, 'O');
                    }
                }
            }
        }
        1 => {
            for y in 0..getmaxy(win) {
                for x in 0..getmaxx(win) {
                    // Roll a dice with a 50% chance using a random boolean
                    if random() {
                        mvwaddch(win, y, x, 'O' as u32 );
                    }
                }
            }
        }
        _ => {
        }
    }

    loop {
        wrefresh(win);
        std::thread::sleep(time::Duration::from_millis(delay_ms));

        for y in 0..getmaxy(win) {
            for x in 0..getmaxx(win) {

                let n_live = count_live_neighbours(win, y, x);
                let cell = canvas_get_cell(win, y, x);
                if cell == 'O' {
                    // Live
                    if n_live < 2 {
                        // Underpopulation
                        canvas_put_cell(win2, y, x, ' ');
                    }
                    else if n_live > 3 {
                        // Overpopulation
                        canvas_put_cell(win2, y, x, ' ');
                    }
                }
                else if cell == ' ' && n_live == 3 {
                    // Reproduction
                    canvas_put_cell(win2, y, x, 'O');
                }
            }
        }
        // Draw changes after considering all cells
        overwrite(win2, win);

        // Handle input
        const KEY_Q: i32 = 'q' as i32;
        const KEY_ESC: i32 = 27;
        match wgetch(win) {
            KEY_RESIZE => {
                let new_rows = LINES();
                let new_cols = COLS();
                wresize(win, new_rows, new_cols);
                wresize(win2, new_rows, new_cols);
            }
            // Escape or 'q' to quit
            KEY_Q | KEY_ESC => {
                break;
            }
            val if val == '+' as i32 => {
                delay_ms = max(delay_ms * 4 / 5, 10);
            }
            val if val == '-' as i32 => {
                delay_ms = delay_ms * 5 / 4;
            }
            _ => {}
        }
    }

    endwin();
}
