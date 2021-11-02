# Motivation

- Bridge prevents a 'Cartesian product' complexity explosion
- Example:
  - Common type ThreadScheduler
  - Can be preemptive or cooperative
  - Can run on Windows or Unix
  - End up with a 2x2 scenario: WindowsPTS, UnixPTS, WindowsCTS, UnixCTS
- Bridge pattern avoids the entity explosin

# Bridge

A mechanism that decouples an interface (hierarchy) from an implementation (hierarchy)
The Bridge pattern attempts to solve the complexity explosion problem by switching from inheritance to the object composition. What this means is that you extract one of the dimensions into a separate class hierarchy, so that the original classes will reference an object of the new hierarchy, instead of having all of its state and behaviors within one class.

# Summary

- Decouple abstraction from implementation
- Both can exist as hierarchies
- A stronger form of encapsulation
