# Lyrids

**An experimental semantic version control system** designed for performance, clarity, and meaningful change tracking.  
Unlike traditional systems like Git, Lyrids focuses on **semantic diffs** instead of full file snapshots, optimizing for local-first development and efficiency at scale.

---

## 🚧 Status

> ⚠️ This project is in early experimental development.  
> Expect breaking changes, minimal documentation, and evolving architecture.

---

## 🧠 Core Concepts

- **Semantic Commits**: Each commit represents logical operations (e.g., "add line", "remove line"), not entire file diffs.
- **Compact History**: Multiple redundant changes collapse into their net effect.
- **Snapshots**: Auto-generated after configurable thresholds to prevent long chains of diffs.
- **Reproducibility**: Repository state is reconstructed by applying changes over snapshots.
- **Simple Internals**: Plaintext or JSON-based metadata stored in `.lyrids/`.

---

## ✨ Key Differences from Git

| Feature                | Git                              | Lyrids                         |
|------------------------|----------------------------------|-------------------------------|
| Storage Model          | Snapshots (content-addressed)    | Semantic diffs + snapshots    |
| Commits                | Full-tree diffs                  | Semantic operations           |
| Merge Strategy         | Tree-based, line-based merge     | Patch-based, merge-aware      |
| Performance Target     | General-purpose                  | Optimized for semantic churn  |
| Conflict Detection     | Line-based                       | Operation-level               |

---

## 🔧 CLI (Planned Commands)

```bash
lyr init           # Initialize a new lyrids repo
lyr status         # Show unstaged changes
lyr commit         # Create semantic
