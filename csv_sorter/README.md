# CSV Sorter
This program reads in a csv of data, stores that data into structs, sorts the structs, and write the newly sorted data to an output file.

## Details
Data is read in, stored in a `Person` struct, and then collected into a vector.
That vector is sorted via selection sort.
The newly sorted data is then written to an output file.


All leading and trailing whitespace is omitted.
All input files must be placed in `src/`
The output file is created upon running the program. If an existing output file is chosen, any data within that file is overwritten.


## Usage
Input files must be in the following format:
firstname, lastname, street, city, state, zipcode

To run, use the following command;
`cargo run [input file] [output file]`
