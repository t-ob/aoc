use std::cmp::max;

fn visible_from_outside(row: &[u32]) -> Vec<bool> {
    let mut curr_max_left = row[0];
    let mut result_left = vec![true];

    for x in row.iter().skip(1) {
        result_left.push(*x > curr_max_left);
        curr_max_left = max(curr_max_left, *x);
    }

    let mut curr_max_left = *row.last().unwrap();
    let mut result_right = vec![true];

    for x in row.iter().rev().skip(1) {
        result_right.push(*x > curr_max_left);
        curr_max_left = max(curr_max_left, *x);
    }

    result_left
        .iter()
        .zip(result_right.iter().rev())
        .map(|(b1, b2)| *b1 || *b2)
        .collect()
}

fn scenic_score_left(row: &[u32]) -> Vec<u32> {
    let mut last_max = vec![0; row.len()];

    let mut curr_col = row.len() - 2;
    let mut heights = vec![row.len() - 1];
    loop {
        while !heights.is_empty() && row[curr_col] >= row[*heights.last().unwrap()] {
            let smaller_height_col = heights.pop().unwrap();
            last_max[smaller_height_col] = curr_col;
        }

        heights.push(curr_col);

        if curr_col == 0 {
            break;
        }
        curr_col -= 1;
    }

    let mut result = vec![0; row.len()];
    let mut i = 1;
    while i < row.len() {
        result[i] = (i - last_max[i]) as u32;
        i += 1;
    }

    result
}

fn scenic_score(row: &[u32]) -> Vec<u32> {
    let mut buf = Vec::from(row);
    let left = scenic_score_left(&buf);
    buf.reverse();
    let right = scenic_score_left(&buf);

    left.iter()
        .zip(right.iter().rev())
        .map(|(l, r)| *l * *r)
        .collect()
}

fn main() {
    let grid =
        common::grid2d::Grid2D::<u32>::from_str_delimeted(&common::io::read_stdin::<String>(), "");
    let transposed = grid.transpose();

    let visible_rows = grid.map_rows(visible_from_outside);
    let visible_columns = transposed.map_rows(visible_from_outside).transpose();
    let visible = visible_rows | visible_columns;

    let part_1 = visible.values().iter().filter(|b| **b).count();

    println!("{}", part_1);

    let scenic_scored_rows = grid.map_rows(scenic_score);
    let scenic_scored_columns = transposed.map_rows(scenic_score).transpose();
    let scenic_scores = scenic_scored_rows * scenic_scored_columns;

    let part_2 = scenic_scores.values().iter().max().unwrap();

    println!("{}", part_2);
}
