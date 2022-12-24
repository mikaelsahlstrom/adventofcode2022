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
