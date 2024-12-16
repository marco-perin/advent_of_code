use std::fs;

const FILE_PATH: &str = "input.txt";

// enum Direction {
//     Up = 0,
//     Right = 1,
//     Down = 2,
//     Left = 3,
// }
// Cannot do var:Direction + 1 to go to turn right, so consts it is :)
const DIR_UP: u8 = 1 << 0;
const DIR_RIGHT: u8 = 1 << 1;
const DIR_DOWN: u8 = 1 << 2;
const DIR_LEFT: u8 = 1 << 3;
const DIR_MAX: u8 = 1 << 4;

// #[derive(PartialEq, Eq)]
// enum Direction {
//     Up = DIR_UP,
//     Right = DIR_RIGHT,
//     Down = DIR_DOWN,
//     Left = DIR_LEFT,
// }

#[derive(PartialEq, Eq)]
// #[derive(Debug)]
enum CellType {
    Empty = 0,
    Crate = 1,
    // Guard = 2, // Not needed, just set start pos as visited and go on
    Visit = 3,
}

// #[derive(Debug)]
struct Cell {
    cell_type: CellType,
    visit_dir: Option<u8>,
}

// impl Cell {
//     fn eq(&self) {

//     }
// }

fn main() {
    let mut done = false;
    let mut pos: [usize; 2];

    let mut dir: u8 = DIR_UP;

    let mut map: Vec<Vec<Cell>>;
    (map, pos) = read_map();

    let mut num_steps = 1;
    let mut num_loops = 0;

    while !done {
        let steps_increment;
        let new_loops;
        (done, steps_increment, new_loops) = step(&mut pos, &mut dir, &mut map);
        println!("pos: {pos:?},\t\tsteps: {num_steps},\t\t loops: {num_loops}");
        num_steps += steps_increment;
        num_loops += new_loops;
    }

    println!("Final visited squares: {num_steps}");
    println!("Possible loops: {num_loops}");

    // print_map(map);
}

fn rotate_dir(dir: u8) -> u8 {
    // Old rotation (+1)
    // return (*dir + 1) % DIR_MAX;
    let d = dir << 1;
    return match d {
        DIR_MAX => DIR_UP,
        d => d,
    };
}

fn match_bits(x: u8, y: u8) -> bool {
    return x & y == y;
}

fn print_map(map: Vec<Vec<Cell>>) {
    for row in map {
        let row_s: String = row
            .iter()
            .map(|cell| {
                return match cell.cell_type {
                    CellType::Crate => '#',
                    CellType::Empty => '.',
                    CellType::Visit => {
                        let dir = cell
                            .visit_dir
                            .expect("Cannot be visited without a direction");
                        if match_bits(dir, DIR_UP) || match_bits(dir, DIR_DOWN) {
                            return '|';
                        } else if match_bits(dir, DIR_LEFT) || match_bits(dir, DIR_RIGHT) {
                            return '-';
                        }
                        return '+';
                    } // CellType::Visit => match cell
                      //     .visit_dir
                      //     .expect("Cannot be visited without a direction")
                      // {
                      //     DIR_DOWN => 'v',
                      //     DIR_LEFT => '<',
                      //     DIR_UP => '^',
                      //     DIR_RIGHT => '>',
                      //     dir => {
                      //         if dir == DIR_RIGHT | DIR_LEFT {
                      //             return '-';
                      //         } else if dir == DIR_UP | DIR_DOWN {
                      //             return '|';
                      //         }
                      //         return '+';
                      //     }
                      // },
                };
            })
            .collect();
        println!("{row_s}");
    }
}

