import React,{Component} from "react";
import Section from "./Components/Section";
import Hero from "./Components/Hero";
import {Column} from "./Components/Grid";
import Box from "./Components/Box";
import LoginForm from "./Components/LoginForm"

export default class Login extends Component {
    constructor(props) {
        super(props)
    }
  
    render() {

        return (
            <Section className="hero is-success is-fullheight">
                <Hero className="hero-body">

                <div className="container has-text-centered">
                    <Column className="is-4 is-offset-4">

                    <h3 className="title has-text-grey">Login</h3>
                    <p className="subtitle has-text-grey">Please login</p>

                    <Box>
                        <LoginForm />
                        

                    </Box>
                    

                    </Column>
                
                
                </div> 

                </Hero>

            </Section>
        )


    }
}