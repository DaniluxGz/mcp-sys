<p align="center">
  <img src="assets/logo.png" alt="mcp-sys logo" width="200"/>
</p>

# mcp-sys — MCP Server for System Diagnostics

A Rust MCP server that exposes live CPU, RAM, processes and ports to any AI agent — Claude, Cursor, Windsurf, Continue.dev and more.

![CI](https://img.shields.io/github/actions/workflow/status/DaniluxGz/mcp-sys/ci.yml?style=flat-square)
![Version](https://img.shields.io/github/v/release/DaniluxGz/mcp-sys?include_prereleases&style=flat-square)
![License](https://img.shields.io/badge/License-MIT-blue?style=flat-square)
![Platform](https://img.shields.io/badge/Platform-Windows-lightgrey?style=flat-square&logo=windows)

## Demo

<p align="center">
  <img src="assets/demo-v0.1.0.gif" alt="mcp-sys demo" width="700"/>
</p>

## What it does

AI chatbots don't execute code — but modern AI agents do. They can run commands directly on your machine. The problem: they do it blind, injecting code into your terminal searching for what they need, sometimes hitting processes, ports, or data they shouldn't touch.

**mcp-sys fixes that** — it gives Claude and any AI agent secure, instant, real-time visibility into your OS state. No blind probing. No unnecessary execution.

**With mcp-sys**
```
Dev: "Why isn't my server starting on port 3000?"
Claude calls mcp-sys →
"Port 3000 is taken by node.exe (PID 14821).
 RAM is at 89% — Chrome is the heaviest process.
 Want me to help free up resources?"
```

> **Why this matters for agentic AI:** When an AI agent executes code directly on your machine, it needs real system context to act safely. mcp-sys provides that context — preventing blind decisions that could affect running processes, occupied ports, or an overloaded system.

---

## Why Rust?

- **Single binary** — one `.exe`, no runtime, no dependencies to install
- **Native performance** — reads CPU, RAM and ports with minimal overhead
- **Memory safe** — no crashes, no leaks, runs quietly in the background
- **Cross-platform core** — Windows today, Linux and macOS ready for v0.3

## Installation

### 1. Download

Grab your binary from the [latest release](https://github.com/DaniluxGz/mcp-sys/releases/latest) — no dependencies. No installer. Just one binary — download and run.

> 🦀 **Rust developer?** Build from source:
> ```bash
> git clone https://github.com/DaniluxGz/mcp-sys
> cd mcp-sys
> cargo build --release
> ```

### 2. Configure

Add this to your MCP client config:
```json
{
  "mcpServers": {
    "mcp-sys": {
      "command": "/absolute/path/to/mcp-sys"
    }
  }
}
```

> 💡 Not sure where the config file is? Ask your AI assistant: *"How do I add an MCP server to [client name]?"*

### 3. Restart and go

Works with Claude Desktop, Claude Code, Cursor, Windsurf, Continue.dev and more.

> ⚠️ **Windows note:** Windows may flag the binary on first run.
> Click **"More info" → "Run anyway"** to proceed.
> This is expected for unsigned open-source binaries.

---

## Tools

| Tool | Description | Returns |
| --- | --- | --- |
| `system_stats` | Snapshot of current system state | CPU %, RAM used/total, disk used/total |
| `list_processes` | Running processes | PID, name, CPU %, RAM per process |
| `list_ports` | Occupied ports | Port number, PID, process name |

---

## Usage Examples

**Debugging a port conflict**
```
You: "Why isn't my server starting on port 3000?"

Claude: Port 3000 is occupied by node.exe (PID 14821).
        RAM is at 89% — Chrome is the heaviest process (1.2GB).
        Want me to help free up resources?
```

**Before a heavy build**
```
You: "Can I run a full Docker build right now?"

Claude: CPU 9% — RAM 61% — Disk 84%.
        Good to go, but consider cleaning disk first.
```

**System overload**
```
You: "My machine feels sluggish."

Claude: Chrome is using 74% RAM. 3 Node processes running in background.
        Want me to help identify what to close?
```

**Unexpected CPU spike**
```
You: "Something is hammering my CPU."

Claude: rust-analyzer at 94% CPU — likely indexing a large project.
        Should settle in a few minutes.
```

---

## Roadmap

| Version | Status | Highlights |
| --- | --- | --- |
| `v0.1.0` | ✅ Released | `system_stats`, `list_processes`, `list_ports` |
| `v0.2.0` | 🔧 In progress | Filters, `kill_process`, automated tests |
| `v0.3.0` | 📅 Planned | Public release, Scoop package, Linux support |

---

## License

MIT — see [LICENSE](LICENSE) for details.