import React,{Component} from "react";


const Notification = (props) => {
    return (
        <div className={"notification " +props.kind+ props.hidden ? "is-invisible" : "" }>
            <button className="delete"></button>
            {
                props.children
            }

        </div>
    )
};


export default Notification;