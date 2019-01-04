import React from "react";
import Box from "../common/Box";

const StudentView = (props) => {
    return (
       
            <Box>
                <article className="media">
                    <div className="media-content">
                        <div className="content">
                            
                            Name:<strong>{props.studentName}</strong><br />
                            Class:<strong>{props.studentClass}</strong><br />
                            Teacher:<strong>{props.studentTeacher}</strong><br />
                           
                        </div>
                    </div>
                </article>
            </Box>
        
    )
};

export {StudentView}