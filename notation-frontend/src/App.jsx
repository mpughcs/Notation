import { useState } from 'react'
import './App.css'
import { BsFillMoonStarsFill } from 'react-icons/bs'
import Head from 'next/head';
import { BiPlusMedical } from 'react-icons/bi';
import { RiAiGenerate } from 'react-icons/ri';
import axios from 'axios';
import Chord from './Chord';
const API_URL = 'http://127.0.0.1:8000';
import React from 'react';
import ChordList from './ChordList';




// c component?
//A:
function App(props) {
  const [count, setCount] = useState(0)
  const [chords, setChords] = useState([]);
  

  function addChord() {
    setChords([...chords, <Chord key={chords.length} />]);
  }
  
  // create a div
 
  const handleChordSubmit = (chordData) => {
    // Handle form data here
    console.log(chordData);
  };
  

  async function generateProgression() {
    doc.getElementById('chord').onSubmit();
    // You can now access the form data in handleChordSubmit function
  }

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
        <ChordList />
        


      </div>
    </div>


  )
}

export default App
