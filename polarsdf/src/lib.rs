// utilities for working with polars dataframes
//
use polars::io::SerReader;
use polars::prelude::*;
use std::fs::File;

//read in a csv file
pub fn read_csv(path: &str) -> DataFrame {
    let file = File::open(path).unwrap();
    CsvReader::new(file).finish().unwrap()
}

//print "n" rows of a dataframe
pub fn print_df(df: &DataFrame, n: usize) {
    println!("{:?}", df.head(Some(n)));
}

//print the schema of a dataframe
pub fn print_schema(df: &DataFrame) {
    println!("{:?}", df.schema());
}

//print the shape of a dataframe
pub fn print_shape(df: &DataFrame) {
    println!("{:?}", df.shape());
}
