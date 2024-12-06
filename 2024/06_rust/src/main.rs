use std::fs;

const FILE_PATH: &str = "input.txt";

// enum Direction {
//     Up = 0,
//     Right = 1,
//     Down = 2,
//     Left = 3,
// }
// Cannot do var:Direction + 1 to go to turn right, so consts it is :)
const DIR_UP: u8 = 0;
const DIR_RIGHT: u8 = 1;
const DIR_DOWN: u8 = 2;
const DIR_LEFT: u8 = 3;
const DIR_MAX: u8 = 4;

// #[derive(Debug)]
enum CellType {
    Empty = 0,
    Crate = 1,
    // Guard = 2, // Not needed, just set start pos as visited and go on
    Visit = 3,
}

fn main() {
    let mut done = false;
    let mut pos: [usize; 2];

    let mut dir: u8 = DIR_UP;

    let mut map: Vec<Vec<CellType>>;
    (map, pos) = read_map();

    let mut num_steps = 1;

    while !done {
        let steps_increment;
        (done, steps_increment) = step(&mut pos, &mut dir, &mut map);
        num_steps += steps_increment;
    }

    println!("Final visited squares: {num_steps}")
}

fn step(pos: &mut [usize; 2], dir: &mut u8, map: &mut Vec<Vec<CellType>>) -> (bool, usize) {
    let mut done_stepping = false;
    let mut num_steps = 0;

    while !done_stepping {
        let next_pos = match advance_pos(&pos, dir) {
            Some(x) => x,
            None => {
                return (true, num_steps);
            }
        };

        let next_cell = match read_cell(&next_pos, map) {
            Some(x) => x,
            None => {
                assert_ne!(pos[1], 129);
                return (true, num_steps);
            }
        };

        match next_cell {
            CellType::Empty => {
                *next_cell = CellType::Visit;
                num_steps += 1;
                *pos = next_pos;
            }
            CellType::Crate => {
                *dir = (*dir + 1) % DIR_MAX;
                done_stepping = true;
            }
            CellType::Visit => {
                *pos = next_pos;
            }
        }
    }
    return (false, num_steps);
}

fn read_cell<'a>(pos: &[usize; 2], map: &'a mut Vec<Vec<CellType>>) -> Option<&'a mut CellType> {
    return map.get_mut(pos[0]).map_or(None, |row| row.get_mut(pos[1]));
}

fn advance_pos(pos: &[usize; 2], dir: &u8) -> Option<[usize; 2]> {
    let mut next_pos: [usize; 2] = pos.clone();

    match *dir {
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
        _ => unreachable!("Dir should be 0,1,2,3"),
    }

    return Some(next_pos);
}

fn read_map() -> (Vec<Vec<CellType>>, [usize; 2]) {
    let input = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file!");

    let lines = input.lines();

    let mut lines_parsed: Vec<Vec<CellType>> = Vec::new();

    let mut pos_opt: Option<[usize; 2]> = None;

    for (line_idx, line) in lines.enumerate() {
        lines_parsed.push(
            line.chars()
                .map(|c| match c {
                    '.' => CellType::Empty,
                    '#' => CellType::Crate,
                    '^' => CellType::Visit, // Count guard pos as first visited
                    x => panic!("unexpected char!!: {x}"),
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
