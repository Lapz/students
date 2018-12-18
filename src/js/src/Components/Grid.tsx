import React,{Component, ReactNode} from "react";



export interface ColumnProps {
    className: string,
    children:ReactNode
}


export interface ColumnsProps {
    children:ReactNode
}

const Columns = (props:ColumnsProps) => {
    <div className="columns">
        {props.children}
    </div>
}


const Column = (props:ColumnProps) => {
    <div className={"cloumn "+props.className}> 
        {
            props.children
        }
    </div>
}

export {Columns,Column};


