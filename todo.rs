use std::time::{SystemTime};

struct Item {
    name  : String,
    state : i16,
    description: String,
    startTime : SystemTime,
    endTime : SystemTime,
    createTime : SystemTime,
}

fn printList(Vec<Item> list){
  for x in 0..list.len() {
    print!("{}\t{}\t[{}]\n",x,list[x].name,if list[x].state == 0 {"Incomplete"} else if list[x].state == 1 {"Started"} else {"Complete"});
  }
}

fn printItem(Item todo){
  print!("{}\t[{}]\n{}i\n",todo.name,if list[x].state == 0 {"Incomplete"}
         else if list[x].state == 1 {"Started"} else
         {"Complete"},todo.description);
}

fn main(){

    let filename = "./todo.list";
    //If filename exists, open it for reading and writing, else error
    //Else create file... notify

    //Spawn 2 Threads...
    //Thread 1 -> Main UI
    /*
     * checkArgs() -> Looks if a filename was given, if not prompt for one...
     * mainMenu()  -> Display a basic prompt (Readline repl loop)
     * showHelp()  -> Display a list of commands
     * printList() -> Display the current to do list
     * toggleListDisplay() -> Permanently display the list above the prompt: toggleable^
     * 
     */
    //Thread 2 -> FILE I/O
    /*
     * updateFile() -> Update the to do list, called by thread one each time
     * checkForUpdates() -> Look if another terminal/application updated the to do list file
     * 
     */
}
