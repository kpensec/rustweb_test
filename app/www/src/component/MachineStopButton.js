import React from 'react'
import {Fab} from '@material-ui/core';
import {Stop} from '@material-ui/icons';

class MachineStopButton extends React.Component {

  stopVm = () => {
    const mid = this.props.machine_id;
    fetch(`http://localhost:8000/vm/down/${mid}`)
      .then(d => d.json())
      .then(d => {console.log("vm stopped", d);})
  }

  render() {
    return (<Fab onClick={this.stopVm}> <Stop color="red"/></Fab>)
  }
}

export default MachineStopButton;
