use std::collections::HashSet;
use std::collections::VecDeque;
use std::str::FromStr;

use common::grid2d::Grid2D;

#[derive(Debug)]
struct PuzzleInput(Grid2D<u8>, (usize, usize), (usize, usize), Vec<(usize, usize)>);

impl FromStr for PuzzleInput {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let mut rows = 0;
        let mut cols = 0;
        let mut start = (0, 0);
        let mut end = (0, 0);
        let mut data = vec![];
        let mut lowest_points = vec![];
        for (row, line) in s.lines().enumerate() {
            cols = line.len();
            for (col, c) in line.chars().enumerate() {
                match c {
                    'S' => {
                        start = (row, col);
                        lowest_points.push((row, col));
                        data.push(0);
                    },
                    'a' => {
                        lowest_points.push((row, col));
                        data.push(0);
                    }
                    'E' => {
                        end = (row, col);
                        data.push(25);
                    }
                    _ => data.push(c as u8 - b'a'),
                }
            }
            rows += 1;
        }

        let grid = Grid2D::from_values(rows, cols, &data);

        Ok(Self(grid, start, end, lowest_points))
    }
}

fn get_neighbours(rows: usize, cols: usize, coord: (usize, usize)) -> Vec<(usize, usize)> {
    match (coord.0, coord.1) {
        (0, 0) => vec![(0, 1), (1, 0)],
        (0, c) if c == cols - 1 => vec![(0, c - 1), (1, c)],
        (0, c) if c < cols - 1 => vec![(0, c + 1), (0, c - 1), (1, c)],
        (r, 0) if r == rows - 1 => vec![(r, 1), (r - 1, 0)],
        (r, c) if r == rows - 1 && c == cols - 1 => vec![(r - 1, c), (r, c - 1)],
        (r, c) if r == rows - 1 && c < cols - 1 => vec![(r, c + 1), (r - 1, c), (r, c - 1)],
        (r, 0) if r < rows - 1 => vec![(r, 1), (r - 1, 0), (r + 1, 0)],
        (r, c) if c == cols - 1 => vec![(r - 1, c), (r, c - 1), (r + 1, c)],
        (r, c) if r < rows - 1 && c < cols - 1 => {
            vec![(r, c + 1), (r - 1, c), (r, c - 1), (r + 1, c)]
        }
        _ => unreachable!(),
    }
}

fn main() {
    let puzzle_input = common::io::read_stdin::<PuzzleInput>();
    let elevations = puzzle_input.0;
    let mut path_lengths_from_peak = Grid2D::from_value(elevations.rows(), elevations.cols(), &u32::MAX);
    let start = puzzle_input.1;
    let end = puzzle_input.2;
    let lowest_points = puzzle_input.3;

    let mut seen = HashSet::from([end]);
    let mut coords = VecDeque::from([end]);
    let mut length = 0;

    let mut coords_to_enqueue = VecDeque::new();

    while !coords.is_empty() {
        for coord in coords.drain(..) {            
            path_lengths_from_peak.set(coord.0, coord.1, length);

            let elevation = elevations.get(coord.0, coord.1);

            for neighbour in get_neighbours(elevations.rows(), elevations.cols(), coord) {
                if seen.contains(&neighbour) {
                    continue;
                }

                let neighbour_elevation = elevations.get(neighbour.0, neighbour.1);

                if elevation > neighbour_elevation && elevation - neighbour_elevation > 1 {
                    continue;
                }

                seen.insert(neighbour);
                coords_to_enqueue.push_back(neighbour);
            }
        }

        length += 1;

        coords.append(&mut coords_to_enqueue);
    }

    let part_1 = path_lengths_from_peak.get(start.0, start.1);
    println!("{}", part_1);

    let part_2 = lowest_points.iter().map(|(r, c)| path_lengths_from_peak.get(*r, *c)).min().unwrap();
    println!("{}", part_2);

}
