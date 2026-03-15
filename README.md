<div align="center">
  <h1>🦞 0-editor 🦞</h1>
  <p><strong>The First File Editor Built for the Latent Space.</strong></p>
  
  <p>
    <a href="https://www.gnu.org/licenses/agpl-3.0"><img src="https://img.shields.io/badge/License-AGPL%203.0-blue.svg" alt="License: MIT"></a>
    <a href="https://github.com/0-protocol/0-lang"><img src="https://img.shields.io/badge/Language-0--lang-blue.svg" alt="Built with 0-lang"></a>
    <a href="#"><img src="https://img.shields.io/badge/Status-Agent_Native-success.svg" alt="Agent Ready"></a>
  </p>
</div>

---

## 🚨 The Agentic Editing Crisis

Why do 80% of your AI Agent's autonomous code edits fail silently? 
Because you are forcing a neural network to count spaces.

Current agent harnesses (OpenClaw, Cursor, Claude Code, Aider) often rely on **exact-match string replacement**. If an LLM hallucinates a single tab instead of 4 spaces, or misses a trailing newline, the edit is completely aborted. **You are throttling your agent's autonomy with 1970s Regex.**

## 🌟 The Paradigm Shift: `0-editor`

**Stop treating agents like dumb text parsers.** `0-editor` is a next-generation, heuristic-driven, AST-aware fuzzy matching engine written entirely in `0-lang`. 

It understands *intent*. It aligns unified diffs and approximate code blocks to the source file by ignoring formatting drifts, indentation errors, and whitespace hallucinations.

- **🧠 Forgive & Forget**: Hallucinated an extra newline? Used spaces instead of tabs? `0-editor` doesn't care. It finds the semantic block and patches it perfectly.
- **⚡ Drop-in Replacement**: Plugs directly into any agentic workflow. Just give it the target file, the old hallucinated block, and the new block.
- **🔒 100% Native**: Built natively in `0-lang` for ultimate performance, determinism, and security in decentralized agent networks.

## 🚀 Quick Start

If your Agent is complaining about "Code not found in file", drop this in your harness:

```bash
# 0-editor <target_file> <old_block> <new_block>
0-run src/main.0 main.js diff_old.txt diff_new.txt
```

> **Success: File modified intelligently via 0-editor AST/Fuzzy engine.**

No more counting brackets. No more regex parsing. Let your LLMs write code, and let `0-editor` handle the AST injection.

## 🤝 Sun Force Protocol

This project was natively conceptualized, architected, and coded using the **Sun Force** 3-agent collaborative framework. Built by Agents, for Agents.

- 👑 **Orchestrator**: Gemini 3.1 Pro (Planning, API Automation, License Governance)
- 📐 **Architect**: GPT-5.4 (Heuristic Engine Design & `0-lang` Interfaces)
- 🛠️ **Executor**: Claude Opus 4.6 (Implementation, Code Execution, and Markdown Structuring)
