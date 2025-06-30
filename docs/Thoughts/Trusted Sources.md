TypeScript types are often suggestions on what the type of the underlying data *should* be.

This is unreliable when compiling with type-safety. Therefore we need to have trusted vs untrusted types.

When calling foreign functions, the types should be assumed to be `untrusted`, meaning their type is not *certain* to be what it is labeled as at runtime.

However, for trusted sources the type *has* to be correct.
For example:
```typescript
let trusted = 'something'
```

The `trusted` variable is guaranteed to be a string here, so it is a trusted type.

However, the following is untrusted:
```typescript
import fetct from 'fetch'
let untrusted = await fetch('google.com').then((res) => res.json())
```

The `untrusted` variable is not guaranteed to be any specific type, as the json result is untrusted.