import {existsSync} from "node:fs";
const files=process.argv.slice(2).filter(arg=>arg.startsWith("@")).map(arg=>arg.slice(1));
process.stdout.write(JSON.stringify(files.map(path=>({path,exists:existsSync(path)}))));
process.exitCode=7;
