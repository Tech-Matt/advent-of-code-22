/* Problem description
 * Input: N° of calories each elf is carrying
 * There are different foods, each with different calories
 * The elves have written a list of their food calories. One per line. Every elf separate his calories
 * from the previous elf with a blank line.
 *
 * Problem: Find the elf carrying the most calories. Return calories value of that elf
 */

/* Solution:
 * -import file
 * -create max_sum
 * -evaluate sum for each elf. If elf_sum is greater than max_sum then max_sum = elf_sum
 * -return max_sum
 */

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
   
    let mut max_sum :u32 = 0;
    //Importing file
    if let Ok(data) = read_lines("data.txt") {
         let mut partial_sum: u32 = 0;

         for line in data {
            //If it's a blank line i will update max_sum and reset partial_sum
            if line.as_ref().unwrap().is_empty() {
                if partial_sum > max_sum {
                    max_sum = partial_sum;
                }

                partial_sum = 0;
                //Proceed to next iteration
                continue;
            }

            //parsing from Result -> String -> Result -> u32
            partial_sum += line.unwrap().parse::<u32>().unwrap();

            //println!("{}", line.unwrap());
         }

         println!("N° of Max calories is: {}", max_sum);

    }
    
}

// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
