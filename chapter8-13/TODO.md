# Things to Do
## Rust - handling the symbol table
DONE
Implemented an Arena (growing vector of environments) to handle closure friendly scopes.
This handles borrow checker problems with sharing mutable state by passing a long lived mutable arena of environments. The memory is managed automatically and uses indexes instead of references.
It grows indefinitely, no GC.
## Next book Stage - 
TODO
## Better hash table for this use case
TODO
Note that Chapter 11 may make this step unnecessary 
rustc-hash https://crates.io/crates/rustc-hash (FxHashMap):
      • The exact hash map used inside the Rust compiler (rustc).
      • Uses the fast FxHash algorithm (developed by Firefox), optimized specifically for short symbol names, AST
      identifiers, and small integers.
      • Zero cryptographic overhead; significantly faster than std::HashMap for compilers/interpreters.
