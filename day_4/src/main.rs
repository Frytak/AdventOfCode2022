use std::fs;

fn main() {
    let input_file_path = "C:\\Users\\fryta\\Pulpit\\~\\Important\\Programming Projects\\AdventOfCode2022\\day_4\\src\\input.txt";
    let input = fs::read_to_string(input_file_path)
        .expect("Something went wrong reading the file");

    let mut fully_containing: u32 = 0;
    let mut containing: u32 = 0;
    for line in input.lines() {
        // Full string of the range
        let first_elf_fstr = line.get(..line.find(",").unwrap());
        let second_elf_fstr = line.get(line.find(",").unwrap()+1..);

        let first_elf = (
            str::parse::<u32>(first_elf_fstr.unwrap().get(..first_elf_fstr.unwrap().find("-").unwrap()).unwrap()).unwrap(), // First number
            str::parse::<u32>(first_elf_fstr.unwrap().get(first_elf_fstr.unwrap().find("-").unwrap()+1..).unwrap()).unwrap() // Second number
        );

        let second_elf = (
            str::parse::<u32>(second_elf_fstr.unwrap().get(..second_elf_fstr.unwrap().find("-").unwrap()).unwrap()).unwrap(), // First number
            str::parse::<u32>(second_elf_fstr.unwrap().get(second_elf_fstr.unwrap().find("-").unwrap()+1..).unwrap()).unwrap() // Second number
        );

        if ((first_elf.0 <= second_elf.0) && (first_elf.1 >= second_elf.1)) || ((second_elf.0 <= first_elf.0) && (second_elf.1 >= first_elf.1)) {
            fully_containing += 1;
        }

        if ((first_elf.0 >= second_elf.0 && first_elf.0 <= second_elf.1) || (first_elf.1 >= second_elf.0 && first_elf.1 <= second_elf.1)) || ((second_elf.0 >= first_elf.0 && second_elf.0 <= first_elf.1) || (second_elf.1 >= first_elf.0 && second_elf.1 <= first_elf.1)) {
            containing += 1; 
            println!("First Elf: {:?} \nSecond Elf: {:?}\n", first_elf, second_elf);
        }
    }

    println!("There are {} assingment pairs with one range fully containing the other.", fully_containing);
    println!("There are {} assingment pairs that overlap.", containing);
}
