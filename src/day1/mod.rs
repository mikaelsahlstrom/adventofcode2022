use crate::utils;

pub fn part1()
{
    if let Ok(lines) = utils::read_lines("./src/day1/input")
    {
        let mut largest_elf = 0;
        let mut elf: u32 = 0;

        for line in lines
        {
            if let Ok(l) = line
            {
                if l == ""
                {
                    if elf > largest_elf
                    {
                        largest_elf = elf;
                    }

                    elf = 0;
                }
                else
                {
                    elf += l.parse::<u32>().unwrap();
                }
            }
        }

        println!("part 1: {}", largest_elf);
    }
}

pub fn part2()
{
    if let Ok(lines) = utils::read_lines("./src/day1/input")
    {
        let mut largest: Vec<u32> = vec![0, 0, 0];
        let mut elf: u32 = 0;

        for line in lines
        {
            if let Ok(l) = line
            {
                if l == ""
                {
                    for i in 0..largest.len()
                    {
                        if elf > largest[i]
                        {
                            largest.insert(i, elf);
                            largest.pop();
                            break;
                        }
                    }

                    elf = 0;
                }
                else
                {
                    elf += l.parse::<u32>().unwrap();
                }
            }
        }

        println!("part 2: {}", largest.iter().sum::<u32>());
    }
}
