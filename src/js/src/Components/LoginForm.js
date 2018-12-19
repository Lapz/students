import React,{Component} from "react";
import Notification from "./Notification";

class LoginForm extends Component {
    constructor(props) {
        super(props);

        this.state = {
            username:"",
            password:"",
            failed:false
        };


        this.handleChange = this.handleChange.bind(this);
        this.handleClick = this.handleClick.bind(this);
        this.handleErrorNotification = this.handleErrorNotification.bind(this);

    }

    handleChange(event) {
      this.setState({
          [event.target.name]:event.target.value
      });

    }

    handleErrorNotification(event) {
        this.setState({
            failed:false
        });
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
                const response = JSON.stringify(data);

                if (response.status == 200) {
                    this.setState({
                        username:"",
                        password:"",
                        failed:false,
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
            console.log(err);
            this.setState({
                username:"",
                password:""
            })
        })
    }

    render() {
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

                
                <Notification hidden={this.state.failed} onClick={this.handleErrorNotification}>
                    You could not login
                </Notification>
                

            </div>
        )
    }

}


export default LoginForm;