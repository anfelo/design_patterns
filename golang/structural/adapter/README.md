# Motivation

- Adapters in the real world are all over the place (e.g. power adapters)
- We cannot modify our gadgets to support every possible interface
- Thus, we use a special device (an adapter) to give us the interface we require from the interface we have

# Adapter

A construct which adapts an existing interface X to conform to the required interface Y.
An adapter wraps one of the objects to hide the complexity of conversion happening behind the scenes. The wrapped object isn’t even aware of the adapter. For example, you can wrap an object that operates in meters and kilometers with an adapter that converts all of the data to imperial units such as feet and miles.

# Summary

- Implementing an Adapter is easy
- Determine the API you have and the API you need
- Create a component which aggregates (has a pointer to,...) the adapter
- Intermediate representations can be pile up: use caching and other optimizations
