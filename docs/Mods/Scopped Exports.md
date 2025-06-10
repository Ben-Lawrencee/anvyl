One of the annoying things about Javascript, is that everything that is exported can be imported from anywhere.

This means that internal behavior which spans across files is now made global to the entire application.

Sometimes you will want to scope you exports, much like how Rust does its modules.

You must explicitly say what is globally accessible, what is accessible within a folder, and what is private.
Everything defaults to private and requires the developer to explicitly make something public.

Perhaps with the new compiled approach, we can enforce these import rules at compile time.