use std::cmp::max;

use common::{complex::Complex, grid2d::Grid2D};

#[derive(Debug, Clone, Copy)]
enum Material {
    Air,
    Rock,
    Sand,
}

fn segment(z0: &Complex<i32>, z1: &Complex<i32>) -> Vec<Complex<i32>> {
    let mut points = vec![*z0];
    let w = *z1 - *z0;
    let u = Complex::new(w.re().signum(), w.im().signum());

    while let Some(tail) = points.last() {
        if *tail == *z1 {
            break;
        }

        points.push(*tail + u);
    }

    points
}

fn cave_get(cave: &Grid2D<Material>, z: Complex<i32>) -> Material {
    *cave.get(z.im() as usize, z.re() as usize)
}

fn cave_set(cave: &mut Grid2D<Material>, z: Complex<i32>, material: Material) {
    cave.set(z.im() as usize, z.re() as usize, material)
}

fn will_fall_into_abyss(max_depths: &Vec<Option<i32>>, z: Complex<i32>) -> bool {
    if let Some(max_depth) = max_depths[z.re() as usize] {
        z.im() > max_depth
    } else {
        true
    }
}

fn simulate(structures: &Vec<Vec<Complex<i32>>>) -> usize {
    let up = Complex::new(0, 1);
    let up_left = Complex::new(-1, 1);
    let up_right = Complex::new(1, 1);

    let mut max_depths = vec![None; 1 << 10];
    let mut cave = Grid2D::<Material>::from_value(1 << 9, 1 << 10, &Material::Air);

    for endpoints in structures {
        for pair in endpoints.windows(2) {
            for point in segment(&pair[0], &pair[1]) {
                let max_depth = max_depths.get_mut(point.re() as usize).unwrap();

                *max_depth = Some(max_depth.map_or(point.im(), |v| max(v, point.im())));

                cave_set(&mut cave, point, Material::Rock);
            }
        }
    }

    let origin = Complex::new(500, 0);
    let mut placed = 0;

    loop {
        let mut z = origin;

        let mut has_moved = false;
        let mut is_falling_into_abyss = false;

        loop {
            match (
                cave_get(&cave, z + up),
                will_fall_into_abyss(&max_depths, z + up),
                cave_get(&cave, z + up_left),
                will_fall_into_abyss(&max_depths, z + up_left),
                cave_get(&cave, z + up_right),
                will_fall_into_abyss(&max_depths, z + up_right),
            ) {
                (Material::Air, false, _, _, _, _) => {
                    z += up;
                    has_moved = true
                }
                (Material::Air, true, _, _, _, _) => {
                    is_falling_into_abyss = true;
                    break;
                }
                (_, _, Material::Air, false, _, _) => {
                    z += up_left;
                    has_moved = true
                }
                (_, _, Material::Air, true, _, _) => {
                    is_falling_into_abyss = true;
                    break;
                }
                (_, _, _, _, Material::Air, false) => {
                    z += up_right;
                    has_moved = true
                }
                (_, _, _, _, Material::Air, true) => {
                    is_falling_into_abyss = true;
                    break;
                }
                _ => break,
            }
        }

        match (has_moved, cave_get(&cave, z), is_falling_into_abyss) {
            (true, _, false) | (false, Material::Air, _) => {
                cave_set(&mut cave, z, Material::Sand);
                placed += 1;
            }
            _ => {
                break;
            }
        }
    }

    placed
}

fn main() {
    let mut structures = common::io::map_stdin_lines_to_vec(|line| {
        let mut points = vec![];
        for pair in line.split(" -> ") {
            let components = pair.split(',').collect::<Vec<_>>();

            let re = components[0].parse::<i32>().unwrap();
            let im = components[1].parse::<i32>().unwrap();

            points.push(Complex::new(re, im));
        }

        points
    });

    let part_1 = simulate(&structures);
    println!("{}", part_1);

    let max_segment_depth = structures
        .iter()
        .map(|v| v.iter().map(|z| z.im()).max().unwrap())
        .max()
        .unwrap();
    println!("{}", max_segment_depth);
    structures.push(vec![
        Complex::new(0, max_segment_depth + 2),
        Complex::new((1 << 10) - 1, max_segment_depth + 2),
    ]);
    let part_2 = simulate(&structures);
    println!("{}", part_2);
}
