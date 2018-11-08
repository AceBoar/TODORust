#[macro_use]
extern crate text_io;
use std::time::{SystemTime,Duration};
use std::thread::sleep;
use std::io::{self,Read};
use std::io::prelude::*;
use std::io::stdout;
use std::fs::File;
//nix mkfifo...

struct Item {
    name  : String,
    state : i16,
    description: String,
    start_time : SystemTime,
    end_time : SystemTime,
    create_time : SystemTime,
}

impl Item {
    fn new(nameArg: String, descriptionArg: String, createTime: SystemTime) -> Item {
        Item{
            name: nameArg,
            state : 0,
            description: descriptionArg,
            start_time : SystemTime::now(),
            end_time : SystemTime::now(),
            create_time : createTime,
        }
    }
    fn print(&self) {
      print!("Task: {}\t State: [{}]\n\t{}i\n",self.name,if self.state == 0 {"Incomplete"}
         else if self.state == 1 {"Started"} else
         {"Complete"},self.description);
    
    }
}

fn print_list(list:&Vec<Item>){
  for x in 0..list.len() {
    print!("{}\t{}\t[{}]\n",x,list[x].name,if list[x].state == 0 {"Incomplete"} else if list[x].state == 1 {"Started"} else {"Complete"});
  }
}

fn print_item(todo:Item){
  print!("{}\t[{}]\n{}i\n",todo.name,if todo.state == 0 {"Incomplete"}
         else if todo.state == 1 {"Started"} else
         {"Complete"},todo.description);
}

//SpawnNewThread

//UI
fn run_ui(mut list:Vec<Item>){
  let mut input:String;
  //WHILE READ LINE REPL
  while(true){
    print!("> ");
    stdout().flush();
    input = read!("{}");
    if(input == "print") {
      print!("Printing\n");
      print_list(&list);
    }else if(input == "add"){
      print!("Name: ");
      stdout().flush();
      let mut name:String = read!("{}\n");
      
      print!("Description: ");
      stdout().flush();
      let mut description:String = read!("{}\n");
      
      print!("{}\n{}\n",name,description);
      let mut item:Item = Item::new(name,description,SystemTime::now());
      list.push(item);
      // add to list
    }else if(input == "show"){
      num = read!();
      print!("Showing {}\n",num);
      let mut num:u32;
      list[num].print();
    }else if(input == "edit"){
      num = read!();
      print!("Editing {}\nWhat value would you like to edit? [name,description,status]",num);
      stdout().flush();
      let mut var:String; 
      let done:bool = false;
      //LOOK HERE
      while(!done){
          var = read!("{}\n");
          if( var == "name"){
            
            done=true;
          }
      }
        
      // additional options here?
    }else if(input == "help"){
      print!("Possible Commands are:\n");
      print!("help -> Print this help menu.\n");
      print!("exit -> Exit the program.\n");
      print!("print -> Print all current items in the To Do list.\n");
      print!("add -> Add a new item to the To Do List.\n");
      print!("show <#> -> Show the details of the specific item.\n");
      print!("edit <#> -> Edit the the listed item.\n");

    }else if(input == "exit"){
      print!("Exiting\n");
      break;
    }else{
      print!("{}: ERROR: Command not found. Try 'help'\n",input)
    }
  }
}

fn main(){
    //Prep Server
    let filename = "./todo.list";
    let mut list:Vec<Item> = Vec::new();
    let mut item:Item = Item::new("Eat".to_string(),"eat a sandwich".to_string(),SystemTime::now());
    item.print();
    run_ui(list);
    //print_item(item);
    //run_ui();
    //Spawn new thread for UI
    //Main (I/O)- waits for new connections or closes connections...
    //          - it also handles I/O
    //          - updateFile() -> Update the to do list, called by thread one each time
    //          - checkForUpdates() -> Look if another terminal/application updated the to do list file
    //          - spawnNewThread()
    //          - removeThread()
    //Worker Thread -> UI
    //
    // checkArgs() -> Looks if a filename was given, if not prompt for one...
    // mainMenu()  -> Display a basic prompt (Readline repl loop)
    // showHelp()  -> Display a list of commands
    // printList() -> Display the current to do list
    // toggleListDisplay() -> Permanently display the list above the prompt: toggleable^
}

