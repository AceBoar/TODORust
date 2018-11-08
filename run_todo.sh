#!/bin/bash
#--------------------------------------------------
#File Name: run_todo.sh
#
#Creation Date: 30-10-2018
#
#Last Modified Date: Wed 07 Nov 2018 11:23:37 PM EST
#
#Author: Douglas Alpuche
#
#Class: 
#
#Assignment:
#--------------------------------------------------

LOCK_FILE="$HOME/.to_do.LOCK"

if [ -e $LOCK_FILE ] ; then
    echo "I'm already running... sending launch request to pipe."
    while read line ; do
        echo $line >> input0
        cat client0
    done
        
else
    echo "I'm starting up, better set the lock"
    touch $LOCK_FILE
    echo "I'm now started"
    ./target/debug/todo
    echo "I'm now ended"
    rm $LOCK_FILE
fi 
