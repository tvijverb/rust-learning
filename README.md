# Rust basic commands
Compile Rust program main.rs
```
rustc main.rs
./main
```

New Rust project no git
```
cargo new <name> --vcs none
cd <name>
```

Compile and run Cargo project
```
cargo run
```

Compile release version
```
cargo build --release
```

Run code check
```
cargo check
```

Update rust
```
rustup update
```

Test rust
```
cargo test ( -- --show-output --test-threads=2)
```

Documentation
```
cargo doc
```

Remove all git (sub)directories
```
( find . -type d -name ".git" && find . -name ".gitignore" && find . -name ".gitmodules" ) | xargs rm -rf
```
