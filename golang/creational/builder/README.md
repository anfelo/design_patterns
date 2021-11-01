# Motivation

- Some objects are simple and can be created in a single constructor call
- Other objects require a lot of ceremony to create
- Having a factory function with 10 arguments is not productive
- Instead, opt for piecewise (piece-by-piece) construction
- Builder provides an API for constructing an object step-by-step

# Builder

When piecewise object construction is complicated, provide an API for doing it succintly.

## Builder Facets

There are cases where the objects we are trying to build are complicated and require many builders to be composed. For this case we can have many builders that agregate the base builder and jump from one builder to the other as they are also a base builder. This way we can use SOC (Separation of Concerns) and have a fluent builder.

## Builder Parameter

Having a builder does not prevent the user from creating its own object from the interface especified. To force the user to use the builder we can use a builder function as a parameter to some action that will take care of building the object and perform some action internally without exposing the object that was created.

## Functional Builder

Another approach is to delay the build action of the object, by stacking all the actions that need to happen to create the object and finally, when all the actions have being stored, we can call the Build method to execute all the actions and build the object. This is a more functional apporach and allows us to extend the creation of the object easier with out having to create more builders.

## Extra

A pattern that goes side by side with the builder pattern is the fluent interface, which makes it easy to chain methods and build the objects with out creating a bunch of variables. This is done by having all the methods return a pointer to the object that is being built.

# Summary

- A builder is a separate component used for building an object
- To make builder fluent, return the receiver, this allows chaining
- Different facets of an object can be built with different builders working in tandem via a common struct
