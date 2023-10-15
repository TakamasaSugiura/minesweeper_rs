/*
    Copyright (C) 2023 Takamasa Sugiura

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

extern crate ncurses;

use ncurses::*;
use rand::Rng;
use regex::Regex;

const WIDTH:usize = 8;
const HEIGHT:usize = 8;

struct Point {
    x: u8,
    y: u8,
}

#[derive(Copy)]
struct PosInfo {
    number_of_bombs: i8,
    opened: bool,
}

//impl Copy for PosInfo {}

impl Clone for PosInfo {
    fn clone(&self) -> PosInfo {
        PosInfo{ number_of_bombs: self.number_of_bombs, opened: self.opened }
    }
}

impl PosInfo {
    fn new() -> PosInfo {
        PosInfo { number_of_bombs: 0, opened: false }
    }
}

fn main()
{
    let mut table = [[PosInfo::new(); WIDTH]; HEIGHT];
    let mut first_time = true;

    initscr();

    display_table(&mut table);
    refresh();

    loop {
        let mut s = String::new();
        mvgetnstr(13, 0, &mut s, 2);

        if let Ok(point) = get_location(s) {
            if first_time {
                init_table(&mut table, &point);
                first_time = false;
            }
        
            let ret = open(&mut table, &point);

            clear();
            display_table(&mut table);

            if ret == 1 {
                mvaddstr(13, 0, "GAME OVER");
                break;
            }
            if is_cleared(&mut table) {
                mvaddstr(13, 0, "GAME CLEAR");
                break;
            }
        }
        refresh();
    }

    refresh();
    getch();
    endwin();
}

fn get_location(input: String) -> Result<Point, String> {
    if input.len() != 2 {
        return Err("Bad input".to_string());
    }
    let exam = input.to_uppercase();
    let re = Regex::new("[A-H][1-8]").unwrap();
    if !re.is_match(&exam) {
        return Err("Bad input".to_string());
    }
    let col_char = exam.chars().nth(0).unwrap();
    let col = col_char as u8 - 'A' as u8;

    let row_char = exam.chars().nth(1).unwrap();
    let row = row_char as u8 - '1' as u8; 

    return Ok(Point{y: row, x: col});
}

fn init_table(table: &mut[[PosInfo; WIDTH]; HEIGHT], first_point: &Point){
    let width:u8 = WIDTH as u8;
    let height:u8 = HEIGHT as u8;
    let mut bombs = 0;
    let mut rng = rand::thread_rng();
    while bombs < 8 {
        let mut x:u8 = rng.gen(); 
        let mut y:u8 = rng.gen();
        x = x % width;
        y = y % height;
        if table[usize::from(y)][usize::from(x)].number_of_bombs == 0 && 
            !(first_point.x == x && first_point.y == y) {
            table[usize::from(y)][usize::from(x)].number_of_bombs = -1;
            bombs += 1;
        }
    }
    for y in 0u8..height {
        for x in 0u8..width {
            let mut cnt:i8 = 0;
            let x_size = usize::from(x);
            let y_size = usize::from(y);

            if table[y_size][x_size].number_of_bombs == -1 {
                continue;
            }
            for y_index in (y as i8 - 1)..(y as i8 + 2) {
                for x_index in (x as i8 - 1)..(x as i8 + 2) {
                    if x_index >= 0 && x_index < width as i8 &&
                        y_index >= 0 && y_index < height as i8 {
                            if table[usize::from(y_index as u8)][usize::from(x_index as u8)].number_of_bombs == -1 {
                                cnt += 1;
                            }
                        }
                }
            }
            table[y_size][x_size].number_of_bombs = cnt;
        }
    }
}

fn display_table(table: &[[PosInfo; WIDTH]; HEIGHT]){
    mvaddstr(0, 0, "<<MINE SWEEPER>>");
    mvaddstr(2, 2, "ABCDEFGH");
    for row in 1..9 {
        mvaddstr(row + 2, 0, &format!("{}", row));
    }
    for y in 0u8..(HEIGHT as u8) {
        for x in 0u8..(WIDTH as u8) {
            let exam1 = table[usize::from(y)][usize::from(x)].number_of_bombs;
            let exam2 = table[usize::from(y)][usize::from(x)].opened;
            let x_pos:i32 = (x + 2).into();
            let y_pos:i32 = (y + 3).into();
            if exam2 == true {
                if exam1 == -1 {
                    mvaddstr(y_pos, x_pos, "B");
                }
                else {
                    mvaddstr(y_pos, x_pos, &format!("{}", exam1));
                }
            }
            else {
                mvaddstr(y_pos, x_pos, "_");
            }
        }
    }
}

fn open(table: &mut[[PosInfo; WIDTH]; HEIGHT], point: &Point) -> i32 {
    let width:i8 = WIDTH as i8;
    let height:i8 = HEIGHT as i8;
    if table[usize::from(point.y)][usize::from(point.x)].opened != false {
        return 2;
    }
    table[usize::from(point.y)][usize::from(point.x)].opened = true;
    if table[usize::from(point.y)][usize::from(point.x)].number_of_bombs == -1 {
        return 1;
    }
    if table[usize::from(point.y)][usize::from(point.x)].number_of_bombs == 0 {
        for y in (point.y as i8 - 1)..(point.y as i8 + 2) {
            for x in (point.x as i8 - 1)..(point.x as i8 + 2) {
                if x >= 0 && x < width &&
                    y >= 0 && y < height {
                        open(table, &Point{ y:y as u8, x:x as u8});
                    }
            }
        }
    }
    return 0;
}

fn is_cleared(table: &[[PosInfo; WIDTH]; HEIGHT]) -> bool{
    let mut count = 0;
    for y in 0u8..(HEIGHT as u8) {
        for x in 0u8..(WIDTH as u8) {
            let exam = table[usize::from(y)][usize::from(x)].opened;
            if exam == false {
                count += 1;
            }
        }
    }
    return count == 8;
}
