struct Item {
    name  : string,
    state : int16,
    description: string,
    startTime : time_t,
    endTime   : time_t,
    createTime : time_t,
}








fn main(){

    let filename = #argv1;
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

