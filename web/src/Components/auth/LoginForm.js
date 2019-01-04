import React,{Component} from "react";
import Notification from "../common/Notification";
import {Redirect} from "react-router";
import store from "store";


class LoginForm extends Component {
    constructor(props) {
        super(props);

        this.state = {
            username:"",
            password:"",
            failed:false,
            redirect:false,
        };


        this.handleChange = this.handleChange.bind(this);
        this.handleClick = this.handleClick.bind(this);
        this.handleClose = this.handleClose.bind(this);

    }

    handleChange(event) {
      this.setState({
          [event.target.name]:event.target.value
      });
    }

    handleClose(event) {
        this.setState({
            failed:false
        })
    }
 
    handleClick(event) {
        event.preventDefault();

        fetch("http://[::1]:8000/auth",{
            method:"POST",
            mode:'cors',
            body: new URLSearchParams(`username=${this.state.username}&password=${this.state.password}`)
        })
        .then(response => 
            response.json()
        ).then(data => 
            {

               

                const response = JSON.parse(JSON.stringify(data));
        
               
                if (response.status === true) {
                    store.set("loggedIn",true); // we are logged in
                    this.setState({
                        username:"",
                        password:"",
                        failed:false,
                        redirect:true,
                    });

                }else {
                    this.setState({
                        username:"",
                        password:"",
                        failed:true,
                        
                    });
                }
                
            }
        )
        .catch(err => {
            console.debug(err);
            this.setState({
                username:"",
                password:"",
                failed:true,
            })
        })
    }

    render() {

        if (this.state.redirect) {
            return (<Redirect to="/dashboard" />)
        }

        return(

            <div>

           
                <form action="">

                
                    <div className="field">
                        <div className="control">
                        <input className="input is-large" name="username" placeholder="Your Username" 
                        value={this.state.username} onChange={this.handleChange}/>
                        </div>
                    </div>

                    <div className="field">
                        <div className="control">
                            <input className="input is-large" name="password" type="password"
                            value={this.state.password} placeholder="Your Password" onChange={this.handleChange}/>
                        </div>
                    </div>

                    <button className="button is-block is-info is-large is-fullwidth" onClick={this.handleClick}>Login</button>


                   
                    
                </form>

                
                {
                    this.state.failed ? (
                    <Notification hidden={false} kind="" handleClose={this.handleClose}>
                       Incorrect Username or Password
                    </Notification>
                    ) : null
                }
                

            </div>
        )
    }

}


export default LoginForm;