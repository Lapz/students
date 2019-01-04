import React,{Component} from "react";
import { Tabs ,Tab} from "../common/Tabs";
import {Container} from  "../common/Grid";
export default class Dashboard extends Component {

    render() {
        return(
           <Container>
               <Tabs>
                   <Tab>Students</Tab>
                   <Tab>Teachers</Tab>
                   <Tab>Class</Tab>
               </Tabs>
           </Container>
        )
    }
}