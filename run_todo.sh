#!/bin/bash
#--------------------------------------------------
#File Name: run_todo.sh
#
#Creation Date: 30-10-2018
#
#Last Modified Date: Tue 30 Oct 2018 10:35:21 AM EDT
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
else
    echo "I'm starting up, better set the lock"
    touch $LOCK_FILE
    echo "I'm now started"
    ./target/debug/todo
    echo "I'm now ended"
    rm $LOCK_FILE
fi 
