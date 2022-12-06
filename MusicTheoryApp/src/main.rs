extern crate rust_music_theory as rustmt;
use rustmt::note::{Note, Notes, PitchClass};
use rustmt::scale::{Scale, ScaleType, Mode, Direction};
use rustmt::chord::{Chord, Number as ChordNumber, Quality as ChordQuality};
use text_io::scan;
use std::{io, fs};
use colored::Colorize;
use std::io::{stdin, stdout, Read, Write};
mod chord_progression;
use crate::chord_progression::chord_progression::*;


fn help(){

    // scroll up
   
println!("
    Notes as strings:
    'C' | 'c' => C,
    'Cs' | 'cs' => C#,
    'D' | 'd' => D,
    'Ds' | 'ds' => D#,
    'E' | 'e' => E,
    'F' | 'f' => F,
    'Fs' | 'fs' => F#,
    'G' | 'g' => G,
    'Gs' | 'gs' => G#,
    'A' | 'a' => A,
    'As' | 'as' => A#,
    'B' | 'b' => B,
    
    Modes of the major scale:
    'Ionian' | 'ionian' => Ionian,
        -contains the notes: 1 2 3 4 5 6 7

    'Dorian' | 'dorian' => Dorian,
        -contains the notes: 1 2 b3 4 5 6 b7

    'Phrygian' | 'phrygian' => Phrygian,
        -contains the notes: 1 b2 b3 4 5 b6 b7

    'Lydian' | 'lydian' => Lydian,
        -contains the notes: 1 2 3 #4 5 6 7

    'Mixolydian' | 'mixolydian' => Mixolydian,
        -contains the notes: 1 2 3 4 5 6 b7

    'Aeolian' | 'aeolian' => Aeolian,
        -contains the notes: 1 2 b3 4 5 b6 b7

    'Locrian' | 'locrian' => Locrian,
        -contains the notes: 1 b2 b3 4 b5 b6 b7
    ");
}

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}


// derive debug for all enums
#[derive(Debug)]
enum EntryError {
    TonicError,
    ModeError,
    DirectionError,
   
}

// This is a helper function that makes inline input easier.
fn inline_user_input(prompt: &str) -> String {
    let to_return;
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    scan!("{}\n", to_return);
    return to_return;
}
// match_tonic calls the PitchClass::from_regex() method to check if the user input is a valid pitch class
fn match_tonic(input: String) -> Result<PitchClass, EntryError>{
    match PitchClass::from_regex(input.as_str()){
        Ok(_) => Ok(PitchClass::from_regex(input.as_str()).unwrap().0),
        _ => Err(EntryError::TonicError),
    }
}

// match_mode calls the Mode::from_regex() method to check if the user input is a valid mode
fn match_mode(input:String)-> Result<Mode,EntryError>{
    match Mode::from_regex(input.as_str()){
        Ok(_) => Ok(Mode::from_regex(input.as_str()).unwrap().0),
        _ => Err(EntryError::ModeError),
    }
}


// get_mode calls get_mode to check if the user input is a valid mode, reprompts if not
fn get_mode()-> Mode{
    loop{
        let usr=inline_user_input("Enter the Mode of the Scale: ");
        let mode = match_mode(usr);
        match mode{
            Ok(_) => return mode.unwrap(),
            Err(_) => println!("{}","Invalid mode. Try again.".red()),
        }
    }
}


// get_mode calls get_tonic to check if the user input is a valid mode, reprompts if not
fn get_tonic() -> PitchClass{
    loop{
        let usr=inline_user_input("Enter the root note: ");
        let tonic = match_tonic(usr);
        match tonic{
            Ok(_) => return tonic.unwrap(),
            Err(_) => println!("{}","Invalid tonic. Try again.".red()),
        }
    }
}


// allow unused because this is a helper function
#[allow(unused)]
// scale_as_vector returns a vector of notes from a scale
fn scale_as_vector(tonic: PitchClass,mode: Mode, direction: Direction ) -> Vec<Note> {
    let scale1 = Scale::new(ScaleType::from_mode(mode), tonic, 4,Some(mode),direction).unwrap();
    return scale1.notes();
}

// print_scale prints a vector of notes
fn print_scale(scale: &Vec<Note>) {
    for note in scale {
        println!("{}", note);
    }
}

// write_notes_to_file: so that we can write user output 
fn write_notes_to_file(scale: &Vec<Note>) {
    let mut file = std::fs::File::create("../notes.txt").unwrap();
    for note in scale {
        writeln!(file, "{},{}", note,note.octave).unwrap();
    }
}
// append_notes_to_file: so that we can chords to the file iteratively
fn append_notes_to_file(notes: &Vec<Note>, file_name: &str) {
    let mut file = fs::OpenOptions::new().write(true).append(true).open(file_name).unwrap();
    for note in notes {
        writeln!(file, "{},{}", note,note.octave).unwrap();
    }
}

