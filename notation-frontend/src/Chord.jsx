import React from 'react';
import axios from 'axios';

const API_URL = 'http://127.0.0.1:8000';

function getChord(tonic, quality, chordType) {
    return axios.get(`${API_URL}/chord/${tonic}/${quality}/${chordType}`);
}


class Chord extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            tonic: '',
            quality: '',
            extension: '',
            // define your state here
        };
    }
    
    
    handleSubmit = async (e) => {
        e.preventDefault();

        const formData = new FormData(e.target);
        this.tonic = formData.get("tonic");
        this.quality = formData.get("quality");
        this.extension = formData.get("extension");

        try {
            const response = await getChord(this.tonic, this.quality, this.extension);
            console.log(response.data);

        } catch (error) {
            console.log(error);
        }
    };
    


    render() {
        return (
            <div className=' hover:translate-x-[.1rem] bg-cassetteWhite text-gray-600 p-2  rounded-lg drop-shadow-xl duration-150 '>
                {/* Your JSX goes here */}
                <form className=' gap-2 text-white' onSubmit={this.handleSubmit} >
                    <div className='flex gap-2 pt-3 flex-nowrap'>
                        <h1 className='text-black inline-block min-w-max'>Tonic</h1>
                        <select className='px-1 inline-block w-min h-min flex-1 bg-opacity-0' name="tonic" >
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
                        <select className='px-1 inline-block w-min h-min flex-1' name="quality">
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
                        <select className='px-1 inline-block w-min h-min flex-1' name="extension">
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
                    <div className='flex gap-2 pt-3 flex-nowrap'>
                        <button className='px-2 py-1 bg-cassettePink rounded-lg' type="submit">testAPI</button>
                    </div>
                </form>


            </div>
        );
    }
}

export default Chord;