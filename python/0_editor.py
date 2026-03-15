#!/usr/bin/env python3
import sys
import os
import ast

def fuzzy_replace(file_path, old_path, new_path):
    try:
        with open(file_path, 'r') as f: content = f.read()
        with open(old_path, 'r') as f: old_text = f.read()
        with open(new_path, 'r') as f: new_text = f.read()
    except FileNotFoundError as e:
        print(f"Error: {e}", file=sys.stderr)
        return False

    if old_text in content:
        new_content = content.replace(old_text, new_text, 1)
        if file_path.endswith(".py"):
            try:
                ast.parse(new_content)
            except SyntaxError as e:
                print(f"🛑 AST Structural Guard Triggered: The proposed patch breaks Python syntax! Reverting.\n{e}", file=sys.stderr)
                return False
        with open(file_path, 'w') as f: f.write(new_content)
        print(f"Success: Exact match replaced in {file_path}")
        return True

    old_lines = [l.strip() for l in old_text.strip().splitlines() if l.strip()]
    new_lines = new_text.splitlines()
    content_lines = content.splitlines()

    if not old_lines:
        print("Error: Old block is empty or only whitespace.", file=sys.stderr)
        return False

    match_start = -1
    match_end = -1

    for i in range(len(content_lines)):
        if content_lines[i].strip() == old_lines[0]:
            curr_old = 1
            curr_content = i + 1
            while curr_old < len(old_lines) and curr_content < len(content_lines):
                if content_lines[curr_content].strip() == old_lines[curr_old]:
                    curr_old += 1
                curr_content += 1
            
            if curr_old == len(old_lines):
                match_start = i
                match_end = curr_content
                break

    if match_start != -1:
        new_content_lines = content_lines[:match_start] + new_lines + content_lines[match_end:]
        new_content_full = "\n".join(new_content_lines) + "\n"
        if file_path.endswith(".py"):
            try:
                ast.parse(new_content_full)
            except SyntaxError as e:
                print(f"🛑 AST Structural Guard Triggered: The proposed fuzzy patch breaks Python syntax! Reverting.\n{e}", file=sys.stderr)
                return False
        with open(file_path, 'w') as f:
            f.write(new_content_full)
        print(f"Success: Fuzzy match replaced lines {match_start+1}-{match_end} in {file_path}")
        return True

    print(f"Error: Could not find matching text in {file_path} even with fuzzy whitespace matching.", file=sys.stderr)
    return False

if __name__ == "__main__":
    if len(sys.argv) < 4:
        print("Usage: 0-editor <target_file> <old_block_file> <new_block_file>")
        sys.exit(1)
    
    success = fuzzy_replace(sys.argv[1], sys.argv[2], sys.argv[3])
    sys.exit(0 if success else 1)
