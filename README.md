# Rust Expression Compiler (Crafting Interpreters Inspired)

A small compiler + virtual machine written in **Rust**, inspired by the book  
**Crafting Interpreters** by Robert Nystrom.

This project currently supports compiling and executing **basic arithmetic expressions**.  
The compiler converts source code into **bytecode**, which is then executed by a simple **stack-based virtual machine**.

## Features

- Expression evaluation
- Bytecode compiler
- Stack-based virtual machine
- Pratt parser for operator precedence

Currently supported operators:

```
+  -  *  /
```

Example:

```
1 + 2 * 3
```

Result:

```
7
```

## Architecture

The project is split into three main stages.

### 1. Scanner

Converts raw source text into a stream of **tokens**.

```
source code → tokens
```

Example:

```
1 + 2 * 3
```

Becomes:

```
NUMBER(1) PLUS NUMBER(2) STAR NUMBER(3)
```

### 2. Parser / Compiler

The parser uses a **Pratt parser** to handle operator precedence and associativity.

It compiles tokens directly into **bytecode instructions**.

```
tokens → bytecode
```

Example bytecode (conceptually):

```
OP_CONSTANT 1
OP_CONSTANT 2
OP_CONSTANT 3
OP_MULTIPLY
OP_ADD
OP_RETURN
```

### 3. Virtual Machine

The VM executes bytecode using a **stack-based execution model**.

```
bytecode → runtime execution → result
```

Execution example:

```
push 1
push 2
push 3
multiply
add
return
```

Result:

```
7
```

## Project Structure

```
src/
 ├── scanner.rs   # text → tokens
 ├── parser.rs    # tokens → bytecode (Pratt parser)
 ├── chunk.rs     # bytecode representation
 ├── vm.rs        # virtual machine
 └── main.rs      # entry point
```

## Example

Input:

```
1 + 2 * 3
```

Output:

```
7
```

## TODO

- Proper compile-time error handling
- Reduce excessive `.clone()` usage
- Improve Rust ownership/borrowing design
- Add more expressions:
  - comparison operators (`>`, `<`, `==`, `!=`)
  - grouping
  - unary operators
- Improve diagnostics and debugging output

## References

- Crafting Interpreters — Robert Nystrom  
- Pratt Parsing (Top Down Operator Precedence)

## Status

Early stage — **expressions only**.