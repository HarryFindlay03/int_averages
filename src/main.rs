//Given a list of integers, use a vector and return the mean (the average value), 
//median (when sorted, the value in the middle position),
//and mode (the value that occurs most often; a hash map will be helpful here) of the list.
use std::collections::HashMap;
fn main() {
    //let mut ints = vec![5, 7, 4, 3, 1, 6, 7];
    //let mut ints = vec![8, 8, 8, 9, 9, 9, 9];
    let mut ints = vec![9, 5, 6, 7, 1, 3, 4, 6, 7, 1, 3, 5, 6, 7, 3, 6, 7, 2, 4, 6, 7];
    println!("List of integers: {:?}", ints);
    //MEAN
    let _mean = get_mean(&ints);
    //println!("MEAN: {}", mean);

    //MODE
    let mode = get_mode(&ints);
    println!("MODE: {:?}", mode);

    //MEDIAN
    let _median = get_median(&mut ints);
    //println!("MEDIAN: {}", median);
}

fn get_mean(ints: &Vec<usize>) -> usize {
    let mut total = 0;
    for num in ints {
        total += num;
    }
    let mean = total / ints.len();
    mean //return value
}

fn get_mode(ints: &Vec<usize>) -> usize {
    let mut mode_map = HashMap::new();
    //Counting how many times a number is used
    for value in ints {
        let count = mode_map.entry(value).or_insert(0);
        *count += 1;
    }

    //Finding the greatest number and finding its key
    let mut mode_val = 0;
    let mut mode_key = 0;
    for(key, val) in mode_map.into_iter() {
        if val > mode_val {
            mode_key = *key;
            mode_val = val;
        }
    }

    mode_key
}

//mutable borrow so vec does not get sorted and then handed back so the other functions don't end up using a sorted list
fn get_median(ints: &mut Vec<usize>) -> usize {
    ints.sort();
    let mid = ints.len() / 2;
    ints[mid]
}

