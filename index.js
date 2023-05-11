const request = require("superagent");
const logNetworkTime = require("superagent-node-http-timings");

// images
// https://www.galaxus.ch/im/Files/5/4/5/9/8/0/8/9/Master_4_Picture_PNG_schwarz_Solar_L.png?impolicy=ProductTileImage&resizeWidth=648&resizeHeight=486&cropWidth=648&cropHeight=486&resizeType=downsize&quality=high
// https://media.s-bol.com/BpyMDBY9kjLJ/g00KJD/550x335.jpg

https: for (let i = 0; i < 10; i++) {
  request
    .get("https://media.s-bol.com/BpyMDBY9kjLJ/g00KJD/550x335.jpg")
    .use(
      logNetworkTime((err, result) => {
        if (err) {
          return console.error(err);
        }
        console.log(result);
      })
    )
    .then((x) => x);
}
