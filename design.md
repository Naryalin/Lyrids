# 📐 Lyrids – Initial Technical Design

Lyrids is an experimental version control system designed to provide semantic commits, flexible branching, and a simplified yet powerful approach to local file versioning.

This document outlines the initial architecture, main components, and core system design decisions.

## 📦 Repository Structure

When `lyrids init` is run, a `.lyrids/` folder is created to hold all version control data.

```
.lyrids/
├── commits/            # Contains .lyr files, one per commit
├── snapshots/          # (future) Compressed repo snapshots
├── index.json          # Staging area state
├── branches.json       # Maps branches to commits and snapshots
└── HEAD                # Currently active branch
```

## 🔧 Core Concepts

### ✅ Semantic Commits

Each commit stores **only the changes** from the previous one, structured (initially) as a line-by-line diff.

- `.lyr` commit file example:
  ```json
  [
    { "type": "add", "file": "main.rs", "line": 5, "content": "println!(\"Hello\");" },
    { "type": "remove", "file": "lib.rs", "line": 3 }
  ]
  ```

### ✅ Staging Area (index.json)

The `index.json` file represents the current modified files staged for commit.

```json
{
  "modified": ["src/main.rs"],
  "added": ["src/utils.rs"],
  "removed": []
}
```
## 🌱 Branching

Each branch maps to:

- The last commit ID
- The last associated snapshot (optional, for future optimization)

```json
// branches.json
{
  "main": {
    "last_commit": "a9f3c7",
    "last_snapshot": null
  }
}
```

The `HEAD` file contains the active branch name, e.g.:

```
main
```

## 🔄 Commit Application

Commits are applied on top of a base snapshot or state to reconstruct the current project files. The engine will follow logic like:

```rust
fn apply_commits(base_dir: &str, commits: Vec<Commit>) {
    // Applies each commit in order to rebuild the final state
}
```

## 🧠 Example Workflow

```bash
lyrids init
lyrids status
lyrids commit -m "first commit"
lyrids branch dev
lyrids checkout dev
lyrids commit -m "add new function"
```

## 🔮 Future Considerations

- Snapshots every N commits
- Smart merging between branches
- Export a branch as a reconstructed directory
- Optional visual CLI (like `tig`)
- Remote collaboration (pull/push)

## 📌 Current Status

- [x] Repo structure created
- [ ] `init` command
- [ ] `status` command
- [ ] `commit` command

This document will evolve as new roadmap phases are implemented.