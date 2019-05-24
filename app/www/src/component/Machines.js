import React from 'react'

import {Table, TableBody, TableHead} from '@material-ui/core';

import Machine from './Machine.js';

class Machines extends React.Component {

  constructor(props) {
    super(props)
    this.state = {machines: []}
  }

  updateFunc() {
    const statusUrl = "http://localhost:8000/vm/status"
    fetch(statusUrl)
    .then(r =>  r.json())
    .then(d => {this.setState({'machines': d['machines']})})
  }

  componentDidMount() {
    const refreshInterval = 10000;
    this.updateFuncInterval = setInterval(() => {this.updateFunc()}, refreshInterval);
    this.updateFunc() 
  }

  render() {
    const machines = this.state.machines;
    return (
      <Table>
        <TableHead> <Machine header={true}/> </TableHead>
        <TableBody> {machines.map(m => (<Machine machine={m}/>))} </TableBody>
      </Table>
    )
  }
}

export default Machines;

