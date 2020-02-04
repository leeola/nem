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
let tratWindow = undefined

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
  scheme: "app",
  privileges: { standard: true, secure: true, supportFetchAPI: true },
}]);


app.on("ready", () => {
  createProtocol("app");

  // let browserWindow = new BrowserWindow({
  //   webPreferences: {
  //     nodeIntegration: false,
  //     contextIsolation: true
  //   }
  // });
  // // browserWindow.webContents.openDevTools();
  // browserWindow.loadFile("index.html");

  createTray();
  createTrayWindow();
});

const createTray = () => {
  tray = new Tray(`${__dirname}/../assets/tmp_icon.png`)
  tray.on('right-click', toggleTrayWindow)
  tray.on('double-click', toggleTrayWindow)
  tray.on('click', function (event) {
    toggleTrayWindow()

    // Show devtools when command clicked
    if (trayWindow.isVisible() && process.defaultApp && event.metaKey) {
      //trayWindow.openDevTools({mode: 'detach'})
    }
  })
}

const createTrayWindow = () => {
  trayWindow = new BrowserWindow({
    width: 300,
    height: 450,
    show: false,
    frame: false,
    fullscreenable: false,
    resizable: false,
    //transparent: true,
    webPreferences: {
      // Prevents renderer process code from not running when window is
      // hidden
      backgroundThrottling: false
    }
  })
  let index_path = path.join(__dirname, "..", "browser", "tray_index.html");
  trayWindow.loadURL(`file://${index_path}`)
  // Hide the window when it loses focus
  trayWindow.on("blur", () => {
    if (!trayWindow.webContents.isDevToolsOpened()) {
      trayWindow.hide()
    }
  })
}

const toggleTrayWindow = () => {
  if (trayWindow.isVisible()) {
    trayWindow.hide()
  } else {
    showWindow()
  }
}

const showWindow = () => {
  const position = getWindowPosition()
  trayWindow.setPosition(position.x, position.y, false)
  trayWindow.show()
  trayWindow.focus()
}

const getWindowPosition = () => {
  const windowBounds = trayWindow.getBounds()
  const trayBounds = tray.getBounds()

  // Center window horizontally below the tray icon
  const x = Math.round(trayBounds.x + (trayBounds.width / 2) - (windowBounds.width / 2))

  // Position window 4 pixels vertically below the tray icon
  const y = Math.round(trayBounds.y + trayBounds.height + 4)

  return {x: x, y: y}
}
