import {afterEach,expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {chmodSync,lstatSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,rmSync,writeFileSync} from "node:fs";
import {dirname,join,sep,win32} from "node:path";
import {decodeProtobufV1,encodeProtobufV1,REAPI_V1} from "../../📡️wire/📦️protobuf/🟦️.ts";
import type {RemoteExecutionContentV1} from "../../🗄️content/🟦️.ts";
import type {RemoteExecutionDigestV1} from "../../🟦️.ts";
import {OFFLOAD_POLICY_V1} from "../../🚀️offload/📐️policy/🟦️.ts";
import {placeRemoteExecutionLinkV1,remoteExecutionLinkEscapesV1,selectRemoteExecutionSandboxV1} from "../../🔒️sandbox/🟦️.ts";
import {testScratchRootV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
import {remoteExecutionThreadsOfMetadataV1} from "../../🟦️.ts";
import {executeRemoteExecutionActionV1,REMOTE_EXECUTION_INPUT_ROOT_ENV_V1,remoteExecutionWardenCancelledV1,remoteExecutionWorkNameV1} from "../🟦️.ts";

const sha=(data:Uint8Array)=>createHash("sha256").update(data).digest("hex"),scratch:string[]=[];
afterEach(()=>{for(const path of scratch.splice(0))rmSync(path,{recursive:true,force:true});});
const temporary=():string=>{const path=mkdtempSync(join(testScratchRootV1("rexec-worker"),"s-"));scratch.push(path);return path;};

class Memory implements RemoteExecutionContentV1{
 blobs=new Map<string,Uint8Array>();results=new Map<string,Uint8Array>();
 put(bytes:Uint8Array):RemoteExecutionDigestV1{const digest={hash:sha(bytes),size:bytes.length};this.blobs.set(digest.hash,bytes);return digest;}
 async has(_:string,hash:string){return this.blobs.has(hash);}
 async missing(_:string,digests:readonly RemoteExecutionDigestV1[]){return digests.filter(digest=>!this.blobs.has(digest.hash));}
 async read(_:string,digest:RemoteExecutionDigestV1){return this.blobs.get(digest.hash);}
 async stream(_:string,digest:RemoteExecutionDigestV1){const bytes=this.blobs.get(digest.hash);return bytes?(async function*(){yield bytes;})():undefined;}
 async write(_:string,digest:RemoteExecutionDigestV1,source:Uint8Array|AsyncIterable<Uint8Array>){if(source instanceof Uint8Array)this.blobs.set(digest.hash,source);else{const parts:Uint8Array[]=[];for await(const part of source)parts.push(part);this.blobs.set(digest.hash,Buffer.concat(parts));}}
 async readActionResult(_:string,hash:string){return this.results.get(hash);}
 async writeActionResult(_:string,hash:string,bytes:Uint8Array){this.results.set(hash,bytes);}
}
const wire=(digest:RemoteExecutionDigestV1)=>({hash:digest.hash,size_bytes:digest.size});

function action(memory:Memory,files:Record<string,string>,args:readonly string[],environment:Record<string,string>,outputs:readonly string[],links:Record<string,string>={}):RemoteExecutionDigestV1{
 const tree=(entries:Record<string,string>,top=false):RemoteExecutionDigestV1=>{
  const direct:Record<string,string>={},nested:Record<string,Record<string,string>>={};
  for(const [path,text] of Object.entries(entries)){const at=path.indexOf("/");if(at<0)direct[path]=text;else(nested[path.slice(0,at)]??={})[path.slice(at+1)]=text;}
  return memory.put(encodeProtobufV1(REAPI_V1.directory,{files:Object.entries(direct).sort().map(([name,text])=>({name,digest:wire(memory.put(new TextEncoder().encode(text))),is_executable:false})),directories:Object.entries(nested).sort().map(([name,child])=>({name,digest:wire(tree(child))})),symlinks:top?Object.entries(links).map(([name,target])=>({name,target})):[]}));
 };
 const root=tree(files,true),command=memory.put(encodeProtobufV1(REAPI_V1.command,{arguments:[...args],environment_variables:Object.entries(environment).map(([name,value])=>({name,value})),output_files:[],output_directories:[],output_paths:[...outputs],working_directory:""}));
 return memory.put(encodeProtobufV1(REAPI_V1.action,{command_digest:wire(command),input_root_digest:wire(root)}));
}

const run=(memory:Memory,digest:RemoteExecutionDigestV1,directory:string,extra:object={})=>executeRemoteExecutionActionV1({instance:"main",action:digest,timeoutSeconds:60,content:memory,directory,workerName:"w",sandbox:selectRemoteExecutionSandboxV1("isolated-root"),materialize:"copy",inlineOutputBytes:1024,signal:new AbortController().signal,actions:"any",...extra} as never);
const outputOf=(memory:Memory,bytes:Uint8Array|null,path:string):string=>{const result=decodeProtobufV1(REAPI_V1.actionResult,bytes!) as {output_files:{path:string;digest:{hash:string}}[]};const row=result.output_files.find(item=>item.path===path);return row===undefined?"":Buffer.from(memory.blobs.get(row.digest.hash)??new Uint8Array()).toString("utf8");};

const SCRIPT="const fs=require('fs');fs.mkdirSync('out',{recursive:true});fs.writeFileSync('out/r.json',JSON.stringify({args:process.argv.slice(2),e:process.env.E,cwd:process.cwd(),home:process.env.HOME}));";

test("an argv action runs without a shell on this host: the program and its arguments arrive as separate strings, the root placeholder is expanded to native paths in arguments and environment, outputs are reported as relative POSIX paths and the root report names the private root",async()=>{
 const memory=new Memory(),directory=temporary(),report=OFFLOAD_POLICY_V1.root.report,mark=OFFLOAD_POLICY_V1.root.placeholder;
 const digest=action(memory,{"p.js":SCRIPT,"in/file.txt":"x"},[process.execPath,"p.js",`${mark}/in/file.txt`,"a b;c|d $HOME `x` *",`--k=${mark}/a/b/`,`--m=${mark}=name`],{E:`${mark}/e/f`,HOME:`${mark}/${OFFLOAD_POLICY_V1.root.home}`,[report]:OFFLOAD_POLICY_V1.virtual.root},["out/r.json",OFFLOAD_POLICY_V1.virtual.root]);
 const result=await run(memory,digest,directory);
 expect(result.status.message).toBe("");expect(result.outcome).toBe("completed");
 const seen=JSON.parse(outputOf(memory,result.actionResult,"out/r.json")) as {args:string[];e:string;cwd:string;home:string};
 const rootReport=outputOf(memory,result.actionResult,OFFLOAD_POLICY_V1.virtual.root);
 expect(rootReport.length).toBeGreaterThan(0);expect(rootReport.endsWith(`${sep}r`)).toBe(true);
 expect(seen.args[0]).toBe(join(rootReport,"in","file.txt"));
 expect(seen.args[1],"no shell: metacharacters stay literal").toBe("a b;c|d $HOME `x` *");
 expect(seen.args[2]).toBe(`--k=${join(rootReport,"a","b")}${sep}`);
 expect(seen.args[3]).toBe(`--m=${rootReport}=name`);
 expect(seen.e).toBe(join(rootReport,"e","f"));expect(seen.home).toBe(join(rootReport,".tmp"));
 expect(seen.cwd.endsWith(`${sep}r`)).toBe(true);
});

test("the action learns its real private input root, so a source-reading compiler plug-in never looks above it (the EPERM of an offloaded mutation derive that climbed into the sandbox-hidden worktree)",async()=>{
 const memory=new Memory(),directory=temporary(),probe="require('fs').mkdirSync('out',{recursive:true});require('fs').writeFileSync('out/r.txt',process.env.SEMIO_OFFLOAD_INPUT_ROOT+'|'+require('fs').realpathSync(process.cwd()));";
 const result=await run(memory,action(memory,{"p.js":probe},[process.execPath,"p.js"],{},["out/r.txt"]),directory),[root,cwd]=outputOf(memory,result.actionResult,"out/r.txt").split("|");
 expect(REMOTE_EXECUTION_INPUT_ROOT_ENV_V1).toBe("SEMIO_OFFLOAD_INPUT_ROOT");expect(root!.length).toBeGreaterThan(0);expect(cwd).toBe(root);
});

test("the private root and the worker directory stay short enough for native Windows: a random 12-character task directory below the worker directory, one-letter root, long host names cut and made unique by a hash",async()=>{
 const memory=new Memory(),directory=temporary(),work=join(directory,"work"),digest=action(memory,{"p.js":SCRIPT},[process.execPath,"p.js"],{},["out/r.json"]);
 const result=await run(memory,digest,directory,{work,keepWork:true});
 expect(result.outcome).toBe("completed");
 const tasks=readdirSync(work);expect(tasks.length).toBe(1);expect(tasks[0]).toMatch(/^[0-9a-f]{12}$/u);
 expect(readdirSync(join(work,tasks[0]!))).toEqual(["r"]);
 expect(remoteExecutionWorkNameV1("g1-m05")).toBe("g1-m05");
 const long=remoteExecutionWorkNameV1("a-very-long-host-name.example.internal-12345"),other=remoteExecutionWorkNameV1("a-very-long-host-name.example.internal-12346");
 expect(long.length).toBeLessThanOrEqual(16);expect(long).not.toBe(other);expect(remoteExecutionWorkNameV1("we ird/na:me")).toBe("we_ird_na_me");
});

test("a tool program is resolved by name from the worker's tools, a missing tool is refused as a policy failure and an unknown executable is exit 127",async()=>{
 const memory=new Memory(),directory=temporary();
 const tool=await run(memory,action(memory,{"p.js":SCRIPT},["@tool:bun","p.js"],{},["out/r.json"]),directory,{tools:{bun:{path:process.execPath,readable:[]}}});
 expect(tool.outcome).toBe("completed");expect(outputOf(memory,tool.actionResult,"out/r.json")).toContain("cwd");
 const missing=await run(memory,action(memory,{"p.js":SCRIPT},["@tool:nope","p.js"],{},["out/r.json"]),directory);
 expect(missing.outcome).toBe("failed");expect(missing.status.message).toContain("does not offer the tool nope");
 const unknown=await run(memory,action(memory,{},["definitely-not-a-program-xyz"],{},["out/r.json"]),directory);
 expect(unknown.outcome).toBe("completed");expect((decodeProtobufV1(REAPI_V1.actionResult,unknown.actionResult!) as {exit_code:number}).exit_code).toBe(127);
});

test("an input link is a link where the host allows it and a copy of its target where it does not (no privilege to create symbolic links needed): files and whole directories",async()=>{
 const directory=temporary();mkdirSync(join(directory,"d","sub"),{recursive:true});writeFileSync(join(directory,"d","t.txt"),"content");writeFileSync(join(directory,"d","sub","n.txt"),"nested");
 const refuse={symlink:async()=>{throw Object.assign(Error("operation not permitted"),{code:"EPERM"});},copy:async(from:string,to:string)=>{const {cp}=await import("node:fs/promises");await cp(from,to,{recursive:true,dereference:true});}};
 expect(await placeRemoteExecutionLinkV1("t.txt",join(directory,"d","file-link"),directory,refuse)).toBe("copy");
 expect(lstatSync(join(directory,"d","file-link")).isSymbolicLink()).toBe(false);expect(readFileSync(join(directory,"d","file-link"),"utf8")).toBe("content");
 expect(await placeRemoteExecutionLinkV1("sub",join(directory,"d","dir-link"),directory,refuse)).toBe("copy");
 expect(lstatSync(join(directory,"d","dir-link")).isSymbolicLink()).toBe(false);expect(readFileSync(join(directory,"d","dir-link","n.txt"),"utf8")).toBe("nested");
 if(process.platform!=="win32"){expect(await placeRemoteExecutionLinkV1("t.txt",join(directory,"d","real-link"),directory)).toBe("link");expect(lstatSync(join(directory,"d","real-link")).isSymbolicLink()).toBe(true);}
 await expect(placeRemoteExecutionLinkV1("missing",join(directory,"d","broken"),directory,refuse)).rejects.toThrow();
});

test("an input link that names a host file or leaves the input root is refused before anything is placed, on every host: absolute targets of either spelling, drive and UNC forms, and ..-escapes, with the same answer for win32 path functions",async()=>{
 const escapes=(root:string,path:string,target:string,flavour?:Parameters<typeof remoteExecutionLinkEscapesV1>[3])=>remoteExecutionLinkEscapesV1(root,path,target,flavour);
 for(const target of ["/etc/passwd","C:\\Windows\\win.ini","\\\\host\\share\\x","\\x","../../outside","a/../../../outside",""])expect(escapes("/w/r","/w/r/d/l",target),target).toBe(true);
 for(const target of ["t.txt","sub/n.txt","../d2/x","."])expect(escapes("/w/r","/w/r/d/l",target),target).toBe(false);
 const directory=temporary(),host=join(directory,"host-secret.txt");writeFileSync(host,"secret");
 const refuse={symlink:async()=>{throw Object.assign(Error("operation not permitted"),{code:"EPERM"});},copy:async()=>{throw Error("must not copy");}};
 await expect(placeRemoteExecutionLinkV1(host,join(directory,"l"),directory,refuse)).rejects.toThrow(/leaves the input root/u);
 const memory=new Memory(),digest=action(memory,{"p.js":SCRIPT},[process.execPath,"p.js"],{},["out/r.json"],{leak:host});
 const result=await run(memory,digest,directory,{work:join(directory,"work"),keepWork:true});
 expect(result.outcome).toBe("failed");expect(result.status.code).toBe(3);expect(result.status.message).toContain("leaves the input root");
 for(const task of readdirSync(join(directory,"work")))expect(readdirSync(join(directory,"work",task,"r")).includes("leak")).toBe(false);
});

const STUB=`#!${process.execPath}
const fs=require("fs"),args=process.argv.slice(2),flags=process.env.CARGO_MAKEFLAGS??"",fifo=/--jobserver-auth=fifo:(\\S+)/u.exec(flags)?.[1];
let tokens=-1;
if(fifo!==undefined){const fd=fs.openSync(fifo,fs.constants.O_RDONLY|fs.constants.O_NONBLOCK);tokens=0;const buffer=Buffer.alloc(64);for(;;){try{const n=fs.readSync(fd,buffer,0,64,null);if(n<=0)break;tokens+=n;}catch{break;}}fs.closeSync(fd);}
fs.mkdirSync(args[args.indexOf("--out-dir")+1],{recursive:true});
fs.writeFileSync(args[args.indexOf("--out-dir")+1]+"/seen.json",JSON.stringify({serial:args.includes("no-parallel-backend"),tokens,front:args.filter(value=>value.startsWith("threads="))}));
`;
const rustcArguments=(type:string,extra:readonly string[]=[])=>["@tool:rustc","--crate-name","t","--edition=2021",".semio-offload/p/src/lib.rs","--crate-type",type,"--emit=dep-info,link","--out-dir",OFFLOAD_POLICY_V1.virtual.out,"-Z","threads=1",...extra];
const rustcEnvironment=()=>({PATH:OFFLOAD_POLICY_V1.environment.path,HOME:`${OFFLOAD_POLICY_V1.root.placeholder}/${OFFLOAD_POLICY_V1.root.home}`,[OFFLOAD_POLICY_V1.root.report]:OFFLOAD_POLICY_V1.virtual.root});
async function codegen(args:readonly string[],threads:(()=>Promise<number>)|undefined):Promise<{seen:{serial:boolean;tokens:number;front:string[]};granted:number|undefined}>{
 const memory=new Memory(),directory=temporary(),stub=join(directory,"rustc-stub");writeFileSync(stub,STUB);chmodSync(stub,0o755);
 const digest=action(memory,{},args,rustcEnvironment(),[OFFLOAD_POLICY_V1.virtual.out,OFFLOAD_POLICY_V1.virtual.root]);
 const result=await run(memory,digest,directory,{actions:"rustc",tools:{rustc:{path:stub,readable:[directory,dirname(process.execPath)]}},sandbox:selectRemoteExecutionSandboxV1("auto"),...threads?{threads}:{}});
 expect(result.status.message).toBe("");expect(result.outcome).toBe("completed");
 const decoded=decodeProtobufV1(REAPI_V1.actionResult,result.actionResult!) as {exit_code:number;execution_metadata:{auxiliary_metadata?:{type_url:string;value:Uint8Array}[]}};
 expect(decoded.exit_code).toBe(0);
 const rows=(decoded as unknown as {output_directories:{path:string;tree_digest:{hash:string}}[]}).output_directories,tree=decodeProtobufV1(REAPI_V1.tree,memory.blobs.get(rows.find(row=>row.path===OFFLOAD_POLICY_V1.virtual.out)!.tree_digest.hash)!) as {root:{files:{name:string;digest:{hash:string}}[]}};
 const seen=JSON.parse(Buffer.from(memory.blobs.get(tree.root.files.find(file=>file.name==="seen.json")!.digest.hash)!).toString("utf8"));
 return{seen,granted:remoteExecutionThreadsOfMetadataV1(decoded.execution_metadata.auxiliary_metadata)};
}

test.skipIf(process.platform==="win32")("a linked unit runs on the threads its worker grants: a grant of four hands rustc a pool with the three other tokens and no serial flag, a grant of one runs the serial back end, no grant at all (a worker that cannot size it) is serial, and the grant is returned in the result",async()=>{
 const four=await codegen(rustcArguments("cdylib"),async()=>4);
 expect(four.seen.serial).toBe(false);expect(four.seen.tokens).toBe(3);expect(four.granted).toBe(4);expect(four.seen.front).toEqual(["threads=1"]);
 const one=await codegen(rustcArguments("cdylib"),async()=>1);
 expect(one.seen.serial).toBe(true);expect(one.seen.tokens).toBe(-1);expect(one.granted).toBe(1);
 const none=await codegen(rustcArguments("cdylib"),undefined);
 expect(none.seen.serial).toBe(true);expect(none.seen.tokens).toBe(-1);expect(none.granted).toBe(1);
 const broken=await codegen(rustcArguments("cdylib"),async()=>{throw Error("host view unreadable");});
 expect(broken.seen.serial).toBe(true);expect(broken.granted).toBe(1);
});

test.skipIf(process.platform==="win32")("a requester that asks for the serial back end gets one thread whatever the worker could grant, and a unit that is not linked is never asked: no pool, no flag, no granted-threads entry",async()=>{
 let asked=0;
 const explicit=await codegen(rustcArguments("cdylib",["-Z","no-parallel-backend"]),async()=>{asked++;return 8;});
 expect(explicit.seen.serial).toBe(true);expect(explicit.seen.tokens).toBe(-1);expect(explicit.granted).toBe(1);expect(asked).toBe(0);
 const library=await codegen(rustcArguments("lib"),async()=>{asked++;return 8;});
 expect(library.seen.serial).toBe(false);expect(library.seen.tokens).toBe(-1);expect(library.granted).toBeUndefined();expect(asked).toBe(0);
});

test("a kill that the host warden announced is a refusal, any other kill stays a result: exit 137 plus a cancel record that names the task pid or a command below its work directory",()=>{
 const work="/w/work/ab12/";
 const record=(pid:number,command:string)=>({pid,command});
 expect(remoteExecutionWardenCancelledV1({exitCode:137,cancels:[record(9,`rustc --crate-name x ${work}r/src/lib.rs`)],work:"/w/work/ab12"})).toBe(true);
 expect(remoteExecutionWardenCancelledV1({exitCode:137,cancels:[record(4242,"rustc /elsewhere/x.rs")],work:"/w/work/ab12",pid:4242})).toBe(true);
 expect(remoteExecutionWardenCancelledV1({exitCode:137,cancels:[record(9,"rustc /w/work/ab123/r/src/lib.rs")],work:"/w/work/ab12",pid:1})).toBe(false);
 expect(remoteExecutionWardenCancelledV1({exitCode:137,cancels:[],work:"/w/work/ab12",pid:1})).toBe(false);
 expect(remoteExecutionWardenCancelledV1({exitCode:1,cancels:[record(1,`rustc ${work}r/x.rs`)],work:"/w/work/ab12",pid:1})).toBe(false);
});

test.skipIf(process.platform==="win32")("a task whose process was killed while the warden's cancel record names its work directory is reported refused with code 8 and the reason warden-cancelled, an unrelated record or none leaves the kill a plain result",async()=>{
 const memory=new Memory(),directory=temporary(),kill="process.kill(process.pid,'SIGKILL');setTimeout(()=>{},5000);";
 const digest=()=>action(memory,{"p.js":kill},[process.execPath,"p.js"],{},[]);
 const taskDirectory=():string=>join(directory,"work",readdirSync(join(directory,"work"))[0]!);
 const named=await run(memory,digest(),directory,{wardenCancels:async()=>[{pid:1,command:`rustc ${taskDirectory()}/r/src/lib.rs`}]});
 expect(named.outcome).toBe("refused");expect(named).toMatchObject({status:{code:8},refusal:"warden-cancelled",actionResult:null});
 expect(named.status.message).toContain("warden");
 const other=await run(memory,digest(),directory,{wardenCancels:async()=>[{pid:1,command:"rustc /somewhere/else/work/zz/r/lib.rs"}]});
 expect(other.outcome).toBe("completed");expect((decodeProtobufV1(REAPI_V1.actionResult,other.actionResult!) as {exit_code:number}).exit_code).toBe(137);
 const none=await run(memory,digest(),directory,{wardenCancels:async()=>{throw Error("unreadable");}});
 expect(none.outcome).toBe("completed");
});
