import init, { WebRocket } from "../pkg/webrocket_client.js";

function createConnection () {

  const wr = new WebRocket();

  wr.on("connect", (socket) => {
    console.log("Connected!");

    socket.on("hello", (message) => {
      console.log(message);
    });

    socket.emit("howdy", "stranger");
  });

  wr.connect();

}

export const run = () => {
  init().then(createConnection);
}
