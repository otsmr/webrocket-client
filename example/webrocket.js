import init, { WebRocket } from "../pkg/webrocket_client.js";

function createConnection () {

  const wr = new WebRocket();

  wr.on("connect", (socket) => {
    console.log("Connected!");

    socket.on("message", (message, callback) => {

      console.log("New message:", message);

      callback({
        ok: "Hello Back"
      });

    })
  });

  wr.on("error", (error) => {
    consoe.log("ERROR!", error);
  });

  wr.connect();

}

export const run = () => {
  init().then(createConnection);
}
