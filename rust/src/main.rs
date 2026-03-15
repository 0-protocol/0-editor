use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: 0-editor-rs <target_file> <old_block_file> <new_block_file>");
        process::exit(1);
    }

    let target_path = &args[1];
    let old_path = &args[2];
    let new_path = &args[3];

    let content = match fs::read_to_string(target_path) {
        Ok(c) => c,
        Err(e) => { eprintln!("Error reading target file {}: {}", target_path, e); process::exit(1); }
    };
    let old_text = match fs::read_to_string(old_path) {
        Ok(c) => c,
        Err(e) => { eprintln!("Error reading old block file {}: {}", old_path, e); process::exit(1); }
    };
    let new_text = match fs::read_to_string(new_path) {
        Ok(c) => c,
        Err(e) => { eprintln!("Error reading new block file {}: {}", new_path, e); process::exit(1); }
    };

    if content.contains(&old_text) {
        let new_content = content.replacen(&old_text, &new_text, 1);
        if let Err(e) = fs::write(target_path, new_content) {
            eprintln!("Failed to write target file: {}", e);
            process::exit(1);
        }
        println!("Success: Exact match replaced in {}", target_path);
        process::exit(0);
    }

    let content_lines: Vec<&str> = content.lines().collect();
    let old_lines: Vec<&str> = old_text.lines().map(|l| l.trim()).filter(|l| !l.is_empty()).collect();
    let new_lines: Vec<&str> = new_text.lines().collect();

    if old_lines.is_empty() {
        eprintln!("Error: Old block is empty or only whitespace.");
        process::exit(1);
    }

    let mut match_start = None;
    let mut match_end = None;

    for i in 0..content_lines.len() {
        if content_lines[i].trim() == old_lines[0] {
            let mut curr_old = 1;
            let mut curr_content = i + 1;

            while curr_old < old_lines.len() && curr_content < content_lines.len() {
                if content_lines[curr_content].trim() == old_lines[curr_old] {
                    curr_old += 1;
                }
                curr_content += 1;
            }

            if curr_old == old_lines.len() {
                match_start = Some(i);
                match_end = Some(curr_content);
                break;
            }
        }
    }

    if let (Some(start), Some(end)) = (match_start, match_end) {
        let mut final_lines = Vec::new();
        final_lines.extend_from_slice(&content_lines[..start]);
        final_lines.extend_from_slice(&new_lines);
        final_lines.extend_from_slice(&content_lines[end..]);

        let new_content = final_lines.join("\n") + "\n";
        if let Err(e) = fs::write(target_path, new_content) {
            eprintln!("Failed to write target file: {}", e);
            process::exit(1);
        }
        println!("Success: Fuzzy match replaced lines {}-{} in {}", start + 1, end, target_path);
        process::exit(0);
    } else {
        eprintln!("Error: Could not find matching text in {} even with fuzzy whitespace matching.", target_path);
        process::exit(1);
    }
}
