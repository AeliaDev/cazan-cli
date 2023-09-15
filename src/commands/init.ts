import * as path from "path";
import * as fs from "fs";

export function init() {
    // Create an assets directory with a sprites directory inside
    let assetsDir = path.join(process.cwd(), "assets");
    let spritesDir = path.join(assetsDir, "sprites");

    fs.mkdirSync(assetsDir);
    fs.mkdirSync(spritesDir);

    console.log("Initialized a new Cazan project");
}