# todo — CLI Task Manager

A fast, minimal terminal app to manage your tasks without leaving the shell. Built with Rust.

## Features

- Add, complete, and delete tasks
- Set **deadlines** with color-coded urgency (overdue, due today, upcoming)
- **Repeating tasks** that auto-reschedule when marked done
- Persistent storage in `~/.todo.json` — same list from anywhere
- Available system-wide after install

## Installation

Requires [Rust](https://rustup.rs/).

```bash
git clone https://github.com/HarshDalmia2005/RUST_TODO.git
cd RUST_TODO
cargo install --path .
```

## Usage

```bash
# List all tasks
todo

# Add tasks
todo add "buy milk"
todo add orange banana apple          # multiple at once

# Add with deadline
todo add --due 2026-04-20 "dentist"

# Add repeating task (auto-reschedules every N days when done)
todo add --repeat 7 "weekly review"

# Combine both
todo add --due 2026-04-20 --repeat 7 "weekly review"

# Mark done (repeating tasks reschedule automatically)
todo done 0
todo done 1 2 3                       # multiple

# Delete tasks
todo delete 0
todo delete 2 4                       # multiple

# Show help
todo help
```

## Example Output

```
  ── TODO LIST ── 3 tasks (1 done, 2 pending)

  0. ✔  dentist
  1. [ ] weekly review  [due in 7 days]  [repeats every 7 days]
  2. [ ] INTER IIT time  [due TODAY]
```

Deadlines are color-coded: 🔴 overdue · 🟡 due today · 🔵 upcoming

## Built With

- [Rust](https://www.rust-lang.org/)
- [serde](https://serde.rs/) + [serde_json](https://docs.rs/serde_json) — JSON persistence
- [chrono](https://docs.rs/chrono) — date arithmetic
