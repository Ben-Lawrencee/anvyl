When extending another component, it should calculate what props overlap, and prevent conflicts by automatically `Omit`ing the overlapping props.

For example;
```tsx
export interface MyComponentProps extends OtherComponentProps {
  // The `shared` prop is of a different type here.
  shared: 'something'
}
```

Compiles into:
```tsx
export interface MyComponentProps extends Omit<OtherComponentProps, 'shared'> {
  // The `shared` prop is of a different type here.
  shared: 'something'
}
```
