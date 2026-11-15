import {runRemoteExecutionSidecarLauncherV1} from "../../../../📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🟦️.ts";
import {join} from "node:path";

if(process.env.SEMIO_SIDECAR_CHILD==="1"){
 const {appendFileSync,readFileSync,existsSync}=await import("node:fs"),log=join(process.env.SIDECAR_TEST_DIR!,"starts.log"),before=existsSync(log)?readFileSync(log,"utf8").split("\n").filter(Boolean).length:0;
 appendFileSync(log,`${before} ${process.pid}\n`);
 if(before===0)setTimeout(()=>process.exit(75),300);
 setInterval(()=>{},1<<30);
}else await runRemoteExecutionSidecarLauncherV1("agent",import.meta.path,process.env);
