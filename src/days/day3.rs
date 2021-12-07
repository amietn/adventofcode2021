use crate::util::read_lines;

pub fn day3() {
    if let Ok(lines) = read_lines("src/inputs/3.txt") {
        let numbers: Vec<String> = lines.map(|line| {
            let l = line.unwrap();
            String::from(l)
        }).collect();

        // part 1
        let mut one_counts = vec![0; 12];
        let mut zero_counts = vec![0; 12];
        for number in numbers.clone() {
            for (i, bit) in number.chars().enumerate() {
                match bit {
                    '0' => zero_counts[i] += 1,
                    '1' => one_counts[i] += 1,
                    _ => (),
                }
            }
        }

        let mut gamma_rate = 0;
        let mut epsilon_rate = 0;
        let mut exponent = 11_i32;
        for (ones, zeroes) in one_counts.iter().zip(zero_counts) {
            match (ones, zeroes) {
                (a, b) if *a >= b => gamma_rate += 2_i32.pow(exponent as u32),
                (a, b) if *a < b => epsilon_rate += 2_i32.pow(exponent as u32),
                _ => (),
            };
            exponent -= 1;
        }

        println!("Gamma rate: {}", gamma_rate);
        println!("Espislon rate: {}", epsilon_rate);
        println!("Submarine power consumption: {}", gamma_rate * epsilon_rate);


        // part 2
        let mut rating_a_numbers = numbers.clone();
        let mut rating_b_numbers = numbers.clone();

        let mut i: usize = 0;
        while rating_a_numbers.len() > 1 {
            let most_common = most_common_bit(rating_a_numbers.clone(), i, '1');
            rating_a_numbers.retain(|n| {
                n.chars().nth(i).unwrap() == most_common
            });
            i += 1;
        }

        i = 0;
        while rating_b_numbers.len() > 1 {
            let most_common = most_common_bit(rating_b_numbers.clone(), i, '0');
            rating_b_numbers.retain(|n| {
                n.chars().nth(i).unwrap() == most_common
            });
            i += 1;
        }

        println!("rating a: {}", rating_a_numbers[0]);
        println!("rating b: {}", rating_b_numbers[0]);
        let oxygen_generator_rating = i32::from_str_radix(&rating_a_numbers[0], 2).unwrap();
        let co2_scrubber_rating = i32::from_str_radix(&rating_b_numbers[0], 2).unwrap();
        println!("Oxygen generator rating: {}", oxygen_generator_rating);
        println!("CO2 scrubber rating: {}", co2_scrubber_rating);
        println!("Submarine life support rating: {}", oxygen_generator_rating * co2_scrubber_rating);
    }
}

fn most_common_bit(numbers: Vec<String>, i: usize, priority: char) -> char {
    let mut ones = 0;
    let mut zeroes = 0;
    for number in &numbers {
        match number.chars().nth(i).unwrap() {
            '0' => zeroes += 1,
            '1' => ones += 1,
            _ => (),
        }
    }

    match (ones, zeroes) {
        (a, b) if a > b && priority == '1' => '1',
        (a, b) if a > b && priority == '0' => '0',
        (a, b) if b > a && priority == '1' => '0',
        (a, b) if b > a && priority == '0' => '1',
        (a, b) if a == b => priority,
        _ => '0'
    }
}