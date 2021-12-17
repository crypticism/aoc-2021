use std::{fs, str::FromStr};

pub type Coord = (usize, usize);

pub fn read_file<T>(path: &str, delimiter: char) -> Result<Vec<T>, Box<dyn::std::error::Error>>
where
    T: Default + FromStr,
{
    let contents: Vec<T> = fs::read_to_string(path)?
        .split(delimiter)
        .map(|v| v.parse::<T>().unwrap_or(Default::default()))
        .collect();
    Ok(contents)
}

pub fn create_matrix<T>(input: &[String]) -> Vec<Vec<T>>
where
    T: Default + Clone + FromStr,
    <T as FromStr>::Err: ::std::fmt::Debug,
{
    let mut matrix: Vec<Vec<T>> = Vec::new();
    for row in input {
        let values: Vec<T> = row
            .split("")
            .filter(|val| !val.is_empty())
            .map(|val| val.parse::<T>().unwrap())
            .collect();
        matrix.push(values);
    }
    pivot_matrix(matrix)
}

pub fn initialize_matrix<T: Default + Clone>(max_x: usize, max_y: usize) -> Vec<Vec<T>> {
    let mut map = Vec::with_capacity(max_y);

    for i in 0..max_y {
        map.push(Vec::with_capacity(max_x));
        for _ in 0..max_x {
            map[i].push(T::default());
        }
    }
    pivot_matrix(map)
}

pub fn pivot_matrix<T: Default + Clone>(input: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut output: Vec<Vec<T>> = Vec::with_capacity(input[0].len());

    for x in 0..input[0].len() {
        output.push(Vec::with_capacity(input.len()));
        for _ in 0..input.len() {
            output[x].push(T::default());
        }
    }

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            output[x][y] = input[y][x].clone();
        }
    }

    output
}

#[allow(dead_code)]
pub fn print_matrix<T: ::std::fmt::Debug + Clone>(grid: &Vec<Vec<T>>) {
    let mut output_string = String::new();
    for y in 0..grid[0].len() {
        let mut output = Vec::new();

        for x in 0..grid.len() {
            output.push(grid[x][y].clone());
        }

        let row: String = output
            .into_iter()
            .map(|value| format!("{:?}", value))
            .collect();

        output_string.push_str(&row);
        output_string.push('\n');
    }
    println!("{}", output_string);
}

#[allow(dead_code)]
pub fn find_neighbors_diagonal<T>(grid: &Vec<Vec<T>>, coord: Coord) -> Vec<Coord> {
    let (x, y) = coord;

    let max_x = grid.len() - 1;
    let max_y = grid[0].len() - 1;

    let mut output = vec![];

    // up-left
    if x > 0 && y > 0 {
        output.push((x - 1, y - 1));
    }

    // up
    if y > 0 {
        output.push((x, y - 1));
    }

    // up-right
    if x < max_x && y > 0 {
        output.push((x + 1, y - 1));
    }

    // right
    if x < max_x {
        output.push((x + 1, y));
    }

    // down-right
    if x < max_x && y < max_y {
        output.push((x + 1, y + 1));
    }

    // down
    if y < max_y {
        output.push((x, y + 1));
    }

    // down-left
    if x > 0 && y < max_y {
        output.push((x - 1, y + 1));
    }

    // left
    if x > 0 {
        output.push((x - 1, y));
    }

    output
}

#[allow(dead_code)]
pub fn find_neighbors<T>(grid: &Vec<Vec<T>>, coord: Coord) -> Vec<Coord> {
    let (x, y) = coord;

    let max_x = grid.len() - 1;
    let max_y = grid[0].len() - 1;

    let mut output = vec![];

    // up
    if y > 0 {
        output.push((x, y - 1));
    }

    // right
    if x < max_x {
        output.push((x + 1, y));
    }

    // down
    if y < max_y {
        output.push((x, y + 1));
    }

    // left
    if x > 0 {
        output.push((x - 1, y));
    }

    output
}
