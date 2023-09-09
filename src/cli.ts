import {Command} from "commander";
import * as pjson from "../package.json";

export let program = new Command();

program
    .version(pjson.version)
    .description("Cazan building tool");
