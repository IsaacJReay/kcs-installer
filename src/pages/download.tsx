import React, { useState, useEffect } from "react";
import axios from "axios";

const Download = () => {
  const [data, setData] = useState({name: ''});
  const [loading, setLoading] = useState(false);
  useEffect(() => {
    axios.get("https://pokeapi.co/api/v2/pokemon/ditto").then((res) => {
      setLoading(true);
      setData(res.data);
      setLoading(false);
    });
  }, []);
  console.log(data, "data");
  if (loading) return "loading...";
  else return (
    <div>
      <center>
        <img
          className="w-96 mt-12 flex items-center justify-center"
          src="/images/koompi-cotent-server-logo.png"
        />
      </center>
      <div>
        <h1>{data.name}</h1>
      </div>
    </div>
  );
};

export default Download;
