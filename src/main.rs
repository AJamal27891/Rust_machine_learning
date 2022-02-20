use nalgebra::{DMatrix, DVector, Scalar};
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::str::FromStr;

fn parse_csv<N, R>(input: R) -> Result<DMatrix<N>, Box<dyn std::error::Error>>
  where N: FromStr + Scalar,
        N::Err: std::error::Error,
        R: BufRead
{
  // initialize an empty vector to fill with numbers
  let mut data = Vec::new();

  // initialize the number of rows to zero; we'll increment this
  // every time we encounter a newline in the input
  let mut rows = 0;

  // for each line in the input,
  for line in input.lines() {
    // increment the number of rows
    rows += 1;
    // iterate over the items in the row, separated by commas
    for datum in line?.split_terminator(",") {
      // trim the whitespace from the item, parse it, and push it to
      // the data array
      data.push(N::from_str(datum.trim())?);
    }
  }

  // The number of items divided by the number of rows equals the
  // number of columns.
  let cols = data.len() / rows;

  // Construct a `DMatrix` from the data in the vector.
  Ok(DMatrix::from_row_slice(rows, cols, &data[..]))
}

let file = File::open("../data/creditcard.csv").unwrap();
let bos: DMatrix<f64> = parse_csv(BufReader::new(file)).unwrap(); 
println!("{}", bos.rows(0, 5))
