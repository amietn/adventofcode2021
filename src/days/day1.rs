use crate::util::read_lines;

pub fn day1() {
    if let Ok(lines) = read_lines("src/inputs/1.txt") {
        // convert lines to int
        let integers: Vec<i32> = lines.map(|s| {
            let i: i32 = s.unwrap().parse().unwrap();
            i
        }).collect();

        // part 1
        let mut increases = 0;

        let mut previous = integers[0];
        for i in &integers[1..] {
            if i > &previous {
                increases += 1;
            }

            previous = *i;
        }

        println!("Increases: {}", increases);

        // part 2
        let mut a = integers[0];
        let mut b = integers[1];
        let mut c = integers[2];

        let mut three_increases = 0;

        for i in &integers[3..] {
            let prev_sum = a + b + c;
            let sum = b + c + i;

            if prev_sum < sum {
                three_increases += 1;
            }

            a = b;
            b = c;
            c = *i;
        }

        println!("Three increases: {}", three_increases);
    }
}