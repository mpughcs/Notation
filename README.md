# Notation v.0.0.2

Notation builds upon the [Rust Music Theory Library](https://github.com/ozankasikci/rust-music-theory) to offer users a way to interface with music theory programmatically. The application is currently in early development by [Max Pugh](https://github.com/mpughcs)


# How to run
### Installing Rust and Cargo
* Follow this link to install [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html)
* Once install run the command to ensure Cargo is installed
    * ```$ cargo --version```
### Running the Program
* Clone the repository
* Navigate to the directory Notation/MusicTheoryApp in the terminal
* Run the command
    * ```$ cargo run```
### CONVERTING NOTES TO PLAYABLE NOTES
* after exiting the command line app, run cd ../ to enter the parent directory. 
* Install the python package with the command
    * ```$ pip install MIDIUtil```
* Here you can view your notes in the file "notes.txt" which contains info about a note's pitch and octave.
* run the commands
    * ``` $ python3 convert.py ```      
    this script will convert the notes in notes.txt to playable notes in via MIDI in the file "output.mid"
* Output.mid can be played in any MIDI player.

### Example execution
```cpp
$ cargo run

    Welcome to the Rust Music Theory Interactive Program!
    This program will allow you to interact with the Rust Music Theory library.
    You can create notes, scales, chords!
    Choose from one of the following options: 

        1. view notes in a scale
        2. view notes in a chord
        3. create chord progression
        4. help
        5. exit
    : 1
    Enter the tonic of the scale: C
    Enter the mode of the scale: Ionian 
    Enter the direction of the scale(ASC or DESC): Asc
    C
    D
    E
    F
    G
    A
    B
 ```
# Version 0.0.2 Features
<!-- create indent -->
* In version 0.0.2, in addition to being able to view notes in a scale and chord, the program also write's these notes to a file called "notes.txt" in the parent directory. 
* The parent directory not contains a python script "notes_to_midi.py" that converts the notes in the "notes.txt" file to a midi file called "output.mid" in the parent directory.





# Music Theory Concepts Implemented 
- If you are unfamiliar with music theory concepts or unsure if the code is producing the correct output, I found this helpful [reference](https://www.thejazzpianosite.com/jazz-piano-lessons/the-basics/modes/) that covers some of the basic concepts implemented in this project. 
# Feature Roadmap
<!-- # make checklist  -->
- [x] View notes in any scale
- [x] Help
- [x] Exit 
- [x] View notes in a chord
- [ ] Create chord progression
- [ ] Make notes and scales audible
- [ ] Make chords audible
