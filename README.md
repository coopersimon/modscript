# Modscript
Dynamically-typed language made for use in personal projects. Design allows the user to call functions written in both modscript and rust.

Language is very simple at the moment, bears a strong resemblance to JavaScript. It currently doesn't have a heap however!

## TODO
* Add Heap, list & object types
* Add passing by reference in functions
* Add for loops, continue & break
* Add import statements
* Add core language functions (->), casting
* Add exceptions
* Better error messages in parser
* Better error messages in runtime

### Lower priority
* Default argument values in functions
* Function types & anonymous functions
* Potentially add options for more strict typing

## Example
func factorial(x) {
    // Simple factorial function.
    if x > 1 {
        return x * factorial(x-1);
    } else {
        return 1;
    }
}
