pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let height = minefield.len();

    if height == 0 {
        return vec![];
    }

    let width = minefield[0].len();
    if width == 0 {
        return vec![String::new(); height];
    }

    let mut result = vec![vec![' '; width]; height];

    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for row in 0..height {
        for col in 0..width {
            if minefield[row].as_bytes()[col] == b'*' {
                result[row][col] = '*';
                continue;
            }

            let mut mine_count = 0;
            for &(dx, dy) in &directions {
                let new_row = row as isize + dx;
                let new_col = col as isize + dy;

                if new_row >= 0
                    && new_row < height as isize
                    && new_col >= 0
                    && new_col < width as isize
                {
                    if minefield[new_row as usize].as_bytes()[new_col as usize] == b'*' {
                        mine_count += 1;
                    }
                }
            }

            if mine_count > 0 {
                result[row][col] = (b'0' + mine_count as u8) as char;
            }
        }
    }

    result
        .into_iter()
        .map(|row| row.iter().collect())
        .collect::<Vec<String>>()
}
