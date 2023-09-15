import {Command} from "commander";
import * as pjson from "../package.json";
import * as commands from "./commands";

export let program = new Command();

program
    .version(pjson.version)
    .description("Cazan building tool");

program
    .command("init")
    .description("Initialize a new project")
    .action(commands.init);
