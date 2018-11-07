#[macro_use]
extern crate text_io;
use std::time::{SystemTime,Duration};
use std::thread::sleep;
use std::io::{self,Read};
use std::io::prelude::*;
use std::io::stdout;
//nix mkfifo...


struct Item {
    name  : String,
    state : i16,
    description: String,
    start_time : SystemTime,
    end_time : SystemTime,
    create_time : SystemTime,
}

fn print_list(list:Vec<Item>){
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
fn run_ui(){
  let mut input:String;
  let mut num:u32;
  //WHILE READ LINE REPL
  while(true){
    print!("> ");
    stdout().flush();
    input = read!("{}");
    if(input == "print") {
      print!("Printing\n");
    
      // print_list(list);
    }else if(input == "add"){
      print!("Name: ");
      stdout().flush();
      let mut name:String = read!("{}\n");
      
      print!("Description: ");
      stdout().flush();
      let mut description:String = read!("{}\n");
      
      print!("{}\n{}\n",name,description);
    
      // add to list
    }else if(input == "show"){
      num = read!();
      print!("Showing {}\n",num);

      // print_item(list[num]);
    }else if(input == "edit"){
      num = read!();
      print!("Editing {}\n",num);

      // additional options here?
    }else if(input == "help"){
      print!("Possible Commands are:\n");
      print!("print -> Print all current items in the To Do list.\n");
      print!("add -> Add a new item to the To Do List.\n");
      print!("show <#>-> Show the details of the specific item.\n");
      print!("edit <#>-> Edit the the listed item.\n");
      print!("help -> Print this help menu.\n");

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
    run_ui();
    //If filename exists, open it for reading and writing, else error
    //Else create file... notify
    //MKFifo for spawning new processes...
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
    
    //std::thread::sleep(std::time::Duration::from_millis(10000));
}

