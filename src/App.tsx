import { useState } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import Sidebar from "./components/sidebar";
import Content from "./components/content";

function App() {
  let win = getCurrentWindow()

  return (
    <>
    <span className="topbar">
      <button>minimize</button>
      <button>maximize</button>
      <button>close</button>
    </span>
    <main className="container">
      <Sidebar/>
      <Content/>
    </main>
    </>

  );
}

export default App;
