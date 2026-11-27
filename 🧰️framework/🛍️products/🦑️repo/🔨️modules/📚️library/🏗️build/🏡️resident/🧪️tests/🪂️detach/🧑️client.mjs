import {spawn} from "node:child_process";
import {openSync} from "node:fs";
import {spawnDetachedV1} from "../../🏗️composition/🌐️host/🪂️detach/🟨️.mjs";
const[mode,log]=process.argv.slice(2),argv=[process.execPath,"-e","setInterval(()=>{},1000)"];
const pid=mode==="relay"?spawnDetachedV1({argv,cwd:process.cwd(),env:process.env,log}):(()=>{const fd=openSync(log,"a"),child=spawn(argv[0],argv.slice(1),{stdio:["ignore",fd,fd],detached:true});child.unref();return child.pid;})();
process.stdout.write(String(pid)+"\n");
setInterval(()=>{},1000);
