/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {


      cassetteBlack: '#161513',
      cassetteWhite: '#fffbf2',
      cassettePink: '#ec5574',
      cassetteYellow: '#efeb6d',
      cassetteOrange: '#ef7b52',
      cassetteGreen: '#268043',
      cassetteBlue: '#278acd',



    },
    fontFamily: {
      // 'Bungee Shade', sans-serif;
      // font-family: 'Rampart One', sans-serif;
      // font-family: 'Barlow', sans-serif;
      // font-family: 'Caveat', cursive;
      'Barlow': ['Barlow', 'sans-serif'],
      'Bungee': ['Bungee Shade', 'sans-serif'],
      'Rampart': ['Rampart One', 'sans-serif'],
      'Caveat': ['Caveat', 'cursive'],
    },
  },
  plugins: [],
}
}

