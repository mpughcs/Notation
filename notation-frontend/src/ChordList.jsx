import React, { useState } from 'react';
import Chord from './Chord';
import { BiPlusMedical } from 'react-icons/bi';
import { RiAiGenerate } from 'react-icons/ri';

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

    generateProgression = async () => {
        for (let chordRef of this.state.chords) {
            // Access the current instance of Chord component and call handleSubmit
            await chordRef.props.onSubmit({ preventDefault: () => {} });
            // You can now access the form data in handleChordSubmit function
            // print responses

            console.log(chordRef);
        }
    };

    render() {
        return (
            <div className='my-4 mx-10 flex-col  hover:translate-x-[.1rem] duration-150 opacity-40 bg-gray-50 text-gray-600 p-4 rounded-lg'>
                <h1 className='text-2xl font-mulish'> Your Progression: </h1>
                <div className='flex gap-4 my-4'>
                    <button
                        onClick={() => this.addChord()}
                        className='flex-col  hover:translate-x-[.1rem] duration-150 bg-slate-50 text-gray-600 p-2 rounded-lg drop-shadow-xl'
                    >
                        <BiPlusMedical className='text-5xl mx-auto text-green-500' />
                        New Chord
                    </button>
                    {
                    
                    this.state.chords.map((chord, index) => (
                        <div key={index} className=' hover:translate-x-[.1rem] duration-150 bg-slate-50 text-gray-600 p-2 rounded-lg drop-shadow-xl'>
                            {chord}
                        </div>
                    ))}
                    <button>
                        <RiAiGenerate
                            onClick={() => this.generateProgression()}
                            className='text-5xl mx-auto text-green-500'
                        />
                    </button>
                </div>
            </div>
        );
    }
}

export default ChordList;
