# Motivation

- Complicated objects aren't designed from scratch
  - They reiterate existing designs
- An existing (partially or fully constructed) design is a Prototype
- We make a copy of the prototype and customize it
  - Requires "deep copy"
- We make the cloning convenient (e.g. via a Factory)

# Prototype

A partially or fully initialised object that you copy (clone) and make use of.

## Deep Copy

When copying an object we need to make sure we handle the pointers to its internal properties. Some properties might be pointers to a value and when trying to modify that value, we might end up modifying the original object.

## Copy Method

We can add a DeepCopy method to our objetcs so they can handle the creation of deep structures within our object. This way we can copy the underlying values of pointers in our object.

## Copy Through Serialization

Another way to perform a deep copy without having to create many methods to handle pointers is by encoding the object into something like bytes array or JSON and then decoding that bute arry into the new object that is being created.

## Prototype Factory

This can be a factory method that takas a prototype object as a parameter. This prototype object can be a base object missing configuration of some fields.
Make it easy and covenient to create new objects that share some common property values.

# Summary

- To implement a prototype, partially construct an object and store it somewhere
- Deep copy the prototype
- Customize the resulting instance
- A prototype factory provides a convenient API for using prototypes
