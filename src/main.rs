extern crate rust_music_theory as rustmt;
use rustmt::note::{Note, Notes, PitchClass};
use rustmt::scale::{Scale, ScaleType, Mode, Direction};
// use rustmt::chord::{Chord, Number as ChordNumber, Quality as ChordQuality};
use text_io::scan;
use std::io::Write; // <--- bring flush() into scope
use std::io;

// This is a helper function that makes inline input easier.
fn inline_user_input(prompt: &str) -> String {
    let mut to_return= String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    scan!("{}\n", to_return);
    return to_return;
}

fn view_notes_in_scale(){
    // reading user inputs into variables

    let tonic:String = inline_user_input("Enter the tonic of the scale: ");
    let mode:String = inline_user_input("Enter the mode of the scale: ");
    let direction:String = inline_user_input("Enter the direction of the scale: ");

    // converting user inputs into the correct types

    let mode_from_reg: Mode = Mode::from_regex(&mode).unwrap().0;
    let scale_direction: Direction;
    let tonic = &tonic[..]; // convert input into a &str
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
        ScaleType::from_mode(mode_from_reg),    // scale type
        PitchClass::from_str(tonic).unwrap(),    // tonic
        4,                      // octave
        Some(mode_from_reg),     // scale mode
        scale_direction,   // scale direction
    ).unwrap();
    let user_notes = usr_scale.notes();

    // print all notes in user_notes followed by a newline
    for note in user_notes {
        println!("{}", note);
    }
}



// entry point for the program

fn display_options(){
    print! ("\x1B[2J\x1B[1;1H"); 
    println!("Welcome to the Rust Music Theory Interactive Program!");
    println!("This program will allow you to interact with the Rust Music Theory library.");
    println!("You can create notes, scales, chords!");
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
            "2" => println!("2"),
            "3" => println!("3"),
            "4" => println!("help"),
            "5" => break,
            _ => println!("invalid input"),
        }
    }
}
fn main() {
    //
    display_options();


}
