use serde::{Serialize,Deserialize};
use chrono::NaiveDate;
use chrono::Local;

#[derive(Debug,Serialize,Deserialize)]
struct Task{
    name: String,
    done: bool,
    #[serde(default)]
    due: Option<String>,
    #[serde(default)]
    repeat: Option<u32>
}

fn load_tasks() -> Vec<Task> {
    let home = std::env::var("HOME").unwrap();
    let path = format!("{}/.todo.json", home);
    let contents = std::fs::read_to_string(path);
    match contents {
        Ok(c) => serde_json::from_str(&c).unwrap_or_else(|_| Vec::new()),
        Err(_) => Vec::new(),
    }
}

fn save_tasks(tasks: &Vec<Task>){
    let home = std::env::var("HOME").unwrap();
    let path = format!("{}/.todo.json", home);
    let json_string = serde_json::to_string_pretty(tasks).unwrap();
    std::fs::write(path, json_string).unwrap();
}

fn print_due(due: &Option<String>, today: chrono::NaiveDate) {
    if let Some(d) = due.as_ref() {
        let due_date = NaiveDate::parse_from_str(d, "%Y-%m-%d").unwrap();
        let days_left = (due_date - today).num_days();
        if days_left < 0 {
            print!(" \x1b[31m[OVERDUE by {} days]\x1b[0m", -days_left);
        } else if days_left == 0 {
            print!(" \x1b[33m[due TODAY]\x1b[0m");
        } else {
            print!(" \x1b[36m[due in {} days]\x1b[0m", days_left);
        }
    }
}

fn print_list(tasks: &Vec<Task>){
    let today = Local::now().date_naive();
    let done_count = tasks.iter().filter(|t| t.done).count();
    let pending = tasks.len() - done_count;

    println!("\n  ── TODO LIST ── {} tasks ({} done, {} pending)\n",
        tasks.len(), done_count, pending);

    if tasks.is_empty() {
        println!("  No tasks yet. Add one with: todo add <task>");
        return;
    }

    for (i, task) in tasks.iter().enumerate() {
        if task.done {
            print!("  {}. \x1b[32m✔\x1b[0m  {}", i, task.name);
        } else {
            print!("  {}. [ ] {}", i, task.name);
            print_due(&task.due, today);
        }
        if let Some(r) = task.repeat {
            print!(" \x1b[35m[repeats every {} days]\x1b[0m", r);
        }
        println!();
    }
    println!();
}

fn print_help() {
    println!("\n  todo — CLI Task Manager\n");
    println!("  USAGE:");
    println!("    todo                               list all tasks");
    println!("    todo add <task> [task2 ...]        add one or more tasks");
    println!("    todo add --due YYYY-MM-DD <task>   add task with deadline");
    println!("    todo add --repeat N <task>         add repeating task (every N days)");
    println!("    todo done <id> [id2 ...]           mark task(s) as done");
    println!("    todo delete <id> [id2 ...]         delete task(s)");
    println!("\n  EXAMPLES:");
    println!("    todo add \"buy milk\"");
    println!("    todo add --due 2026-04-20 \"dentist\"");
    println!("    todo add --repeat 7 \"weekly review\"");
    println!("    todo done 0 2");
    println!("    todo delete 1\n");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut tsk: Vec<Task> = load_tasks();

    if args.len() < 2 {
        print_list(&tsk);
        return;
    }

    let command = &args[1];

    if command == "add" {
        if args.len() < 3 {
            println!("Usage: todo add [--due DATE] [--repeat N] <task> ...");
            return;
        }

        let mut i = 2;
        let mut due: Option<String> = None;
        let mut repeat: Option<u32> = None;
        let mut added: Vec<String> = Vec::new();

        while i < args.len() {
            let arg = &args[i];
            if arg == "--due" && i + 1 < args.len() {
                due = Some(args[i+1].to_string());
                i += 2;
            } else if arg == "--repeat" && i + 1 < args.len() {
                repeat = Some(args[i+1].parse::<u32>().unwrap());
                i += 2;
            } else {
                added.push(arg.to_string());
                tsk.push(Task {
                    name: arg.to_string(),
                    done: false,
                    due: due.clone(),
                    repeat,
                });
                i += 1;
            }
        }
        save_tasks(&tsk);
        for name in &added {
            println!("✅ Added: {}", name);
        }
        print_list(&tsk);

    } else if command == "delete" {
        if args.len() < 3 {
            println!("Usage: todo delete <id> [id ...]");
            return;
        }

        let mut indices: Vec<usize> = Vec::new();
        for i in 2..args.len() {
            indices.push(args[i].parse::<usize>().unwrap());
        }
        indices.sort();
        indices.reverse();

        let mut deleted_names: Vec<String> = Vec::new();
        for idx in &indices {
            deleted_names.push(tsk[*idx].name.clone());
            tsk.remove(*idx);
        }
        save_tasks(&tsk);
        deleted_names.reverse();
        for name in &deleted_names {
            println!("🗑  Deleted: {}", name);
        }
        print_list(&tsk);

    } else if command == "done" {
        if args.len() < 3 {
            println!("Usage: todo done <id> [id ...]");
            return;
        }

        let today = Local::now().date_naive();

        for i in 2..args.len() {
            let idx = args[i].parse::<usize>().unwrap();
            let name = tsk[idx].name.clone();

            if let Some(repeat_days) = tsk[idx].repeat {
                let base = if let Some(ref d) = tsk[idx].due {
                    NaiveDate::parse_from_str(d, "%Y-%m-%d").unwrap_or(today)
                } else {
                    today
                };
                let next = base + chrono::Duration::days(repeat_days as i64);
                let next_str = next.format("%Y-%m-%d").to_string();
                println!("✅ Done! \"{}\" → rescheduled, next due: {}", name, next_str);
                tsk[idx].due = Some(next_str);
                tsk[idx].done = false;
            } else {
                tsk[idx].done = true;
                println!("✅ Done: {}", name);
            }
        }
        save_tasks(&tsk);
        print_list(&tsk);

    } else {
        print_help();
    }
}
