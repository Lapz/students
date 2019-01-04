import React,{Component} from "react";
import Section from "../common/Section";
import Hero from "../common/Hero";
import {Column} from "../common/Grid";
import Box from "../common/Box";
import LoginForm from "./LoginForm";
import {withRouter} from "react-router"

export default class Login extends Component {  
    render() {

        return (
            <Section className="hero is-fullheight">
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