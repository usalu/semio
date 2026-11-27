import {writeFileSync} from "node:fs";
import {startScriptProcessOwnerWatchV1} from "../../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🐕️watchdog/🟦️.ts";
const [watched,lost]=process.argv.slice(2);
writeFileSync(watched,String(process.env.SEMIO_SCRIPT_PROCESS_OWNER??""));
startScriptProcessOwnerWatchV1({onLost:owner=>{writeFileSync(lost,String(owner));process.exit(0);}});
setInterval(()=>{},100);
