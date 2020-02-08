//
// Based heavily off of: https://github.com/anderejd/electron-wasm-rust-example
//
// With various tweaks to update to Electron 7.
//

let {app, protocol, BrowserWindow, Tray} = require("electron");
let {readFile} = require("fs");
let {extname} = require("path");
let {URL} = require("url");
let path = require("path");

let tray = undefined
let hoverWindow = undefined

let createProtocol = (scheme, normalize = true) => {
  protocol.registerBufferProtocol(scheme, (req, callback) => {
    let pathName = new URL(req.url).pathname;
    pathName = decodeURI(pathName);
    // Setting the path relative to the root of electron_gui
    let filePath = path.join(__dirname, "..", pathName);
    readFile(filePath, (error, data) => {
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
  scheme: "app",
  privileges: { standard: true, secure: true, supportFetchAPI: true },
}]);


app.on("ready", () => {
  createProtocol("app");

  hoverWindow = new BrowserWindow({
    width: 700,
    height: 400,
    // show: false,
    frame: false,
    fullscreenable: false,
    resizable: false,
    webPreferences: {
      nodeIntegration: false,
      contextIsolation: true
    }
  });

  // TODO: auto launch this at startup in dev?
  hoverWindow.openDevTools({mode: 'detach'})

  let hover_html = path.join(__dirname, "..", "browser", "hover.html");
  hoverWindow.loadURL(`file://${hover_html}`)

  // Hide the window when it loses focus
  hoverWindow.on("blur", () => {
    if (!hoverWindow.webContents.isDevToolsOpened()) {
      hoverWindow.hide()
    }
  })

  createTray();
});

const createTray = () => {
  tray = new Tray(`${__dirname}/../assets/tmp_icon.png`)
  tray.on('right-click', showHoverWindow)
  tray.on('double-click', showHoverWindow)
  tray.on('click', function (event) {
    showHoverWindow()

    // Show devtools when command clicked
    if (hoverWindow.isVisible() && process.defaultApp && event.metaKey) {
      hoverWindow.openDevTools({mode: 'detach'})
    }
  })
}

const toggleHoverWindow = () => {
  if (hoverWindow.isVisible()) {
    hoverWindow.hide()
  } else {
    showWindow()
  }
}

const showHoverWindow = () => {
  hoverWindow.show()
  hoverWindow.focus()
}
