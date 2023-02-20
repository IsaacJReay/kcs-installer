import React, { useState, useEffect, Fragment } from "react";
import HeadLogo from "../components/head_logo";
import { invoke } from "@tauri-apps/api/tauri";

interface InstallStatus {
  progress: number;
  status_list: statusListItem[];
}

interface statusListItem {
  name: string;
  status: string;
}

const Install = () => {
  const [data, setData] = useState({
    progress: 0,
    status_list: [
      {
        name: "",
        status: "",
      },
    ],
  });

  const [final, setFinal] = useState(false);

  const fetchData = async () => {
    invoke("get_install_status")
      .then((res) => {
        const res1 = res as InstallStatus;
        setData(res1);
      })
      .catch((e) => console.log(e));
  };

  useEffect(() => {
    setInterval(() => {
      fetchData();
    }, 500);
  }, []);

  useEffect(() => {
    if (data.progress >= 100) {
      setFinal(true);
    }
  });

  const handleClick = () => {
    invoke("reboot");
  };

  return (
    <div>
      <HeadLogo />
      <div className="relative container flex justify-center sm:pt-1 md:pt-2 lg:pt-6 sm:px-20 md:px-36 sm:h-44 md:h-52 lg:h-64 overflow-auto">
        <div className="grid grid-flow-row-dense grid-cols-10 justify-start w-full">
          {data.status_list.map((each) => {
            if (each.status === "done") {
              return (
                <Fragment>
                  <svg
                    aria-hidden="true"
                    className="col-span-1 w-5 h-5  text-green-500 dark:text-green-400 flex-shrink-0"
                    fill="currentColor"
                    viewBox="0 0 20 20"
                    xmlns="http://www.w3.org/2000/svg"
                  >
                    <path
                      fill-rule="evenodd"
                      d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
                      clip-rule="evenodd"
                    ></path>
                  </svg>
                  <h1 className="col-span-9 text-dark-600 h-8 truncate">
                    {each.name}
                  </h1>
                </Fragment>
              );
            } else if (each.status === "working") {
              return (
                <Fragment>
                  <svg
                    aria-hidden="true"
                    className="col-span-1 w-6 h-6 pl-2 text-gray-200 animate-spin dark:text-gray-600 fill-blue-600"
                    viewBox="0 0 100 101"
                    fill="none"
                    xmlns="http://www.w3.org/2000/svg"
                  >
                    <path
                      d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z"
                      fill="currentColor"
                    ></path>
                    <path
                      d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z"
                      fill="currentFill"
                    ></path>
                  </svg>
                  <span className="sr-only">Loading...</span>
                  <h1 className="col-span-9 text-dark-600 h-8 truncate">
                    {each.name}
                  </h1>
                </Fragment>
              );
            } else if (each.status === "pending") {
              return (
                <Fragment>
                  <h1></h1>
                  <h1 className="col-span-9 text-slate-600 h-8 truncate">
                    {each.name}
                  </h1>
                </Fragment>
              );
            }
          })}
        </div>
      </div>
      {!final ? (
        <Fragment>
          <div
            className="absolute bottom-0 inset-x-0 sm:pb-4 sm:px-20 md:px-36 md:pb-12"
            id="progress_bar"
          >
            <div className="flex justify-between mb-1">
              <span className="text-base font-medium text-dark-700">
                Progress
              </span>
              <span className="text-sm font-medium text-dark-700">
                {data.progress}%
              </span>
            </div>
            <div className="w-full bg-gray-200 rounded-full h-5 dark:bg-gray-700">
              <div
                className="bg-blue-600 h-5 rounded-full"
                style={{ width: data.progress + "%" }}
              ></div>
            </div>
          </div>
        </Fragment>
      ) : (
        <Fragment>
          <div className="absolute bottom-0 inset-x-0 sm:pb-4 sm:px-20 md:px-36 md:pb-8">
            <div className="grid grid-rows-1 grid-flow-col gap-4 justify-center sm:pt-20 md:pt-24 lg:pt-30 w-full">
              <button
                onClick={handleClick}
                className="row-span-1 w-full bg-transparent hover:bg-blue-900 text-blue-900 font-semibold hover:text-white sm:py-1 sm:px-8 md:py-3 md:px-14 md:text-2xl lg:py-8 lg:px-16 border border-blue-900 hover:border-transparent rounded-lg"
              >
                REBOOT
              </button>
            </div>
          </div>
        </Fragment>
      )}
    </div>
  );
};

export default Install;
