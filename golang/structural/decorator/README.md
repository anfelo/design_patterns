# Motivation

- Want to augment an object with additional functionality
- Do not want to rewrite or alter existing code (OCP)
- Want to keep new functionality separate (SRP)
- Need to be able to interact with existing structures
- Solution: embed the decorated object and provide additional functionality

# Decorator

Facilitates the addition of behaviors to individual objects through embedding.

## Multiple Aggregation

In this case we have two different objects that share some fields, however, they have different behaviors. Now imagine we have a third object that aggregates the other two object behaviors and fields. This new object can perform all of the actions that object 1 and object 2 can perform but we don't want the other similar properties to collide. For this we can create a decorator that aggregates both objects and provides the same API, handling the collision of similar properties.

# Summary

- A decorator embeds the decorated object
- Adds utility fields and method to augment the object's
- Often used to emulate multiple inheritance (may require extra work)
