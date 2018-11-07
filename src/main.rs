//Many of these can likely be deleted

extern crate nix;

use std::fmt;
use std::io::prelude::*;
use std::fs::File;
use std::time::{SystemTime,Duration};
use std::thread;
use nix::sys::socket;
use std::env;
use std::sync::mpsc;
//nix mkfifo...

//An Item in the To Do List
struct Item {
    name  : String,
    state : i16,
    description: String,
    start_time : SystemTime,
    end_time : SystemTime,
    create_time : SystemTime,
}

//Print the given ToDo List
fn print_list(list:Vec<Item>){
  for x in 0..list.len() {
    print!("{}\t{}\t[{}]\n",x,list[x].name,if list[x].state == 0 {"Incomplete"} else if list[x].state == 1 {"Started"} else {"Complete"});
  }
}

//Print an Individual Item
fn print_item(todo:Item){
  print!("{}\t[{}]\n{}i\n",todo.name,if todo.state == 0 {"Incomplete"}
         else if todo.state == 1 {"Started"} else
         {"Complete"},todo.description);
}

//SpawnNewThread
/*
//UI
fn run_ui(){
       let line:String = std::fs::read("/dev/stdin")?;
       while(true){
        if(line == "print") {
            print_list(toDoList);
        }else if(line == "add"){
        
        }else if(line == "show"){
           let num:u32 = std::fs::read("/dev/stdin");
            print_item(toDoList[num]); 
        }else if(line == "edit"){
                  
        }else if(line == "help"){
          print!("Possible Commands are:\n");
          print!("print -> Print all current items in the To Do list.\n");
          print!("add -> Add a new item to the To Do List.\n");
          print!("show <#>-> Show the details of the specific item.\n");
          print!("edit <#>-> Edit the the listed item.\n");
          print!("help -> Print this help menu.\n");
        
        }else{
          print!("{}: ERROR: Command not found. Try 'help'",args[0])
        }
       }
}
*/


//The Main Function...
fn main() {
    //A one way pipe for sending data between threads...
    let (tx,rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("1 print");
        tx.send(val).unwrap();
    });
    let mut recv = rx.recv().unwrap();
    //let filename:String = format!("/dev/pts/{fd}",fd=recv.remove(0));
    let filename:String = String::from("/root/TODORust/tmp");
    print!("{}",filename);
    let mut file = File::create(filename).unwrap();
    file.write_all(b"print").unwrap();
}

