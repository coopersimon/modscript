# Modscript
Dynamically-typed language made for use in personal projects. Design allows the user to call functions written in both modscript and rust.

Language is very simple at the moment, bears a strong resemblance to JavaScript.

See [msi](https://github.com/coopersimon/msi) for a REPL to test modscript in.

Call core functions using `->`. Example:
```
[1,2,3]->len() == 3
```

## TODO
* Add tuple or pair type (?)
* Improve import statements (paths, global imports)
* Export statements (?)
* Add `type` core function
* Add exceptions or error handling or error/result type
* Add more context to error struct
* Iteration for strings and objects
* Fix/remove C-style for loop
* Add enums, potentially structs
* Switch/match statements
* Some sort of ternary expression

### Tidiness
* Clean expr parser
* Add more compile error messages in parser

### Lower priority
* Default argument values in functions
* Potentially add options for more strict typing
* More compile time context checking (functions)
* Add casting (outside of core functions?)
* Const data, package-global data
* Object inheritance, methods (?)
* Threading (?)

## Types and how to declare them:
* Integer (64-bit): `var x = 1;`
* Float (64-bit precision): `var x = 1.;`
* Bool: `var x = true; var y = false;`
* String: `var x = "hello"; var y = "";`
* List: `var x = [1, 2.2, "three"]; var y = [];`
* _List indexing_: `x[0] == 1; x[-1] == "three";`
* Object: `var x = {a: 3, b: "str"}; var y = {};`
* _Object member access_: `x.a == 3;`
* Hash map: `var x = {[1]: 22, ["key"]: "value", [2.2]: "anytype"}; var y = {[]};`
* _Hash map access_: `x[1] == 22; x[2.2] == "anytype";`
* Null: `var x = null; var y;`

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
* `contains(x)`: returns true if item is in the list.

### Object:
* `clone()`: copies object into new reference.
* `is_field(x)`: checks if a field (x as string) exists in the object.
* `similar(x)`: checks if all the fields in the object exist in x (a different object).
* `same(x)`: checks if all the fields in the two objects are identical.
Note: `similar` and `same` don't check if the values in the fields are the same. Use `==` for this.

### Hash map:
* `clone()`: copies map into new reference.
* `insert(k, v)`: inserts key `k` and value `v`.
* `is_key(x)`: returns true if key is in the map.
* `is_value(x)`: returns true if value is in the map.
* `values()`: returns list of all the values in the map.
* `keys()`: returns list of all the keys in the map.

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
