import React,{Component, ReactNode} from "react";



export interface PageProps {
    children:ReactNode
}

export const Page = (props:PageProps) => {
    <div>
        {props.children}
    </div>
}