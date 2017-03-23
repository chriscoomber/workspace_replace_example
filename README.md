# workspace_replace_example
Example of an issue I saw with [replace] in workspaces

# Cargo build/test

```
$ cargo multi build
---------------------
Executing cargo build
---------------------

first_crate:
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs

second_crate:
warning: package replacement is not used: https://github.com/rust-lang/crates.io-index#url:1.4.0
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
```

```
$ cargo multi test
--------------------
Executing cargo test
--------------------

first_crate:
   Compiling first_crate v0.1.0 (file:///data/cc3/workspace_replace_test/first_crate)
    Finished dev [unoptimized + debuginfo] target(s) in 0.42 secs
     Running /data/cc3/workspace_replace_test/target/debug/deps/first_crate-7ae9bbcaf7d93dc6

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests first_crate

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured


second_crate:
warning: package replacement is not used: https://github.com/rust-lang/crates.io-index#url:1.4.0
   Compiling second_crate v0.1.0 (file:///data/cc3/workspace_replace_test/second_crate)
    Finished dev [unoptimized + debuginfo] target(s) in 0.39 secs
     Running /data/cc3/workspace_replace_test/target/debug/deps/second_crate-b380d6e083e8b33e

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests second_crate

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```
