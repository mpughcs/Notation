import React, { useState } from 'react';
import axios from 'axios';

const API_URL = 'http://127.0.0.1:8000';

const Chord = ({ onChordStateChange }) => {
    const [tonic, setTonic] = useState('C');
    const [quality, setQuality] = useState('major');
    const [extension, setExtension] = useState('triad');
    const [responseData, setResponseData] = useState('');


    const handleSubmit = async (e) => {
        e.preventDefault();

        try {
            const response = await axios.get(`${API_URL}/chord/${tonic}/${quality}/${extension}`);
            const chordData = { tonic, quality, extension };
            setResponseData(response.data);
            onChordStateChange(chordData); // Sending state to the parent
            console.log(response.data);
        } catch (error) {
            console.log(error);
        }
    };

    return (
        <div className='hover:translate-x-[.1rem] bg-cassetteWhite text-gray-600 p-2 rounded-lg drop-shadow-xl duration-150'>
                           <form className=' gap-2 text-white' onSubmit={handleSubmit} >
                    <div className='gap-2 pt-3 flex-nowrap'>
                        <h1 className='text-black inline-block min-w-max'>Tonic</h1>
                        <select className='px-1 inline-block h-min bg-opacity-0' name="tonic"  onChange={(e) => setTonic(e.target.value)} >
                            <option value="C">C</option>
                            <option value="Cs">C#</option>
                            <option value="D">D</option>
                            <option value="Ds">D#</option>
                            <option value="E">E</option>
                            <option value="F">F</option>
                            <option value="Fs">F#</option>
                            <option value="G">G</option>
                            <option value="Gs">G#</option>
                            <option value="A">A</option>
                            <option value="As">A#</option>
                            <option value="B">B</option>
                        </select>
                        :
                    </div>
                    <div className='flex gap-2 pt-3 flex-nowrap'>
                        <h1 className='text-black inline-block min-w-max'>Quality</h1>
                        <select className='px-1 inline-block h-min' name="quality" onChange={(e) => setQuality(e.target.value)}>
                            <option value="major">Major</option>
                            <option value="minor">Minor</option>
                            <option value="diminished">Diminished</option>
                            <option value="augmented">Augmented</option>
                            <option value="halfdiminished">HalfDiminished</option>
                            <option value="dominant">Dominant</option>
                            <option value="suspended2">Sus2</option>
                            <option value="suspended4">Sus4</option>
                        </select>
                        :
                    </div>
                    <div className='flex gap-2 pt-3 flex-nowrap'>
                        <h1 className='text-black inline-block min-w-max'>Extension</h1>
                        <select className='px-1 inline-block w-min h-min' name="extension" onChange={(e) => setExtension(e.target.value)}>
                            <option value="triad">Triad</option>
                            <option value="seventh">Seventh</option>
                            <option value="majorseventh">Maj7</option>
                            <option value="Ninth">Ninth</option>
                            <option value="eleventh">Eleventh</option>
                            <option value="thirteenth">Thirteenth</option>
                        </select>
                        :
                    </div>
                    {/* submission button */}
                    <div className='flex gap-2 pt-3 flex-nowrap justify-between'>
                        <button className='px-2 py-1 bg-cassettePink rounded-lg' type="submit">testAPI</button>
                    </div>
                </form>

        </div>
    );
};

export default Chord;
