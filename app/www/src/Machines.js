import React from 'react'

import Table from '@material-ui/core/Table';
import TableBody from '@material-ui/core/TableBody';
import TableHead from '@material-ui/core/TableHead';
import TableRow from '@material-ui/core/TableRow';
import TableCell from '@material-ui/core/TableCell';

import Machine from './Machine.js';

class Machines extends React.Component {

  constructor(props) {
    super(props)
    this.state = {machines: []}
  }

  componentDidMount() {
    this.updateFunc = setInterval(() => {
      fetch("http://localhost:8000/vm/status")
      .then(r =>  r.json())
      .then(d => {console.log(d); this.setState({'machines': d['machines']})})

    }, 10000)
  }

  render() {
    return (
      <Table>
        <TableHead>
          <TableRow>
            <TableCell>id</TableCell>
            <TableCell>name</TableCell>
            <TableCell>provider</TableCell>
            <TableCell>state</TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {this.state.machines.map(machine => (
          <Machine
            machine={machine}
          />
          ))}
        </TableBody>
      </Table>
    )
  }
}

export default Machines;

