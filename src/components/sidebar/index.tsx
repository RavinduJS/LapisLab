import { useState } from "react";
import logo from "../../assets/logo.svg"
import { invoke } from "@tauri-apps/api/core";
import "./index.css";

function Sidebar() {


  return (
    <div className="main">
      <span className="logo">
        <img src={logo}/>
        <p>LAPIS LAB</p>
      </span>
      
      <div>

      </div>
    </div>
  );
}

export default Sidebar;