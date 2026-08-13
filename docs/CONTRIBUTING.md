# Contributing

Thanks for your interest in Vaultkey! This document outlines how to contribute to the project.

## Code of Conduct

Be respectful and constructive. We're building a secure password manager—let's keep it that way.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/shuva-kharel/vaultkey.git`
3. Create a branch: `git checkout -b feature/your-feature`
4. Make changes and test
5. Push and open a pull request

## Development Setup

```bash
# Install dependencies (see docs/SETUP.md)

# Build both components
cd vaultkey-server && cargo build
cd ../vaultkey-cli && cargo build

# Run tests
cargo test

# Check formatting
cargo fmt -- --check

# Lint
cargo clippy
```

## Code Style

We follow standard Rust conventions:

- Use `cargo fmt` before committing
- Run `cargo clippy` to catch common mistakes
- Keep functions small and focused
- Add tests for new features
- Document public APIs

```bash
# Format your code
cargo fmt

# Run clippy
cargo clippy -- -D warnings
```

## Testing

### Unit Tests

```bash
# Run all tests
cargo test

# Run tests for a specific crate
cd vaultkey-cli && cargo test

# Run a specific test
cargo test test_encryption
```

### Integration Tests

Tests in `tests/` directory run against the actual server and CLI.

```bash
# Start the server
cd vaultkey-server && cargo run &

# Run integration tests
cd ../vaultkey-cli && cargo test --test '*'
```

## Submitting Changes

### Before You Start

- Check [existing issues](https://github.com/shuva-kharel/vaultkey/issues) to avoid duplicate work
- For large features, open an issue for discussion first
- Comment on issues you plan to work on

### Commit Messages

Write clear, descriptive commit messages:

```
Add password strength checker

- Implement ZXCVBN algorithm
- Add strength UI indicator
- Add tests for strength scoring
```

**Format**:
- First line: concise summary (50 chars max)
- Blank line
- Detailed explanation if needed
- Reference issues: "Fixes #123"

### Pull Requests

1. Keep changes focused on a single feature/fix
2. Update documentation if needed
3. Add tests for new functionality
4. Include a clear PR description:
   ```markdown
   ## Description
   What does this change do?

   ## Related Issues
   Fixes #123

   ## Testing
   How was this tested?

   ## Checklist
   - [ ] Code follows style guidelines
   - [ ] Tests added/updated
   - [ ] Documentation updated
   - [ ] No new warnings from clippy
   ```

## Areas to Contribute

### High Priority

- [ ] Real FIDO2/YubiKey support
- [ ] Web UI
- [ ] Browser extension
- [ ] Import from other password managers
- [ ] Rate limiting and security hardening

### Medium Priority

- [ ] Argon2 key derivation
- [ ] Audit logging
- [ ] Password strength checker
- [ ] Team sharing
- [ ] Mobile app

### Lower Priority

- [ ] Database optimization
- [ ] Additional password generation options
- [ ] Internationalization
- [ ] Theme customization

## Security Contributions

**Do not open public issues for security vulnerabilities.**

Report security issues privately to: `security@vaultkey.example.com`

Include:
- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

## Documentation

Documentation improvements are always welcome:

- Fix typos and clarity issues
- Add examples
- Improve setup instructions
- Document edge cases
- Add troubleshooting sections

Docs live in the `docs/` directory and in README.md.

## Architecture Decisions

Major changes (API design, architecture, new modules) should be discussed in an issue first. Here's what to include:

1. **Problem**: What issue does this solve?
2. **Proposed Solution**: How does your approach work?
3. **Alternatives**: What other approaches were considered?
4. **Tradeoffs**: Pros and cons of your solution
5. **Implementation Plan**: How will this be built?

## Review Process

All PRs require:
- At least one approval
- All CI checks passing
- No new clippy warnings
- Tests covering new code

Reviewers will provide feedback. Please address comments constructively.

## Licensing

By contributing, you agree that your code is licensed under the MIT License.

## Questions?

- Check the [documentation](../docs/)
- Review existing [pull requests](https://github.com/shuva-kharel/vaultkey/pulls)
- Open a [discussion](https://github.com/shuva-kharel/vaultkey/discussions)

## Recognition

Contributors will be acknowledged in:
- CHANGELOG.md
- GitHub contributors page
- Release notes

Thank you for making Vaultkey better! 🔐