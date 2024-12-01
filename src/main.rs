use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn p1_1() {
    let mut l1: Vec<i32> = Vec::new();
    let mut l2: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines("./src/Day-1-Historian-Hysteria/input"){
        for line in lines.flatten() {
            l1.push(line.split("   ").collect::<Vec<&str>>()[0].parse::<i32>().unwrap());
            l2.push(line.split("   ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap());
        }
    }
    l1.sort();
    l2.sort();
    let mut diff: i32 = 0;
    println!("{}", l2.len());
    for i in 0..l1.len() {
        println!("{}", i);
        diff += i32::abs(l1[i] - l2[i]);
    }
    println!("{}", diff);
}

fn p1_2() {
    let mut l1: Vec<i32> = Vec::new();
    let mut l2: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines("./src/Day-1-Historian-Hysteria/input"){
        for line in lines.flatten() {
            l1.push(line.split("   ").collect::<Vec<&str>>()[0].parse::<i32>().unwrap());
            l2.push(line.split("   ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap());
        }
    }
    l1.sort();
    l2.sort();
    let mut ans = 0;
    let frequencies = l2 // https://stackoverflow.com/a/70234563
          .iter()
          .copied()
          .fold(HashMap::new(), |mut map, val|{
              map.entry(val)
                 .and_modify(|frq|*frq+=1)
                 .or_insert(1);
              map
          });
    for val in l1.iter(){
        ans += frequencies.get(val).unwrap_or(&0) * val;
    }
    println!("{}", ans);
}

fn main(){
    p1_1();
    p1_2();
}