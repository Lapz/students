import React,{Component, ReactNode} from "react";



export interface HeroProps {
    className: string,
    children:ReactNode
}

const Hero = (props:HeroProps) => {
    <div className={props.className}>
        {props.children}
    </div>
}

export default Hero;