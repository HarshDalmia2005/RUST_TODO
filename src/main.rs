use serde::{Serialize,Deserialize};
#[derive(Debug,Serialize,Deserialize)]



struct Task{
    name: String,
    done: bool
}

fn load_tasks() -> Vec<Task> {
    let home = std::env::var("HOME").unwrap();
    let path=format!("{}/todo.json",home);
    let contents=std::fs::read_to_string(path);
    match contents {
        Ok(c)=>{
            return serde_json::from_str(&c).unwrap();
        }
        Err(_)=>{
            return Vec::new();
        }
    }
}

fn save_tasks(tasks: &Vec<Task>){
    let home = std::env::var("HOME").unwrap();
    let path=format!("{}/todo.json",home);
    let json_string=serde_json::to_string_pretty(tasks).unwrap();
    std::fs::write(path,json_string).unwrap();
}

fn print_list(tasks: &Vec<Task>){
    println!("      TODO LIST      \n");
    for (i,task) in tasks.iter().enumerate(){
        if task.done{
            println!("{}.  ✔    {}",i,task.name);
        }
        else{
            println!("{}. [ ]    {}",i,task.name);
        }
    }
}

fn main() {
    let args:Vec<String>=std::env::args().collect();
    println!("{:?}",args);
    let mut tsk:Vec<Task>=load_tasks();

    if args.len()<2{
        print_list(&tsk);
        return;
    }

    let command=&args[1];
    if command=="add"{
        if args.len()<3{
            println!("Usage: todo add <task name>");
            return;
        }

        for i in 2..args.len(){
            let task_name=&args[i];
            tsk.push(Task{
                name:task_name.to_string(),
                done:false
            });
        }
        save_tasks(&tsk);
        print_list(&tsk);
    }else if command=="delete"{
        if args.len()<3{
            println!("Usage: todo add <task name>");
            return;
        }

        let mut indices:Vec<usize>=Vec::new();
        for i in 2..args.len(){
            let task_id=&args[i];
            indices.push(task_id.parse::<usize>().unwrap());
        }
        indices.sort();
        indices.reverse();
        for i in indices{
            tsk.remove(i);
        }
        save_tasks(&tsk);
        print_list(&tsk);
    }else if command=="done"{
        if args.len()<3{
            println!("Usage: todo add <task name>");
            return;
        }

        for i in 2..args.len(){
            let task_id=&args[i];
            tsk[task_id.parse::<usize>().unwrap()].done=true;
        }
        save_tasks(&tsk);
        print_list(&tsk);
    }
    
}
