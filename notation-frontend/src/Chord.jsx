import React from 'react';
import axios from 'axios';

const API_URL = 'http://127.0.0.1:8000';



function getChord(tonic, quality, chordType) {
    axios.get(`${API_URL}/chord/${tonic}/${quality}/${chordType}`)
    .then(function (response) {
      console.log(response.data);
    })
    .catch(function (error) {
      console.log(error);
    });  
  }

class Chord extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            // define your state here
        };
    }
    handleSubmit = (e) => {
        e.preventDefault(); // Prevent the default form submission behavior
      
        const formData = new FormData(e.target); // Create a FormData object from the form
      
        const tonic = formData.get("tonic");
        const quality = formData.get("quality");
        const extension = formData.get("extension");
      
        // Now you can use the values of the form fields (tonic, quality, extension) as needed
        getChord(tonic, quality, extension);
      
        // You can also update the component's state or make API calls here
      };

    render() {
        return (
            <div className=' hover:translate-x-[.1rem] duration-150 bg-slate-50 text-gray-600 p-2 rounded-lg drop-shadow-xl'>
                {/* Your JSX goes here */}
                <form className='flex-col gap-2 text-white' onSubmit = {this.handleSubmit} >
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
                            <option value="halfDiminished">HalfDiminished</option>
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
                            <option value="ninth">Ninth</option>
                            <option value="eleventh">Eleventh</option>
                            <option value="thirteenth">Thirteenth</option>
                        </select>
                        :
                    </div>
                    {/* submission button */}
                    <div className='flex gap-2 pt-3 flex-nowrap'>
                        <button className='px-2 py-1 bg-green-500 rounded-lg' type="submit">testAPI</button>
                    </div>
                </form>


            </div>
        );
    }
}

export default Chord;