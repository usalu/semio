import {test,expect} from "bun:test";
import {join,resolve} from "node:path";
import {pathToFileURL} from "node:url";

/** 🪶️ Size law: retained compiler bytes follow the deliverables, never the dependency closure. Target dir `🧪️tests/🪶️size-law/🟦️.ts` of the retention module; `SEMIO_ROSTER_MODULE` points the test at another inventory owner. */
test("completed Cargo retention claims only deliverable artifacts, however many dependency libraries were built",async()=>{
 const owner=process.env.SEMIO_ROSTER_MODULE??resolve(import.meta.dir,"../../📋️inventory/🟦️.ts"),api=await import(pathToFileURL(owner).href),root=resolve(process.cwd()),origin={version:1,workspace:root,mode:"prototype",manifest:join(root,"Cargo.toml"),cwd:root,buildDirectory:join(root,"build"),targetDirectory:join(root,"target"),cargoHome:join(root,"cargo")};
 const digest=(index:number)=>index.toString(16).padStart(64,"0"),unit=(kind:string,path:string,index:number)=>({message:{target:{kind:[kind]}},depInfo:[],inputs:[],artifacts:[{path:join(origin.targetDirectory,path),sha256:digest(index)}]});
 const libraries=Array.from({length:400},(_,index)=>unit(index%50===0?"proc-macro":"lib",`lib${index}.rlib`,index+1)),binary=unit("bin","emitter",9999),cdylib=unit("cdylib","component.wasm",10000),observation={version:1,manifest:origin.manifest,cwd:root,buildDirectory:origin.buildDirectory,status:0,cancelled:false,units:[...libraries,binary,cdylib],buildScripts:[],invocationInputs:[],buildResources:[],compilerResources:[]};
 const oracle=observation.units.filter(row=>row.message.target.kind.some(kind=>!["lib","rlib","proc-macro","custom-build"].includes(kind))).flatMap(row=>row.artifacts.map(artifact=>artifact.path)).sort(),roster=api.completedCargoClosureRosterV1(observation,origin);
 expect(api.completedCargoOutputClaimsV1(observation,origin).map((row:{path:string})=>row.path).sort()).toEqual(oracle);
 expect(roster.filter((row:{custody:string})=>row.custody==="retained")).toHaveLength(oracle.length);
 expect(roster).toHaveLength(oracle.length);
 expect(roster.some((row:{path:string})=>row.path.endsWith("lib1.rlib")),"intermediates own no roster row").toBe(false);
});
