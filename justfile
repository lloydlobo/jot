# Cargo check
c:
  cargo check

# Lint with clippy
cl:
  cargo clippy

# Lint and fix with clippy
clf:
  cargo clippy fix

# Create and serve rustdoc for jot w/o dependecies
d:
  cargo d -p jot -r --no-deps --open

# Default `just` command to list all commands
default:
  just --list

# Run bin in jot/src/jot.rs
r:
  cargo r -p jot

# Run with `RUST_BACKTRACE=1` environment variable to display a backtrace
rtrace:
  RUSTBACKTRACE=1 cargo r -p jot

# Test lib jot
t:
  cargo test -p jot

# Watch and Run bin in jot/src/jot.rs
w:
  cargo watch -x 'r -p jot'

# Watch and test project jot
wt:
  cargo watch -x 't -p jot'
