use std::collections::HashMap;

fn main() {
    let map = HashMap::from([
        ("A X", 1 + 3),
        ("A Y", 2 + 6),
        ("A Z", 3 + 0),
        ("B X", 1 + 0),
        ("B Y", 2 + 3),
        ("B Z", 3 + 6),
        ("C X", 1 + 6),
        ("C Y", 2 + 0),
        ("C Z", 3 + 3)
    ]);


    let mut sum = 0;

    for line in std::fs::read_to_string("./input").unwrap().lines() {
        sum += map[line];
    }

    println!("sum: {}", sum);



    let map2 = HashMap::from([
        ("A X", 3),
        ("A Y", 4),
        ("A Z", 8),
        ("B X", 1),
        ("B Y", 5),
        ("B Z", 9),
        ("C X", 2),
        ("C Y", 6),
        ("C Z", 7)
    ]);

    let mut sum2 = 0;

    for line in std::fs::read_to_string("./input").unwrap().lines() {
        sum2 += map2[line];
    }

    println!("sum: {}", sum2);
}
