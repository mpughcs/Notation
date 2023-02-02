# Notation
Notation builds upon the [Rust Music Theory Library](https://github.com/ozankasikci/rust-music-theory) to offer users a way to interface with music theory and output midi audio files programmatically. The application is currently in early development by [Max Pugh](https://github.com/mpughcs)

# Version 0.0.3 Features
<!-- create indent -->
* Notation now supports chord progressions! which are a series of chords that are played in succession.
* When a user creates a chord progression, it is written to a file called "../progressions/progression_name.txt" in the parent directory.
* The user can then convert the chord progression and notes to playable audio by running the command
    * ```$ python3 notes_to_midi.py``` and following the instructions in the terminal.


### [Click this Link for example execution](https://www.youtube.com/watch?v=5GBCHi0R9Ak&feature=youtu.be)

# How to run

## Installing Rust and Cargo
* Follow this link to install [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html)
* Once install run the command to ensure Cargo is installed
    * ```$ cargo --version```
## Running the Program
* Clone the repository
* Navigate to the directory Notation/MusicTheoryApp in the terminal
* Run the command

    * ```$ cargo run```
## Converting notes to playable audio
* after exiting the command line app, run cd ../ to enter the parent director 
* Install the python package with the command
    * ```$ pip install MIDIUtil```
* Here you can view your notes in the file "notes.txt" which contains info about a note's pitch and octave.
* run the commands
    * ``` $ python3 notes_to_midi.py ``` 
* this script will convert the notes in scale.txt to midi notes saved to scale.mid and the notes from progrssions/your_progression_name.txt to MidiProg/your_progression_name.mid
* .mid files can be played in any MIDI player.
---
### Note about outputting .mid files on Mac OS
* .mid files are not playable by default on Mac OS. To play .mid files on Mac OS, you must install a MIDI player. 
* There is a command line MIDI player called [timidity](https://www.mankier.com/1/timidity) that can be installed with the command
    * ```$ brew install timidity```
    * Once installed,navigate to enclosing folder of .mid file you want to play and run the command
        * ```$ timidity output.mid```
    * This will produce a playable audio file in the terminal. which can be opened with the command
        * ```$ open output.ogg```
* Another option is to install a GUI MIDI player. I recommend [MuseScore](https://musescore.org/en/download) which is free and very popular among composers and students.

---







# Music Theory Concepts Implemented 
- If you are unfamiliar with music theory concepts or unsure if the code is producing the correct output, I found this helpful [reference](https://www.thejazzpianosite.com/jazz-piano-lessons/the-basics/modes/) that covers some of the basic concepts implemented in this project. 
# Feature Roadmap
<!-- # make checklist  -->
### 0.0.1
- [x] Help option
- [x] Exit option
- [x] View notes in any scale

### 0.0.2
- [x] View notes in a chord
- [x] Create chord progression
- [x] View notes in a chord progression
- [x] Scales and single chords written to file ../scale.txt
- [x] Chord progressions written to file ../progressions/progression_name.txt

### 0.0.3
- [x] Convert notes to playable audio using custom python interpreter
- [x] Midi files generated from notes_to_midi.py are saved in MidiProg/ directory

