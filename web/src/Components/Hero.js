import React,{Component} from "react";



const Hero = (props) => {
    return (
        <div className={props.className}>
            {props.children}
        </div>
    )
}

export default Hero;