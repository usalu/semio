// fixture process of the executor tests: serves the fd 3 protocol with a fake custody session and fake commands
import {createReadStream} from "node:fs";
import {serveExecutorV1,readLinesV1} from "../../../🏭️executor/🟨️.mjs";
import {writeSync} from "node:fs";

let opens=0;
const input=createReadStream(null,{fd:3,autoClose:false,encoding:"utf8"});
const result=await serveExecutorV1({
 lines:readLinesV1(input),
 write:text=>writeSync(3,text),
 limits:{maximumRequests:Number(process.env.FIXTURE_MAX??"1000"),maximumAgeMilliseconds:600000,idleMilliseconds:process.env.FIXTURE_IDLE?Number(process.env.FIXTURE_IDLE):undefined},
 openSession:async()=>{if(process.env.FIXTURE_CUSTODY_FAIL)throw new Error("image busy");opens++;let runs=0;return{run:async(body,{signal})=>{runs++;return body({runs,opens})},close:async()=>{}};},
 run:async(request,control,signal)=>{
  const[command,...rest]=request.argv;
  if(command==="echo"){console.log(JSON.stringify({rest,env:process.env.FIXTURE_VAR??null,leaked:process.env.FIXTURE_LEAK??null,cwd:process.cwd(),runs:control.runs,opens:control.opens}));console.error("err "+rest.join(" "));}
  else if(command==="set"){process.env.FIXTURE_LEAK="leaked";process.chdir("/");process.exitCode=4;console.log("set");}
  else if(command==="sleep"){await new Promise((resolve,reject)=>{const timer=setTimeout(resolve,Number(rest[0]));signal.addEventListener("abort",()=>{clearTimeout(timer);reject(signal.reason);},{once:true});});}
  else if(command==="refuse-key")throw Object.assign(new Error("refused build.nx.child-selection-undeclared {}"),{key:"build.nx.child-selection-undeclared"});
  else if(command==="refuse")throw new Error("Undeclared original Repo build selection");
  else if(command==="service-refuse")throw Object.assign(new Error("Service request refused (expired)"),{name:"ServiceRefusalV1",code:"expired",fields:{reason:"x"}});
  else if(command==="throw")throw new Error("boom");
  else if(command==="big"){console.log("x".repeat(Number(rest[0])));}
 },
});
process.exit(result.reason==="custody"?3:0);
