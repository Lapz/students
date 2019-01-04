import React from "react";

const Notification = (props) => {
    const {kind,hidden,handleClose} = props;
    return (
        <div className= {`notification ${kind} ${(hidden ? "is-invisible" : "")}`}>
            <button className="delete" onClick={handleClose}></button>
            {
                props.children
            }

        </div>
    )
};


export default Notification;