import React from 'react';

class Chord extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            // define your state here
        };
    }

    render() {
        return (
            <div className='flex-col  hover:translate-x-[.1rem] duration-150 bg-slate-50 text-gray-600 p-2 rounded-lg drop-shadow-xl'>
                {/* Your JSX goes here */}

            </div>
        );
    }
}

export default Chord;