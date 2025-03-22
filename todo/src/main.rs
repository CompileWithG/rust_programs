
use serde::{Serialize,Deserialize};
use std::fs;
use std::io::{self,Write};



#[derive(Serialize,Deserialize,Debug)]
struct Todo{
    id:u32,
    task:String,
    completed:bool
}
struct TodoList{
    todos:Vec<Todo>,
}
impl TodoList{
    fn new()->Self{
        TodoList{ todos: Vec::new() }

    }
    fn add(&mut self,Task:String){
        let id: u32=self.todos.len() as u32 +1;
        self.todos.push(
            Todo{
                id:id,
                task:Task,
                completed:false,
            }
        )
    }
    fn view(&self){
        for todo in &self.todos{
            println!("No. {} ||  {} || completed: {}",
                todo.id,todo.task,todo.completed);
        }
    }
    fn delete(&mut self,id:u32){
        self.todos.retain(|todo| todo.id!=id );
    }
    fn save_to_file(&self,file_path:&str)->io::Result<()>{
        let data=serde_json::to_string(&self.todos)?;
        fs::write(file_path,data)?;
        Ok(())
    }
    fn view_from_file(file_path:&str)-> io::Result<Self>{
        let data=fs::read_to_string(file_path)?;
        let data1:Vec<Todo>=serde_json::from_str(&data)?;
        Ok(TodoList{todos:data1})
    }
}




fn main()-> io::Result<()> {
    let mut  todo_list=TodoList::new();
    let file_path="todos.json";
    if  let Ok(loaded_list)=TodoList::view_from_file(file_path){
        todo_list=loaded_list;
    }
    loop{
        println!("\n Todo App");
        println!("1.Add Task");
        println!("2.View Task");
        println!("3.Delete Task");
        println!("4.Exit");

        print!("choose an option:\n");
        io::stdout().flush()?;
        let mut choice =String::new();
        io::stdin().read_line(&mut choice)?;
        let choice :u32 =match choice.trim().parse(){
            Ok(num)=> num,
            Err(_)=>{
                println!("Enter valid input");
                continue;
            }
        };
        match choice {
            1=>{
                print!("Enter Task:\n ");
                io::stdout().flush()?;
                let mut Task=String::new();
                io::stdin().read_line(&mut Task)?;
               todo_list.add(Task.trim().to_string());
               println!("Task added");
            }
            2=>{
                println!("\nTasks:\n");
                todo_list.view();
            }
            3=>{
                let mut id_to_delete=String::new();
                println!("Enter id of task you want to delete:");
                io::stdin().read_line(&mut id_to_delete)?;
                let id_to_delete1:u32 = match id_to_delete.trim().parse(){
                    Ok(num)=> num,
                    Err(_)=>{
                        println!("Invalid id");
                        continue;
                    }
                };
                todo_list.delete(id_to_delete1);
                println!("Task {} deleted",id_to_delete1);
            }
            4=>{
                todo_list.save_to_file(file_path)?;
                println!("Todo list saved ..");
                break;
            }
            _=>{
                println!("invalid option,please try again");
                continue;
            }

        }
            
    
    }
    Ok(())
    
}
