import React from 'react'

import {TableCell, TableRow} from '@material-ui/core';

import MachineStartButton from './MachineStartButton.js'
import MachineStopButton from './MachineStopButton.js'

const statusBrief = ["id", "name", "state"];

class Machine extends React.Component {

  render() {
    const headerLine = this.props.header;
    const data = statusBrief;
    let mapper;

    if (headerLine) {
      mapper = (e) => (<TableCell>{e}</TableCell>);
    } else {
      const machine = this.props.machine;
      const state = machine.state;
      let add_button = (button, state) => (<span>{state}{button}</span>)
      if (state === "poweroff") {
        machine.state = add_button(<MachineStartButton machine_id={machine.id}/>, state)
      } else  {
        machine.state = add_button(<MachineStopButton machine_id={machine.id}/>, state)
      }
      mapper = (k) => (<TableCell>{machine[k]}</TableCell>);
    }

    return ( <TableRow> { data.map(mapper)} </TableRow>)
  }
}

Machine.defaultProps = {
  header: false
}

export default Machine;
