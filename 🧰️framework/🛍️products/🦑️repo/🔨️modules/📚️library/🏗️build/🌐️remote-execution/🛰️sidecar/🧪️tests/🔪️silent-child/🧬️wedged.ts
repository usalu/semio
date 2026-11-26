import {appendFileSync} from "node:fs";
appendFileSync(process.env.F59_PIDS!,`${process.pid}\n`);
setInterval(()=>{},1000);
