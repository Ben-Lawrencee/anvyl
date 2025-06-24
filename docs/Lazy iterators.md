These are iterators which are constructed at compile-time, for the most efficient and flexible interface when dealing with iterable collections.

An example of this is:

```rust
let count = my_vec.iter().map(|v| v.value).sum()

// Compiles into roughly:

let count = {
  let mut sum = 0;

  for (let v in my_vec) {
    let value = v.value;
    sum += value;
  }

  sum
}
```

This is a lot more compact, and does all necessary iteration in a single loop.

This is of course a very simple example.
But here is what the code would be (roughly) in Javascript

```javascript

let count = my_array.map((v) => v.value).reduce(((sum, value) => sum += value), 0)

// Roughly equavalent to:

let count;

const mapped_array = []

for (const v in my_array) {
	mapped_array.push(v.value)
}

let sum = 0;

for (const value in mapped_array) {
  sum += value;
}

count = sum;
```

Which iterates twice, and creates a whole other array.