// Copyright (c) 2025 Kevin Kassil
// Conway's Game of Life

extern crate ncurses;

use ncurses::*;
use std::cmp::{max, min};
//use std::fs;
//use std::io;
use std::{thread, time};

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
    char::from_u32(mvwinch(win, y, x) & 0xFF).unwrap()
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
    start_color();
    init_pair(1, COLOR_WHITE, COLOR_BLUE);      // Regular files
    init_pair(2, COLOR_YELLOW, COLOR_BLUE);     // Directories

    // let mut all_done: bool = false;

    struct UpdateCell {
        x: i32,
        y: i32,
        state: char,
    }
    let mut cell_updates = Vec::new(); ////Vec<UpdateCell>;

    for j in 0..3 {
        for i in 0..3 {
            let y = getmaxy(win)/2 + j;
            let x = getmaxx(win)/2 + i;
            canvas_put_cell(win, y, x, 'O');
        }
    }
    wrefresh(win);

    loop {

        std::thread::sleep(time::Duration::from_millis(200));

        for y in 0..getmaxy(win) {
            for x in 0..getmaxx(win) {

                let n_live = count_live_neighbours(win, y, x);
                let cell = canvas_get_cell(win, y, x);
                if cell == 'O' {
                    // Live
                    if n_live < 2 {
                        // Underpopulation
                        cell_updates.push(UpdateCell{x: x, y: y, state: ' '});
                    }
                    else if n_live > 3 {
                        // Overpopulation
                        cell_updates.push(UpdateCell{x: x, y: y, state: ' '});
                    }
                }
                else if cell == ' ' && n_live == 3 {
                    // Reproduction
                    cell_updates.push(UpdateCell{x: x, y: y, state: 'O'});
                }

            }
        }
        // Draw changes after considering all cells
        for cell in &cell_updates {
            canvas_put_cell(win, cell.y, cell.x, cell.state);
        }
        cell_updates.clear();

        wrefresh(win);

        // Handle input
        //let ch = wgetch(win);
        // match ch {
        //     KEY_ESC => {
        //         break;
        //     }
        //     _ => {
        //     }
        // }
    }

    endwin();
    return;
}
