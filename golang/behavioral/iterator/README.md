# Motivation

- Iteration (traversal) is a core functionality of various data structures
- An iterator is a type that facilitates the traversal
  - Keeps a pointer to the current element
  - Knows how to move to different element
- Go allows iteration with range
  - Built-in support in many objects (arrays, slicess, etc)
  - Can be supported in our own structs

# Iterator

An object that facilitates the traversal of some data structure

# Summary

- An iterator specifies how you can traverse an object
- Moves along the iterated collection, indication when last element has been reached
- Not idiomatic in Go (no standar Iterable interface)
