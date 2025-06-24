I really like how Rust has different methods / properties which are accessible based on the generic of a struct.

For example, `Autocomplete<string>` should not offer a `getOptionLabel` method, since the string itself is the label.

This is of course just an example, but how could this be made possible?

The syntax for components has not been finalized yet, and I think this is really worth considering to potentially be a part of the syntax.

Perhaps something like the following:

```rust
component Autocomplete<T> {
  options: T[]
  value: T[]
  onChange: (new: T[]) => unknown
  getOptionLabel: (e: T) => string
}

// For any implementation that the first generic is *not* a string
impl Autocomplete<T excludes string> {
  // Make it optional instead
  getOptionLabel?: (e: T) => string
}

pub fn Autocomplete() {
	... component logic ...
}

pub fn Autocomplete<T excludes string>() {
	.. component logic ...
}
```
