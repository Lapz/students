import React,{Component} from "react";
import { StudentView } from "./StudentView";
import { Container } from "../common/Grid";


const students = [
    {
        "name":"John",
        "teacher":"Dan",
        "class":"Cave",
        
       

       
    }, {
        "name":"James",
        "teacher":"Dan",
        "class":"Cave",
    }, {
        "name":"Henry",
        "teacher":"Matthew",
        "class":"Green"
    }
];


export default class Students extends Component {
    render() {

        
        const studentsView = students.map((student,index) => {
            return(
               <li key={index}>
                    <StudentView  id={index} studentName={student.name}  studentClass= {student.class} studentTeacher={student.teacher}/>
               </li>
            )
        });

        return(
             <Container>
                 <ul>
                    {studentsView}
                 </ul>
             </Container>
        )
    }
}