import {afterEach,expect,test} from "bun:test";
import {existsSync,lstatSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {webcrypto} from "node:crypto";
import {fileURLToPath} from "node:url";
import {stageArtifacts} from "../../../../../../../🔨️modules/🏃️process/📦️artifacts/📤️publication/🟦️.ts";
import {componentModelGenerationPathV1,componentModelGenerationV1,componentModelLayoutV1} from "../../📋️contract/🟦️.ts";
import {promoteComponentModelGenerationV1,readComponentModelHeadFileV1,resolveComponentModelRequestV1} from "../../🧭️head/🟦️.ts";
import fixture from "./🔣️.json" with {type:"json"};

const leasesModule=fileURLToPath(new URL("../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts",import.meta.url));
const holderProgram=`import {acquireResourceLease} from ${JSON.stringify(leasesModule)};
const [directory,resource]=process.argv.slice(2);
const lease=await acquireResourceLease({directory,resource,mode:"shared",signal:new AbortController().signal});
process.stdout.write("READY\\n");
for await (const _ of process.stdin) {}
lease.release();`;
type Id=keyof typeof fixture.generations;
const boxes:Array<()=>Promise<void>>=[];
afterEach(async()=>{while(boxes.length)await boxes.pop()!();});

/** 🏠️ One sandbox: a lease directory, a products root and live holder processes standing in for long-lived readers. */
function sandbox(){
 const root=realpathSync(mkdtempSync(join(tmpdir(),"cg-"))),leases=join(root,"l"),products=join(root,"p"),source=join(root,"s"),program=join(root,"holder.ts"),holders:Bun.Subprocess<"pipe","pipe","inherit">[]=[];
 for(const directory of[leases,products,source])mkdirSync(directory);writeFileSync(program,holderProgram);
 const layout=componentModelLayoutV1(products,fixture.intent),name=(id:Id)=>componentModelGenerationV1({receipt:Buffer.from(fixture.generations[id].receipt)}),path=(id:Id)=>componentModelGenerationPathV1(layout,name(id));
 const options=()=>({leaseDirectory:leases,signal:AbortSignal.timeout(fixture.publicationMilliseconds)});
 const stage=(id:Id,immutable=true)=>{const files=new Map<string,string>();for(const[file,text]of Object.entries(fixture.generations[id].files)){const target=join(source,id+"-"+files.size);writeFileSync(target,text);files.set(file,target);}return stageArtifacts(path(id),fixture.owner,files,{...options(),...immutable?{immutable:true,timeoutMs:fixture.publicationMilliseconds}:{}});};
 const publish=async(id:Id)=>{await stage(id);return promoteComponentModelGenerationV1(layout,name(id),options());};
 const read=async()=>{const request=await resolveComponentModelRequestV1(products,fixture.intent);const child=Bun.spawn([process.execPath,program,leases,"artifact:"+request.path],{stdin:"pipe",stdout:"pipe",stderr:"inherit"});const first=await child.stdout.getReader().read();expect(new TextDecoder().decode(first.value)).toBe("READY\n");holders.push(child);return{request,release:async()=>{child.stdin.end();await child.exited;}};};
 boxes.push(async()=>{for(const child of holders){child.stdin.end();await child.exited;}rmSync(root,{recursive:true,force:true});});
 return{layout,products,leases,name,path,options,stage,publish,read};
}

/** 🚨️ The incident: a reader of N must not delay N+1; the new reader sees N+1. @see ./🥒️.feature */
test("a reader holding generation N does not block publication of N+1 and a new reader sees N+1",async()=>{
 const box=sandbox();await box.publish("n");const held=await box.read();expect(held.request.path).toBe(box.path("n"));
 const started=Date.now(),outcome=await box.publish("m");
 expect(Date.now()-started).toBeLessThan(fixture.publicationMilliseconds);
 expect((await readComponentModelHeadFileV1(box.layout.head))?.generation).toBe(box.name("m"));
 expect(outcome.held).toEqual([box.name("n")]);expect(outcome.head.previous).toEqual([box.name("n")]);
 expect(readFileSync(join(box.path("n"),"📦️component.wasm"),"utf8")).toBe("alpha");
 expect((await resolveComponentModelRequestV1(box.products,fixture.intent)).path).toBe(box.path("m"));
 expect(readFileSync(join(box.path("m"),"📦️component.wasm"),"utf8")).toBe("bravo");
 await held.release();
},60000);

/** 🧹️ The ledger: a held generation survives its supersession and is reclaimed by the first publication after its holder left. */
test("a superseded generation is reclaimed once its holder has exited",async()=>{
 const box=sandbox();await box.publish("n");const held=await box.read();await box.publish("m");expect(existsSync(box.path("n"))).toBe(true);await held.release();
 const outcome=await box.publish("o");
 expect([...outcome.retired].sort()).toEqual([box.name("m"),box.name("n")].sort());expect(outcome.head.previous).toEqual([]);
 expect(readdirSync(box.layout.directory).filter(entry=>!entry.startsWith("head")).sort()).toEqual([box.name("o")]);
},60000);

/** ♻️ The same bytes published again are reused in place under the shared lease, while the exclusive path stays blocked. */
test("republishing the held generation reuses it in place and the exclusive path is blocked",async()=>{
 const box=sandbox();await box.publish("n");const held=await box.read(),file=join(box.path("n"),"📦️component.wasm"),before=lstatSync(file);
 const started=Date.now();await box.stage("n");expect(Date.now()-started).toBeLessThan(fixture.publicationMilliseconds);
 const after=lstatSync(file);expect(after.ino).toBe(before.ino);expect(after.mtimeMs).toBe(before.mtimeMs);
 const signal=AbortSignal.timeout(fixture.blockedMilliseconds),files=new Map<string,string>();for(const[name,text]of Object.entries(fixture.generations.n.files)){const target=join(box.leases,"x-"+files.size);writeFileSync(target,text);files.set(name,target);}
 await expect(stageArtifacts(box.path("n"),fixture.owner,files,{leaseDirectory:box.leases,signal})).rejects.toBeDefined();expect(signal.aborted).toBe(true);
 await held.release();
},60000);

/** 🧮️ Generation names equal the digest of an independent implementation. */
test("generation names equal the independent SHA-256 of their receipt",async()=>{
 const box=sandbox();
 for(const id of Object.keys(fixture.generations)as Id[]){const independent=Buffer.from(await webcrypto.subtle.digest("SHA-256",Buffer.from(JSON.stringify([Buffer.from(await webcrypto.subtle.digest("SHA-256",Buffer.from(fixture.generations[id].receipt))).toString("hex"),null,null])))).toString("hex").slice(0,24);expect(box.name(id)).toBe(independent);}
});
