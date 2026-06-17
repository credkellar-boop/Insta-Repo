# Contributing to Insta-Repo

We welcome pull requests for new heuristic signatures, performance optimizations, and integrations.

## Adding a Signature
1. Add the regex pattern to the appropriate list in the core engine or rules configuration.
2. Create a dummy file in `tests/fixtures/` that triggers your new signature.
3. Update `tests/integration_test.rs` to ensure the scanner correctly flags your new fixture.
4. Run `./scripts/audit.sh` to ensure all tests pass before submitting your PR.
5. 
