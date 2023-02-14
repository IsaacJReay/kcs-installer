import React from "react";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import Install from "./pages/install";
import Home from "./pages/home";

const Router = () => {
  return (
    <BrowserRouter>
      <Routes>
        <>
          <Route path="/" element={<Home />}>
            <Route element={<Home />} />
          </Route>
          <Route path="/install" element={<Install />}>
            <Route element={<Install />} />
          </Route>
        </>
      </Routes>
    </BrowserRouter>
  );
};

export default Router;
