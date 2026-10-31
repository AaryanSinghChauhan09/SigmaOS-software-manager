# SigmaOS Software Manager

This crate defines a package-backend interface and a manager that delegates
search, install, and remove operations to registered backends. The manager
starts with no backends. Its test-only `StubAptBackend` supplies sample data and
does not call the system package manager, so test results do not demonstrate
real package installation.

Add and document a production backend, package trust checks, and transaction
recovery before using this API to change a host.

## License

GPL-3.0-or-later. See [`LICENSE`](LICENSE) and [`Cargo.toml`](Cargo.toml).
