/** 🏷️ The error of a slice worker keeps what names the refused place across the process boundary: the old message carried only text and stack, so the service log lost `root` and `entry` of an `EPERM` and could not print its `where:` line. Judged with a real child process over the real IPC channel. */
import {afterAll,expect,test} from "bun:test";
import {mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {pathToFileURL} from "node:url";
import {createProcessWorkerV1,errorMessageV1,errorOfMessageV1} from "../../🏗️composition/🌐️host/👷️slice-pool/🧵️process/🟨️.mjs";
import {deniedPlaceOfV1} from "../../🏛️service/🧯️failure/🟨️.mjs";

const base=mkdtempSync(join(tmpdir(),"worker-error-"));
afterAll(()=>rmSync(base,{recursive:true,force:true}));
const processModule=pathToFileURL(join(import.meta.dir,"..","..","🏗️composition","🌐️host","👷️slice-pool","🧵️process","🟨️.mjs")).href;

const runChild=(body:string)=>new Promise<any>((resolve,reject)=>{
  const file=join(base,"child-"+Math.random().toString(16).slice(2)+".mjs");
  writeFileSync(file,`import {errorMessageV1,parentPort} from ${JSON.stringify(processModule)};\n${body}\n`);
  const worker=createProcessWorkerV1(pathToFileURL(file),{},{});
  worker.on("message",message=>resolve(message));worker.on("error",reject);worker.on("exit",code=>{if(code!==0)reject(Error("child exit "+code));});
});

test("a real child process that fails with a refused place hands root, entry, code and errno to the service; the old text-and-stack message loses them",async()=>{
  const refused=`const error=Object.assign(Error("Relative filesystem fstatat failed (EPERM)"),{code:"EPERM",errno:-1,root:"/Users/w/Documents/semio",entry:"inner"});`;
  const modern=await runChild(refused+`parentPort.postMessage({type:"error",...errorMessageV1(error)});`);
  const arrived=errorOfMessageV1(modern);
  expect(arrived.message).toBe("Relative filesystem fstatat failed (EPERM)");
  expect(arrived).toMatchObject({code:"EPERM",errno:-1,root:"/Users/w/Documents/semio",entry:"inner"});
  expect(deniedPlaceOfV1(arrived)).toBe("root=/Users/w/Documents/semio entry=inner code=EPERM");
  const legacy=await runChild(refused+`parentPort.postMessage({type:"error",message:String(error.message),stack:String(error.stack)});`);
  expect(deniedPlaceOfV1(Object.assign(new Error(legacy.message),{stack:legacy.stack})),"without the data the place is gone").toBeUndefined();
});

test("a refusal of the graph keeps its name and values and wins over a plain code, text without data stays an error without extras",()=>{
  const refusal=Object.assign(new Error("unserved build.graph.wanted-expired {}"),{name:"RepoError",code:"unserved",key:"build.graph.wanted-expired",values:{wanted:3}});
  const arrived=errorOfMessageV1({type:"error",...errorMessageV1(refusal)});
  expect(arrived).toMatchObject({name:"RepoError",code:"unserved",key:"build.graph.wanted-expired",values:{wanted:3}});
  const plain=errorOfMessageV1({type:"error",message:"boom",stack:"Error: boom\n    at x"});
  expect(plain.message).toBe("boom");expect(plain.stack).toBe("Error: boom\n    at x");expect(plain.root).toBeUndefined();expect(plain.code).toBeUndefined();
  expect(errorMessageV1("text")).toEqual({message:"text",stack:""});
  expect(errorMessageV1(Object.assign(new Error("x"),{root:7,entry:{},code:"E",errno:1.5})).data).toEqual({code:"E"});
});
