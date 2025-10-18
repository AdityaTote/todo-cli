# 📋 Todo CLI

<div align="center">

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)

**A beautiful, fast, and intuitive command-line todo manager written in Rust** 🦀

[Features](#-features) • [Installation](#-installation) • [Usage](#-usage) • [Examples](#-examples) • [Building](#-building-from-source)

</div>

---

## ✨ Features

- 🎯 **Priority Management** - Organize tasks with High 🔴, Medium 🟡, and Low 🟢 priorities
- ✅ **Status Tracking** - Track completion with Pending ◯ and Completed ✓ states
- 🔍 **Smart Filtering** - Filter by priority or status instantly
- 📊 **Sorting** - Sort tasks by priority (high-to-low or low-to-high)
- 💾 **Persistent Storage** - All todos are automatically saved to JSON
- 🎨 **Beautiful UI** - Clean, colorful terminal output with formatted tables
- ⚡ **Fast & Lightweight** - Built with Rust for maximum performance
- 📝 **Rich Descriptions** - Add detailed descriptions to your tasks

---

## 🚀 Installation

### Prerequisites

- Rust 1.70+ and Cargo

### Install from source

```bash
git clone https://github.com/yourusername/todo-cli.git
cd todo-cli
cargo build --release
```

The binary will be available at `target/release/todo-cli`

### Add to PATH (Optional)

```bash
# Linux/macOS
sudo cp target/release/todo-cli /usr/local/bin/

# Or add an alias to your shell config
echo 'alias todo="~/path/to/todo-cli/target/release/todo-cli"' >> ~/.zshrc
source ~/.zshrc
```

---

## 📖 Usage

### Basic Syntax

```bash
todo [COMMAND] [ARGUMENTS]
```

### Available Commands

| Command         | Description              | Example                                   |
| --------------- | ------------------------ | ----------------------------------------- |
| `add`           | Create a new todo        | `todo add "Task name" high "Description"` |
| `list`          | Display all todos        | `todo list`                               |
| `delete`        | Remove a todo            | `todo delete "Task name"`                 |
| `done`          | Mark a todo as completed | `todo done "Task name"`                   |
| `list-priority` | Filter by priority       | `todo list-priority high`                 |
| `list-status`   | Filter by status         | `todo list-status pending`                |
| `sort`          | Sort by priority         | `todo sort high-to-low`                   |
| `help`          | Show help message        | `todo help`                               |

---

## 💡 Examples

### Adding Tasks

```bash
# Add a high priority task
todo add "Fix production bug" high "Critical bug affecting users"

# Add a medium priority task
todo add "Write documentation" medium "Update API docs"

# Add a low priority task
todo add "Refactor old code" low "Technical debt cleanup"
```

### Viewing Tasks

```bash
# List all todos
todo list

# Output:
# ╔═══════════════════════════════════════════════════════════════════════════════════════════════════╗
# ║                                    📋 YOUR TODO LIST                                              ║
# ╠═══════════════════════════════════════════════════════════════════════════════════════════════════╣
# │ NAME                      │ STATUS       │ PRIORITY     │ DESCRIPTION                              │
# ├───────────────────────────────────────────────────────────────────────────────────────────────────┤
# │ Fix production bug        │ ◯ Pending    │ 🔴 HIGH      │ Critical bug affecting users             │
# │ Write documentation       │ ◯ Pending    │ 🟡 MEDIUM    │ Update API docs                          │
# │ Refactor old code         │ ◯ Pending    │ 🟢 LOW       │ Technical debt cleanup                   │
# ╚═══════════════════════════════════════════════════════════════════════════════════════════════════╝
```

### Filtering Tasks

```bash
# Show only high priority tasks
todo list-priority high

# Show only completed tasks
todo list-status completed

# Show only pending tasks
todo list-status pending
```

### Sorting Tasks

```bash
# Sort from high to low priority
todo sort high-to-low

# Sort from low to high priority
todo sort low-to-high
```

### Completing Tasks

```bash
# Mark a task as done
todo done "Fix production bug"

# Output:
# ✓ Todo 'Fix production bug' marked as completed!
```

### Deleting Tasks

```bash
# Remove a task
todo delete "Refactor old code"

# Output:
# Todo deleted successfully
```

---

## 🎨 Screenshot

```
╔═══════════════════════════════════════════════════════════════════════════════════════════════════╗
║                                    📋 YOUR TODO LIST                                              ║
╠═══════════════════════════════════════════════════════════════════════════════════════════════════╣
│ NAME                      │ STATUS       │ PRIORITY     │ DESCRIPTION                              │
├───────────────────────────────────────────────────────────────────────────────────────────────────┤
│ Fix production bug        │ ✓ Completed  │ 🔴 HIGH      │ Critical bug affecting users             │
│ Write documentation       │ ◯ Pending    │ 🟡 MEDIUM    │ Update API docs                          │
│ Refactor old code         │ ◯ Pending    │ 🟢 LOW       │ Technical debt cleanup                   │
╚═══════════════════════════════════════════════════════════════════════════════════════════════════╝

📊 Summary: 3 total │ ✓ 1 completed │ ◯ 2 pending
```

---

## 🏗️ Building from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/todo-cli.git
cd todo-cli

# Build in debug mode
cargo build

# Build in release mode (optimized)
cargo build --release

# Run tests
cargo test

# Run directly with cargo
cargo run -- list
```

---

## 📦 Project Structure

```
todo-cli/
├── Cargo.toml          # Project configuration and dependencies
├── Cargo.lock          # Dependency lock file
├── README.md           # This file
├── data/               # Storage directory for todos
│   └── .todo_store.json
└── src/
    ├── main.rs         # CLI entry point and command handling
    └── lib.rs          # Core logic and data structures
```

---

## 🛠️ Technology Stack

- **Language**: Rust (Edition 2024)
- **Serialization**: `serde` + `serde_json`
- **Storage**: JSON file-based persistence
- **CLI**: Native Rust arg parsing

---

## 📝 Data Storage

Todos are automatically saved to `data/.todo_store.json` in JSON format:

```json
[
	{
		"name": "Fix production bug",
		"status": "Completed",
		"priority": "High",
		"description": "Critical bug affecting users"
	},
	{
		"name": "Write documentation",
		"status": "Pending",
		"priority": "Medium",
		"description": "Update API docs"
	}
]
```

---

## 🤝 Contributing

Contributions are welcome! Here's how you can help:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Areas for Improvement

- [ ] Add due dates for tasks
- [ ] Implement task categories/tags
- [ ] Add search functionality
- [ ] Create configuration file for customization
- [ ] Add export/import functionality
- [ ] Implement task editing
- [ ] Add recurring tasks
- [ ] Create interactive TUI mode

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---
