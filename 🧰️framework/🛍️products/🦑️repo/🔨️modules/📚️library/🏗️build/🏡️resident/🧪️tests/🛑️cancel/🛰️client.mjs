import {requestPlanV1} from "../../🚪️client/🛰️plan/🟨️.mjs";
const[,,root,implementation]=process.argv,controller=new AbortController();
process.on("SIGTERM",()=>controller.abort(new Error("terminated")));
process.on("SIGHUP",()=>controller.abort(new Error("hangup")));
requestPlanV1({root,implementation,targets:["app:test"],agent:"client",language:"en",signal:controller.signal,onProgress:()=>{process.stdout.write("progress\n");}}).then(()=>process.exit(0),()=>process.exit(143));
