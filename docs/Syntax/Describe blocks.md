
> Instead of having massive function signatures, or long lists of variants.
> 
> Instead there is a describe block which details all possible properties to a signature in one place.
> 
> All variants, all generics, everything.
> 
> Addons / modifiers could generate these describe blocks.

```
describe function_name {
	name: string
	age: number
}

describe Default for function_name {
	name: 'someone'
	age: 20
}

describe David for function_name {
	...Default
	name: 'david',
}

// Original / normal function invoke
function_name({ name: 'david', age: 15 })

// Default implementation call
function_name::Default()

// Spread operator with the Default implementation
function_name({ ...Default, name: 'david' })

// Named variant
function_name::David()
```