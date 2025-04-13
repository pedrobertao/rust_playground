# Security Policy

## Supported Versions

This project is for **educational and personal study purposes**. While security is considered in the examples and code, this repository is **not intended for production use**. However, we still welcome responsible disclosure of any security-related issues.

| Version | Supported |
|---------|-----------|
| Main (default branch) | ✅ |
| Archived versions or tags | ❌ |

---

## Reporting a Vulnerability

If you discover a security vulnerability or potential risk in this repository:

1. **Do not open a public issue.**
2. Include:
   - A clear description of the issue
   - Steps to reproduce (if applicable)
   - Potential impact

We will investigate and respond as quickly as possible.

---

## Security Considerations

While this is a Rust learning repository, we aim to promote secure development practices. Here are a few things we focus on:

- Safe handling of user input
- Avoiding `unsafe` blocks unless explicitly for learning
- Encouraging use of tools like [Clippy](https://github.com/rust-lang/rust-clippy) and [cargo-audit](https://github.com/RustSec/cargo-audit)
- Using updated dependencies with `cargo audit`

If you're using code from this repository in production, please conduct your own thorough security review.

---

## Tools and References

- [RustSec Advisory Database](https://rustsec.org/)
- [cargo-audit](https://github.com/RustSec/cargo-audit)
- [Clippy linter](https://github.com/rust-lang/rust-clippy)
- [The Rust Programming Language - Official Book](https://doc.rust-lang.org/book/)

---

## Thank You 🙏

Thanks for helping make the Rust learning community safer and more robust!
