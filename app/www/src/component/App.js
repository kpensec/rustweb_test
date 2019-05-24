import React from 'react';

import './App.css';

import {AppBar} from '@material-ui/core';

import Machines from './Machines.js'

function App() {
  return (
    <div className="App">
      <AppBar color="primary" position="static">
        <h1>Vagrant Status</h1>
      </AppBar>
      <Machines/>
    </div>
  );
}

export default App;
