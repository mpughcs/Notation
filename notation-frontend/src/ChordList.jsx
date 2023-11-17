
import React, { useState } from 'react';
import Chord from './Chord';
import { BiPlusMedical } from 'react-icons/bi';
import { RiAiGenerate } from 'react-icons/ri';
import { RiFileMusicFill } from 'react-icons/ri';
import { GrMusic } from 'react-icons/gr';

const ChordList = () => {
    const [chords, setChords] = useState([]);
    const [responses, setResponses] = useState('');
    const [progressionName, setProgressionName] = useState('My project');
    const [chordState, setChordState] = useState({});


    const handleChordResponse = (chordResponse) => {
        setResponses((prevResponses) => prevResponses + chordResponse + '\n');
    };

    const handleChordStateChange = (chordData) => {
        setChordState(chordData); // Update the state in the parent component
        // update it in the list of chords

        console.log('Chord state in parent:', chordData);
    };

    // const handleChordStateChange = (chordData) => {
    //     setChords((prevChords) => [...prevChords, chordData]);
    // };

    const getAllChords = () => {
        // Loop through chord data and create a string
        const chordDataString = chords
            .map((chord, index) => {
                return `Chord ${index + 1}: Tonic - ${chord.tonic}, Quality - ${chord.quality}, Extension - ${chord.extension}`;
            })
            .join('\n');
        
        // Log the chord data string or use it for your separate API request
        console.log('Chords Data:', chordDataString);

        // Make your separate API request with chordDataString as the body
        // Example:
        // axios.post('YOUR_API_ENDPOINT', { chordData: chordDataString })
        //   .then(response => {
        //       console.log(response.data);
        //   })
        //   .catch(error => {
        //       console.error(error);
        //   });
    };
    
    


    const addChord = () => {
        setChords((prevChords) => [
            ...prevChords,
            <Chord key={prevChords.length} onChordStateChange={handleChordStateChange} />,
        ]);
    };

    const handleDelete = (index) => {
        setChords((prevChords) => prevChords.filter((chord, i) => i !== index));
    };

    const printChordData = () => {
        console.log('Chords Data:');
        chords.forEach((chord, index) => {
            if (chord && chord.props) {
                const { tonic, quality, extension } = chord.props;
                console.log(`Chord ${index + 1}: Tonic - ${tonic}, Quality - ${quality}, Extension - ${extension}`);
            }
        });
    };

    return (
        <div className='my-4 mx-4 flex-col bg-cassetteBlack text-gray-600 p-4 rounded-lg z-10 h-[270px]'>
            <span className='text-2xl font-Barlow text-cassetteWhite'> Your Progression: </span>
            <input
                name='progressionName'
                value={progressionName}
                onChange={(e) => setProgressionName(e.target.value)}
                className='text-2xl resize-none relative p-1 font-Caveat text-cassetteWhite'
                id=''
                cols='30'
                rows='1'
            ></input>
            <div className='flex gap-4 my-4 items-center justify-between'>
                <button
                    onClick={() => addChord()}
                    className='flex-col hover:translate-x-[.1rem] duration-150 text-cassetteYellow rounded-lg drop-shadow-xl'
                >
                    <BiPlusMedical className='text-5xl mx-auto ' />
                    New Chord
                </button>
                <div className=' flex gap-2 w-[70%] overflow-x-auto whitespace-nowrap overflow-y-hidden'>
                    {chords.map((chord, index) => (
                        <div key={index} className='flex hover:translate-x-[.1rem] duration-150 rounded-lg drop-shadow-xl'>
                            {chord}
                            <button
                                className='px-2 py-1 text-3xl text-cassettePink rounded-lg relative right-8 top-16'
                                onClick={() => handleDelete(index)}
                            >
                                -
                            </button>
                        </div>
                    ))}
                </div>
                <button onClick={getAllChords} className='flex-col hover:translate-x-[.1rem] duration-150 text-cassettePink rounded-lg drop-shadow-xl'>
                    <RiFileMusicFill className='text-5xl mx-auto ' />
                    Generate File
                </button>
            </div>
        </div>
    );
};

export default ChordList;
