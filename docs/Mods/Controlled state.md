When you have a component which you want to have some state either controlled or uncontrolled, having to handle both is a bit tricky.

There should be a modifier which allows a simple interface for handling both uncontrolled, and controlled state.

```tsx
controllable(#value)
component TextField(value: string, onValueChanged) {
	return (
		<input #value onChange=onValueChanged />
	)
}
```
