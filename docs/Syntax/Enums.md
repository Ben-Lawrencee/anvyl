Yes, this is yet another enum implementation in javascript/typescript.

```
enum Option<T> {
  Some(value: T)
  None
}

const something = Some('whatever')
const nothing = None
```

Transpiles into:

```typescript
const something = { type: 'Option.Some', value: 'whatever' }
const nothing = { type: 'Option.None', value: null }
```

> This requires more research, because this approach is expensive with memory.

