import React from "react";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import Install from "./pages/install";
import Home from "./pages/home";
import Input from "./pages/select";

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
          <Route path="/input" element={<Input />}>
            <Route element={<Input />} />
          </Route>
        </>
      </Routes>
    </BrowserRouter>
  );
};

export default Router;
