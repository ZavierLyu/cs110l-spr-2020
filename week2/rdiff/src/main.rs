use grid::Grid; // For lcs()
use std::env;
use std::fs::File; // For read_file_lines()
use std::io::{self, BufRead}; // For read_file_lines()
use std::process;
use std::cmp::max;

pub mod grid;

/// Reads the file at the supplied path, and returns a vector of strings.
// #[allow(unused)] // TODO: delete this line when you implement this function
fn read_file_lines(filename: &String) -> Result<Vec<String>, io::Error> {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(err) => return Err(err),
    };
    let mut vec_line_str: Vec<String> = Vec::new();
    for line in io::BufReader::new(file).lines() {
        let line_str = line?;
        vec_line_str.push(line_str);
    }
    Ok(vec_line_str)
    // Be sure to delete the #[allow(unused)] line above
}

// #[allow(unused)] // TODO: delete this line when you implement this function
fn lcs(seq1: &Vec<String>, seq2: &Vec<String>) -> Grid {
    // Note: Feel free to use unwrap() in this code, as long as you're basically certain it'll
    // never happen. Conceptually, unwrap() is justified here, because there's not really any error
    // condition you're watching out for (i.e. as long as your code is written correctly, nothing
    // external can go wrong that we would want to handle in higher-level functions). The unwrap()
    // calls act like having asserts in C code, i.e. as guards against programming error.
    let (m,n) = (seq1.len(), seq2.len());
    let mut dp: Grid = Grid::new(m+1, n+1);
    for i in 1..=m {
        for j in 1..=n {
            if seq1[i-1] == seq2[j-1] {
                dp.set(i, j, dp.get(i-1,j-1).unwrap() + 1).unwrap();
            } else {
                dp.set(i, j, max(dp.get(i-1, j).unwrap(), dp.get(i, j-1).unwrap())).unwrap();
            }
        }
    }
    dp
    // Be sure to delete the #[allow(unused)] line above
}

// #[allow(unused)] // TODO: delete this line when you implement this function
fn print_diff(lcs_table: &Grid, lines1: &Vec<String>, lines2: &Vec<String>, i: usize, j: usize) {
    if i > 0 && j > 0 && lines1[i - 1] == lines2[j - 1] {
        print_diff(lcs_table, lines1, lines2, i-1, j-1);
        println!("  {}", lines1[i - 1]);
    } else if j > 0 && (i == 0 || lcs_table.get(i, j-1).unwrap() >= lcs_table.get(i-1, j).unwrap()) {
        print_diff(lcs_table, lines1, lines2, i, j-1);
        println!("> {}", lines2[j - 1]);
    } else if i > 0 && (j == 0 || lcs_table.get(i, j-1).unwrap() < lcs_table.get(i-1, j).unwrap()) {
        print_diff(lcs_table, lines1, lines2, i-1, j);
        println!("< {}", lines1[i - 1]);
    } else {
        println!();
    }
    // Be sure to delete the #[allow(unused)] line above
}

// #[allow(unused)] // TODO: delete this line when you implement this function
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename1 = &args[1];
    let filename2 = &args[2];

    let file_text1 = read_file_lines(&filename1).expect("read file lines");
    let file_text2 = read_file_lines(&filename2).expect("read file lines");
    let lcs_grid = lcs(&file_text1, &file_text2);
    print_diff(&lcs_grid, &file_text1, &file_text2, file_text1.len(), file_text2.len());
    // Be sure to delete the #[allow(unused)] line above
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_file_lines() {
        let lines_result = read_file_lines(&String::from("handout-a.txt"));
        assert!(lines_result.is_ok());
        let lines = lines_result.unwrap();
        assert_eq!(lines.len(), 8);
        assert_eq!(
            lines[0],
            "This week's exercises will continue easing you into Rust and will feature some"
        );
    }

    #[test]
    fn test_lcs() {
        let mut expected = Grid::new(5, 4);
        expected.set(1, 1, 1).unwrap();
        expected.set(1, 2, 1).unwrap();
        expected.set(1, 3, 1).unwrap();
        expected.set(2, 1, 1).unwrap();
        expected.set(2, 2, 1).unwrap();
        expected.set(2, 3, 2).unwrap();
        expected.set(3, 1, 1).unwrap();
        expected.set(3, 2, 1).unwrap();
        expected.set(3, 3, 2).unwrap();
        expected.set(4, 1, 1).unwrap();
        expected.set(4, 2, 2).unwrap();
        expected.set(4, 3, 2).unwrap();

        println!("Expected:");
        expected.display();
        let result = lcs(
            &"abcd".chars().map(|c| c.to_string()).collect(),
            &"adb".chars().map(|c| c.to_string()).collect(),
        );
        println!("Got:");
        result.display();
        assert_eq!(result.size(), expected.size());
        for row in 0..expected.size().0 {
            for col in 0..expected.size().1 {
                assert_eq!(result.get(row, col), expected.get(row, col));
            }
        }
    }
}
