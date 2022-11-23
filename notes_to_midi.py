from midiutil.MidiFile import MIDIFile

# create your MIDI object
mf = MIDIFile(1)     # only 1 track
track = 0   # the only track
fileName='notes.txt'

time = 0    # start at the beginning
mf.addTrackName(track, time, "Sample Track")
mf.addTempo(track, time, 120)


class Note: 
  def __init__(self, name, octave):
    self.name = name
    self.octave = octave
  def __repr__(self):
    return f"{self.name},{self.octave}"
    
def write_to_disk():
  with open("output.mid", 'wb') as outf:
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



   

testscale=get_notes_from_file("notes.txt")

def midi_from_note(note_name, octave):

    note_index = set_of_notes.index(note_name.upper())
    midi_number = 12 * (int(octave) + 1) + note_index
    return midi_number

def scale_to_midi(list_of_notes):
  # add some notes
  channel = 0
  volume = 100
  for i,note in enumerate(list_of_notes):
    # print(list_of_notes[i].name)
    pitch = midi_from_note(list_of_notes[i].name,list_of_notes[i].octave)          # C4 (middle C)
    time = i             # start on beat equal to it's index in the list of notes passed
    duration = 1         # 1 beat long
    mf.addNote(track, channel, pitch, time, duration, volume)
  write_to_disk()


def chord_to_midi(list_of_notes):
  channel = 0
  volume = 100
  for i,note in enumerate(list_of_notes):
    # print(list_of_notes[i].name)
    pitch = midi_from_note(list_of_notes[i].name,list_of_notes[i].octave)          # C4 (middle C)
    time = 0            # start on beat equal to it's index in the list of notes passed
    duration = 4         # 1 beat long
    mf.addNote(track, channel, pitch, time, duration, volume)
  write_to_disk()



chord_to_midi(testscale)

