# Motivation

- Object creation logic becomes too convoluted
- Struct has too many fields, need to initialize all correctly
- Wholesale object creation (non-piecewise, unlike Builder) can be outsourced to
  - A separate function (Factory Function, a.k.a. Constructor)
  - That may exist in a separate struct (Factory)

# Factory

A component responsible solely for the wholesale (not piecewise) creation of objects.

## Factory Function

A function that when called with some initialization parameters, return an instance of the object. This is particularly useful if we have some additional steps of validations that need to happen before the creation of the object. In this case, the factory fuction is the place to perform all the steps and validations and return the entire object created.

## Interface Factory

There are time when we want to protect the object that has being created from modification of its properties. For we make the Factory Method, return not the not the intance of the object but an interface. The user can still call te interface methods to retrieve some information abount the object or even call some exposed setter to modify the object.

Additionally, we could have two different objects that inherit from the same interface and they could be constructed in the same Factory Method. This could be useful in some cases.

## Factory Generator

Instead of having a fixed factory method, we could create a method that creates factories functions and allow for a more custom and flexible creation of objects. The user would first create one factory with some properties defined and after call the factory with some other details about the object.

Having the factories stored in variables makes it easy to provide them as parameter to other functions.

## Prototype Factory

Preconfigured objects created through a factory method. This is useful when creating base objects that will be defined in detail later

# Summary

- A factory function (a.k.a. constructor) is a helper function for making struct instances
- A factory is any entity that can take care of object creation
- Can be a function or a dedicated struct
