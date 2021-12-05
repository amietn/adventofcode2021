use crate::util::read_lines;

pub fn day2() {
    if let Ok(lines) = read_lines("src/inputs/2.txt") {
        let instructions: Vec<(String, i32)> = lines.map(|line| {
            let l = line.unwrap();
            let splits: Vec<&str> = l.split(" ").collect();
            let direction = String::from(splits[0]);
            let amount: i32 = String::from(splits[1]).parse().unwrap();
            (direction, amount)
        }).collect();

        // part 1
        let mut horizontal = 0;
        let mut depth = 0;

        for (direction, amount) in &instructions {
            match (direction.as_str(), amount) {
                ("forward", x) => horizontal += x,
                ("up", x) => depth -= x,
                ("down", x) => depth += x,
                (_, _) => (),
            }
        }

        println!("Final position: ({}, {})", horizontal, depth);
        println!("Multiplication: {}", horizontal * depth);

        // part 2
        horizontal = 0;
        depth = 0;
        let mut aim = 0;

        for (direction, amount) in &instructions {
            match (direction.as_str(), amount) {
                ("forward", x) => {
                    horizontal += x;
                    depth += aim * x;
                }
                ("up", x) => aim -= x,
                ("down", x) => aim += x,
                (_, _) => (),
            }
        }

        println!("Final position: ({}, {})", horizontal, depth);
        println!("Multiplication: {}", horizontal * depth);
    }
}