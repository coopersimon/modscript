# Modscript
Dynamically-typed language made for use in personal projects. Design allows the user to call functions written in both modscript and rust.

Language is very simple at the moment, bears a strong resemblance to JavaScript.

See [msi](https://github.com/coopersimon/msi) for a REPL to test modscript in.

## TODO
* Add assign to list components
* Add object types
* Add mutable arguments in functions
* Local import statements
* Add core language functions (->), casting
* Add type function
* Add exceptions
* Better error messages in parser
* Better error messages in runtime

### Lower priority
* Default argument values in functions
* Function types & anonymous functions
* Potentially add options for more strict typing

### Core functions:
#### Int:
* `to_string()`
* `to_float()`
* `abs()`

#### Float:
* `to_string()`
* `abs()`
* `floor()`
* `ceil()`
* `round()`

#### String:
* `len()`
* `clone()`
* `concat(x)`
* `parse_num()`

#### List:
* `len()`
* `clone()`
* `append(x)`
* `concat(x)`

## Example
```func factorial(x) {
    // Simple factorial function.
    if x > 1 {
        return x * factorial(x-1);
    } else {
        return 1;
    }
}```
