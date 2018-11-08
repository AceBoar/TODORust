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
    fn print(&self) -> String{
      return format!("Task: {}\t State: [{}]\n\t{}\n",self.name,if self.state == 0 {"Incomplete"}
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


fn run_cmd(list:&mut Vec<Item>, cmd_str: String) -> String {
    let input_args:Vec<String> = cmd_str.split(",").map(|x| x.to_string().trim().to_string()).collect();

    //DEBUGGING
    print!("Run CMD: '{}'\n",cmd_str);
    print!("inputArgs: '{:?}'\n",input_args);
    //END DEBUGGING
    
    if input_args[0] == "print"  {
        return get_list(&list);
    }else if input_args[0] == "add" {
        let name:String =input_args[1].clone();
        let description:String =input_args[2].clone();

        let item:Item = Item::new(name,description,SystemTime::now());
        list.push(item);
        return "Added Successfully\n".to_string()
        // add to list
    }else if input_args[0] == "show" {
        let num:usize=input_args[1].parse::<usize>().unwrap();

        return list[num].print();
    }else if input_args[0] == "edit" {
        let num:usize= input_args[1].parse::<usize>().unwrap();

        //LOOK HERE
        if input_args[2] == "n" {
          list[num].name=input_args[3].clone();
          // re-bind name 
        }else if input_args[2] == "d" {
          list[num].description=input_args[3].clone();
          // re-bind desc 
        }else if input_args[2] == "s" {
          if input_args[3]=="i" {
            list[num].state = 0;        
          }else if input_args[3]=="s" {
            list[num].state = 1;
            list[num].start_time = SystemTime::now();
          }else if input_args[3]=="c" {
            list[num].state = 2;
            list[num].end_time = SystemTime::now();
          }

          // re-bind status
        }
    }else {
        return "print -> print list\nadd, NAME, DESC -> add item\nshow, # -> Show info about item\n".to_string();
    }

    return "Command Failed... try 'help'".to_string();

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

