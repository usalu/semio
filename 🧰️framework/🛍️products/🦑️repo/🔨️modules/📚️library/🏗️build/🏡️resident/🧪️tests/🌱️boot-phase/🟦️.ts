/** 🌱️ Contract: while the client waits for a starting graph service it names the phase the service reports in its log (the files it walked included), in both languages, and falls back to the plain wait text while the service has said none.
 * @see ../../🚪️client/🛰️plan/🟨️.mjs */
import {expect,test} from "bun:test";
import {mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {basename,join} from "node:path";
import {connectHostV1} from "../../🚪️client/🛰️plan/🟨️.mjs";

const platform=process.platform==="win32"?"win32":"linux",base=platform==="win32"?tmpdir():"/tmp";
const privateEnvironment=(directory:string):Record<string,string>=>platform==="win32"?{LOCALAPPDATA:directory}:{XDG_RUNTIME_DIR:directory};

const waitWith=async(log:string|undefined):Promise<{en:string;de:string}|undefined>=>{
 const directory=mkdtempSync(join(base,"boot-phase-")),controller=new AbortController();let told:{en:string;de:string}|undefined;
 try{
  await connectHostV1({
   implementation:basename(directory)+"-boot",environment:privateEnvironment(directory),platform,languages:["en","de"],signal:controller.signal,
   spawnService:async({runtime}:{runtime:{log:string}})=>{if(log!==undefined)writeFileSync(runtime.log,log);return{alive:()=>true};},
   onProgress:(frame:{message:{en:string;de:string};phase:string})=>{if(frame.message?.en?.includes("Waiting for the repo graph service")){told??=frame.message;controller.abort();}},
  }).catch(()=>undefined);
  return told;
 }finally{rmSync(directory,{recursive:true,force:true});}
};

test("the wait names the boot phase of the service and the files it walked, in both languages",async()=>{
 const said=await waitWith("noise\n[repo] still working after 5 s; last phase started: source.capture\n[repo] still working after 10 s; last phase started: source.capture (walked 432 files, now some/path.ts)\n");
 expect(said?.en).toMatch(/^Waiting for the repo graph service \(\d+ ms\); it is in source\.capture \(432 files\)\.$/u);
 expect(said?.de).toMatch(/^Warte auf den Repo-Graphdienst \(\d+ ms\); er befindet sich in source\.capture \(432 files\)\.$/u);
});

test("a service that has said no phase leaves the plain wait text, and a later phase replaces the earlier one",async()=>{
 const plain=await waitWith("[script] {\"owner\":\"repository:build\"}\n");
 expect(plain?.en).toMatch(/^Waiting for the repo graph service \(\d+ ms\)\.$/u);
 const later=await waitWith("[repo] still working after 5 s; last phase started: source.capture\n[repo] still working after 50 s; last phase started: source.served\n");
 expect(later?.en).toContain("it is in source.served.");
});
