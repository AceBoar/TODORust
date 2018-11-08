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
    fn new(name_arg: String, description_arg: String, create_time: SystemTime) -> Item {
        Item{
            name: name_arg,
            state : 0,
            description: description_arg,
            start_time : SystemTime::now(),
            end_time : SystemTime::now(),
            create_time : create_time,
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
  //print!("{}",resp);
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
  loop{
    print!("> ");
    stdout().flush();
    input = read!("{}");
    if(input == "print") {
      print_list(&list);
    }else if input == "add" {
      print!("Name: ");
      stdout().flush();
      let name:String = read!("{}\n");
      
      print!("Description: ");
      stdout().flush();
      let description:String = read!("{}\n");
      
      let item:Item = Item::new(name,description,SystemTime::now());
      list.push(item);
      // add to list
    }else if input == "show" {
      let num:usize = read!();
      print!("Showing {}\n",num);

      if num>=list.len() {
        println!("ERROR: Invalid item");
      }else{
        list[num].print();
      }
    }else if input == "edit" {
      let num:usize = read!();
      
      let mut var:String; 
      println!("Editing {}",num);

      //LOOK HERE
      loop{
        print!("What value would you like to edit? [(n)ame,(d)escription,(s)tatus,done]: ");
        stdout().flush();
        var = read!("{}\n");
        if var == "name"|| var == "n" {
          print!("New name: ");
          stdout().flush();
          let name:String = read!("{}\n");    

          list[num].name=name;
          // re-bind name 
        }else if var == "description"||var=="d" {
          print!("New description: ");
          stdout().flush();
          let desc:String = read!("{}\n");    

          list[num].description=desc;
          // re-bind desc 
        }else if var == "status"||var=="s" {
          print!("Change status [(i)ncomplete,(s)tarted,(c)omplete]: ");
          stdout().flush();
          let status:String = read!("{}\n"); 

          if status=="incomplete"||status=="i" {
            list[num].state = 0;        
          }else if status=="started"||status=="s" {
            list[num].state = 1;
            list[num].start_time = SystemTime::now();
          }else if status=="complete"||status=="c" {
            list[num].state = 2;
            list[num].end_time = SystemTime::now();
          }else{
            println!("Invalid status: \"{}\"",status);
          }

          // re-bind status
        }else if var=="done" {
          break;
        }else{
          println!("Invalid command: \"{}\"",var);
        }
      }
        
      // additional options here?
    }else if input == "help" {
      print!("Possible Commands are:\n");
      print!("help -> Print this help menu.\n");
      print!("exit -> Exit the program.\n");
      print!("print -> Print all current items in the To Do list.\n");
    }
  }
}

fn run_cmd(list:&mut Vec<Item>, cmd_str: String) -> String {
    let input_args:Vec<String> = cmd_str.split(" ").map(|x| x.to_string().trim().to_string()).collect();
    //DEBUGGING
    print!("Run CMD: '{}'\n",cmd_str);
    print!("inputArgs: '{:?}'\n",input_args);
    //END DEBUGGING
    
    if input_args[0] == "print"  {
        return get_list(&list);
    }else if input_args[0] == "add" {
        print!("Name: ");
        stdout().flush();
        let name:String =input_args[1].clone();
        let description:String =input_args[2].clone();
        print!("{}\n{}\n",name,description);
        let item:Item = Item::new(name,description,SystemTime::now());
        list.push(item);
        // add to list
    }else if input_args[0] == "show" {
        let num:u32=input_args[1].parse::<u32>().unwrap();
        print!("Showing {}\n",num);
        //list[num].print();
    }else if input_args[0] == "edit" {
        let num:u32= input_args[1].parse::<u32>().unwrap();
        print!("Editing {}\nWhat value would you like to edit? [name,description,status]",num);
        stdout().flush();
        let mut var:String; 
        let mut done:bool = false;
        //LOOK HERE
        while !done {
            var = read!("{}\n");
            if var == "name" {

                done=true;
            }
        }

        // additional options here?
    }else if input_args[0] == "help" {
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
    let mut list:Vec<Item> = Vec::new();

    //Create some default items -- Just for demo sake
    let mut item:Item = Item::new("Eat".to_string(),"eat a sandwich".to_string(),SystemTime::now());
    list.push(item);
    item = Item::new("Sleep".to_string(),"Take a nap.".to_string(),SystemTime::now());
    list.push(item);



    //A one way pipe for sending data between threads...
    let (send_to_server,recv_from_server) = mpsc::channel();
    let mut send_to_thread= Vec::new();
    
    //For each thread
    let (mut sender,mut receiver) = mpsc::channel();
    send_to_thread.push(sender);

    //Client Thread
    let sender = send_to_server.clone(); 
    thread::spawn(move || {
        let mut input_file = File::open("input0").unwrap();
        loop{
            let mut val:String=String::new();
            input_file.read_to_string(&mut val).unwrap();
            if( val.len() < 2){
                continue;
            }
            print!("Input: {}",val);
            sender.send(val).unwrap();
            let mut resp:String = receiver.recv().unwrap();
            let mut file = File::create("client0").unwrap();
            file.write_all(resp.as_bytes()).unwrap();
        }

    });

    let (sender,receiver) = mpsc::channel();
    send_to_thread.push(sender);
    let sender = send_to_server.clone(); 
    /*thread::spawn(move || {
        let mut input_file = File::open("input1").unwrap();
        let mut val:String=String::new();
        input_file.read_to_string(&mut val).unwrap();
        sender.send(val).unwrap();
        let mut resp:String = receiver.recv().unwrap();
        let mut file = File::create("client1").unwrap();
        file.write_all(resp.as_bytes()).unwrap();
    });
*/

    //INIFINTE PROCESSING LOOP
    let mut cmd:String;
    let mut requester:u32;
    let mut resp:String;
    loop{
    //Command Processing
    cmd = recv_from_server.recv().unwrap();
    requester=cmd.remove(0).to_digit(10).unwrap();
    
    //String Space
    cmd.remove(0);
    print!("Server received command from: {}\n{}\n\n\n",requester,cmd);
    
    resp = run_cmd(&mut list,cmd);
    print!("Server about to send to: {}\n{}\n\n\n",requester,resp);
    send_to_thread[(requester as usize)].send(resp).unwrap();
    }
}

