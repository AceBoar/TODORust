#!/bin/bash
#--------------------------------------------------
#File Name: run_todo.sh
#
#Creation Date: 30-10-2018
#
#Last Modified Date: Wed 07 Nov 2018 11:35:49 PM EST
#
#Author: Douglas Alpuche
#
#Class: 
#
#Assignment:
#--------------------------------------------------

LOCK_FILE="/tmp/.to_do.LOCK"

if [ -e $LOCK_FILE ] ; then
    echo "I'm already running... sending launch request to pipe."
    cat $LOCK_FILE | sed 's/NAME/input/' |xargs mkfifo
    cat $LOCK_FILE | sed 's/NAME/client/' |xargs mkfifo
    echo 'NAME1' > $LOCK_FILE 
    while read line ; do
        echo $line >> input0
        cat client0
    done
        
else
    echo "I'm starting up, better set the lock"
    echo 'NAME0' > $LOCK_FILE
    echo "I'm now started"
    ./target/debug/todo
    echo "I'm now ended"
    rm $LOCK_FILE
fi 
