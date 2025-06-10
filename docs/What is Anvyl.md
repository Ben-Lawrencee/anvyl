## What is the goal?

Provide an interface for writing Rust code which generates extensions or modifications to a React component.

## What does it solve?

It allows for the easy creation of flexible and modular components without having to write the functionality yourself.

## Should it use FFI's?

A Foreign Function Interface (FFI), allows one language to call a function in a different language.

If we did add FFI's, then we could write Rust code which is executed during runtime.
This means we could have a modifier `with_websocket`, to add a websocket connection to the component. It would be passed as a prop `websocket`, and could be called or interfaced with at runtime in the browser.

I imagine this would only be possible server-side.
But perhaps we could have a Rust side and a Typescript side, for backend and frontend execution respectively.