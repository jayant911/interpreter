# 🧠 Lox Interpreter in Rust

A tree-walk interpreter for the [Lox](https://craftinginterpreters.com/the-lox-language.html) programming language, built from scratch in **Rust** — following the legendary book [**Crafting Interpreters**](https://craftinginterpreters.com/) by **Robert Nystrom**.

> _"A handbook for making programming languages."_

---

## 📖 About

This project is a Rust implementation of the **jlox** tree-walk interpreter described in Part II of _Crafting Interpreters_. While the original book uses Java, this version reimagines the interpreter idiomatically in Rust, leveraging its strong type system, ownership model, and pattern matching.

### What is Lox?

Lox is a dynamically typed, high-level scripting language featuring:

- **Variables & expressions** — `var x = 10; print x + 5;`
- **Control flow** — `if`, `else`, `while`, `for`
- **Functions** — first-class, with closures
- **Classes** — single inheritance with `super`
- **Automatic memory management**

```lox
class Greeter {
  init(name) {
    this.name = name;
  }

  greet() {
    print "Hello, " + this.name + "!";
  }
}

var g = Greeter("world");
g.greet(); // Hello, world!
```

---

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.92.0 or later)

### Build

```bash
git clone https://github.com/jayanta911/interpreter.git
cd interpreter
cargo build --release
```

### Run a Lox script

```bash
cargo run -- path/to/script.lox
```

### Launch the REPL

```bash
cargo run
```

You'll be greeted with an interactive prompt:

```
> print "hello";
hello
> 2 + 3
5
```

Press **Ctrl+D** (EOF) to exit.

---

## 🏗️ Project Structure

```
interpreter/
├── Cargo.toml          # Project manifest & dependencies
├── src/
│   ├── main.rs         # CLI entry point, REPL loop, file runner
│   └── scanner.rs      # Lexer — tokenizes raw source into tokens
├── LICENSE             # MIT License
└── README.md
```

---

## 🗺️ Roadmap

Following the chapter progression of _Crafting Interpreters_ (Part II):

- [x] **Chapter 4** — Scanning (Lexer / Tokenizer)
- [ ] **Chapter 5** — Representing Code (AST)
- [ ] **Chapter 6** — Parsing Expressions
- [ ] **Chapter 7** — Evaluating Expressions
- [ ] **Chapter 8** — Statements and State
- [ ] **Chapter 9** — Control Flow
- [ ] **Chapter 10** — Functions
- [ ] **Chapter 11** — Resolving and Binding
- [ ] **Chapter 12** — Classes
- [ ] **Chapter 13** — Inheritance

---

## 📦 Dependencies

| Crate    | Purpose                           |
| -------- | --------------------------------- |
| `miette` | Beautiful, rich error diagnostics |
| `bytes`  | Efficient byte buffer management  |
| `clap`   | CLI argument parsing              |

---

## 📚 References

- 📘 [Crafting Interpreters](https://craftinginterpreters.com/) — Robert Nystrom
- 🦀 [The Rust Programming Language](https://doc.rust-lang.org/book/)

---

## 📄 License

This project is licensed under the **MIT License** — see the [LICENSE](LICENSE) file for details.

---

## 👤 Author

**Jayanta Pradhan**

---

> _Built with ❤️ and Rust, one token at a time._
