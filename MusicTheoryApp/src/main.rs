extern crate rust_music_theory as rustmt;
use rustmt::note::{Note, Notes, PitchClass};
use rustmt::scale::{Scale, ScaleType, Mode, Direction};
use rustmt::chord::{Chord, Number as ChordNumber, Quality as ChordQuality, self};
use text_io::scan;
use core::num;
use std::f32::consts::PI;
use std::io;
use colored::Colorize;
use std::io::{stdin, stdout, Read, Write};
mod chord_progression;
use crate::chord_progression::chord_progression::*;


fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

        


// use method print chords from chord_progression.rs


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


// This is a helper function that makes inline input easier.
fn inline_user_input(prompt: &str) -> String {
    let to_return;
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    scan!("{}\n", to_return);
    return to_return;
}



fn scale_as_vector(tonic: PitchClass,mode: Mode, direction: Direction ) -> Vec<Note> {
    let scale1 = Scale::new(ScaleType::from_mode(mode), tonic, 4,Some(mode),direction).unwrap();
    return scale1.notes();
}



fn print_scale(scale: &Vec<Note>) {
    for note in scale {
        println!("{}", note);
    }
}
fn write_notes_to_file(scale: &Vec<Note>) {
    let mut file = std::fs::File::create("../notes.txt").unwrap();
    for note in scale {
        writeln!(file, "{},{}", note,note.octave).unwrap();
    }
}

fn view_notes_in_scale(){
    // reading user inputs into variables
    let tonic:String = inline_user_input("Enter the tonic of the scale: ");
    let mode:String = inline_user_input("Enter the mode of the scale: ");
    let direction:String = inline_user_input("Enter the direction of the scale (asc/desc): ");

    // converting user inputs into the correct types
    let mode_from_string: Mode = Mode::from_regex(&mode).unwrap().0;
    let scale_direction: Direction;
    let tonic = &tonic[..]; // convert input into a &str
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
        ScaleType::from_mode(mode_from_string),    // scale type
        PitchClass::from_str(tonic).unwrap(),    // tonic
        4,                      // octave
        Some(mode_from_string),     // scale mode
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
    let root:String= inline_user_input("Enter Root of the chord: ");
    let quality:String= inline_user_input("Enter chord quality: ");
    let extension: String = inline_user_input("Enter the superscript number of the chord (3, 7, maj7, etc): ");
    let root: &str = &root[..]; 
    let quality: &str = &quality[..]; 
    let extension: &str = &extension[..]; 
    // store quality as a Quality converted from &str regex to Quality
    let quality_from_string:ChordQuality= ChordQuality::from_regex(quality).unwrap().0;
    let chord = Chord::new(PitchClass::from_str(root).unwrap(), quality_from_string, ChordNumber::from_regex(extension).unwrap().0);
    let user_notes:&Vec<Note>= &chord.notes();
    for note in user_notes {
        println!("{}", note);
    }
    write_notes_to_file(user_notes);

}

// fn chord_as_vector(root:PitchClass,quality:ChordQuality,extension:ChordNumber)-> Vec<Note>{
//     let chord = Chord::new(root,quality,extension);
//     let to_return:Vec<Note>=chord.notes();
//     return to_return
// }


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
            _ => println!("invalid input"),
        }
    }
    

}


fn create_progression(){
    let chord_name= inline_user_input("Enter the name of your chord progression: ");
    let num_chords=inline_user_input("How many chords will be in your progression?: ").parse::<i32>().unwrap();
    let mut i: i32=0;
    let mut user_chords:ChordProgression= ChordProgression::new(chord_name);
    
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
    user_chords.print_prog()
}
// fn test_progressions(){
//     let mut c1:ChordProgression= ChordProgression::new("251".to_string());
//     let prog_name=c1.get_name();

//     let chord1 = Chord::new(PitchClass::B,ChordQuality::Major,ChordNumber::Seventh);
//     let chord2 = Chord::new(PitchClass::C,ChordQuality::Minor,ChordNumber::Triad);
//     c1.add_chord(chord1);
//     c1.add_chord(chord2);
  
//     println!("Progression name: {}",prog_name);
//     println!("Number of chords in Prog: {}",c1.get_num_chords());
//     c1.print_prog();
// }
fn main() {
    // let scale2= scale_as_vector(PitchClass::A, Mode::Locrian, Direction::Ascending);
    // let test_chord= chord_as_vector(PitchClass::C, ChordQuality::Minor, ChordNumber::Triad );
    // print_scale(test_chord);
    // test_progressions();
    display_options();
    
}
