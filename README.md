# ast_serialize - Python Parser and AST Serializer

This is a fast Python extension for parsing Python files and serializing the AST using 
the native binary format used by mypy. This will eventually replace the current mypy parser,
which uses the Python stdlib `ast` module for parsing.

**This is work in progress.**

## Development

*You must use the `new-parser` branch in the mypy repository to use this with mypy.*

Development build (fast compilation, unoptimized):

```bash
maturin develop
```

Optimized development build:
```bash
maturin develop --release
```

Run Rust unit tests:
```bash
cargo test
```

Use `mypy/test/testcheck.py` in the `new-parser` branch to run mypy type checking tests 
using ast_serialize. The test runner enables the new runner by default. Note that many
tests are still failing.

## Creating PRs

You can create PRs in this repository, or you can target the `new-parser` mypy branch. 
Contributions are welcome! Run mypy tests (see above) to get ideas about bugs and missing 
functionality. If your contributions needs changes in both mypy and ast_serialize, please
mention this in the PR summary.

## Building

```bash
# Build all wheels (stable ABI + free-threaded)
./build_wheels.sh

# Or manually:
# Stable ABI wheel (Python 3.9+)
maturin build --release

# Free-threaded wheel (Python 3.13t)
maturin build --release --no-default-features --features free-threaded --interpreter python3.13t
```

If you see `Python 3.13t not found`, you'll need to install the free-threaded build of CPython 3.13
in PATH.

## Acknowledgments

This is a wrapper around the Ruff parser. Credits to Ruff maintainers for developing a fast Python parser!

## License

MIT