fn step(pos: &mut [usize; 2], dir: &mut u8, map: &mut Vec<Vec<Cell>>) -> (bool, usize, usize) {
    let mut done_stepping = false;
    let mut num_steps = 0;
    let mut num_loops = 0;

    while !done_stepping {
        if raycast_path(*pos, rotate_dir(*dir), map) {
            num_loops += 1;
        }

        let next_pos = match advance_pos(*pos, *dir) {
            Some(x) => x,
            None => {
                return (true, num_steps, num_loops);
            }
        };

        // match advance_pos(&pos, rotate_dir(*dir)) {
        //     Some(right_pos) => match read_cell(&right_pos, map) {
        //         Some(right_cell) => {
        //             if (*right_cell).cell_type == CellType::Visit {
        //                 num_loops += 1;
        //             }
        //         }
        //         None => {
        //             assert_ne!(pos[1], 129);
        //             return (true, num_steps, num_loops);
        //         }
        //     },
        //     None => {}
        // };

        let next_cell = match read_cell(&next_pos, map) {
            Some(x) => x,
            None => {
                // assert_ne!(pos[1], 129);
                return (true, num_steps, num_loops);
            }
        };

        match next_cell.cell_type {
            CellType::Empty => {
                next_cell.cell_type = CellType::Visit;
                next_cell.visit_dir = Some(*dir);
                num_steps += 1;
                *pos = next_pos;
            }
            CellType::Crate => {
                let curr_cell = read_cell(pos, map).unwrap();
                let curr_dir = curr_cell
                    .visit_dir
                    .expect("Curr Cell must have a visit dir");
                curr_cell.visit_dir = Some(curr_dir | rotate_dir(*dir));
                *dir = rotate_dir(*dir);
                done_stepping = true;
            }
            CellType::Visit => {
                assert_ne!(next_cell.visit_dir, None);
                next_cell.visit_dir = Some(next_cell.visit_dir.unwrap() | *dir);
                *pos = next_pos;
            }
        }
    }
    return (false, num_steps, num_loops);
}

// Returns true if finds a path with the same dir, false otherwise.
fn raycast_path(pos: [usize; 2], dir: u8, map: &mut Vec<Vec<Cell>>) -> bool {
    // let mut done_stepping = false;
    let mut pos = pos;

    println!("Loop from: {pos:?}");
    loop {
        let next_pos = match advance_pos(pos, dir) {
            Some(x) => x,
            None => {
                return false;
            }
        };

        let next_cell = match read_cell(&next_pos, map) {
            Some(x) => x,
            None => {
                // assert_ne!(pos[1], 129);
                return false;
            }
        };

        match next_cell.cell_type {
            CellType::Empty => {
                pos = next_pos;
                assert_eq!(pos, next_pos)
            }
            CellType::Crate => {
                // TODO: maybe this leads to other existing paths?
                // TODO: possible recursion?
                return false;
            }
            CellType::Visit => {
                let next_dir = next_cell.visit_dir.expect("Visited cell must have dir");

                if dir & next_dir == dir {
                    return true;
                }
                dbg!(dir);
                dbg!(next_dir);
            }
        };
    }
}

fn read_cell<'a>(pos: &[usize; 2], map: &'a mut Vec<Vec<Cell>>) -> Option<&'a mut Cell> {
    return map.get_mut(pos[0]).map_or(None, |row| row.get_mut(pos[1]));
}

fn advance_pos(pos: [usize; 2], dir: u8) -> Option<[usize; 2]> {
    let mut next_pos: [usize; 2] = pos;

    match dir {
        // Direction::Up => {
        DIR_UP => {
            if next_pos[0] == 0 {
                return None;
            }
            next_pos[0] -= 1;
        }
        // Direction::Down => next_pos[0] += 1,
        DIR_DOWN => next_pos[0] += 1,
        // Direction::Right => next_pos[1] += 1,
        DIR_RIGHT => next_pos[1] += 1,
        // Direction::Left => {
        DIR_LEFT => {
            if next_pos[1] == 0 {
                return None;
            }
            next_pos[1] -= 1;
        }
        n => unreachable!("Dir should be 1,2,4,8 (got {n})"),
    }

    return Some(next_pos);
}

fn read_map() -> (Vec<Vec<Cell>>, [usize; 2]) {
    let input = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file!");

    let lines = input.lines();

    let mut lines_parsed: Vec<Vec<Cell>> = Vec::new();

    let mut pos_opt: Option<[usize; 2]> = None;

    for (line_idx, line) in lines.enumerate() {
        lines_parsed.push(
            line.chars()
                .map(|c| Cell {
                    cell_type: match c {
                        '.' => CellType::Empty,
                        '#' => CellType::Crate,
                        '^' => CellType::Visit, // Count guard pos as first visited
                        x => panic!("unexpected char!!: {x}"),
                    },
                    visit_dir: if c == '^' { Some(DIR_UP) } else { None },
                })
                .collect(),
        );

        match line.find('^') {
            Some(col_idx) => {
                assert_eq!(pos_opt, None, "More than one guard found in the map!");
                pos_opt = Some([line_idx, col_idx]);
            }
            None => {}
        }
    }

    assert_ne!(pos_opt, None, "No guard found in the map");

    return (lines_parsed, pos_opt.unwrap());
}
