import { useState } from 'react'
import './App.css'

import { invoke } from '@tauri-apps/api/tauri';

function App() {
  const supportedSlashNotation = [24, 25, 26, 27, 28, 29, 30, 31, 32]
  const [output, setOutput] = useState([
    {
      subnet: 1,
      networkAddress: "192.168.1.0",
      subnetMask: "255.255.255.255/24",
      usableIpRange: "192.168.1.1 - 192.168.1.254",
      broadcastAddress: "192.168.1.255"
    }
  ]);

  const onSubmit = (e) => {
    e.preventDefault();

    const ipAddress = e.target.elements.ip.value;
    const slashNotation = Number(e.target.elements.slashNotation.value.substring(1));

    invoke("display_ip", { ipAddress: ipAddress, slashNotation: slashNotation })
      .then(message => {
        console.log(message)

        setOutput(
          message.map(row => JSON.parse(row))
        )        
      });
  }

  return (
    <div>
      <div className="window">
        <div className="title-bar"> 
          <h1 className="title">scalc gui - subnetting calculator</h1>
        </div> 

        <div className="separator"></div>
      
        <div className="window-pane">
          <form onSubmit={onSubmit}>
            <input aria-label="ip address" type="text" name="ip" placeholder="ip address"/>
            <select name="slashNotation">
              {supportedSlashNotation.map(notation =>
                <option>/{notation}</option>
              )}
            </select>
          </form>
        </div>
      </div> 

      <div className="window">
        <div className="window-pane" style={{fontSize: 16}}>
          <table>
            <tr>
              <th>subnets</th>
              <th>network address</th>
              <th>subnet mask</th>
              <th>usable ip address range</th>
              <th>broadcast address</th>
            </tr>
            {
              output.map(row => 
                <tr>
                  <td>{row.subnet}</td>
                  <td>{row.networkAddress}</td>
                  <td>{row.subnetMask}</td>
                  <td>{row.usableIpRange}</td>
                  <td>{row.broadcastAddress}</td>
                </tr>
              )
            }
          </table>
        </div>
      </div>
    </div>
  )
}

export default App
