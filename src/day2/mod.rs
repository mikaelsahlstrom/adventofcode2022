use crate::utils;

// A = rock
// B = paper
// C = scissors

// X = rock
// Y = paper
// Z = scissors

fn score(them: &str, us: &str) -> u32
{
    let score = [
        ("A", "X", 3 + 1), ("A", "Y", 6 + 2), ("A", "Z", 0 + 3),
        ("B", "X", 0 + 1), ("B", "Y", 3 + 2), ("B", "Z", 6 + 3),
        ("C", "X", 6 + 1), ("C", "Y", 0 + 2), ("C", "Z", 3 + 3)
    ];

    score.iter().find(|&i| i.0 == them && i.1 == us).unwrap().2
}

pub fn part1()
{
    if let Ok(lines) = utils::read_lines("./src/day2/input")
    {
        let mut my_score: u32 = 0;
        for line in lines
        {
            if let Ok(l) = line
            {
                let mut parts = l.split_whitespace();
                my_score += score(parts.next().unwrap(), parts.next().unwrap());
            }
        }

        println!("part 1: {}", my_score);
    }
}

// X = lose
// Y = draw
// Z = win

fn get_us<'a>(them: &'a str, what: &'a str) -> &'a str
{
    let us = [
        ("A", "X", "Z"), ("A", "Y", "X"), ("A", "Z", "Y"),
        ("B", "X", "X"), ("B", "Y", "Y"), ("B", "Z", "Z"),
        ("C", "X", "Y"), ("C", "Y", "Z"), ("C", "Z", "X")
    ];

    us.iter().find(|&i| i.0 == them && i.1 == what).unwrap().2
}

pub fn part2()
{
    if let Ok(lines) = utils::read_lines("./src/day2/input")
    {
        let mut my_score: u32 = 0;
        for line in lines
        {
            if let Ok(l) = line
            {
                let mut parts = l.split_whitespace();
                let them = parts.next().unwrap();
                let us = get_us(them, parts.next().unwrap());
                my_score += score(them, us);
            }
        }

        println!("part 2: {}", my_score);
    }
}
