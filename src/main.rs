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

fn main()
{
    let mut table = [[[0i8; WIDTH]; HEIGHT]; 2];
    let mut first_time = true;

    initscr();

    display_table(&mut table);
    refresh();

    loop {
        let mut s = String::new();
        mvgetnstr(13, 0, &mut s, 2);

        if let Ok(location) = get_location(s) {
            if first_time {
                table[1][usize::from(location.y)][usize::from(location.x)] = 1;
                init_table(&mut table);
                first_time = false;
                table[1][usize::from(location.y)][usize::from(location.x)] = 0;
            }
        
            let ret = open(&mut table, location.y.try_into().unwrap(), location.x.try_into().unwrap());

            clear();
            display_table(&mut table);
            refresh();

            if ret == 1 {
                mvaddstr(13, 0, "GAME OVER");
                break;
            }
            if is_cleared(&mut table[1]) {
                mvaddstr(13, 0, "GAME CLEAR");
                break;
            }
        }
        else {
            continue;
        }
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

fn init_table(table: &mut[[[i8; WIDTH]; HEIGHT]; 2]){
    let width:u8 = WIDTH as u8;
    let height:u8 = HEIGHT as u8;
    let mut bombs = 0;
    let mut rng = rand::thread_rng();
    while bombs < 8 {
        let mut x:u8 = rng.gen(); 
        let mut y:u8 = rng.gen();
        x = x % width;
        y = y % height;
        if table[0][usize::from(y)][usize::from(x)] == 0 && 
            table[1][usize::from(y)][usize::from(x)] == 0 {
            table[0][usize::from(y)][usize::from(x)] = -1;
            bombs += 1;
        }
    }
    for y in 0u8..height {
        for x in 0u8..width {
            let mut cnt:i8 = 0;
            let x_size = usize::from(x);
            let y_size = usize::from(y);

            if table[0][y_size][x_size] == -1 {
                continue;
            }
            if x > 0 && y > 0 && table[0][y_size - 1][x_size - 1] == -1 {
                cnt += 1;
            }
            if x > 0 && y < height - 1 && table[0][y_size + 1][x_size - 1] == -1 {
                cnt += 1;
            }
            if x < width - 1 && y > 0 && table[0][y_size - 1][x_size + 1] == -1 {
                cnt += 1;
            }
            if x < width - 1 && y < height - 1 && table[0][y_size + 1][x_size + 1] == -1 {
                cnt += 1;
            }
            if x > 0 && table[0][y_size][x_size - 1] == -1 {
                cnt += 1;
            }
            if x < width - 1 && table[0][y_size][x_size + 1] == -1 {
                cnt += 1;
            }
            if y > 0 && table[0][y_size - 1][x_size] == -1 {
                cnt += 1;
            }
            if y < height - 1 && table[0][y_size + 1][x_size] == -1 {
                cnt += 1;
            }
            table[0][y_size][x_size] = cnt;
        }
    }
}

fn display_table(table: &[[[i8; WIDTH]; HEIGHT]; 2]){
    mvaddstr(0, 0, "<<MINE SWEEPER>>");
    mvaddstr(2, 2, "ABCDEFGH");
    for row in 1..9 {
        mvaddstr(row + 2, 0, &format!("{}", row));
    }
    for y in 0u8..(HEIGHT as u8) {
        for x in 0u8..(WIDTH as u8) {
            let exam1 = table[0][usize::from(y)][usize::from(x)];
            let exam2 = table[1][usize::from(y)][usize::from(x)];
            let x_pos:i32 = (x + 2).into();
            let y_pos:i32 = (y + 3).into();
            if exam2 == 1 {
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

fn open(table: &mut[[[i8; WIDTH]; HEIGHT]; 2], y:u8, x:u8) -> i32 {
    let width:i8 = WIDTH as i8;
    let height:i8 = HEIGHT as i8;
    if table[1][usize::from(y)][usize::from(x)] != 0 {
        return 2;
    }
    table[1][usize::from(y)][usize::from(x)] = 1;
    if table[0][usize::from(y)][usize::from(x)] == -1 {
        return 1;
    }
    if table[0][usize::from(y)][usize::from(x)] == 0 {
        for y_index in (y as i8 - 1)..(y as i8 + 2) {
            for x_index in (x as i8 - 1)..(x as i8 + 2) {
                if x_index >= 0 && x_index < width &&
                    y_index >= 0 && y_index < height {
                        open(table, y_index as u8, x_index as u8);
                    }
            }
        }
    }
    return 0;
}

fn is_cleared(table: &[[i8; WIDTH]; HEIGHT]) -> bool{
    let mut count = 0;
    for y in 0u8..(HEIGHT as u8) {
        for x in 0u8..(WIDTH as u8) {
            let exam = table[usize::from(y)][usize::from(x)];
            if exam == 0 {
                count += 1;
            }
        }
    }
    return count == 8;
}
