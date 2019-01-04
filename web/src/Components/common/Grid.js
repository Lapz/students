import React from "react";




const Columns = (props) => {
    return (
        <div className="columns">
            {props.children}
        </div>
    )
}


const Column = (props) => {
    return (
        <div className={"column "+props.className}> 
            {
                props.children
            }
        </div>
    )
}

export {Columns,Column};


