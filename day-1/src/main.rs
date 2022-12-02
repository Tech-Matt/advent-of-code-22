/* Problem description
 * Input: N° of calories each elf is carrying
 * There are different foods, each with different calories
 * The elves have written a list of their food calories. One per line. Every elf separate his calories
 * from the previous elf with a blank line.
 *
 * Problem 1: Find the elf carrying the most calories. Return calories value of that elf
 * Problem 2: Find top 3 elves and return sum
 */



use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
   
    let mut max_sum :u32 = 0;
    let mut sum_vec = Vec::new();
    //Importing file
    if let Ok(data) = read_lines("data.txt") {
         let mut partial_sum: u32 = 0;

         for line in data {
            //If it's a blank line i will update max_sum and reset partial_sum
            if line.as_ref().unwrap().is_empty() {
                if partial_sum > max_sum {
                    max_sum = partial_sum;
                }

                //Store sum in vector
                sum_vec.push(partial_sum);

                partial_sum = 0;
                //Proceed to next iteration
                continue;
            }

            //parsing from Result -> String -> Result -> u32
            partial_sum += line.unwrap().parse::<u32>().unwrap();

            //println!("{}", line.unwrap());
         }

         //Get the sum of top 3 elves
         sum_vec.sort();
         let len = sum_vec.len();
         let top3_sum: u32 = sum_vec[len - 1] + sum_vec[len - 2] + sum_vec[len-3];


         println!("N° of Max calories is: {}", max_sum);
         println!("N° of Top 3 calories is: {}", top3_sum);
    }
    
}

// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
