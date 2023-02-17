use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let map = HashMap::from([
        ("a", 1),
        ("b", 2),
        ("c", 3),
        ("d", 4),
        ("e", 5),
        ("f", 6),
        ("g", 7),
        ("h", 8),
        ("i", 9),
        ("j", 10),
        ("k", 11),
        ("l", 12),
        ("m", 13),
        ("n", 14),
        ("o", 15),
        ("p", 16),
        ("q", 17),
        ("r", 18),
        ("s", 19),
        ("t", 20),
        ("u", 21),
        ("v", 22),
        ("w", 23),
        ("x", 24),
        ("y", 25),
        ("z", 26),
        ("A", 27),
        ("B", 28),
        ("C", 29),
        ("D", 30),
        ("E", 31),
        ("F", 32),
        ("G", 33),
        ("H", 34),
        ("I", 35),
        ("J", 36),
        ("K", 37),
        ("L", 38),
        ("M", 39),
        ("N", 40),
        ("O", 41),
        ("P", 42),
        ("Q", 43),
        ("R", 44),
        ("S", 45),
        ("T", 46),
        ("U", 47),
        ("V", 48),
        ("W", 49),
        ("X", 50),
        ("Y", 51),
        ("Z", 52)
    ]);

    let mut sum = 0;

    for line in std::fs::read_to_string("./input").unwrap().lines() {
        let (a, b) = line.split_at(line.len() / 2);
        let a: HashSet<_> = a.chars().collect();
        let b: HashSet<_> = b.chars().collect();
        let intersection: HashSet<_> = a.intersection(&b).collect();
        let c = intersection.iter().next().unwrap();

        sum += map.get(c.to_string().as_str()).unwrap();
    }

    println!("{}", sum);


    let input = std::fs::read_to_string("./input").unwrap();
    let mut lines = input.lines();
    let mut sum2 = 0;

    while let Some(a) = lines.next() {
        let b = lines.next().unwrap();
        let c = lines.next().unwrap();

        let a: HashSet<char> = a.chars().collect();
        let b: HashSet<char> = b.chars().collect();
        let c: HashSet<char> = c.chars().collect();

        let intersection: HashSet<char> = [a.clone(), b, c].iter()
            .fold(a, |acc, x| acc.intersection(x).cloned().collect());

        let c = intersection.iter().next().unwrap();
        sum2 += map.get(c.to_string().as_str()).unwrap();
    }

    println!("{}", sum2);
}
