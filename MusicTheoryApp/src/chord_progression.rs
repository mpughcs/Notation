pub mod chord_progression{
    use rustmt::chord::Chord;

    pub struct ChordProgression{
        prog_name: String,
        chord_progression: Vec<Chord>,
        
    }
    impl ChordProgression{
        pub fn new(name:String)->Self{
            Self{
                prog_name: name,
                chord_progression: Vec::<Chord>::new(),
            }
        }
        pub fn get_name(&self) -> String {
            return self.prog_name.clone();
        }
    }


}
