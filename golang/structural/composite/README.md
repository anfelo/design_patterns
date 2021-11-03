# Motivation

- Objects use other objects' fields/methods through embedding
- Composition lets us make compound objects
  - E.g., a mathematical expresion composed of simple expressions; or
  - A shape group made of several different shapes
- Composite design pattern is used to treat both sigle (scalar) and composite objects uniformly
  - I.e., Foo and []Foo have common APIs

# Composite

A mechanism for treating individual (scalar) objects and compositions of objects in a uniform manner

## Example: Geometric Shapes

The idea is that you can have a scalar shape like Circle or Square or you could have a more complex shape that is composed of these scalar ones and you will be able to call the same methods in the complex shape and in the scalar shapes without worrying about how to handle each. The key is to have methods that handle the different use cases.

# Summary

- Objects can use other objects via composition
- Some composed and singular objects need similar/identical behaviors
- Composite design pattern lets us treat both types of objects uniformly
- Iteration supported with the Iterator design pattern
