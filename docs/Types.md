> Shall types have a concept of being `trusted`?
> Since this will actually run in a Javascript environment, types are dynamic.
> But if we track which types are *certain* to be what they are typed as, then we give them the `trusted` attribute.
> 
> Trusted types can only be created from other trusted types.
> This will encourage strict typings throughout the program.
> 
> Perhaps in function signatures, you can specify trusted / untrusted types and handle them respectively.
> 
> Only interfaces within the project can be `trusted`, this is because we can *never* trust external sources to give the types we expect.

