import React, { Component } from 'react';
import logo from './logo.svg';
import {BrowserRouter,Route} from  "react-router-dom";
import './App.css';
import Login from './Login';

class App extends Component {
  render() {
    return (
      <BrowserRouter>
        <Route path="/" component = {Login}/>
        
      </BrowserRouter>
    );
  }
}

export default App;
