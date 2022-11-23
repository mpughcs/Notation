pub mod chord_progression{
    use rustmt::chord::{Chord, self};

    pub struct ChordProgression{
        prog_name: String,
        chord_progression: Vec<Chord>,
        num_chords: i32,
    }
    impl ChordProgression{
        pub fn new(name:String)->Self{
            Self{
                prog_name: name,
                chord_progression: Vec::<Chord>::new(),
                num_chords:0
            }
        }
        pub fn get_name(&self) -> String {
            return self.prog_name.clone();
        }
        pub fn add_chord(& mut self,to_add:Chord){
            self.chord_progression.push(to_add); 
            self.num_chords+=1;
        }
        pub fn print_prog(&self){
            let count:i32=0;
            for chord in &self.chord_progression{
                println!("root: {}, quality:{}, extension: {}",
                format!("{}", chord.root.to_string()),
                format!("{}", chord.quality.to_string()),
                format!("{}", chord.number.to_string())
            );
            }
        }
        pub fn get_num_chords(&self)->i32{
            return self.num_chords;
        }
    }


}
