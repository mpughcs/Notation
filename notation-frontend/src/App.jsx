import { useState } from 'react'
import './App.css'
import { BsFillMoonStarsFill } from 'react-icons/bs'
import Head from 'next/head';
import { BiPlusMedical } from 'react-icons/bi';
import { RiAiGenerate } from 'react-icons/ri';
import axios from 'axios';
import Chord from './Chord';
const API_URL = 'http://127.0.0.1:8000';


var chord = [];

console.log(chord.toString());

function getChord(tonic, quality, chordType) {
  axios.get(`${API_URL}/chord/${tonic}/${quality}/${chordType}`)
  .then(function (response) {
    console.log(response.data);
  })
  .catch(function (error) {
    console.log(error);
  });  
}

  
// chords stuct 
function App() {
  const [count, setCount] = useState(0)
  
  return (
    <div className="App px-10 pt-3 h-screen bg-gradient-to-tr from-black to-blue-950">
      <Head>
        <title>Notation</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <div className=''>
        <nav className="my-3 flex justify-between text-slate-400">
          <div className='flex flex-col'>
            <h1 className="font-mulish text-3xl lg:text-4xl ">
              Notation
            </h1>
            <h2>Create chord progressions, Generate midi</h2>
          </div>

          <ul className="flex items-center p-0 m-0 text-slate-400">
            <li className="hover:translate-x-1 duration-100">
              <BsFillMoonStarsFill className="cursor-pointer text-xl" />
            </li>
            <li className="hover:translate-x-1 duration-100 ml-8">
              <h1 className="p-1 rounded-s-xl bg-gradient-to-br from-green-200 to-green-100 text-gray-700 lg:p-2 lg:text-xl font-mulish">
                About
              </h1>
            </li>
          </ul>
        </nav>

        {/* project section */}

        <div className='my-4 mx-10 flex-col  hover:translate-x-[.1rem] duration-150 opacity-40 bg-gray-50 text-gray-600 p-4 rounded-lg'>
          <h1 className='text-2xl font-mulish'>Your Progression: </h1>
          <div className='flex gap-4 my-4'>
            <button onClick={() => testApi('c','minor', 'eleventh')} className="flex-col  hover:translate-x-[.1rem] duration-150 bg-slate-50 text-gray-600 p-2 rounded-lg drop-shadow-xl">
              <BiPlusMedical className='text-5xl mx-auto text-green-500' />
              New Chord
            </button>
            <Chord />
          </div>


        </div>

      </div>
    </div>


  )
}

export default App
