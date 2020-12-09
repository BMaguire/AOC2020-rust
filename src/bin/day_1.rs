
use aoc2020_rust::input_parser::*;
use std::collections::HashMap;


fn main() {
    println!("Running AOC day1");
    if let Ok(contents) = file_reader::read_file("inputs/input_day_1.dat") {

        let string_values = file_reader::split_by(&contents, "\n");
        let values: Vec<i64> = string_values.into_iter().map(|value| value.parse::<i64>().unwrap()).collect();
        let other_values = values.clone();
        if let Some((v1, v2)) = part_1(values) {
            println!("Day1 Part1 Answer: {:?}", v1 * v2);

        } else {
            println!("Not found :(");
        };
        
        if let Some((v1,v2,v3)) = part_2(other_values) {
            println!("Day1 Part2 Answer: {:?}", v1 * v2 * v3);

        } else {
            println!("Not found :(");
        };

    } else {
        println!("Something went wrong");
    }
}

fn part_1(values: Vec<i64>) -> Option<(i64, i64)> {
    // find the 2 entries that sum to 2020

    // store each values distance from 2020 in a map
    let mut difference_map: HashMap<i64, i64> = HashMap::new();

    // lookup in the map if the current value matches that distance
    for value in values {
        let key = 2020 - value;
        match difference_map.get(&key) {
            Some(x) => {
                return Some((key, *x))
            }
            _ => {
                difference_map.insert(value, key);
            }
        } 
    }
    
    println!("{:?}", difference_map);
    None
}

fn part_2(values: Vec<i64>) -> Option<(i64, i64, i64)> {
    // find 3 values that sum to 2020
    // bad solution
    for index_1 in 0..values.len() {
        for index_2 in 0..values.len() {
            for index_3 in 0..values.len() {
                match values[index_1] + values[index_2] + values[index_3] {
                    2020 => {
                        return Some((values[index_1],values[index_2],values[index_3]))
                    }
                    _ => {} 
                }
            }
        }
    }
    
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input= vec![1721, 979, 366, 299, 675, 1456];
        
        assert_eq!(part_1(input), Some((1721, 299)))
    }
    
    fn test_p2() {
        let input= vec![1721, 979, 366, 299, 675, 1456];
        
        assert_eq!(part_2(input), Some((979, 366, 675)));
        
    }
} 