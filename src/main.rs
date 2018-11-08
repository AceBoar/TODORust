#[macro_use]
extern crate text_io;

extern crate nix;

use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self,Read};
use std::io::prelude::*;
use std::io::stdout;
use std::sync::mpsc;
use nix::sys::socket;
use std::time::{SystemTime,Duration};
use std::thread;
use std::thread::sleep;

//An Item in the To Do List
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
      print!("Task: {}\t State: [{}]\n\t{}\n",self.name,if self.state == 0 {"Incomplete"}
         else if self.state == 1 {"Started"} else
         {"Complete"},self.description);
    
    }
}

fn get_list(list:&Vec<Item>) -> String{
  let mut resp=String::from("");
  for x in 0..list.len() {
      resp.push_str(&format!("{}\t{}\t[{}]\n",x,list[x].name,if list[x].state == 0 {"Incomplete"} else if list[x].state == 1 {"Started"} else {"Complete"}));
  }
  //DEBUGGING
  print!("{}",resp);
  //END DEBUGGING
  return resp;
}
fn print_list(list:&Vec<Item>){
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

//UI
fn run_ui(mut list:Vec<Item>){
  let mut input:String;
  //WHILE READ LINE REPL
  while(true){
    print!("> ");
    stdout().flush();
    input = read!("{}");
    if(input == "print") {
      print_list(&list);
    }else if(input == "add"){
      print!("Name: ");
      stdout().flush();
      let mut name:String = read!("{}\n");
      
      print!("Description: ");
      stdout().flush();
      let mut description:String = read!("{}\n");
      
      let mut item:Item = Item::new(name,description,SystemTime::now());
      list.push(item);
      // add to list
    }else if(input == "show"){
      let mut num:usize = read!();
      print!("Showing {}\n",num);

      if(num>=list.len()){
        println!("ERROR: Invalid item");
      }else{
        list[num].print();
      }
    }else if(input == "edit"){
      let mut num:usize = read!();
      
      let mut var:String; 
      let mut done:bool = false;
      println!("Editing {}",num);

      //LOOK HERE
      while(!done){
        print!("What value would you like to edit? [(n)ame,(d)escription,(s)tatus,done]: ");
        stdout().flush();
        var = read!("{}\n");
        if( var == "name"|| var == "n"){
          print!("New name: ");
          stdout().flush();
          let mut name:String = read!("{}\n");    

          list[num].name=name;
          // re-bind name 
        }else if(var == "description"||var=="d"){
          print!("New description: ");
          stdout().flush();
          let mut desc:String = read!("{}\n");    

          list[num].description=desc;
          // re-bind desc 
        }else if(var == "status"||var=="s"){
          print!("Change status [(i)ncomplete,(s)tarted,(c)omplete]: ");
          stdout().flush();
          let mut status:String = read!("{}\n"); 

          if(status=="incomplete"||status=="i"){
            list[num].state = 0;        
          }else if(status=="started"||status=="s"){
            list[num].state = 1;
            list[num].start_time = SystemTime::now();
          }else if(status=="complete"||status=="c"){
            list[num].state = 2;
            list[num].end_time = SystemTime::now();
          }else{
            println!("Invalid status: \"{}\"",status);
          }

          // re-bind status
        }else if(var=="done"){
          break;
        }else{
          println!("Invalid command: \"{}\"",var);
        }
      }
        
      // additional options here?
    }else if(input == "help"){
      print!("Possible Commands are:\n");
      print!("help -> Print this help menu.\n");
      print!("exit -> Exit the program.\n");
      print!("print -> Print all current items in the To Do list.\n");
    }
  }
}

fn run_cmd(list:&mut Vec<Item>, cmdStr: String) -> String {
    let inputArgs:Vec<String> = cmdStr.split(" ").map(|x| x.to_string()).collect();
    //DEBUGGING
    print!("Run CMD: '{}'\n",cmdStr);
    print!("inputArgs: '{:?}'\n",inputArgs);
    //END DEBUGGING
    
    if(inputArgs[0] == "print") {
        return get_list(&list);
    }else if(inputArgs[0] == "add"){
        print!("Name: ");
        stdout().flush();
        let name:String =inputArgs[1].clone();
        let description:String =inputArgs[2].clone();
        print!("{}\n{}\n",name,description);
        let mut item:Item = Item::new(name,description,SystemTime::now());
        list.push(item);
        // add to list
    }else if(inputArgs[0] == "show"){
        let num:u32=inputArgs[1].parse::<u32>().unwrap();
        print!("Showing {}\n",num);
        //list[num].print();
    }else if(inputArgs[0] == "edit"){
        let num:u32= inputArgs[1].parse::<u32>().unwrap();
        print!("Editing {}\nWhat value would you like to edit? [name,description,status]",num);
        stdout().flush();
        let mut var:String; 
        let mut done:bool = false;
        //LOOK HERE
        while(!done){
            var = read!("{}\n");
            if( var == "name"){

                done=true;
            }
        }

        // additional options here?
    }else if(inputArgs[0] == "help"){
        print!("Possible Commands are:\n");
        print!("help -> Print this help menu.\n");
        print!("exit -> Exit the program.\n");
        print!("print -> Print all current items in the To Do list.\n");
    }
    return "".to_string();

}


//The Main Function...
fn main() {
    
    //Initialize ToDo List
    let filename = "./todo.list";
    let mut list:Vec<Item> = Vec::new();
    let mut item:Item = Item::new("Eat".to_string(),"eat a sandwich".to_string(),SystemTime::now());
    list.push(item);
    item = Item::new("Sleep".to_string(),"Take a nap.".to_string(),SystemTime::now());
    list.push(item);
    //A one way pipe for sending data between threads...
    let (sendToServer,recvFromServer) = mpsc::channel();
    let mut send_to_thread= Vec::new();
    let mut receivers= Vec::new();
    
    
    //For each thread
    let (sender,receiver) = mpsc::channel();
    send_to_thread.push(sender);
    receivers.push(receiver);
    let (sender,receiver) = mpsc::channel();
    send_to_thread.push(sender);
    receivers.push(receiver);

    //Client Thread
    thread::spawn(move || {
        let val = String::from("0 print");
        sendToServer.send(val).unwrap();
        let mut resp = receivers[0].recv().unwrap();
        print!("Client 0: \n{}\n\n\n",resp);

    });

    while(true){
    //Command Processing
    let mut cmd = recvFromServer.recv().unwrap();
    let mut requester:u32=cmd.remove(0).to_digit(10).unwrap();
    cmd.remove(0);

    let mut file = File::create(filename).unwrap();
    let mut resp = run_cmd(&mut list,cmd);
    print!("Server about to send to: {}\n{}\n\n\n",requester,resp);
    send_to_thread[(requester as usize)].send(resp).unwrap();
    }
}

