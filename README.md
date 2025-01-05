# Persistent Counter example

A variant of the simple counter example, with database and files.

## Simple Counter

There are 3 events — `Increment`, `Decrement` and `Reset` that operate on a
simple model struct with a `count` field.

There are tests to demonstrate how sending these events to the core, performs
the selected operation on the model and then uses the `Render` capability to ask
the UI to re-render.
