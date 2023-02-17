fn read_input() -> Vec<Vec<i32>> {
    let mut groups = Vec::new();
    let mut group = Vec::new();
    for line in std::fs::read_to_string("./input").unwrap().lines() {
        if line.is_empty() {
            groups.push(group);
            group = Vec::new();
        } else {
            group.push(line.parse().unwrap());
        }
    }
    groups.push(group);
    groups
}

fn main() {
    let groups = read_input();
    let mut max = 0;
    for group in groups.iter() {
        let sum: i32 = group.iter().sum();
        if sum > max {
            max = sum;
        }
    }
    println!("max sum is {}", max);


    // sort the groups sums and get sum of 3 largest

    let mut sums = Vec::<i32>::new();
    for group in groups.iter() {
        sums.push(group.iter().sum());
    }
    sums.sort();
    sums.reverse();
    let sum = sums[0] + sums[1] + sums[2];


    println!("sum of top 3 is {}", sum);
}
