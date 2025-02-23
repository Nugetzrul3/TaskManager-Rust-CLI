use std::io;
use task_manager::{utils, TaskManager, TaskUpdate, Task};

fn add_task(task_manager: &mut TaskManager) {
    println!("Enter title: ");
    let mut title_input = String::new();
    io::stdin()
        .read_line(&mut title_input)
        .expect("Please enter a title");

    let title = title_input.trim();

    println!("Enter description: ");
    let mut description_input = String::new();
    io::stdin()
        .read_line(&mut description_input)
        .expect("Please enter a description");

    let description = description_input.trim();

    println!("Enter estimated duration: ");
    let mut estimated_duration_input = String::new();
    io::stdin()
        .read_line(&mut estimated_duration_input)
        .expect("Please enter an estimated duration");

    let estimated_duration: u32 = estimated_duration_input.trim().parse().unwrap_or(0);

    println!("Enter due date: ");
    let mut due_date_input = String::new();
    io::stdin()
        .read_line(&mut due_date_input)
        .expect("Please enter a title");

    let due_date = due_date_input.trim();

    println!("Enter priority: ");
    let mut priority_input = String::new();
    io::stdin()
        .read_line(&mut priority_input)
        .expect("Please enter a title");

    let priority = utils::get_priority(priority_input.trim());

    task_manager.add_task(title, description, estimated_duration, due_date, priority);

}

fn print_filter_options() {
    println!("----FILTER OPTIONS----");
    println!(
        "
    1. By Status
    2. By Priority
    3. None
        "
    )
}

fn list_tasks(task_manager: &mut TaskManager) {
    print_filter_options();
    println!("Enter filter option");

    let mut filter_option_input = String::new();
    io::stdin()
        .read_line(&mut filter_option_input)
        .expect("Please choose an option");

    let default = -1;
    let filter_option = filter_option_input.trim().parse().unwrap_or(default);

    match filter_option {
        1 => {
            println!("Enter status type filter: ");
            let mut filter_type_input = String::new();
            io::stdin()
                .read_line(&mut filter_type_input)
                .expect("Please enter a status filter");

            let filter_type = filter_type_input.trim();
            let filter = utils::get_filter("status", filter_type);

            task_manager.list_tasks(Some(filter));
        }
        2 => {
            println!("Enter priority type filter: ");
            let mut filter_type_input = String::new();
            io::stdin()
                .read_line(&mut filter_type_input)
                .expect("Please enter a priority filter");

            let filter_type = filter_type_input.trim();
            let filter = utils::get_filter("priority", filter_type);

            task_manager.list_tasks(Some(filter));
        }
        3 => {
            task_manager.list_tasks(None);
        }
        _ => {
            task_manager.list_tasks(None);
        }
    }
}

fn pring_update_options() {
    println!("----UPDATE OPTIONS----");
    println!(
        "
    1. Update status
    2. Update priority
    3. Update due date
    4. Update description
    5. Update estimated duration
    6. Update title
        "
    );
}

