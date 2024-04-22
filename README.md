Key takeaways from this exercice:

- How to use external crates.

- Returning Result Types.

- Rust handles all prints in a buffer so we need to tell "No, i want to flush this print now!".

- We need to.owned() because trim_end() actually creates a standalone String that would be droped if we didnt allocate new memory for her.

- this funny thingy `<u8>` is called the turbo fish operator and is used to specify the type of a generic parameter when calling a function.