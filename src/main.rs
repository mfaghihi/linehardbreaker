//
// The purpose of this file is to receive a file containing a very large array of numbers in a line and break the
// line containing the array into different lines by adding end-of-line in different positions in the original line 
//

use std::io::Write; 
use std::fs::*;
// crate for regular expressions
use regex::*;
fn main() {
    // Read a file as a string
    let thestring = std::fs::read_to_string("./test.txt").unwrap();
    
    // Split the string by ", ". All of the elements seperated by a comma and a space will be stored in split.
    // Note that split does not contain any ", "
    let  split = thestring.split(", ");

    // Create a file to be written as the output.
    let mut buffer = File::create("result.txt").unwrap();
    
    // regex to match a number followed by "];" or "]"
    let re = Regex::new(r"\d];").unwrap();
    let re2 = Regex::new(r"\d]").unwrap();

    // This counter is used to adjust the number of elements in each line of the output
    let mut i =0;
    for s in split {
        // check if it is end of the array
        if re.is_match(s) || re2.is_match(s) {
            // Here we seperate the numbre from "]" in the string s
            let lastsplit = s.split("]");
            // convert the split lastsplit to a vector of strings
            let lastsplit_vec = lastsplit.collect::<Vec<&str>>();
            // write to the file
            buffer.write_fmt(format_args!("{}\n];\n", lastsplit_vec[0])).unwrap();
            
            break;

        }

        // write to the file
        buffer.write_fmt(format_args!("{}, ", s)).unwrap();
        
        // keep the number of elements in the current line
        i = i + 1;
        
        // Check if we have enough elements in the line
        if i == 25 {
            // Go to the next line
            buffer.write_fmt(format_args!("\n ")).unwrap();
            
            // reset the counter for the number of elements in a line
            i=0;
        }

    }
}
