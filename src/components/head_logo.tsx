import React from "react";

const HeadLogo = () => {
  return (
    <center>
      <div className="grid grid-rows-1 grid-flow-col gap-4 justify-center sm:py-8 md:pt-12 lg:pt:16">
        <div className="row-span-1">
          <img
            className="sm:h-[90px] md:h-[120px] xl:h-[150px] flex items-center justify-center"
            src="/images/koompi-cotent-server-logo.png"
          />
        </div>
      </div>
    </center>
  );
};

export default HeadLogo;