fn update_task(task_manager: &mut TaskManager, id: i32) {
    pring_update_options();

    let mut update_option_input = String::new();
    io::stdin()
        .read_line(&mut update_option_input)
        .expect("Please choose an update option");

    let default: i32 = -1;
    let update_option: i32 = update_option_input.trim().parse().unwrap_or(default);

    match update_option {
        1 => {
            println!("New Status: ");
            let mut status_input: String = String::new();
            io::stdin()
                .read_line(&mut status_input)
                .expect("Enter a status update option");
            let status = utils::get_status(&status_input.trim());
            task_manager.update_task(id, TaskUpdate::ByStatus(status));
            
        }
        2 => {
            println!("New Priority: ");
            let mut priority_input: String = String::new();
            io::stdin()
                .read_line(&mut priority_input)
                .expect("Enter a status update option");
            let status = utils::get_priority(&priority_input.trim());
            task_manager.update_task(id, TaskUpdate::ByPriority(status));
            
        }
        3 => {
            println!("New Due Date: ");
            let mut due_date_input: String = String::new();
            io::stdin()
                .read_line(&mut due_date_input)
                .expect("Enter a status update option");
            task_manager.update_task(id, TaskUpdate::ByDueDate(due_date_input.trim().to_string()));

        }
        4 => {
            println!("New Description: ");
            let mut description_input: String = String::new();
            io::stdin()
                .read_line(&mut description_input)
                .expect("Enter a status update option");
            task_manager.update_task(id, TaskUpdate::ByDescription(description_input.trim().to_string()));
        
        }
        5 => {
            println!("New Estimated Duration: ");
            let mut est_dur_input: String = String::new();
            io::stdin()
                .read_line(&mut est_dur_input)
                .expect("Enter a status update option");
            let est_duration: u32 = est_dur_input.trim().parse().unwrap_or_else(|_| {
                let task: &Task = task_manager.get_task(id);
                task.get_est_duration()
            });
            task_manager.update_task(id, TaskUpdate::ByEstDuration(est_duration));
            
        }
        6 => {
            println!("New Title: ");
            let mut title_input: String = String::new();
            io::stdin()
                .read_line(&mut title_input)
                .expect("Enter a title");
            let title = title_input.trim();
            task_manager.update_task(id, TaskUpdate::ByTitle(title.to_string()));
            

        }
        -1 => {
            println!("Invalid input");
            return;
        }
        _ => {
            println!("Unkown option");
            return;
        }
    }
}

fn print_options() {
    println!(
        "
    1. Add task
    2. View task by task ID
    3. Update task
    4. List tasks
    5. Delete task by ID
    6. Save tasks to JSON
    7. Quit"
    )
}

fn main() {
    let mut task_manager = TaskManager::new();

    // Load tasks
    match task_manager.load_tasks() {
        Ok(()) => {
            println!("Tasks loaded successfully")
        },
        Err(_) => {
            
        }
    }

    print_options();

    loop {
        let mut option: String = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Please enter an option");

        option = option.trim().to_string();

        match option.as_str() {
            "1" => {
                add_task(&mut task_manager);
                print_options();
            }
            "2" => {
                println!("Enter ID: ");
                let mut id_input: String = String::new();
                io::stdin()
                    .read_line(&mut id_input)
                    .expect("Please enter an ID");

                let default = -1;
                let id: i32 = id_input.trim().parse().unwrap_or(default);

                if id.eq(&default) {
                    println!("Invalid ID");
                } else {
                    task_manager.view_task(id);
                    print_options();
                }
            }
            "3" => {
                println!("Enter ID: ");
                let mut id_input: String = String::new();
                io::stdin()
                    .read_line(&mut id_input)
                    .expect("Please enter an ID");

                let default = -1;
                let id: i32 = id_input.trim().parse().unwrap_or(default);

                if id.eq(&default) {
                    println!("Invalid ID");
                } else {
                    update_task(&mut task_manager, id);
                }

                print_options();
            }
            "4" => {
                list_tasks(&mut task_manager);
                print_options();
            }
            "5" => {
                println!("Enter ID: ");
                let mut id_input: String = String::new();
                io::stdin()
                    .read_line(&mut id_input)
                    .expect("Please enter an ID");

                let default = -1;
                let id: i32 = id_input.trim().parse().unwrap_or(default);

                if id.eq(&default) {
                    println!("Invalid ID");
                } else {
                    task_manager.delete_task(id);
                    print_options();
                }
            }
            "6" => {
                match task_manager.save_tasks() {
                    Ok(()) => {
                        println!("Tasks saved successfully")
                    }
                    Err(e) => {
                        println!("{}", e)
                    }
                }

                print_options();
            }
            "7" => {
                break;
            }
            _ => {
                println!("Unknown option");
                print_options();
            }
        }
    }
}
