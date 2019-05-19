import React from 'react';

import './App.css';

import AppBar from '@material-ui/core/AppBar';
import Drawer from '@material-ui/core/Drawer';

import Machines from './Machines.js';

function App() {
  return (
    <div className="App">
      <AppBar color="primary" position="static">
        <h1>Rocket Application</h1>
      </AppBar>
      <Machines/>
    </div>
  );
}

export default App;
