# 🦀 32-Day Rust Learning Plan

This is a structured 32-day journey through [The Rust Programming Language Book](https://doc.rust-lang.org/book/), covering all essential concepts with hands-on practice and a complete final project.

---

## 📅 Daily Breakdown

### Week 1 – Getting Started with Rust

| Day | Topic                                                                            |
| --- | -------------------------------------------------------------------------------- |
| 1   | Chapter 1: Getting Started – Install Rust, `cargo`, and run your first project   |
| 2   | Chapter 2: Programming a Guessing Game – Input handling, `rand`, `loop`, `match` |
| 3   | Hands-on: Customize the guessing game, experiment with `println!` and variables  |
| 4   | Chapter 3.1–3.2: Variables, mutability, shadowing, scalar and compound types     |
| 5   | Chapter 3.3–3.4: Functions and comments                                          |
| 6   | Chapter 3.5–3.6: Control flow – `if`, `loop`, `while`, `for`                     |
| 7   | Review and practice: build small logic exercises using loops and conditions      |

### Week 2 – Ownership, Structs, Enums, and Project Organization

| Day | Topic                                                              |
| --- | ------------------------------------------------------------------ |
| 8   | Chapter 4: Ownership – stack vs heap, ownership rules              |
| 9   | Chapter 4 (continued): Borrowing and references                    |
| 10  | Hands-on: Create a text analysis CLI to count characters and words |
| 11  | Chapter 5: Structs and defining methods                            |
| 12  | Chapter 6: Enums and pattern matching                              |
| 13  | Hands-on: Build a command-line calculator using enums and `match`  |
| 14  | Chapter 7: Managing projects with packages, crates, and modules    |

### Week 3 – Collections, Errors, Generics, Testing, and CLI

| Day | Topic                                                                    |
| --- | ------------------------------------------------------------------------ |
| 15  | Chapter 8: Collections – Strings, Vectors, HashMaps                      |
| 16  | Chapter 9: Error Handling – `panic!`, `Result`, `unwrap`, `expect`       |
| 17  | Hands-on: Build a simple CSV reader with error handling                  |
| 18  | Chapter 10: Generics – abstract over types                               |
| 19  | Chapter 10 (continued): Traits and lifetimes                             |
| 20  | Chapter 11: Writing automated tests                                      |
| 21  | Chapter 12: CLI project – setup, `env::args`, and planning               |
| 22  | Chapter 12 (continued): File reading, logic implementation, and testing  |
| 23  | Refactor CLI project – modularize, add error handling, improve structure |

### Week 4 – Advanced Rust Concepts

| Day | Topic                                                                   |
| --- | ----------------------------------------------------------------------- |
| 24  | Chapter 13: Iterators and closures                                      |
| 25  | Chapter 14: Cargo and crates.io in depth                                |
| 26  | Chapter 15: Smart pointers – `Box`, `Rc`, `RefCell`                     |
| 27  | Chapter 16: Concurrency – threads, channels, `Mutex`                    |
| 28  | Chapters 17–19: OOP in Rust, patterns, and advanced features            |
| 29  | Chapter 20: Final project example – building a multithreaded web server |

---

## 🔨 Final Rust Project (Days 30–32)

### Day 30 – Planning and Setup

- Choose a project type: CLI, API, parser, file watcher, etc.
- Define the scope: input, output, structure
- Create with `cargo new`, configure `Cargo.toml`
- Add crates like `clap`, `serde`, `reqwest`, etc.

### Day 31 – Implementation and Testing

- Write the core logic and functionality
- Handle errors with `Result`, `Option`, and pattern matching
- Write unit and integration tests using `cargo test`

### Day 32 – Refactoring and Documentation

- Organize code into modules (`mod`, `lib.rs`)
- Add documentation comments (`///`)
- Write a README.md with instructions and usage
- Optionally run `cargo clippy`, `cargo fmt`, and publish on GitHub

---

## 🧠 Tips

- Use [Rustlings](https://github.com/rust-lang/rustlings) for daily exercises.
- Take notes and save code snippets you find useful.
- Keep your project ideas small and focused.

Happy coding crustacean! 🚀
