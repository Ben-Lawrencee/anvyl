> The idea behind this is to easily allow using these types as values and as actual types interchangeably.
> 
> The only problem I could imagine is when there are cases where it is a bit ambiguous whether you are referring to the type or the value?
> 
> This needs to be investigated.


```tsx
type field = 'date-created' | 'date-modified'

for (const f of field) {
	console.log(f)
}

// Transpiles into

type filed = 'date-created' | 'date-modified'

for (const f of ['date-created', 'date-modified']) {
	console.log(f)
}
```
