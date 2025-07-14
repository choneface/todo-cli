# crix-todo 🦀✅

A terminal-based todo list manager written in Rust, with a clean CLI and powerful interactive TUI.

Manage your tasks efficiently from your terminal — add, list, and mark todos with keyboard-driven speed.

---

## 📦 Installation

```bash
cargo install crix-todo
```

---

## 🛠 Features

- ✅ Add todos from the CLI with optional priority, due date, and tags
- 📋 List todos with rich filtering options
- 🖥 Interactive **TUI** for managing, expanding, and marking tasks complete
- 📂 Local `todo.json` file for storage — no sync, no cloud
- 🔤 Keybinding help shown inside the TUI

---

## 🚀 Usage

### Add a new todo

```bash
todo add "Finish Rust project" --priority 1 --due 2025-07-10 --tags work,urgent
```

### List todos

```bash
todo list
```

With filters:

```bash
todo list --priority 1 --tag work
```

### Launch the interactive TUI

```bash
todo edit
```

From there, you can navigate, mark tasks done, expand to see details, and quit with `q`.

---

## ⌨️ TUI Keybindings

| Key         | Action            |
|-------------|-------------------|
| ↑ / ↓       | Move selection    |
| ⏎ (Enter)   | Toggle done       |
| Space       | Expand details    |
| q           | Quit TUI          |

---

## 📂 Data Storage

Todos are stored in a plain JSON file (`todo.json`) in the same directory you run the CLI from.  
There’s no sync or account system — it’s just your todos, locally managed.

---

## 🧪 Development

Clone the repo and run:

```bash
cargo run -- edit
```

Or try adding tasks with:

```bash
cargo run -- add "Test the CLI"
```

---

## 🔗 Coming Soon

- Enhanced todo view
