# Motivation

- You are calling foo.Bar()
- This assumes that foo is in the same process as Bar()
- What if, later on, you want to put all Foo-related operations into a separate process?
  - Can you avoid changing your code?
- Proxy to the rescue!
  - Same interface, entirely different behavior
- This is called a communication proxy
  - Other types: logging, virtual, guarding, ...

# Proxy

A type that functions as an interface to a particular resource. That resource may be remote, expensive to construct, or may require logging or some other added functionality.

## Protection Proxy

This is an interface that wraps some other object to protect it from being used given certain conditions.

## Virtual Proxy

A virtual proxy is an object that when created it just returns a none complete of 'virtual' implementation of the object, when someone requires to use it then the object will be created in a lazy form.
This helps to prevent high load tasks to start when they are not yet required.

## Proxy vs. Decorator

- Proxy tries to provide an identical interface; decorator provides an enhanced interface
- Decorator typically aggregates (or has pointer to) what it is decorating; proxy doesn't have to
- Proxy might not even be working with a materialized object

# Summary

- A proxy has the same interface as the underlying object
- To create a proxy, simply replicate the existing interface of an object
- Add relevant functionality to the redefined methods
- Different proxies (communication, logging, caching, etc.) have completely different behaviors
