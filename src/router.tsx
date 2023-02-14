import React from "react";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import Download from "./pages/download";
import Home from "./pages/home";

const Router = () => {
  return (
    <BrowserRouter>
      <Routes>
        <>
          <Route path="/" element={<Home />}>
            <Route element={<Home />} />
          </Route>
          <Route path="/download" element={<Download />}>
            <Route element={<Download />} />
          </Route>
        </>
      </Routes>
    </BrowserRouter>
  );
};

export default Router;
