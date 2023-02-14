import React, { useState, useEffect, Fragment } from "react";
import { invoke } from '@tauri-apps/api/tauri'

interface InstallStatus {
  progress: number,
  status_list: statusListItem[],
}

interface statusListItem {
  name: string,
  status: string
}

const Install = () => {
  const [data, setData] = useState({
    progress: 0,
    status_list: [{
      name: "",
      status: ""
    }]
  });

  const fetchData = async () => {
    invoke("get_install_status",)
      .then((res) => {
        const res1 = res as InstallStatus;
        setData(res1)
      })
      .catch((e) => console.log(e))
    console.log("getting data");
  }

  useEffect(() => {
    // setInterval(() => {
      fetchData();
    // }, 500)
  }, []);

  const arr: statusListItem[] = data.status_list;
  console.log("data", arr);


  return (
    <div>
      <center>
        <div className="grid grid-rows-1 grid-flow-col gap-4 justify-center sm:py-10 md:pb-16 md:pt-12 lg:pb-18 lg:pt:16">
          <div className="row-span-1">
            <img
              className="sm:h-[90px] md:h-[115px] xl:h-[130px] flex items-center justify-center"
              src="/images/koompi-cotent-server-logo.png"
            />
          </div>
        </div>
      </center>
      <div className="relative container flex justify-center sm:px-20 md:px-36 overflow-auto sm:m-h-2/5 md:m-h-96">
        <div className="grid grid-flow-row-dense grid-cols-10 justify-start w-full">
          {arr !== undefined && arr.map((each) => {
            if (each.status === "done") {
              return (
                <Fragment>
                  <svg aria-hidden="true" className="col-span-1 w-5 h-5  text-green-500 dark:text-green-400 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                    <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"></path>
                  </svg>
                  <h1 className="col-span-9 text-dark-600 h-8 truncate">
                    {each.name}
                  </h1>
                </Fragment>
              )
            }
            else if (each.status === "working") {
              return (
                <Fragment>
                  <svg aria-hidden="true" className="col-span-1 w-6 h-6 pl-2 text-gray-200 animate-spin dark:text-gray-600 fill-blue-600" viewBox="0 0 100 101" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z" fill="currentColor"></path>
                    <path d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z" fill="currentFill"></path>
                  </svg>
                  <span className="sr-only">Loading...</span>
                  <h1 className="col-span-9 text-dark-600 h-8 truncate">
                    {each.name}
                  </h1>
                </Fragment>
              )
            }
            else if (each.status === "pending") {
              return (
                <Fragment>
                  <h1></h1>
                  <h1 className="col-span-9 text-grey-600 h-8 truncate">
                    {each.name}
                  </h1>
                </Fragment>
              )
            }
          })}          
        </div>
      </div>
      <div className="absolute bottom-0 inset-x-0 sm:pb-10 sm:px-20 md:px-36 md:pb-16" id="progress_bar">
        <div className="flex justify-between mb-1">
          <span className="text-base font-medium text-dark-700">Progress</span>
          <span className="text-sm font-medium text-dark-700">
            {data.progress}%
          </span>
        </div>
        <div className="w-full bg-gray-200 rounded-full h-5 dark:bg-gray-700">
          <div className="bg-blue-600 h-5 rounded-full" style={{ width: data.progress + "%" }}></div>
        </div>
      </div>
    </div>
  );
};

export default Install;
