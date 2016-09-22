Ruroonga Expr
===

[![Build Status](https://travis-ci.org/cosmo0920/ruroonga_expr.svg?branch=master)](https://travis-ci.org/cosmo0920/ruroonga_expr)

## A Groonga Expr Builder for Rust.

ruroonga_expr provides Groonga expression builder and generator for query syntax. It reduces runtime errors about Groonga expression.

## Usage

Add following lines to your Cargo.toml:

```toml
[dependencies]
ruroonga_expr = "~0.1.0"
```

and following lines to your crate root:

```rust,ignore
extern crate ruroonga_expr;
```

### example

```rust
extern crate ruroonga_expr as expr;

use expr::dsl::*;

fn main() {
    let lexpr = fulltext_expr("Rust").column("language").prepare();
    let rexpr = fulltext_expr("Haskell").column("language").prepare();
    let comb_lexpr = greater_equal_expr("n_likes", "10").prepare();
    let result = (comb_lexpr % (lexpr + rexpr)).build();
    println!("{}", result);
    // #=> 'n_likes:>=10 (language:@Rust + language:@Haskell)'
}
```

### Target Rust Version

1.11.0 or later.

### Minimum required Groonga Version

6.0.3 or later.

## LICENSE

[MIT](LICENSE).
