// Copyright (c) 2025 Kevin Kassil
// Conway's Game of Life

extern crate ncurses;

use ncurses::*;
use rand::random;
use std::cmp::{max, min};
use std::io::Write;
use std::time::{Duration, Instant};

// Cell values
const ALIVE: u32 = 'O' as u32;
const DEAD:  u32 = ' ' as u32;

fn count_live_neighbours(win: WINDOW, y_ctr: i32, x_ctr: i32) -> i32 {
    let mut n_live: i32 = 0;

    for y in max(0, y_ctr - 1)..=min(y_ctr + 1, getmaxy(win) - 1) {
        for x in max(0, x_ctr - 1)..=min(x_ctr + 1, getmaxx(win) - 1) {
            if x != x_ctr || y != y_ctr {
                // don't count yourself
                let cell = canvas_get_cell(win, y, x);
                if cell != DEAD {
                    n_live += 1;
                }
            }
        }
    }
    n_live
}

fn canvas_get_cell(win: WINDOW, y: i32, x: i32) -> u32 {
    mvwinch(win, y, x) & 0xFF
}

fn canvas_put_cell(win: WINDOW, y: i32, x: i32, cell_state: u32) {
    mvwaddch(win, y, x, cell_state);
}

fn update_game(win: WINDOW, win2: WINDOW) {
    for y in 0..getmaxy(win) {
        for x in 0..getmaxx(win) {

            let n_live = count_live_neighbours(win, y, x);
            let cell = canvas_get_cell(win, y, x);
            if cell == ALIVE {
                // Live
                if n_live < 2 {
                    // Underpopulation
                    canvas_put_cell(win2, y, x, DEAD);
                }
                else if n_live > 3 {
                    // Overpopulation
                    canvas_put_cell(win2, y, x, DEAD);
                }
            }
            else if cell == DEAD && n_live == 3 {
                // Reproduction
                canvas_put_cell(win2, y, x, ALIVE);
            }
        }
    }
    // Draw changes after considering all cells
    overwrite(win2, win);
}

fn main() {
    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    let win = stdscr();
    keypad(win, true);
    scrollok(win, true);
    nodelay(win, true);                         // Non-blocking input
    let win2 = newwin(getmaxy(win), getmaxx(win), 0, 0);
    keypad(win2, true);
    scrollok(win2, true);
    nodelay(win2, true);
    start_color();
    init_pair(1, COLOR_WHITE, COLOR_BLUE);      // Regular files
    init_pair(2, COLOR_YELLOW, COLOR_BLUE);     // Directories

    let initial_mode = 1;
    match initial_mode {
        0 => {
            for j in -3..4 {
                for i in -3..4 {
                    if i!=0 && j!=0 {
                        let y = getmaxy(win)/2 + j;
                        let x = getmaxx(win)/2 + i;
                        canvas_put_cell(win,  y, x, ALIVE);
                        canvas_put_cell(win2, y, x, ALIVE);
                    }
                }
            }
        }
        1 => {
            for y in 0..getmaxy(win) {
                for x in 0..getmaxx(win) {
                    // Roll a dice with a 50% chance using a random boolean
                    if random() {
                        mvwaddch(win, y, x, ALIVE);
                    }
                }
            }
        }
        _ => {
        }
    }

    // Target frame rate
    let mut delay_ms = 100;
    let mut last_turn = Instant::now() - Duration::from_millis(10000);

    let mut maybe_log = std::fs::OpenOptions::new()
        .append(true)
        .create(true) // Create the file if it doesn't exist
        .open("conway.log");

    loop {

        // Handle the periodic redraw every 100ms
        if last_turn.elapsed() >= Duration::from_millis(delay_ms) {
            update_game(win, win2);
            last_turn = Instant::now();
            wrefresh(win);
        }

        let max_sleep_msec = max(0, (Instant::now() - last_turn).as_millis() as i32 + delay_ms as i32);
        timeout(max_sleep_msec);

        // Handle input
        const KEY_Q: i32 = 'q' as i32;
        const KEY_ESC: i32 = 27;
        if let Ok(ref mut log) = maybe_log {
            writeln!(log, "Sleeping {} ms", max_sleep_msec);
        }
        let key = wgetch(win);
        match key {
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
        if key != ERR {
        }
        else {
            if let Ok(ref mut log) = maybe_log {
                writeln!(log, "Slept {} ms", (Instant::now() - last_turn).as_millis());
            }
        }
    }
    endwin();
}
