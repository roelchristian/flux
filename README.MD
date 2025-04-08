# Flux interpreter

A simple interpreter for the Flux programming language written in Rust. Flux is a functional, expression-oriented, prefix-style language designed with simplicity, composability, and clarity in mind.

## Features

### Prefix Notation
All operations and function calls follow prefix (Polish) notation.
```flux
+ 1 2 ; returns 3
* + 1 2 3 ; (1 + 2) * 3 = 9
```