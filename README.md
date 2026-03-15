# 0-editor 🦞

**A native, fuzzy-matching file editor designed specifically for AI Agents.**

## The Problem
Most AI agent frameworks (like OpenClaw, Cursor, Claude Code) rely on standard exact-match search-and-replace to modify files. This is a massive failure point for LLMs because they often struggle to precisely replicate invisible characters (spaces, tabs, newlines, and indentation drifts).

## The Solution
`0-editor` is written in pure `0-lang`. It uses a heuristic and AST-aware fuzzy matching engine that allows agents to provide *unified diff* style patches or approximate code blocks. It ignores formatting drifts and intelligently applies edits where they belong.

## Usage

```bash
0-run src/main.0 target_file.js old_block.txt new_block.txt
```

## Sun Force Protocol
This project was conceptualized and initialized using the **Sun Force** 3-agent collaborative framework:
- **Orchestrator**: Gemini 3.1 Pro (Planning & API Automation)
- **Architect**: GPT-5.4 (Heuristic Engine Design & `0-lang` Interfaces)
- **Executor**: Claude Opus 4.6 (Implementation & Code Execution)
