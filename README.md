# MusicTheoryApp
This project uses the [Rust Music Theory](https://github.com/ozankasikci/rust-music-theory) library to offer users a way to interface with music theory concepts in a command line application. Deve







# How to run
### Installing Rust and Cargo
* Follow this link to install [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html)
* Once install run the command to ensure Cargo is installed
    * $ cargo --version
### Running the Program
* Clone the repository
* Navigate to the directory
* Run the command
    * $ cargo run
* Follow the prompts

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
    Enter the direction of the scale: Asc
    C
    D
    E
    F
    G
    A
    B
 ```

# Feature Roadmap
<!-- # make checklist  -->
- [x] View notes in any scale
- [ ] View notes in a chord
- [ ] Create chord progression
- [ ] Help
- [x] Exit 
- [ ] Implement more music theory concepts
-  