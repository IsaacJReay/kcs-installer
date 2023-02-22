import React, { useState, useEffect, Fragment } from "react";
import HeadLogo from "../components/head_logo";
import { invoke } from "@tauri-apps/api/tauri";
import { Link, useNavigate } from "react-router-dom";

interface DisksInfo {
  name: string;
  info: string;
}

interface EventTargetInt {
  target: EventTargetValue;
}

interface EventTargetValue {
  value: React.SetStateAction<string>;
}

interface DiskandIPArgs {
  selected_disk: string;
  selected_content_disk: string;
  master_ip: string;
}

const Input = () => {
  const default_value: string = "";

  const [MainStorage, setMainStorage] = useState(default_value);
  const handleMainStorageChange = (event: EventTargetInt) => {
    setMainStorage(event.target.value);
  };

  const [ContentStorage, setContentStorage] = useState(default_value);
  const handleContentStorageChange = (event: EventTargetInt) => {
    setContentStorage(event.target.value);
  };

  const [MainStorageMissing, setMainStorageMissing] = useState(false);
  const [ContentStorageMissing, setContentStorageMissing] = useState(false);
  const [IPAddrMissing, setIPAddrMissing] = useState(false);

  // const [continueStep, setcontinueStep] = useState(false);

  const [data, setData] = useState([{ name: "", info: "" }]);

  const fetchData = async () => {
    invoke("get_disks")
      .then((res) => {
        const res1 = res as DisksInfo[];
        setData(res1);
      })
      .catch((e) => console.log(e));
  };

  useEffect(() => {
    fetchData();
  }, []);

  const handleSubmit = (event: { preventDefault: () => void }) => {
    event.preventDefault();
    if (MainStorage === default_value) {
      setMainStorageMissing(true);
    } else if (ContentStorage === default_value) {
      setContentStorageMissing(true);
    } else {
      invoke("set_disk_and_ip", {
        args: {
          selected_disk: MainStorage,
          selected_content_disk: ContentStorage,
        },
      });
      if (import.meta.env.PROD) {
        invoke("start_installation");
      }
      window.eval("window.location.replace('/install')");
    }
  };

  return (
    <div>
      <HeadLogo />
      <center>
        <form
          onSubmit={handleSubmit}
          className="w-full max-w-lg sm:pt-1 md:pt-4 lg:pt-10"
        >
          <div className="flex flex-wrap -mx-3 mb-6">
            <div className="w-full px-3">
              <label
                className="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2"
                htmlFor="grid-password"
              >
                Main Storage Device
              </label>
              <div className="relative">
                <select
                  value={MainStorage}
                  onChange={handleMainStorageChange}
                  className="block appearance-none w-full bg-gray-100 border border-gray-200 text-gray-700 py-3 px-4 pr-8 rounded leading-tight focus:outline-none focus:bg-white focus:border-gray-500"
                  id="grid-state"
                >
                  <option value="">Select Storage</option>
                  {data.map((option_data) => {
                    return (
                      <option value={option_data.name}>
                        {option_data.info}
                      </option>
                    );
                  })}
                </select>
                <div className="pointer-events-none absolute inset-y-0 right-0 flex items-center px-2 text-gray-700">
                  <svg
                    className="fill-current h-4 w-4"
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 20 20"
                  >
                    <path d="M9.293 12.95l.707.707L15.657 8l-1.414-1.414L10 10.828 5.757 6.586 4.343 8z" />
                  </svg>
                </div>
              </div>
              {MainStorageMissing && (
                <p className="text-red-600 text-xs italic">
                  Main Storage hasn't been set yet
                </p>
              )}
            </div>
          </div>
          <div className="flex flex-wrap -mx-3 mb-6">
            <div className="w-full px-3">
              <label
                className="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2"
                htmlFor="grid-password"
              >
                Content Storage Device
              </label>
              <div className="relative">
                <select
                  value={ContentStorage}
                  onChange={handleContentStorageChange}
                  className="block appearance-none w-full bg-gray-100 border border-gray-200 text-gray-700 py-3 px-4 pr-8 rounded leading-tight focus:outline-none focus:bg-white focus:border-gray-500"
                  id="grid-state"
                >
                  <option value="">Select Storage</option>
                  {data.map((option_data) => {
                    return (
                      <option value={option_data.name}>
                        {option_data.info}
                      </option>
                    );
                  })}
                </select>
                <div className="pointer-events-none absolute inset-y-0 right-0 flex items-center px-2 text-gray-700">
                  <svg
                    className="fill-current h-4 w-4"
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 20 20"
                  >
                    <path d="M9.293 12.95l.707.707L15.657 8l-1.414-1.414L10 10.828 5.757 6.586 4.343 8z" />
                  </svg>
                </div>
              </div>
              {ContentStorageMissing && (
                <p className="text-red-600 text-xs italic">
                  Content Storage hasn't been set yet
                </p>
              )}
            </div>
          </div>
          <div className="flex flex-wrap -mx-3 mb-6 sm:pt-4 md:pt-10 xl:pt-56">
            <div className="w-full md:w-1/2 px-3 mb-6 md:mb-0">
              <Link to="/">
                <button className="appearance-none block w-full bg-red-600 text-white border border border-slate-300 rounded py-3 px-4 mb-3 leading-tight focus:outline-none focus:border-slate-600 focus:bg-red-300">
                  Back
                </button>
              </Link>
            </div>
            <div className="w-full md:w-1/2 px-3 mb-6 md:mb-0">
              <button
                type="submit"
                className="appearance-none block w-full bg-blue-600 text-white border border-slate-300 rounded py-3 px-4 mb-3 leading-tight focus:outline-none focus:border-slate-600 focus:bg-blue-300"
              >
                Apply
              </button>
            </div>
          </div>
        </form>
      </center>
    </div>
  );
};

export default Input;
