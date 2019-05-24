import React from 'react'
import {Fab} from '@material-ui/core';
import {PlayArrow} from '@material-ui/icons';

class MachineStartButton extends React.Component {

  startVm = () => {
    const mid = this.props.machine_id;
    fetch(`http://localhost:8000/vm/up/${mid}`)
      .then(d => d.json())
      .then(d => {console.log("vm started", d);})
  }

  render() {
    return (<Fab onClick={this.startVm}> <PlayArrow color="green"/></Fab>)
  }
}

export default MachineStartButton;
