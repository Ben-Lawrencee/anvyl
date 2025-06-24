> In Rust, `match` blocks are forcefully *exhaustive*, meaning all possible variants of the argument being matched, must be accounted for.
> 
> Doing this in Anvyl will require a pretty complex type system.


```rust
match 'something' {
	'other' => {
		...
	}
	// Catch all
	_ => {
		...
	}
}
```

This will compile into:

```typescript
(() => {
	if ('something' === 'other') {
		...
		return
	}

	// Catch all
	...
})()
```
