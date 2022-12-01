use std::{io::{self, Read}, str::FromStr, fmt::Debug};

pub fn read_stdin_lines_to_vec<T: FromStr + Debug>()  -> Vec<T> where <T as FromStr>::Err: Debug {
    io::stdin().lines().map(|line| line.expect("Unable to read line").parse::<T>().expect("Unable to parse line")).collect()
}

pub fn collect_stdin_lines<T: FromStr + Debug>(delimiter: &str) -> Vec<Vec<T>> where <T as FromStr>::Err: Debug {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).expect("Unable to read stdin to string");
    buf.split(delimiter).map(|group| group.lines().map(|line| line.parse().expect("Unable to parse line")).collect()).collect()
}

pub fn map_stdin_lines_to_vec<F: Fn(String) -> Result<T, U>, T, U: Debug>(f: F) -> Vec<T> {
    io::stdin().lines().map(|line| f(line.expect("Unable to read line")).expect("Unable to parse line")).collect()
}

pub fn read_stdin<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).expect("Unable to read stdin to string");
    buf.parse().expect("Unable to parse string")
}