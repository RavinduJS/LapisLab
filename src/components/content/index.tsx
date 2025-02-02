import { useState } from "react";

import { invoke } from "@tauri-apps/api/core";
import "./index.css";

  
function Content() {

  
  return (
      <div className="content">
        <div className="serverInfo">
          <div className="info">
            <div className="section">
        
            </div>
            <div className="section">

            </div>
          </div>
          <div className="players">
            
          </div>
        </div>
        <div className="controls">
          <div>

          </div>
        </div>
      </div>
  );
}

export default Content;