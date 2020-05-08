Authors: Daniel Hammer, Andrew Shelton
Date: March 21, 2020

Project 2: Tokenizer

Compile the program by executing the following command:

    gcc tokenizer.c -o tokenizer


Run the program by executing the following command:

    ./tokenizer [name of input file] [name of output file to be created]


This program reads in a text file and parses through, classifying all
non-whitespace characters into token catagories. If multiple characters compose
a valid lexeme, those characters are classified as a single token. If a
character is found that is not present in the predetermined category list
(defined in get_token_type()), an error message is displayed and that character
is skipped. If a semicolon is found, a new statement begins. All output is
printed to the output file in the following format:

Statement #[statement number]
Lexeme [token number] is [token literal] and is a [token type]
Lexeme [token number] is [token literal] and is a [token type]
Lexeme [token number] is [token literal] and is a [token type]
------------------------------------------ [if the end of a statement]


There are no known bugs in the program, and it compiles without errors
using the -Wall flag.
IMPORTANT: This program has the capability to classify LETTERS and STRINGS,
but that functionality is commented out as the instructions did not call for it
