import React,{Component, ReactNode} from "react";



export interface PageProps {
    className: string,
    children:ReactNode
}

const Section = (props:PageProps) => {
    <div className={props.className}>
        {props.children}
    </div>
}

export default Section;