# todo — CLI To-Do List App

A fast, minimal terminal app to manage your tasks without leaving the shell. Built with Rust.

## Installation

Requires [Rust](https://rustup.rs/) installed.

```bash
git clone https://github.com/HarshDalmia2005/RUST_TODO.git
cd RUST_TODO
cargo install --path .
```

After install, the `todo` command is available system-wide.

Tasks are saved to `~/todo.json` — the same list from every directory.

---

## Usage

### List all tasks
```bash
todo
```
```
      TODO LIST

0. [ ]    buy milk
1.  ✔    learn rust
2. [ ]    exercise
```

### Add tasks
```bash
todo add "buy milk"
todo add orange banana pierogi apple    # add multiple at once
```

### Mark as done
```bash
todo done 0          # mark task 0 as done
todo done 1 2 3      # mark multiple tasks as done
```

### Delete tasks
```bash
todo delete 0        # delete task 0
todo delete 2 4      # delete multiple tasks
```

---

## How It Works

- Tasks are stored as JSON in `~/todo.json`
- Each task has a `name` and `done` flag
- Indices are based on list position (0-based)

---

## Built With

- [Rust](https://www.rust-lang.org/)
- [serde](https://serde.rs/) + [serde_json](https://docs.rs/serde_json) — JSON serialization
