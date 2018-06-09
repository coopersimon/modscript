# Modscript
Dynamically-typed language made for use in personal projects. Design allows the user to call functions written in both modscript and rust.

Language is very simple at the moment, bears a strong resemblance to JavaScript.

See [msi](https://github.com/coopersimon/msi) for a REPL to test modscript in.

Call core functions using `->`. Example:
```
[1,2,3]->len() == 3
```

## TODO
* Add pair type(?)
* Add mutable arguments in functions
* Local import statements
* Improve import statements (paths, global imports)
* Add `type` function
* Add exceptions
* Better error messages in parser
* Better error messages in runtime
* Iteration for strings and objects
* Function types & anonymous functions

### Tidiness
* Clean expr parser, improve core functions

### Lower priority
* Default argument values in functions
* Potentially add options for more strict typing
* Add casting (outside of core functions?)

## Core functions:
### Int:
* `to_string()`: converts to string.
* `to_float()`: converts to float.
* `abs()`: absolute value.

### Float:
* `to_string()`: converts to string.
* `abs()`: absolute value.
* `floor()`: rounds down to nearest whole.
* `ceil()`: rounds up to nearest whole.
* `round()`: rounds to nearest whole.

### String:
* `len()`: finds length of string in characters (not implemented).
* `clone()`: copies string into new reference.
* `concat(x)`: concatenates list onto existing list (more efficient than adding lists together!).
* `parse_num()`: tries to convert string into int (if possible) or float.

### List:
* `len()`: finds length of list.
* `clone()`: copies list into new reference.
* `append(x)`: adds element to the end of the list.
* `concat(x)`: adds list onto the end of the list.
* `front()`: returns element at the front of the list.
* `back()`: returns element at the back of the list.

### Object:
* `clone()`: copies object into new reference.
* `is_field(x)`: checks if a field (x as string) exists in the object.
* `similar(x)`: checks if all the fields in the object exist in x (a different object).
* `same(x)`: checks if all the fields in the two objects are identical.
Note: `similar` and `same` don't check if the values in the fields are the same. Use `==` for this.

### Pair:
* `first()`
* `second()`

## Example
```
func factorial(x) {
    // Simple factorial function.
    if x > 1 {
        return x * factorial(x-1);
    } else {
        return 1;
    }
}
```
