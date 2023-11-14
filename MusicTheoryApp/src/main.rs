extern crate rust_music_theory as rustmt;
use rustmt::note::{Note, Notes, PitchClass};
use rustmt::scale::{Scale, ScaleType, Mode, Direction};
use rustmt::chord::{Chord, Number as ChordNumber, Quality as ChordQuality, self};
mod chord_progression;
use rocket::serde::json::Json;
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

#[macro_use] extern crate rocket;

// function to display helpful information to the user




#[get("/help")]
fn help() -> String{
 return "
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
    
    Chord extensions/numbers:
        Triad,
        Seventh,
        MajorSeventh,
        Ninth,
        Eleventh,
        Thirteenth,
    
    Chord qualities:
        Major,
        Minor,
        Diminished,
        Augmented,
        HalfDiminished,
        Dominant,
        Suspended2,
        Suspended4,
    ".to_string();
}

// allow unused because this is a helper function
// #[allow(unused)]
// scale_as_vector returns a vector of notes from a scale
fn scale_as_vector(tonic: PitchClass,mode: Mode, direction: String ) -> Vec<Note> {
    let scale_direction: Direction;
    if direction.to_uppercase() =="ASC"{
        scale_direction = Direction::Ascending;
    } else if direction.to_uppercase() =="DESC"{
        scale_direction = Direction::Descending;
    } else {
        println!("Invalid direction. Defaulting to ascending.");
        scale_direction = Direction::Ascending;
    }
    let scale1 = Scale::new(ScaleType::from_mode(mode), tonic, 4,Some(mode),scale_direction).unwrap();
    return scale1.notes();
}


fn chord_as_vector(root:PitchClass,quality:ChordQuality,extension:ChordNumber)-> Vec<Note>{
    let chord = Chord::new(root,quality,extension);
    let to_return:Vec<Note>=chord.notes();
    return to_return
}


pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Attaching CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
// fn main() {
//     display_options();
// }
#[get("/scale/<tonic>/<mode>/<direction>")]
fn scale(tonic: &str, mode: &str, direction: String ) -> String {
    let mut to_return = String::new();
    for note in scale_as_vector(PitchClass::from_str(tonic).unwrap(), Mode::from_regex(mode).unwrap().0, direction) {
        to_return.push_str(&format!("{}\n", note));
    }
    return to_return;
}


// this endpoint takes a root, quality, and extension and returns a vector of notes
#[get("/chord/<root>/<quality>/<extension>")]
fn get_chord(root: &str, quality: &str, extension: &str) -> String{
    let mut to_return = String::new();
    let chord = chord_as_vector(PitchClass::from_str(root).unwrap(), ChordQuality::from_regex(quality).unwrap().0, ChordNumber::from_regex(extension).unwrap().0);
    to_return.push_str(&format!("{}\n", chord.len()).to_string());
    for note in chord {
        to_return.push_str(&format!("{},{}\n", note, note.octave).to_string());
    }
    return to_return;
}

#[get("/")]
fn intstructions() -> String {
    return "<h1>Welcome to the Rust Music Theory API! To use this API, you can use the following endpoints:</h1>\n
    /scale/<tonic>/<mode>/<direction>\n
    /chord/<root>/<quality>/<extension>\n
    /help".to_string();

}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(CORS)
    .mount("/", routes![scale, intstructions,get_chord,help]
)
}