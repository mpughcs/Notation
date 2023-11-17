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


function App(props) {
  const [count, setCount] = useState(0)
  const [chords, setChords] = useState([]);

  

  return (
    <div className="App p-5 h-screen bg-cassetteBlack relative z-40 ">
      <div className="m-2 p-2 h-[80%] rounded-lg bg-cassetteWhite relative z-10 mt-5 ">
        <div className=''>
          <nav className="m-3  flex justify-between text-neutral-600">
            <div className='flex flex-col'>
              <h1 className="font-Bungee text-4xl lg:text-8xl ">
                Notation
              </h1>
              {/* i need a good one liner to describe the app */}
              <h2 className='pt-2 ml-5 text-2xl font-Barlow'>
                A midi chord generator for songwriters and producers.
              </h2>
            </div>

            <ul className="flex items-end">
              <li className="hover:translate-x-1 duration-100 ml-8">
                <a href="https://google.com" className="p-1 rounded-s-xl bg-gradient-to-br bg-right-bottom text-gray-700 lg:p-2 lg:text-2xl font-Barlow text-xl">
                  About
                </a>
              </li>
            </ul>
          </nav>
          <div className=''>

            
            <div className='absolute top-[10rem] md:top-[8rem] lg:top-[11.6rem] left-0 w-full z-10'>
              <div className='bg-cassettePink m-0 h-14'></div>
              <div className='bg-cassetteOrange m-0 w-full h-14'></div>
              <div className='bg-cassetteYellow m-0 w-full h-14'></div>
              <div className='bg-cassetteGreen m-0 w-full h-14'></div>
              <div className='bg-cassetteBlue m-0 w-full h-14'></div>
            </div>



            {/* Chord list */}
            <div className='p-2 relative z-20 '><ChordList /></div>
          </div>
        </div>
        <div className='flex justify-end py-11 px-3'>
          <span className='text-3xl font-Caveat text-cassetteBlack px-3'> BPM: </span>
          <input type="number" name="progressionName" defaultValue="120" className='text-3xl resize-none relative p-1 font-Caveat text-cassetteWhite w-20' id="" cols="30" rows="1"></input>
        </div>
      </div>

      <p className='text-center py-7 font-Barlow text-gray-600'>

        <span className='text-cassettePink'> generate </span>
        <span className='text-cassetteOrange'> midi </span>
        <span className='text-cassetteYellow'> chord </span>
        <span className='text-cassetteGreen'> progressions </span>

      </p>
      <div className='trapezoid'></div>




    </div>


  )
}

export default App
