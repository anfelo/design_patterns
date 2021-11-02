# Motivation

- For some components it only makes sense to have one in the system
  - Database repository
  - Object factory
- E.g. the constructor call is expensive
  - We only do it once
  - We give everyone the same instance
- Want to prevent anyone creating additional copies
- Need to take care of lazy instantiation

# Singleton

A component which is instantiated only once. This is particularlly useful if we want to load some expensive object into memory.

## Problems with singletons

It is difficult to mock singletons when testing and this makes it hard to test functionality. This is because we depend on the concrete implementation of the singletons. This goes against the Dependency Inversion Principle (DIP).

## Singleton and Dependency Inversion

A simple fix is to pass as a dependency the interface of what our singleton looks like to the method thta is using it directly. That way, when we are going to use the method, we need to pass a class (e.g. database) that can perform the singletons functionality.

# Singleton

- Lazy one-time initialization using sync.Once (Golang)
- Adhere to DIP: depend on interfaces, not concrete types
- Singleton is not scary but can lead to problems with dependencies
