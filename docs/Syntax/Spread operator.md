```typescript
const [first, ...middle, second_to_last, last] = []

// Compiles into:

const [first, ...middle] = []
const last = middle.pop()
const second_to_last = middle.pop()
```
