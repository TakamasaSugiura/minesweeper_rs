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

use rand::Rng;
// use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use serde_json::json;
use std::collections::HashMap;

// static mut DICT: HashMap<String, String> = HashMap::new();

struct Point {
    x: u8,
    y: u8,
}

#[derive(Serialize, Deserialize)]
struct CellInfo {
    around_bombs: i8,
    opened: bool,
}

// impl Clone for CellInfo {
//     fn clone(&self) -> CellInfo {
//         CellInfo{ around_bombs: self.around_bombs, opened: self.opened }
//     }
// }

// impl CellInfo {
//     fn new() -> CellInfo {
//         CellInfo { around_bombs: 0, opened: false }
//     }
// }

fn main() {
    //let mut dict: HashMap<String, String> = HashMap::new();
    let rows = 8;
    let cols = 8;
    let mut table : Vec<Vec<CellInfo>> = Vec::new();
    for row_index in 0..rows {
        table.push(Vec::new());
        for col_index in 0..cols {
            table[row_index].push(CellInfo{around_bombs: 0, opened: false});
        }
    }
    let point = Point{x: 0, y: 0};
    init_table(&mut table, 8, &point);
    for row_index in 0..rows {
        for col_index in 0..cols {
            print!("{}", table[row_index][col_index].around_bombs);
        }
        println!("");
    }
    let data = json!(table);
    println!("{}", data.to_string());
}

fn init_table(table: &mut Vec<Vec<CellInfo>>, bombs: i8, first_point: &Point){
    let width = table[0].len() as u8;
    let height = table.len() as u8;
    let mut set_bombs = 0;
    let mut rng = rand::thread_rng();
    while set_bombs < bombs {
        let mut x:u8 = rng.gen(); 
        let mut y:u8 = rng.gen();
        x = x % width;
        y = y % height;
        if table[usize::from(y)][usize::from(x)].around_bombs == 0 && 
            !(first_point.x == x && first_point.y == y) {
            table[usize::from(y)][usize::from(x)].around_bombs = -1;
            set_bombs += 1;
        }
    }
    for y in 0u8..height {
        for x in 0u8..width {
            let mut cnt:i8 = 0;
            let x_size = usize::from(x);
            let y_size = usize::from(y);

            if table[y_size][x_size].around_bombs == -1 {
                continue;
            }
            for y_index in (y as i8 - 1)..(y as i8 + 2) {
                for x_index in (x as i8 - 1)..(x as i8 + 2) {
                    if x_index >= 0 && x_index < width as i8 &&
                        y_index >= 0 && y_index < height as i8 {
                            if table[usize::from(y_index as u8)][usize::from(x_index as u8)].around_bombs == -1 {
                                cnt += 1;
                            }
                        }
                }
            }
            table[y_size][x_size].around_bombs = cnt;
        }
    }
}