// takes in the chord progression struct and writes it to a file with the given name
fn write_prog_to_file(to_write:&ChordProgression, file_name:&str){
    let mut i=0;
    // if file doesn't exist, create it
    let destination = format!("../{}.txt",file_name);
    let file = std::fs::File::create(destination.clone()).unwrap();
    let mut file = fs::OpenOptions::new().write(true).append(true).open(destination.clone()).unwrap();
    writeln!(file,"{}", to_write.get_num_chords()).unwrap();
    writeln!(file,"-").unwrap();
    while i < to_write.get_num_chords(){
        let iterator = i as usize;
        let notes_to_add = chord_as_vector(to_write.chord_progression[iterator].root, to_write.chord_progression[iterator].quality,to_write.chord_progression[iterator].number);
        let note_count=notes_to_add.len();
        writeln!(file,"{}", note_count).unwrap();
        append_notes_to_file(&notes_to_add,&destination);
        if i!= to_write.get_num_chords()-1{
            writeln!(file,"-").unwrap();
        }
        i+=1;
    }
}

fn view_notes_in_scale(){

    let tonic= get_tonic();

    let mode:Mode = get_mode();
    let direction:String = inline_user_input("Enter the direction of the scale (asc/desc): ");

    
    
    let scale_direction: Direction;
    // let tonic = &tonic[..]; // convert input into a &str


    // rust is strongly typed, needing to redeclare variable here instead of just mutating it. This leads to less bugs but adds a line of redundant code. types are known at runtime
    
    if direction.to_uppercase() =="ASC"{
        scale_direction = Direction::Ascending;
    } else if direction.to_uppercase() =="DESC"{
        scale_direction = Direction::Descending;
    } else {
        println!("Invalid direction. Defaulting to ascending.");
        scale_direction = Direction::Ascending;
    }

    // creating scale object
    let usr_scale = Scale::new(
        ScaleType::from_mode(mode),    // scale type
        tonic,    // tonic
        4,                      // octave
        Some(mode),     // scale mode
        scale_direction,   // scale direction
    ).unwrap();
    let user_notes = &usr_scale.notes();

    // print all notes in user_notes followed by a newline
    print_scale(user_notes);
    write_notes_to_file(user_notes);
    pause();
}



// takes in 
// returns a vector containing the notes in a scale 

fn view_notes_in_chord(){
    let root:PitchClass= get_tonic();
    let quality:String= inline_user_input("Enter chord quality: ");
    let extension: String = inline_user_input("Enter the superscript number of the chord (3, 7, maj7, etc): ");
    let quality: &str = &quality[..]; 
    let extension: &str = &extension[..]; 
    //use match to check if the root is a valid note
  
    // store quality as a Quality converted from &str regex to Quality
    let quality_from_string:ChordQuality= ChordQuality::from_regex(quality).unwrap().0;

    let chord = Chord::new(root, quality_from_string, ChordNumber::from_regex(extension).unwrap().0);
    

    let user_notes:&Vec<Note>= &chord.notes();
    for note in user_notes {
        println!("{}", note);
    }
    write_notes_to_file(user_notes);
    pause();

}

fn chord_as_vector(root:PitchClass,quality:ChordQuality,extension:ChordNumber)-> Vec<Note>{
    let chord = Chord::new(root,quality,extension);
    let to_return:Vec<Note>=chord.notes();
    return to_return
}


// entry point for the program

fn display_options(){
    print! ("\x1B[2J\x1B[1;1H"); 
    println!(
        "{}\n > {}\n > {}\n",
        format!("Welcome to Notation!").bold().green().italic(),
        format!("This program will allow you to interact with the Rust Music Theory library.").green(),
        format!("You can create notes, scales, chords!").green(),
   
    );
   
    loop{
        println!("Choose from one of the following options: \n");
        println!("  1. view notes in a scale");
        println!("  2. view notes in a chord");
        println!("  3. create chord progression");
        println!("  4. help");
        println!("  5. exit");
        //get command line input
        let input = inline_user_input( ":");
        match input.as_str(){
            "1" => view_notes_in_scale(),
            "2" => view_notes_in_chord(),
            "3" => create_progression(),
            "4" => help(),
            "5" => break,
            _ => println!("{}", "Invalid Input, Try again".red())
        }
    }
    

}


fn create_progression(){
    let prog_name= inline_user_input("Enter the name of your chord progression: ").to_owned();
    let num_chords=inline_user_input("How many chords will be in your progression?: ").parse::<i32>().unwrap();
    let mut i: i32=0;
    let mut user_chords:ChordProgression= ChordProgression::new(prog_name.clone());
    // convert prog_name to &str
    
    while i < num_chords{
        println!("Chord {} ",format!("{}", (&i+1).to_string()));
        let root:String= inline_user_input("Enter Root of the chord: ");
        let quality:String= inline_user_input("Enter chord quality: ");
        let extension: String = inline_user_input("Enter the superscript number of the chord (3, 7, maj7, etc): ");
        let root: &str = &root[..]; 
        let quality: &str = &quality[..]; 
        let extension: &str = &extension[..]; 
        // store quality as a Quality converted from &str regex to Quality
        let quality_from_string:ChordQuality= ChordQuality::from_regex(quality).unwrap().0;
        let chord = Chord::new(PitchClass::from_str(root).unwrap(), quality_from_string, ChordNumber::from_regex(extension).unwrap().0);
        user_chords.add_chord(chord);
        i+=1;
    }
    // pass chord progression and progto function that writes to file
    write_prog_to_file(&user_chords, &prog_name);

}

fn main() {
   
    display_options();
}
