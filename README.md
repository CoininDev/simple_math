# Simple Math
## What is:
Simple Math is a project made for learning purposes.
It is a simple math interpreter which lets you write variables and use them.
This was made using the Rust Programming Language.

## Modes:
You can run simple_math with:
  1. Interactive REPL mode with GNU readline features (by rustyline);
  2. Evaluate an entire file;

## Features:
It already supports:
  - Basic operations: `+`, `-`, `*`, `/`.
  - Parenthesis: `(a + b * 2)`.
  - Variables: `new_var = var * 5`

## Usage:
simple_math [OPTIONS]

Options:
  -i             Enter interactive mode
  -f <filename>  Evaluate a file and print the result

Here is a simple example of a .math file

```math
pi = 3
radius = 420

circle_area = pi * (radius * radius)

result = circle_area
```
The interpreter seaches for a "result" variable, if it doesn't exists, it prints the last one as the result.

## WIP:
Things I am still working in is:
  - Floating points
  - Compilation to machine code
