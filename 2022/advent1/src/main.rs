use std::io::{self, BufRead};

fn main() -> io::Result<()>
{
    let mut lines = io::stdin().lock().lines();
    let mut elf_cals = Vec::new();
    let mut current_elf = 0;
    let mut max_cal = 0;
    let mut max_cal2 = 0;
    let mut max_cal3 = 0;

    while let Some(line) = lines.next()
    {
        let input = line.unwrap();

        if input.len() == 0
        {
            if max_cal < current_elf
            {
                max_cal3 = max_cal2;
                max_cal2 = max_cal;
                max_cal = current_elf;
            }
            else if max_cal2 < current_elf
            {
                max_cal3 = max_cal2;
                max_cal2 = current_elf;
            }
            else if max_cal3 < current_elf
            {
                max_cal3 = current_elf
            }

            elf_cals.push(current_elf);
            current_elf = 0;
        }
        else
        {
            current_elf += input.parse::<isize>().unwrap();
        }
    }

    println!("{}", max_cal);
    println!("{}", max_cal + max_cal2 + max_cal3);

    Ok(())
}
