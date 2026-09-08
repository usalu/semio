---
name: repo
kind: infrastructure
summary: Monorepo tools that are ai-ready.
---

## ✨️ Features

- 🤖️ AI integrations
  - ✅️ Currently
    - 🧑️‍✈️ Copilot
    - 🌊️ Cascade
    - 🖱️ Cursor Agent
    - ❄️ Claude Code
    - ⚙️ Codex
  - 📅️ Possible
    - 🦾️ Droid
    - …
- 🧑️‍💻️ IDE integrations
  - ✅️ Currently
    - 💻️ VSCode
    - 🌊️ Windsurf
    - 🖱️ Cursor
    - Antigravity
    - ⚙️ Codex
    - ❄️ Claude Code
    - 🦾️ Droid
  - 📅️ Possible
    - InteliJ
    - PyCharm
    - WebStorm
    - Rider
    - Android Studio
    - …
- 🗣️ Language integrations
  - ✅️ Currently
    - 🟦️ Typescript
    - 🐹️ Go
    - 🐍️ Python
    - 🦀️ Rust
    - 🟣️ C#
  - 📅️ Possible
    - ♨️ Java
    - 🟪️ Kotlin
    - ➕️ C++
    - © C
    - 🐘️ PHP
    - 💎️ Ruby
    - 🐦️ Swift
    - …
- 📦️ Sandbox integrations
  - ✅️ Currently
    - 🐋️ Devcontainers
  - 📅️ Possible
    - 🦭️ Podman
    - …
- 👥️ Tracker integrations
  - ✅️ Currently
    - 🐙️ GitHub
  - 📅️ Possible
    - 🦊️ GitLab
    - 🪣️ Bitbucket
    - …

### 🤖️ AI

| System          | Agents | Skills | Hooks |
| --------------- | :----: | :----: | :---: |
| 🧑️‍✈️ Copilot      |   ✅️   |   ✅️   |  ✅️   |
| 🌊️ Cascade      |   ✅️   |   ✅️   |  ✅️   |
| 🖱️ Cursor Agent |   ✅️   |   ✅️   |  ✅️   |
| ❄️ Claude Code  |   ✅️   |   ✅️   |  ✅️   |
| ⚙️ Codex        |   ✅️   |   ✅️   |  ❌️   |
| (🦾️ Droid)      |   ✅️   |   ✅️   |  ✅️   |

###

## 🥇️ Why repo is a game changer

### 📈️ Requirements + Docs + Stats + Semantics

#### 🚀️ Agents love TDD

[Test-Driven-Development (TDD)](https://en.wikipedia.org/wiki/Test-driven_development) is a game changer for agents 🚀️

##### ↔ Multi-lanugage development

Ever had the problem that you domain-expert devs write Python/C++/C/Javascript/Ruby/Lua/… but not Typescript/Go/Rust/C#/Julia/… or vice versa?

No problem, let them write what they know, until the tests are extended and let agents reimplement it natively until the tests succeeds ✅️

## 😥️ What, you'll have to abandon

### ❌️ No granular files, only godfiles

### ❌️ No normal docstrings

### ❌️ No inline comments

## 😲️ What, you'll get

### 🚀️ Zero-touch development

### 🧪️ Shared test infrastructure

### 💯️ Consistent requirements

### 📑️ Conistent docs

### 🔮️ Future proof infrastructure

### 📊️ Meaningful stats

## Configuration

Repo-wide settings live in [`.🧬semio/🦑️repo/config.toml`](../.🧬semio/🦑️repo/config.toml) at the monorepo root.

| Key                  | Default    | Description                                                                 |
| -------------------- | ---------- | --------------------------------------------------------------------------- |
| `logging.session`    | `false`    | Write per-session `session.json` under `.🧬semio/🦑️repo/⚡️/🤖️/…` on agent hooks       |
| `logging.operations` | `true`     | Append derived `agent.<operation>.<phase>` events (requires `session = true`)      |
| `logging.plan`       | `true`     | Track agent plan steps in `session.json` (requires `session = true`)        |
| `logging.detail`     | `standard` | `minimal` (event only), `standard` (+ response), or `full` (+ native stdin) |

Set `logging.session = true` to enable session-file logging for debugging or coordinator ingestion.

# 🔨️ Modules

- [`💻️client/⌨️cli`](🔨️modules/💻️client/⌨️cli) – Command line tool for monorepo interactions
- [`💻️client/🔌️mcp`](🔨️modules/💻️client/🔌️mcp) – MCP server the agent clients speak to
- [`💻️client/🧩️vscode`](🔨️modules/💻️client/🧩️vscode/README.md) – Visual Studio Code extension, owner of the technology catalog
- [`💻️client/🪶️sqlite`](🔨️modules/💻️client/🪶️sqlite/README.md) – SQLite schema of the client's local-only entity store
- [`🖥️server/🎛️coordinator`](🔨️modules/🖥️server/🎛️coordinator/README.md) – Coordinator REST API and event log
- [`🖥️server/🧬️schema`](🔨️modules/🖥️server/🧬️schema) – PostgreSQL schema of the shared server state
- [`📚️library`](🔨️modules/📚️library) – Taxonomy, discovery, normalization and the derived schema catalog
- [`🧪️test`](🔨️modules/🧪️test/README.md) – Cross-module test harness
