import React, { Component } from 'react';
import {BrowserRouter,Route,Switch} from  "react-router-dom";
import Login from './Components/auth/Login';
import Dashboard from './Components/dashboard/Dashboard';
import {Redirect} from "react-router";
import store from "store";
import Students from './Components/students/Students';

const PrivateRoute = ({component,...rest}) => {
  return (
      <Route {...rest} render= {(props) => props.loggedIn ?  <Component {...props} />: <Redirect to="/"></Redirect>} />
  )
}
class App extends Component {

  constructor() {
   

    super();

    


    this.checkLoggedIn = this.checkLoggedIn.bind(this);

  }


  checkLoggedIn() {
     
      if (store.get("loggedIn")) {
        return true
      }else {
        return false
      }
  }


  render() {
    return (
      <BrowserRouter>
        <Switch>
            <Route exact path="/" component = {Login} />
            <Route exact path="/dashboard" component ={Dashboard} />
            <Route exact path="/students" component ={Students} />
            {/* <PrivateRoute exact loggedIn={this.checkLoggedIn} path="/dashboard" component = {Dashboard}/> */}
        </Switch>
      
      </BrowserRouter>
    );
  }
}

export default App;
