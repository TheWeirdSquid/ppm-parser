/*
David Bootle
PPM Parser (Rust)
*/

// standard imports
use std::env;
use std::fs::File;
use std::path::Path;
use std::process;
use std::io::{BufReader};

// custom
mod ppm;
mod imageio;

use ppm::PPM;
use imageio::{parse_header, read_image_data};

fn print_help_text() {
    let executable_name = env::args().nth(0).unwrap();

    println!("PPM IMAGE TOOL\n");
    println!("SYNTAX: {} <file> [options]\n", executable_name);
    println!("If run with no options, the tool will output the width and height of the image.\n");
    println!("OPTIONS:");
    println!("c/C - Create copy");
    println!("g/G - Convert to grayscale");
    println!("n/N - Convert to negative");
    println!("r/R - Rotate clockwise");
    println!("s/S - Half size (shrink image by 2x)");
    println!("l/L - Apply LSD-like filter");
    println!("f/F - Flip image horizontally");
    println!("ir/IR - Isolate red channel");
    println!("ig/IG - Isolate blue channel");
    println!("ib/IB - Isolate green channel");
}
 
fn main() {
 
    // if user only types the executable name and no other arguments, print the help text and exit
    if env::args().len() == 1 {
        print_help_text();
        return;
    }
 
    // since the arguments actually exist, create a vector to store them
    let args: Vec<String> = env::args().collect();

    /* Open Input File */
    let input_file_path = Path::new(&args[1]); // create a new path to represent the input file

    // open the file
    let input_file =  match File::open(&input_file_path) {
        Ok(file) => file, // set input_file to the file if the file exists
        Err(_) => { // otherwise exit with an error message
            eprintln!("Error opening file. Make sure the specified file exists.");
            process::exit(1);
        }
    };

    let mut reader = BufReader::new(input_file); // create a buffered reader to read the file
     
    // create a PPM object which represents the original image in memory, and is what will be read and modified by the program
    let mut image: PPM = PPM::new();

    // process header information and get the end location of the header
    let header_length = parse_header(&mut reader, &mut image);

    // if no options were used, print image header information
    if args.len() == 2 {
        // print image width and height
        println!("Image Dimensions: {} x {}", image.width, image.height);

        // print format subtype
        println!("Format Subtype: {}", image.magic);

        // print bit depth
        let image_bit: &str = match image.maxc {
            255 => "8-bit",
            65535 => "16-bit",
            _ => "Unknown",
        };
        println!("Bit Depth: {}", image_bit);
    }

    // if there are additional arguments, then perform image operations
    if args.len() > 2 {
        
        // verify that the image is of P6 format
        if image.magic != "P6" {
            eprintln!("This image is in {} format. This tool only supports P6 format.", image.magic);
            process::exit(1);
        }

        // read image pixel data
        read_image_data(&mut reader, &mut image, header_length);
    }

}