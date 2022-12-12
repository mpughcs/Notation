#!/bin/bash

# get the name of the progression on the command line

# add the .txt extension to the name
filename=$1

# echo "filename is $filename"
python3 notes_to_midi.py <<< $filename 
