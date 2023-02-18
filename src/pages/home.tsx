import React from "react";
import HeadLogo from "../components/head_logo";
import { Link } from "react-router-dom";
import { invoke } from "@tauri-apps/api/tauri";

const Home = () => {
  return (
    <div>
      <HeadLogo />
      <center>
        <div className="grid grid-rows-1 grid-flow-col gap-4 justify-center sm:pt-20 md:pt-24 lg:pt-30 w-full">
          <Link to="/input">
            <button
              className="row-span-1 w-full bg-transparent hover:bg-blue-900 text-blue-900 font-semibold hover:text-white sm:py-4 sm:px-8 md:py-6 md:px-14 md:text-2xl lg:py-8 lg:px-16 border border-blue-900 hover:border-transparent rounded-lg"
            >
              START
            </button>
          </Link>
        </div>
      </center>
      <div className="absolute bottom-0 left-0 sm:pb-6 sm:px-10 md:px-16 md:pb-10 ">
        <p className="text-slate-600 sm:text-xs md:text-base">
          KOOMPI © 2020 All Rights Reserved.
        </p>
      </div>
      <div className="absolute bottom-0 right-0 sm:pb-6 sm:px-10 md:px-16 md:pb-10 ">
        <a className="text-sky-600 sm:text-xs md:text-base" href="">
          Help
        </a>
      </div>
    </div>
  );
};

export default Home;
