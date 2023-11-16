import React, { useState } from 'react';
import Chord from './Chord';
import { BiPlusMedical } from 'react-icons/bi';
import { RiAiGenerate } from 'react-icons/ri';
import { RiFileMusicFill } from "react-icons/ri";
import { GrMusic } from "react-icons/gr";

class ChordList extends React.Component {
    constructor(props) {
        super(props); // Make sure to call the super constructor

        this.state = {
            chords: [],
            responses: '',
        };
    }

    handleChordResponse = (chordResponse) => {
        this.setState((prevState) => ({
            responses: prevState.responses + chordResponse + '\n',
        }));
    };

    addChord = () => {
        this.setState((prevState) => ({
            chords: [
                ...prevState.chords,
                <Chord key={prevState.chords.length} onSubmit={this.handleChordResponse} />,
            ],
        }));
    };
    processProgression = async () => {
        const { chords } = this.state;
        let responses = '';

        for (let i = 0; i < chords.length; i++) {
            const chordComponent = chords[i];
            if (chordComponent && chordComponent.props) {
                const { tonic, quality, extension } = chordComponent.props;

                try {
                    const response = await Chord.handleSubmit(tonic, quality, extension);
                    responses += `${response.data} `;
                } catch (error) {
                    console.log(`Error fetching chord: ${error}`);
                }
            }
        }

        console.log('Processed Progression:', responses);
        // You can update state or perform any other action with the responses here
    };


    render() {
        return (
            <div className='my-4 mx-4 flex-col  bg-cassetteBlack text-gray-600 p-4 rounded-lg z-10'>
                
                <span className='text-2xl font-Barlow text-cassetteWhite '> Your Progression: </span>
                <input name="progressionName" className='text-2xl resize-none relative p-1 font-Barlow text-cassetteWhite' id="" cols="30" rows="1"></input>
                <div className='flex  gap-4 my-4 items-center justify-between'>
                    <button
                        onClick={() => this.addChord()}
                        className='flex-col hover:translate-x-[.1rem] duration-150 text-cassetteYellow rounded-lg drop-shadow-xl'
                    >
                        <BiPlusMedical className='text-5xl mx-auto ' />
                        New Chord
                    </button>
                    <div className=' flex flex-wrap gap-1 kw-[70%]'>

                    {
                    
                    this.state.chords.map((chord, index) => (
                        <div key={index} className=' hover:translate-x-[.1rem] duration-150 rounded-lg drop-shadow-xl '>
                            {chord}
                        </div>
                    ))}
                    </div>

                    
                      <button
                        onClick={() => this.addChord()}
                        className='flex-col hover:translate-x-[.1rem] duration-150 text-cassettePink rounded-lg drop-shadow-xl'
                    >
                        <RiFileMusicFill className='text-5xl mx-auto ' />
                        Generate File
                    </button>
                </div>

            </div>
        );
    }
}

export default ChordList;
