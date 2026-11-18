import {createHash} from "node:crypto";
import {runHostForegroundV1} from "../../🏗️composition/🌐️host/🟨️.mjs";
const[,,implementation]=process.argv;
const code=await runHostForegroundV1({implementation,identify:async({root})=>createHash("sha256").update(root).digest("hex"),policy:{residentCheckMilliseconds:50},loadGraph:async()=>{const error=new TypeError("fixture graph is not iterable "+"x".repeat(Number(process.env.SEMIO_HOST_FIXTURE_MESSAGE_PAD??0)));error.code="E_FIXTURE";throw error;},log:(event,detail)=>process.stderr.write("[DEBUG] "+event+" "+detail+"\n")});
process.exit(code);
