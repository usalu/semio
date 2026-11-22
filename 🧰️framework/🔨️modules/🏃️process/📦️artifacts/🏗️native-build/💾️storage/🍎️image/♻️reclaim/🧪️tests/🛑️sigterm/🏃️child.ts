import {execFile} from "node:child_process";
import {createNativeImageDarwinReclaimPortV1,reclaimNativeImageV1} from "../../🟦️.ts";

const [image,root,mode]=process.argv.slice(2) as [string,string,"canonical"|"scratch"],controller=new AbortController();
const exec=(program:string,args:readonly string[])=>new Promise<string>((resolve,reject)=>{execFile(program,[...args],{encoding:"utf8",maxBuffer:67108864},(error,stdout)=>error?reject(error):resolve(stdout));});
const marked=(program:string,args:readonly string[])=>{console.log("run "+(args[0]==="image"?"attach":args[0]));return exec(program,args);};
process.once("SIGTERM",()=>{controller.abort();setTimeout(()=>process.exit(143),20000).unref();});
const port=createNativeImageDarwinReclaimPortV1({image,cwd:"/",...mode==="canonical"?{mountPoint:root}:{},
 run:(program,args)=>controller.signal.aborted?Promise.reject(Error("cancelled")):marked(program,args),
 cleanupRun:(program,args)=>exec(program,args)});
const outcome=await reclaimNativeImageV1(port,{check:()=>{if(controller.signal.aborted)throw Error("cancelled");},progress:done=>console.log("progress "+done)}).then(()=>"completed",error=>String(error?.message??error).split("\n")[0]);
console.log("end "+outcome);
process.exit(controller.signal.aborted?143:0);
