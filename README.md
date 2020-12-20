# lsquare

Simple implementation of least square method.

## Example

```rust
extern crate lsquare;

use lsquare::Function;
use rand::*;

// Representation of a form of function [A log x + Bx + C]
struct SomeEquation {}

impl Function< f64, 3 > for SomeEquation {
    fn var(x: f64) -> [f64; 3] {
        [x.ln(), x, 1.0]
    }
}

fn main() {
    let mut rng = rand::thread_rng();

    // Generate of function [3 log x + 6x + 5] from 'SomeEquation'
    let f = SomeEquation::function([3.0, 6.0, 5.0]);

    // 1024 x-data in [0, 100)
    let xs = (0..1024).map(|_| rng.gen_range(0f64, 100f64)).collect::< Vec< _ > >();

    // 1024 y-data with error
    let ys = xs.iter().map(|x| f(*x) + rng.gen_range(-1f64, 1f64)).collect::< Vec< _ > >();

    // Calculate coefficient
    // Expect: [3, 6, 5]
    println!("{:?}", SomeEquation::least_square(xs.iter(), ys.iter()));
}
```
