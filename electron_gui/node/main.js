//
// Based heavily off of: https://github.com/anderejd/electron-wasm-rust-example
//
// With various tweaks to update to Electron 7.
//

let {app,  protocol,  BrowserWindow} = require("electron");
let {readFile} = require("fs");
let {extname} = require("path");
let {URL} = require("url");

let createProtocol = (scheme, normalize = true) => {
  protocol.registerBufferProtocol(scheme, (req, callback) => {
    let pathName = new URL(req.url).pathname;
    pathName = decodeURI(pathName);
    readFile(__dirname + "/" + pathName, (error, data) => {
      let extension = extname(pathName).toLowerCase();
      let mimeType = "";
      if (extension === ".js") {
        mimeType = "text/javascript";
      } else if (extension === ".html") {
        mimeType = "text/html";
      } else if (extension === ".css") {
        mimeType = "text/css";
      } else if (extension === ".svg" || extension === ".svgz") {
        mimeType = "image/svg+xml";
      } else if (extension === ".json") {
        mimeType = "application/json";
      } else if (extension === ".wasm") {
        mimeType = "application/wasm";
      }
      callback({
        mimeType,
        data
      });
    });
  });
}

protocol.registerSchemesAsPrivileged([{
  scheme: 'app',
  privileges: { standard: true, secure: true, supportFetchAPI: true },
}]);


app.on("ready", () => {
  createProtocol("app");
  let browserWindow = new BrowserWindow({
    webPreferences: {
      nodeIntegration: false,
      contextIsolation: true
    }
  });
  // browserWindow.webContents.openDevTools();
  browserWindow.loadFile("index.html");
});
