import {existsSync,readFileSync} from "node:fs";
import {join} from "node:path";
import {createHash} from "node:crypto";
import {runHostForegroundV1} from "../../🏗️composition/🌐️host/🟨️.mjs";
const[,,implementation,graphPath,policyJson]=process.argv,delay=Number(process.env.HOST_FIXTURE_LOAD_MS??0);
const file=root=>existsSync(join(root,"graph.json"))?join(root,"graph.json"):graphPath;
const code=await runHostForegroundV1({implementation,identify:async({root})=>createHash("sha256").update(readFileSync(file(root))).digest("hex"),policy:policyJson?JSON.parse(policyJson):undefined,loadGraph:async({root,signal})=>{if(delay)await new Promise((resolve,reject)=>{const t=setTimeout(resolve,delay);signal?.addEventListener("abort",()=>{clearTimeout(t);reject(signal.reason);},{once:true});});return JSON.parse(readFileSync(existsSync(join(root,"graph.json"))?join(root,"graph.json"):graphPath,"utf8"));},log:(event,detail)=>process.stderr.write("[DEBUG] "+event+" "+detail+"\n")});
process.exit(code);
