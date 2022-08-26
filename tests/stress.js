import http from "k6/http";

export const options = {
  stages: [
    {
      duration: "2m",
      target: 100,
    },
    {
      duration: "5m",
      target: 100,
    },
    {
      duration: "2m",
      target: 400,
    },
    {
      duration: "2m",
      target: 400,
    },
  ],
};

export default function () {
  let response = http.get("https://blog-rust.herokuapp.com/");
}
