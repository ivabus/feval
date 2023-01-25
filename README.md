# fEval

> Fast EVALuator

Simple command line tool, that helps you to fastly evaluate expressions. Based on [evalexpr](https://crates.io/crates/evalexpr) crate.

## Operators

Supported binary operators:

| Operator | Precedence | Description |
|----------|------------|-------------|
| ^ | 120 | Exponentiation |
| * | 100 | Product |
| / | 100 | Division (integer if both arguments are integers, otherwise float) |
| % | 100 | Modulo (integer if both arguments are integers, otherwise float) |
| + | 95 | Sum or String Concatenation |
| - | 95 | Difference |
| < | 80 | Lower than |
| \> | 80 | Greater than |
| <= | 80 | Lower than or equal |
| \>= | 80 | Greater than or equal |
| == | 80 | Equal |
| != | 80 | Not equal |
| && | 75 | Logical and |
| &#124;&#124; | 70 | Logical or |
| = | 50 | Assignment |
| += | 50 | Sum-Assignment or String-Concatenation-Assignment |
| -= | 50 | Difference-Assignment |
| *= | 50 | Product-Assignment |
| /= | 50 | Division-Assignment |
| %= | 50 | Modulo-Assignment |
| ^= | 50 | Exponentiation-Assignment |
| &&= | 50 | Logical-And-Assignment |
| &#124;&#124;= | 50 | Logical-Or-Assignment |
| , | 40 | Aggregation |
| ; | 0 | Expression Chaining |

Supported unary operators:

| Operator | Precedence | Description |
|----------|------------|-------------|
| - | 110 | Negation |
| ! | 110 | Logical not |

## Functions

| Identifier           | Argument Amount | Argument Types         | Description |
|----------------------|-----------------|------------------------|-------------|
| `min`                | >= 1            | Numeric                | Returns the minimum of the arguments |
| `max`                | >= 1            | Numeric                | Returns the maximum of the arguments |
| `len`                | 1               | String/Tuple           | Returns the character length of a string, or the amount of elements in a tuple (not recursively) |
| `floor`              | 1               | Numeric                | Returns the largest integer less than or equal to a number |
| `round`              | 1               | Numeric                | Returns the nearest integer to a number. Rounds half-way cases away from 0.0 |
| `ceil`               | 1               | Numeric                | Returns the smallest integer greater than or equal to a number |
| `if`                 | 3               | Boolean, Any, Any      | If the first argument is true, returns the second argument, otherwise, returns the third  |
| `typeof`             | 1               | Any                    | returns "string", "float", "int", "boolean", "tuple", or "empty" depending on the type of the argument  |
| `math::is_nan`       | 1               | Numeric                | Returns true if the argument is the floating-point value NaN, false if it is another floating-point value, and throws an error if it is not a number  |
| `math::is_finite`    | 1               | Numeric                | Returns true if the argument is a finite floating-point number, false otherwise  |
| `math::is_infinite`  | 1               | Numeric                | Returns true if the argument is an infinite floating-point number, false otherwise  |
| `math::is_normal`    | 1               | Numeric                | Returns true if the argument is a floating-point number that is neither zero, infinite, [subnormal](https://en.wikipedia.org/wiki/Subnormal_number), or NaN, false otherwise  |
| `math::ln`           | 1               | Numeric                | Returns the natural logarithm of the number |
| `math::log`          | 2               | Numeric, Numeric       | Returns the logarithm of the number with respect to an arbitrary base |
| `math::log2`         | 1               | Numeric                | Returns the base 2 logarithm of the number |
| `math::log10`        | 1               | Numeric                | Returns the base 10 logarithm of the number |
| `math::exp`          | 1               | Numeric                | Returns `e^(number)`, (the exponential function) |
| `math::exp2`         | 1               | Numeric                | Returns `2^(number)` |
| `math::pow`          | 2               | Numeric, Numeric       | Raises a number to the power of the other number |
| `math::cos`          | 1               | Numeric                | Computes the cosine of a number (in radians) |
| `math::acos`         | 1               | Numeric                | Computes the arccosine of a number. The return value is in radians in the range [0, pi] or NaN if the number is outside the range [-1, 1] |
| `math::cosh`         | 1               | Numeric                | Hyperbolic cosine function |
| `math::acosh`        | 1               | Numeric                | Inverse hyperbolic cosine function |
| `math::sin`          | 1               | Numeric                | Computes the sine of a number (in radians) |
| `math::asin`         | 1               | Numeric                | Computes the arcsine of a number. The return value is in radians in the range [-pi/2, pi/2] or NaN if the number is outside the range [-1, 1] |
| `math::sinh`         | 1               | Numeric                | Hyperbolic sine function |
| `math::asinh`        | 1               | Numeric                | Inverse hyperbolic sine function |
| `math::tan`          | 1               | Numeric                | Computes the tangent of a number (in radians) |
| `math::atan`         | 1               | Numeric                | Computes the arctangent of a number. The return value is in radians in the range [-pi/2, pi/2] |
| `math::atan2`        | 2               | Numeric, Numeric       | Computes the four quadrant arctangent in radians |
| `math::tanh`         | 1               | Numeric                | Hyperbolic tangent function |
| `math::atanh`        | 1               | Numeric                | Inverse hyperbolic tangent function. |
| `math::sqrt`         | 1               | Numeric                | Returns the square root of a number. Returns NaN for a negative number |
| `math::cbrt`         | 1               | Numeric                | Returns the cube root of a number |
| `math::hypot`        | 2               | Numeric                | Calculates the length of the hypotenuse of a right-angle triangle given legs of length given by the two arguments |
| `str::to_lowercase`  | 1               | String                 | Returns the lower-case version of the string |
| `str::to_uppercase`  | 1               | String                 | Returns the upper-case version of the string |
| `str::trim`          | 1               | String                 | Strips whitespace from the start and the end of the string |
| `str::from`          | >= 0            | Any                    | Returns passed value as string |
| `bitand`             | 2               | Int                    | Computes the bitwise and of the given integers |
| `bitor`              | 2               | Int                    | Computes the bitwise or of the given integers |
| `bitxor`             | 2               | Int                    | Computes the bitwise xor of the given integers |
| `bitnot`             | 1               | Int                    | Computes the bitwise not of the given integer |
| `shl`                | 2               | Int                    | Computes the given integer bitwise shifted left by the other given integer |
| `shr`                | 2               | Int                    | Computes the given integer bitwise shifted right by the other given integer |

## Constants
| Identifier | Value | Description |
|------------|-------|-------------|
| `math::pi` | 3.141592653589793 (`std::f64::consts::PI`) | Pi |
| `math::e` | 2.718281828459045 (`std::f64::consts::E`) | Euler's number |

## License

This crate is primarily distributed under the terms of the MIT license.