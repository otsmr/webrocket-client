import init, { WebRocket } from "../pkg/webrocket_client.js";

function createConnection () {

  const wr = new WebRocket();

  wr.on("connect", (socket) => {
    console.log("Connected!");

    socket.on("message", (message, callback) => {

      console.log("New message:", message.plaintext);

      callback({
        ok: {
          to: message.from,
          payload: JSON.stringify({
            id: message.id,
            to: message.from,
            from: message.to,
            kind: "status",
            status: "received"
          }),
        }
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
