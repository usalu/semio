/** 📥️ Worker script that the leases standing tests start as a child process. */
import {acquireQueuedResourceLease} from "../../../🟦️.ts";
import {registerLeaseImplementationProviderV1,setLeasePriorityV1,type LeasePriorityV1} from "../../🟦️.ts";

/** 👷️ One lease participant in its own process: it holds (and idles) or waits (and reports), as its configuration says. */
const config=JSON.parse(process.argv[2]!) as {directory:string;resource:string;role:"hold"|"wait";generation:string;priority?:LeasePriorityV1;holdMilliseconds?:number};
registerLeaseImplementationProviderV1({components:()=>({"graph.generation":config.generation}),bound:resource=>resource.startsWith("graph:")?["graph.generation"]:[]});
if(config.priority)setLeasePriorityV1(config.priority);
const emit=(row:Record<string,unknown>):void=>{process.stdout.write(JSON.stringify(row)+"\n");};
const lease=await acquireQueuedResourceLease({directory:config.directory,resource:config.resource,mode:"exclusive",signal:new AbortController().signal,owner:"w"+process.pid});
emit({event:"acquired",pid:process.pid,at:Date.now()});
if(config.role==="hold")await new Promise<void>(()=>setTimeout(()=>undefined,600000));
else{await new Promise(done=>setTimeout(done,config.holdMilliseconds??0));lease.release();emit({event:"released"});}
process.exit(0);
