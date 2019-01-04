import React, { Component } from 'react';
import {BrowserRouter,Route,Switch} from  "react-router-dom";
import Login from './Components/auth/Login';
import Dashboard from './Components/Dashboard';

class App extends Component {
  render() {
    return (
      <BrowserRouter>
        <div>
            <Route path="/" component = {Login}/>
            <Route path="/dashboard" component = {Dashboard}/>
        </div>
      
      </BrowserRouter>
    );
  }
}

export default App;
