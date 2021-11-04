# Motivation

- Textual input needs to be processed
  - E.g., turned into linked structures
  - AST = Abstract Syntax Tree
- Some Examples:
  - Programming language compilers, interpreters and IDEs
  - HTML, XML and similar
  - Numeric expressions (3 + 4 / 5)
  - Regular expressions
- Turning strings into linked structures in a complicated process

# Interpreter

- A component that processes structured text data. Does so by turning into separate lexical tokens (lexing) and then interpreting sequences of said tokens (parsing).

# Summary

- Barring simple cases, an interpreter acts in two stages
- Lexing turns text into a set of tokens
- Parsing tokens into meaningful constructs (AST = Abstract Syntax Tree)
- Parsed data can then be transversed using the Visitor pattern
