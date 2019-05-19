import React from 'react'
import TableCell from '@material-ui/core/TableCell';
import TableRow from '@material-ui/core/TableRow';

class Machine extends React.Component {
  render() {
    return (
      <TableRow>
        <TableCell> { this.props.machine.id } </TableCell>
        <TableCell> { this.props.machine.name } </TableCell>
        <TableCell> { this.props.machine.provider } </TableCell>
        <TableCell> { this.props.machine.state } </TableCell>
      </TableRow>
    )
  }
}

export default Machine;
