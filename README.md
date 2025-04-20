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
lyr commit         # Create semantic commit from changes
lyr log            # Show commit DAG
lyr branch         # Create/switch branches
lyr checkout       # Reconstruct folder from a given commit/branch
lyr merge          # Merge two branches (patch-aware)
lyr snapshot       # Manually create a snapshot
lyr clone <repo>   # Clone a lyrids repo (experimental)
```
## 📁 Structure Overview

```
.lyrids/
├── commits/           # Each commit as a semantic patch
├── snapshots/         # Snapshots of full project state
├── branches.json      # Branch pointers
├── index.json         # Staging area (optional)
└── config.json        # User config
```

## 🪪 License

This project is licensed under the [MIT License](LICENSE).

## 🙌 Contributing

Contributions are currently closed while the project reaches a functional MVP.
Once stable, contribution guidelines and issue tracking will be enabled.

## 📍 Inspiration

Lyrids is inspired by tools like [Git](https://git-scm.com/), [Pijul](https://pijul.org/), and [Darcs](https://darcs.net/), but takes a distinct approach to how change is stored and replayed.