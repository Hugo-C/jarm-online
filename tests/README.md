# Test

This folder contains all kind of tests.  
Check the [Makefile](../Makefile) to run them.

## Integration tests

Those tests are Rust files and run tests from a public interface point of view. They can be of Rust function or API call.
Some are specifically marked with `#[ignore = "Integration tests"]` since they require an external service to be accessible (like Redis).
Test needing only SQLite should not be marked as such, as it works locally without setup.

## End to end tests

Those tests are Python files in `playwright_end_to_end` that test the GUI and backend combined behavior. 