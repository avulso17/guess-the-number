# 🎯 Guess the Number – A Rust Terminal Game

This is a simple command-line game written in Rust where the player tries to guess a randomly generated secret number.

## 📌 Description

The game picks a random number between 1 and 100 and prompts the player to guess until they get it right. After each attempt, the game will inform the player whether their guess was too low, too high, or correct.

It demonstrates basic Rust concepts such as:

- Handling user input via `io::stdin`
- Using external crates like `rand`
- Pattern matching with `match` and the `Result` type
- Comparison with `std::cmp::Ordering`
- Managing loops and conditional logic
- Input parsing and error handling

## 🚀 How to Run

1. Clone the repository:
```bash
git clone https://github.com/your-username/guess-the-number-rust.git
cd guess-the-number-rust
```
   
2. Run the game with Cargo:
```bash
cargo run
```

## 📦 Dependency

This project uses the rand crate to generate the secret number.

Make sure to include the following in your Cargo.toml:
```toml
[dependencies]
rand = "0.8"
```

## 💡 Purpose

This project was created as part of a learning journey into the Rust programming language. It's a great beginner-friendly exercise to practice:

- Input handling
- Safe typing
- Error management
- Structuring simple terminal apps in Rust

Feel free to fork, modify, or contribute!
