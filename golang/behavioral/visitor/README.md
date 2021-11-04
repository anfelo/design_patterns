# Motivation

- Need to define a new operation on an entire type hierarchy
  - E.g., given a document model (list, paragraphs, etc.), we want to add printing functionality
- Do not want to keep modifying every type in the hierarchy
- Want to have the new functionality separate (SRP)
- This approach is often used for traversal
  - Alternative to Iterator
  - Hierarchy members help you traverse themselves

# Visitor

A pattern where a component (visitor) is allowed to traverse the entire hierarchy of types. Implemented by propagating a single Accept() method throughout the entire hierarchy.

## Intrusive Visitor

One implementation of the visitor pattern is one where some action needs to be implemented on every interface in order to traverse the entire hierarchy.

## Reflective Visitor

The reflective visitor needs to check the internals of each interface to understand their nature and handle the next action accordingly. The down side is that every time there is a new interface added to the hierarchy we need to go and modify the action handler so the case is covered. This breaks the OCP.

## Consideration on Dispatch

- Which function to call?
- Single dispatch: depends on name of request and type of receiver
- Double dispatch: depends on name of request and type of two receivers (type of visitor, type of element beign visited)

# Summary

- Propagate an Accept(v \*Visitor) method throughout the entire hierarchy
- Create a visitor with VisitFoo(f Foo), VisitBar(b Bar), ... for each element in the hierarchy
- Each Accept() simply calls Visitor.VisitXxx(self)
