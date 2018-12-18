import React,{Component} from "react";
import Section from "./Components/Section";
import Hero from "./Components/Hero";
import { Interface } from "readline";


export interface LoginProps {

}

export default class Login extends Component<LoginProps,{}> {
    constructor(props:LoginProps) {
        super(props)
    }
  
    render() {

        return (
            <Section className="hero is-success is-fullheight">
                <Hero className="hero-body">

                </Hero>

            </Section>
        )


    }
}