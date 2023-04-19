// defining the module 
pub mod chord_progression{
    use rustmt::chord::{Chord};

// defining the chord progression struct
    pub struct ChordProgression{
        #[allow(dead_code)]
        prog_name: String,
        pub chord_progression: Vec<Chord>,
        num_chords: i32,
    }
    // the constructor for the chord progression,
    // returns object of type ChordProgression
    impl ChordProgression{
        pub fn new(name:String)->Self{
            Self{
                prog_name: name,
                chord_progression: Vec::<Chord>::new(),
                num_chords:0
            }
        }
        // accessor for the name of the progresion
        pub fn get_name(&self) -> String {
            return self.prog_name.clone();
        }
        // add chord to the progression
        pub fn add_chord(& mut self,to_add:Chord){
            self.chord_progression.push(to_add); 
            self.num_chords+=1;
        }
        // add chord to the progression
        pub fn print_prog(&self){
            for chord in &self.chord_progression{
                println!("root: {}, quality:{}, extension: {}",
                format!("{}", chord.root.to_string()),
                format!("{}", chord.quality.to_string()),
                format!("{}", chord.number.to_string())
            );
            }
        }
        // accessor for the number of chords in the progression
        pub fn get_num_chords(&self)->i32{
            return self.num_chords;
        }
    }


}
