import React from "react";
import { Link } from "react-router-dom";

const Home = () => {
  return (
    <div>
      <center>
        <div className="grid grid-rows-1 grid-flow-col gap-4 justify-center sm:py-10 md:pb-16 md:pt-12 lg:pb-18 lg:pt:16 xl:pb-20 xl:pt-20 ">
          <div className="row-span-1">
            <img
              className="sm:h-[90px] md:h-[120px] xl:h-[150px] flex items-center justify-center"
              src="/images/koompi-cotent-server-logo.png"
            />
          </div>
        </div>
      </center>
      <center>
        <div className="grid grid-rows-1 grid-flow-col gap-4 justify-center sm:pt-24 md:pt-30 lg:pt-40 w-full">
          <Link to="/download">
            <button id="start-button"
              className="row-span-1 w-full bg-transparent hover:bg-blue-900 text-blue-900 font-semibold hover:text-white sm:py-4 sm:px-8 md:py-6 md:px-14 md:text-2xl lg:py-8 lg:px-16 border border-blue-900 hover:border-transparent rounded-lg">
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
