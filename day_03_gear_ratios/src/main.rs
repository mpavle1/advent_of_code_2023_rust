const INPUT: &str =
"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

fn check_if_there_is_adjacent_symbol(start_position: (i32, i32), end_position: (i32, i32), rows: &Vec<&str>) -> bool {
    for i in (start_position.0 - 1)..(end_position.0 + 1) {
        for j in (start_position.1 - 1)..(end_position.1 + 1) {

            if i < 0 || j < 0 || i > rows.len() as i32 || j > rows[0].len() as i32 {
                continue;
            }


            // println!("{}{}", i , j);
            let current_row:Option<&str> = Some(rows[i as usize]);
            if current_row.is_some() {
                let current_char = Some(current_row.expect("REASON").chars().nth(j as usize).unwrap());
                if current_char.is_some() {
                    if !current_char.expect("REASON2").is_numeric() && current_char.expect("REASON2") != '.' {
                        return true;
                    }
                }
            }
            
        }
    }

    return false;
}

fn main() {
    let rows: Vec<&str> = INPUT.split("\n").collect();

    let mut sum: i32 = 0;


    for i in 0..rows.len() {
        let current_row = rows[i];


        let mut is_last_character_numeric = false;
        let mut start_position: (i32, i32) = (0,0);
        let mut end_position:(i32, i32) = (0,0);


        for j in 0..current_row.len() {
            let current_char = current_row.chars().nth(j).unwrap();

            if current_char.is_numeric() {
                if !is_last_character_numeric {
                    start_position = (i as i32, j as i32);
                }
                end_position = (i as i32, j as i32);
                is_last_character_numeric = true;
                continue;
            }

            if check_if_there_is_adjacent_symbol(start_position, end_position, &rows) {
                println!("{} {},{}:{},{}", &current_row[(start_position.1 as usize)..=(end_position.1 as usize)], start_position.1, end_position.1, start_position.0, end_position.0);
                let current_chars = &current_row[(start_position.1 as usize)..=(end_position.1 as usize)].parse::<i32>().unwrap();

                sum += current_chars;
            }
        }
    }

    println!("{}", sum);
}
