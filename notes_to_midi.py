# create your MIDI object
import sys
from midiutil.MidiFile import MIDIFile
mf = MIDIFile(1)     # only 1 track
track = 0   # the only track
time = 0    # start at the beginning

mf.addTrackName(track, time, "Sample Track")
mf.addTempo(track, time, 120)

class Note: 
  def __init__(self, name, octave):
    self.name = name
    self.octave = octave

  def __repr__(self):
    return f"{self.name},{self.octave}"

class Chord:
  note_list:Note =[]
  length=len(note_list)
  def __init__(self)->None:
    self.note_list = []
    self.length = len(self.note_list)
  def add_note(self, note):
    self.note_list.append(note)
  def __repr__(self):
    return str(self.note_list)

def write_to_disk(output_file):
  fileOut=output_file+".mid"
  with open(fileOut, 'wb') as outf:
    mf.writeFile(outf)

set_of_notes = ('C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#', 'A', 'A#', 'B')
def midi_from_note(note_name, octave):
    note_index = set_of_notes.index( note_name.upper())
    midi_number = 12 * (int(octave) + 1) + note_index
    return midi_number

# def sting_to_midi_number(note_name):
def get_notes_from_file(fileName):
    list_of_notes = []
    try: 
       with open(fileName) as f:
        for line in f:
            line=line.strip().split(",") 
            list_of_notes.append(Note(line[0],line[1]))
    except IOError:
        print("File not accessible")
    return list_of_notes


def get_chords_from_file(filename):
  chords=[]
  with open(filename) as file_in:
    lines = []
    for line in file_in:
        lines.append(line.strip())
        
  numChords=lines[0]
  for i, line in enumerate(lines):
    if i==0:
      continue
    if line=="-":
      num_notes=int(lines[i+1])
      # print(num_notes)
      counter=0
      tonic_index=i+2
      chord_to_add=Chord()
      while counter<num_notes:
        note_to_add=lines[tonic_index+counter]
        chord_to_add.add_note(note_to_add)
        # print(note_to_add)
        counter+=1
        if(counter==num_notes):
          chords.append(chord_to_add)
          # chord_to_add.note_list=[]
  # print(chords)
  return(chords)


def print_chords(chords):
  for i, chord in enumerate(chords):
    print(chord)


def midi_from_note(note_name, octave):

    note_index = set_of_notes.index(note_name.upper())
    midi_number = 12 * (int(octave) + 1) + note_index
    return midi_number

def scale_to_midi(list_of_notes):
  # add some notes
  channel = 0
  volume = 100
  for i,note in enumerate(list_of_notes):
    pitch = midi_from_note(list_of_notes[i].name,list_of_notes[i].octave)          # C4 (middle C)
    time = i             # start on beat equal to it's index in the list of notes passed
    duration = 1         # 1 beat long
    mf.addNote(track, channel, pitch, time, duration, volume)
    # print(note)
  # print("reached here")
  
  write_to_disk("output")

def progression_to_midi(chords,output_file):
  channel = 0
  volume = 100
  # print(chords)
  for i,chord in enumerate(chords):
    for j,note in enumerate(chord.note_list):
      note=Note(note.split(",")[0],note.split(",")[1])
      pitch = midi_from_note(note.name,note.octave)          # C4 (middle C)
      time = i*2          # start on beat equal to it's index in the list of notes passed
      duration = 2         # 2 beat long
      mf.addNote(track, channel, pitch, time, duration, volume)
  # print("reached here")
  write_to_disk(output_file)



def process_scale():
  notes=get_notes_from_file("notes.txt")
  print("Notes read")
  scale_to_midi(notes)

def process_progression(file):
  fileTxt=file+".txt"
  chordList=get_chords_from_file(fileTxt)
  print("File read")
  progression_to_midi(chordList,file)
  print("Midi file written")

def scaleOrChord():
  while(True):
    print("Choose an option: ")
    print("\ts: Process a scale")
    print("\tp: Process a progression")
    print("\tq: Quit")
    usr=input("(s/p/q): ").lower()

    if usr=="s":
      try:
        print("Processing scale")
        process_scale()
        print("Scale processed")
        print("File saved as output.mid")
      except:
        print("Input Error, is notes.txt formatted correctly?")
      break
    elif usr=="p":
      try:
        fileName=input("Enter the name of progression: ")
        print("\nProcessing progression")
        process_progression(fileName)
        print("Progression processed")
        print("File saved as "+fileName+".mid")
      except:
        print("Processing Error, is ",fileName,".txt formatted correctly?")
        break
    elif usr=="q":
      print("Quitting")
      break

    else:
      print("Invalid input, please try again")
      pause=input("Press enter to continue")

scaleOrChord()