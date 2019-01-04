import React from "react";



const Tabs = (props) => {
    return (
        <div className="tabs">
            <ul>
                {props.children}
            </ul>
        </div>
    )
};


const Tab = (props) => {
    return (
        <li><a>{props.children}</a></li>
    )
};


export {Tabs,Tab}