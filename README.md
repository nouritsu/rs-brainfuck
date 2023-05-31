# About
This project is an extremely powerful, fast and memory safe interpreter for the iconic language '[brainfuck](https://esolangs.org/wiki/Brainfuck)'.  
The entire language is written using only 8 characters - `<>[]+-,.`  
Note that there are a few more operators that have been (or are being) worked on as a sort of extension to brainfuck (think of it as bf++ :D)

# Introduction
The 8 characters each perform a single operation on a massive 30,000 element long array of unsigned integers (usize).  
The operations are as follows [[Source]](https://gist.github.com/roachhd/dce54bec8ba55fb17d3a) - 
```brainfuck
> = increases memory pointer, or moves the pointer to the right 1 block.
< = decreases memory pointer, or moves the pointer to the left 1 block.
+ = increases value stored at the block pointed to by the memory pointer
- = decreases value stored at the block pointed to by the memory pointer
[ = like c while(cur_block_value != 0) loop.
] = if block currently pointed to's value is not zero, jump back to [
, = like c getchar(). input 1 character.
. = like c putchar(). print 1 character to the console
```
Any other characters apart from these 8 are ignored and treated as comments in the original language implementation.

# Installation
Compile the interpreter on your own by cloning this repository and running the folllowing command -
```
cargo build --release
```
You will find your compiled executable somewhere in the directory `target/release`.
You can also install the package (build release and copy to path) using the following command -
```
cargo install --path .
```
After this, you should be able to run `rs-brainfuck.exe` (or `rs-brainfuck` if you're not on Windows) given that Cargo's bin path is in $PATH.

# Usage
There are three command line options that can modify the behaviour of the program.
| Short Option 	| Long Option 	|          Expects         	|                                    Effect                                    	|
|:------------:	|:-----------:	|:------------------------:	|:----------------------------------------------------------------------------:	|
|    <N.A.>    	|   --arrlen  	| usize (unsigned integer) 	|          Changes the size of the internal array. 30,000 by default.          	|
|      -f      	|    --file   	|    String (file path)    	| Interprets file provided, if not provided interpreter will run in REPL mode. 	|
|      -t      	|    --time   	| bool (just put the flag) 	|                 Shows time taken for each step of execution.                 	|
There are 2 modes the interpreter can run in, file mode or REPL mode.  
The time option will print time after each line in REPL mode.

## Errors
All errors reported by the interpreter are descriptive and self explanatory.  
If the interpreter encounters an error in file mode, execution will stop the moment the error is caught and program will quit.  
Errors caught in REPL mode will not cause the program to close, use `Ctrl-C` to close program from terminal.

# Planned Features
## 'At line ..., at character ...' for errors
This feature will tell you what line and character the error was caught at.  
Eg - `"Loop unclosed on line 6, character 2. Close with ']'."`

## The ^ operator
The operator will print the current status of the program similar to this -
```
Internal Array (capacity: 30,000)
[ ... [123] [512] [215] ...]
              ^
          ptr = 6
```
This feature will autoexecute in the REPL.

## ~~Report time taken~~ Implemented!
Passing in a flag `-t` will report times taken for each process - Lexing, Parsing, Interpreting  
Eg -
```
[INFO] : Lexed tokens in 0.61 seconds.
[INFO] : Parsed instructions in 0.09 seconds.
[INFO] : Interpreted instructions in 2.1 seconds.
[OK] : Execution completed sucessfully.
```

