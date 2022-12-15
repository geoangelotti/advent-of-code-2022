pub mod utils {

    use std::fs::File;
    use std::io::{prelude::*, BufReader};

    pub fn get_lines(path: &str) -> Vec<String> {
        let file = File::open(path).unwrap();
        BufReader::new(file)
            .lines()
            .map(|line| line.unwrap())
            .collect::<Vec<String>>()
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
