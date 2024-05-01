extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;

fn main() {
   if args().len() != 3 {
       eprintln!("Usage: `source` `target`");
       return;
   }

   // Handle opening the source file with error checking
   let mut input_file = match File::open(args().nth(1).unwrap()) {
       Ok(file) => BufReader::new(file),
       Err(error) => {
           eprintln!("Error opening source file: {}: {}", args().nth(1).unwrap(), error);
           return;
       },
   };

   // Handle opening the target file with error checking
   let output_file = match File::create(args().nth(2).unwrap()) {
       Ok(file) => file,
       Err(error) => {
           eprintln!("Error creating target file: {}: {}", args().nth(2).unwrap(), error);
           return;
       },
   };

   let mut encoder = GzEncoder::new(output_file, Compression::default());
   let start = Instant::now();
   copy(&mut input_file, &mut encoder).unwrap();
   let output_file = encoder.finish().unwrap();
   println!(
       "Source len: {:?}",
       input_file.get_ref().metadata().unwrap().len()
   );
   println!("Target len: {:?}", output_file.metadata().unwrap().len());
   println!("Elapsed: {:?}", start.elapsed());
}
