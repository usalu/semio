/** 📸️ Contract: a checkout snapshot is ONE immutable source generation named after the digest of what it holds. A live edit during or after the capture never changes it, a build that runs inside it compiles exactly the captured bytes, and the digest equals an independent Python derivation from the same tree. */
import {afterAll,beforeAll,expect,test} from "bun:test";
import {spawn} from "node:child_process";
import {chmodSync,constants,copyFileSync,existsSync,statfsSync,lstatSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,symlinkSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import Ajv from "ajv";
import {makeTestTreeWritableV1,testPythonV1} from "../../../🧪️testing/🧰️toolchain/🟦️.ts";
import {registeredToolExecutableV1} from "../../../🧰️tools/🏃️run/🟦️.ts";
import {captureCheckoutSnapshotV1,checkoutRowsDigestV1,checkoutRowsTextV1,checkoutSnapshotBaseV1,checkoutSnapshotEnvironmentV1,checkoutSnapshotLiveTextV1,checkoutSnapshotOfV1,forgetCheckoutSnapshotStepV1,holdCheckoutSnapshotV1,checkoutSnapshotProtocolV1,checkoutSnapshotProvesV1,pruneCheckoutSnapshotsV1,readCheckoutSnapshotRowsV1,verifyCheckoutSnapshotV1,type CheckoutRowV1,type CheckoutSnapshotRequestV1} from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";

const python=testPythonV1(),protocol=checkoutSnapshotProtocolV1(),environment={SEMIO_LANGUAGE:"en"};
let sandbox="";
beforeAll(()=>{sandbox=realpathSync(mkdtempSync(join(tmpdir(),"cs-")));});
afterAll(()=>{makeTestTreeWritableV1(sandbox);rmSync(sandbox,{recursive:true,force:true});});

const quiet:NonNullable<CheckoutSnapshotRequestV1["observation"]>={now:()=>Date.now()+60_000,sleep:async()=>{},sleepSync:()=>{},onRetry:()=>{},onSettle:()=>{}};
let counter=0;
const fresh=(name:string):string=>{const directory=join(sandbox,name+String(counter++));mkdirSync(directory,{recursive:true});return directory;};
const write=(root:string,path:string,body:string|Uint8Array,executable=false):void=>{const file=join(root,path);mkdirSync(dirname(file),{recursive:true});writeFileSync(file,body);if(executable&&process.platform!=="win32")chmodSync(file,0o755);};
const capture=(workspace:string,state:string,more:Partial<CheckoutSnapshotRequestV1>={})=>captureCheckoutSnapshotV1({workspace,state,environment,observation:quiet,retention:{minimumFreeBytes:0,retained:10},entries:readdirSync(workspace),...more});
const bytesOf=(row:{text?:string;base64?:string}):Uint8Array=>row.text!==undefined?Buffer.from(row.text,"utf8"):Buffer.from(row.base64!,"base64");

function fixtureTree(root:string):void{
 for(const row of fixture.tree.files)write(root,row.path,bytesOf(row),(row as {executable?:boolean}).executable===true);
 for(const link of fixture.tree.links)symlinkSync(link.target,join(root,link.path));
 for(const path of fixture.tree.excluded)write(root,path,"excluded\n");
}

const derive=`import hashlib,json,os,sys
root=sys.argv[1]
skip={".snapshot.json",".snapshot.rows"}
rows=[]
for base,dirs,names in os.walk(root):
    for n in names+[d for d in dirs if os.path.islink(os.path.join(base,d))]:
        p=os.path.join(base,n)
        rel=os.path.relpath(p,root).replace(os.sep,"/")
        if rel in skip: continue
        if os.path.islink(p):
            rows.append((rel.encode(),"l\\t%s\\t%s\\n"%(rel,os.readlink(p))))
        else:
            h=hashlib.sha256(open(p,"rb").read()).hexdigest()
            x="x" if os.access(p,os.X_OK) and os.name!="nt" else "-"
            rows.append((rel.encode(),"f\\t%s\\t%s\\t%s\\n"%(rel,h,x)))
rows.sort(key=lambda r:r[0])
text="".join(r[1] for r in rows)
print(json.dumps({"digest":hashlib.sha256(text.encode()).hexdigest(),"text":text}))
`;
const independent=(root:string):{digest:string;text:string}=>JSON.parse(Bun.spawnSync([python!,"-I","-c",derive,root]).stdout.toString());

test("the protocol is closed under its schema and every text exists in both languages with the same placeholders",()=>{
 const ajv=new Ajv({strict:false}),row=ajv.compile({definitions:schema.definitions,$ref:"#/definitions/Row"}),receipt=ajv.compile({definitions:schema.definitions,$ref:"#/definitions/Receipt"});
 expect(row({kind:"file",path:"a",sha256:"0".repeat(64),bytes:1,executable:false})).toBe(true);
 expect(row({kind:"link",path:"a",target:"b"})).toBe(true);
 expect(row({kind:"file",path:"a",sha256:"zz",bytes:1,executable:false})).toBe(false);
 expect(receipt({version:1,origin:"/o",digest:"0".repeat(64),files:1,links:0,bytes:2})).toBe(true);
 const placeholders=(text:string)=>[...text.matchAll(/\{([a-z]+)\}/gu)].map(match=>match[1]).sort();
 expect(Object.keys(protocol.texts).sort()).toEqual(["de","en"]);
 for(const key of Object.keys(protocol.texts.en) as (keyof typeof protocol.texts.en)[])expect(placeholders(protocol.texts.de[key])).toEqual(placeholders(protocol.texts.en[key]));
 expect(protocol.idCharacters).toBeGreaterThanOrEqual(8);
 expect(protocol.maximumRootCharacters).toBeLessThanOrEqual(256);
 for(const name of [...protocol.excludeRootEntries,...protocol.excludeEverywhere,...protocol.linkEverywhere])expect(name.includes("/")).toBe(false);
});

test("the language-agnostic fixture yields its recorded rows, digest and id, and Python derives the same digest from the snapshot on disk",async()=>{
 const live=fresh("fixture"),state=fresh("state");
 fixtureTree(live);
 const snapshot=await capture(live,state);
 const expected=fixture.expected,executable=(row:{executable?:boolean}):boolean=>row.executable===true&&process.platform!=="win32";
 const rows:CheckoutRowV1[]=snapshot.rows as CheckoutRowV1[];
 expect(rows.filter(row=>row.kind==="file").map(row=>row.path)).toEqual(expect.arrayContaining(fixture.tree.files.map(row=>row.path)));
 for(const path of fixture.tree.excluded)expect(rows.some(row=>row.path===path||row.path.startsWith(path.split("/")[0]+"/"))).toBe(false);
 if(process.platform!=="win32"||!fixture.tree.files.some(executable)){
  expect(checkoutRowsTextV1(rows)).toBe(expected.text);
  expect(snapshot.digest).toBe(expected.digest);
  expect(snapshot.id).toBe(expected.id);
 }
 expect(snapshot.files).toBe(expected.files);
 expect(snapshot.links).toBe(expected.links);
 expect(snapshot.bytes).toBe(expected.bytes);
 expect(snapshot.root).toBe(join(checkoutSnapshotBaseV1(live,state),snapshot.id));
 expect(snapshot.root.length).toBeLessThanOrEqual(protocol.maximumRootCharacters);
 expect(checkoutRowsDigestV1(rows)).toBe(snapshot.digest);
 for(const row of fixture.tree.files)expect(readFileSync(join(snapshot.root,row.path))).toEqual(Buffer.from(bytesOf(row)));
 expect(lstatSync(join(snapshot.root,"alias.txt")).isSymbolicLink()).toBe(true);
 if(process.platform!=="win32")for(const row of fixture.tree.files)expect(lstatSync(join(snapshot.root,row.path)).mode&0o222).toBe(0);
 if(python){const answer=independent(snapshot.root);expect(answer.digest).toBe(snapshot.digest);expect(answer.text).toBe(checkoutRowsTextV1(rows));}
 expect(readdirSync(checkoutSnapshotBaseV1(live,state)).filter(name=>!name.startsWith(".")).sort()).toEqual([snapshot.id]);
 expect(JSON.parse(readFileSync(join(snapshot.root,protocol.receiptName),"utf8"))).toEqual({version:1,origin:live,digest:snapshot.digest,files:snapshot.files,links:snapshot.links,bytes:snapshot.bytes});
});

test("live edits, deletions and additions after the capture leave the snapshot byte-identical and verifiable; an unchanged tree reuses it, a changed tree gets a new one",async()=>{
 const live=fresh("after"),state=fresh("state");
 fixtureTree(live);
 const first=await capture(live,state);
 write(live,"data/ü.txt","edited\n");rmSync(join(live,"ab/c.txt"));write(live,"added.txt","new\n");
 const proof=await verifyCheckoutSnapshotV1(first.root);
 expect(proof).toEqual({ok:true,digest:first.digest,changed:[],extra:[]});
 expect(readFileSync(join(first.root,"data/ü.txt"),"utf8")).toBe("grüezi\n");
 expect(existsSync(join(first.root,"ab/c.txt"))).toBe(true);
 expect(existsSync(join(first.root,"added.txt"))).toBe(false);
 const second=await capture(live,state);
 expect(second.digest).not.toBe(first.digest);expect(second.reused).toBe(false);
 expect(readFileSync(join(second.root,"data/ü.txt"),"utf8")).toBe("edited\n");
 const again=await capture(live,state);
 expect(again.root).toBe(second.root);expect(again.reused).toBe(true);expect(again.digest).toBe(second.digest);
 expect((await verifyCheckoutSnapshotV1(first.root)).ok).toBe(true);
 if(process.platform!=="win32"){chmodSync(join(first.root,"z.bin"),0o644);writeFileSync(join(first.root,"z.bin"),"tampered");}
 else writeFileSync(join(first.root,"z.bin"),"tampered");
 const tampered=await verifyCheckoutSnapshotV1(first.root);
 expect(tampered.ok).toBe(false);expect(tampered.changed).toEqual(["z.bin"]);
});

test("a file edited or deleted while the capture runs is observed again and the snapshot holds the final state of the tree",async()=>{
 const live=fresh("during"),state=fresh("state");
 fixtureTree(live);
 let edited=false,deleted=false;
 const snapshot=await capture(live,state,{afterCopy:async path=>{
  if(!edited&&path.endsWith("data/ü.txt")){edited=true;write(live,"data/ü.txt","during\n");}
  if(!deleted&&path.endsWith("ab/c.txt")){deleted=true;rmSync(path);}
 }});
 expect([edited,deleted]).toEqual([true,true]);
 expect(readFileSync(join(snapshot.root,"data/ü.txt"),"utf8")).toBe("during\n");
 expect(existsSync(join(snapshot.root,"ab/c.txt"))).toBe(false);
 expect(snapshot.rows.some(row=>row.path==="ab/c.txt")).toBe(false);
 expect((await verifyCheckoutSnapshotV1(snapshot.root)).ok).toBe(true);
 expect(independent(snapshot.root).digest).toBe(snapshot.digest);
});

test("a cancelled capture rejects and leaves no directory behind",async()=>{
 const live=fresh("cancel"),state=fresh("state"),controller=new AbortController();
 fixtureTree(live);
 await expect(capture(live,state,{signal:controller.signal,afterCopy:()=>{controller.abort(Error("stop"));}})).rejects.toThrow("stop");
 expect(readdirSync(checkoutSnapshotBaseV1(live,state))).toEqual([]);
});

test("node_modules stay links to the live packages, the state directory is never copied and the environment names the root and the live state",async()=>{
 const live=fresh("links"),state=join(fresh("outer"),"machine");
 write(live,"src/a.rs","a\n");write(live,"pkg/node_modules/dep/index.js","x\n");symlinkSync(join(live,"pkg","node_modules"),join(live,"node_modules"));
 mkdirSync(join(live,"sub"));symlinkSync("/nonexistent/installed",join(live,"sub","node_modules"));
 const snapshot=await capture(live,join(live,".cache-state"));
 const links=snapshot.rows.filter(row=>row.kind==="link").map(row=>row.path).sort();
 expect(links).toEqual(["node_modules","pkg/node_modules","sub/node_modules"]);
 expect(existsSync(join(snapshot.root,".cache-state","s"))).toBe(false);
 expect(existsSync(join(snapshot.root,"pkg","node_modules","dep","index.js"))).toBe(true);
 const names=protocol.environment,variables=checkoutSnapshotEnvironmentV1({...snapshot,state});
 expect(variables[names.root]).toBe(snapshot.root);expect(variables[names.state]).toBe(state);expect(variables[names.origin]).toBe(live);expect(variables[names.digest]).toBe(snapshot.digest);
 expect(checkoutSnapshotLiveTextV1("error at "+snapshot.root+"/src/a.rs:1",snapshot)).toBe("error at "+live+"/src/a.rs:1");
});

test("a root longer than the declared maximum is refused in both languages",async()=>{
 const live=fresh("long"),state=join(fresh("deep"),"x".repeat(protocol.maximumRootCharacters));
 write(live,"a","a");
 await expect(capture(live,state)).rejects.toThrow(/characters/u);
 await expect(capture(live,state,{environment:{SEMIO_LANGUAGE:"de"}})).rejects.toThrow(/Zeichen/u);
});

test("only the declared closure of root entries is captured by default",async()=>{
 const live=fresh("closure"),state=fresh("state");
 write(live,"Cargo.toml","[workspace]\n");write(live,protocol.closureRootEntries[0]!+"/a.rs","a");write(live,"♻️other/b.txt","b");
 const snapshot=await captureCheckoutSnapshotV1({workspace:live,state,environment,observation:quiet,retention:{minimumFreeBytes:0}});
 expect(snapshot.rows.map(row=>row.path)).toEqual(["Cargo.toml",protocol.closureRootEntries[0]+"/a.rs"].sort((left,right)=>Buffer.compare(Buffer.from(left),Buffer.from(right))));
});

test("a snapshot a live process holds is never removed, whatever the cap or the free floor says; a dead holder protects nothing",async()=>{
 const live=fresh("hold"),state=fresh("state"),ids:string[]=[];
 for(let step=0;step<3;step++){write(live,"a.txt","v"+step);const snapshot=await capture(live,state);ids.push(snapshot.id);const used=new Date(Date.now()-(3-step)*60_000);utimesSync(join(snapshot.root,protocol.receiptName),used,used);}
 const base=checkoutSnapshotBaseV1(live,state);
 rmSync(join(base,protocol.holdersName),{recursive:true,force:true});
 await holdCheckoutSnapshotV1(live,ids[0]!,state,process.pid);
 const removed=await pruneCheckoutSnapshotsV1({workspace:live,state,retained:0,maximumTotalBytes:0,minimumFreeBytes:Number.MAX_SAFE_INTEGER,free:()=>0});
 expect([...removed].sort()).toEqual([ids[1]!,ids[2]!].sort());
 expect(readdirSync(base).filter(name=>!name.startsWith("."))).toEqual([ids[0]!]);
 const holders=join(base,protocol.holdersName,ids[0]!);
 writeFileSync(join(holders,"2147483646"),"");
 await pruneCheckoutSnapshotsV1({workspace:live,state});
 expect(readdirSync(holders)).toEqual([String(process.pid)]);
});

test("the free-space floor and the byte cap remove unprotected snapshots oldest first, with no age protection",async()=>{
 const live=fresh("floor"),state=fresh("state"),ids:string[]=[];
 for(let step=0;step<4;step++){write(live,"a.txt","v"+step);const snapshot=await captureCheckoutSnapshotV1({workspace:live,state,environment,observation:quiet,retention:{minimumFreeBytes:0,retained:10},entries:["a.txt"]});ids.push(snapshot.id);const used=new Date(Date.now()-(4-step)*60_000);utimesSync(join(snapshot.root,protocol.receiptName),used,used);rmSync(join(checkoutSnapshotBaseV1(live,state),protocol.holdersName),{recursive:true,force:true});}
 const readings=[0,50,200];
 const removed=await pruneCheckoutSnapshotsV1({workspace:live,state,retained:10,maximumTotalBytes:Number.MAX_SAFE_INTEGER,minimumFreeBytes:100,free:()=>readings.shift()!});
 expect(removed).toEqual([ids[0]!,ids[1]!]);
 expect(readdirSync(checkoutSnapshotBaseV1(live,state)).filter(name=>!name.startsWith("."))).toEqual([ids[2]!,ids[3]!].sort());
 const capped=await pruneCheckoutSnapshotsV1({workspace:live,state,retained:10,maximumTotalBytes:1,minimumFreeBytes:0,free:()=>Number.MAX_SAFE_INTEGER});
 expect(capped).toEqual([ids[2]!,ids[3]!]);
});

test("a dead capture's stage directory is removed, a live one is kept",async()=>{
 const live=fresh("stage"),state=fresh("state"),base=checkoutSnapshotBaseV1(live,state);
 mkdirSync(join(base,".t2147483646-dead"),{recursive:true});mkdirSync(join(base,".t"+process.pid+"-live"),{recursive:true});mkdirSync(join(base,".told-legacy"),{recursive:true});
 await pruneCheckoutSnapshotsV1({workspace:live,state});
 expect(readdirSync(base).sort()).toEqual([".t"+process.pid+"-live"]);
});

test("a named step reuses its snapshot only for the live owner that recorded it, until it is forgotten or a fresh one is demanded",async()=>{
 const live=fresh("step"),state=fresh("state"),owner=process.pid;
 write(live,"a.txt","one");
 const first=await capture(live,state,{step:"queue:x",owner});
 write(live,"a.txt","two");
 const again=await capture(live,state,{step:"queue:x",owner});
 expect([again.root,again.reused,again.attempts,again.digest]).toEqual([first.root,true,0,first.digest]);
 const fresher=await capture(live,state,{step:"queue:x",owner,fresh:true});
 expect(fresher.digest).not.toBe(first.digest);
 expect((await capture(live,state,{step:"queue:x",owner})).root).toBe(fresher.root);
 await forgetCheckoutSnapshotStepV1(live,"queue:x",state);
 write(live,"a.txt","three");
 expect((await capture(live,state,{step:"queue:x",owner})).digest).not.toBe(fresher.digest);
});

test("a step mapping never outlives its owner or crosses owners, and a step without an owner is never reused",async()=>{
 const live=fresh("owner"),state=fresh("state"),base=checkoutSnapshotBaseV1(live,state);
 write(live,"a.txt","one");
 const first=await capture(live,state,{step:"s",owner:process.pid});
 write(live,"a.txt","two");
 expect((await capture(live,state,{step:"s"})).digest).not.toBe(first.digest);
 expect((await capture(live,state,{step:"s",owner:process.ppid})).digest).not.toBe(first.digest);
 const dead=2147483646;
 write(live,"a.txt","three");
 const recorded=await capture(live,state,{step:"s",owner:process.pid});
 const file=join(base,protocol.stepPrefix+"s");
 writeFileSync(file,JSON.stringify({id:recorded.id,owner:dead}));
 write(live,"a.txt","four");
 expect((await capture(live,state,{step:"s",owner:process.pid})).digest).not.toBe(recorded.digest);
 writeFileSync(join(base,protocol.stepPrefix+"gone"),JSON.stringify({id:recorded.id,owner:dead}));
 writeFileSync(join(base,protocol.stepPrefix+"legacy"),recorded.id);
 writeFileSync(join(base,protocol.stepPrefix+"mine"),JSON.stringify({id:recorded.id,owner:process.pid}));
 await pruneCheckoutSnapshotsV1({workspace:live,state,retained:10,minimumFreeBytes:0,maximumTotalBytes:Number.MAX_SAFE_INTEGER});
 expect(readdirSync(base).filter(name=>name.startsWith(protocol.stepPrefix)).sort()).toEqual([protocol.stepPrefix+"mine",protocol.stepPrefix+"s"].sort());
});

test("capturing a large file does not grow the disk where the filesystem clones (APFS, btrfs, xfs)",async()=>{
 const live=fresh("clone"),state=fresh("state"),probe=join(live,"probe");
 const big=Buffer.alloc(256*1048576,7);
 writeFileSync(probe,big);
 let clones=true;
 try{copyFileSync(probe,probe+".c",constants.COPYFILE_FICLONE_FORCE);rmSync(probe+".c");}catch{clones=false;}
 if(!clones)return;
 const free=():number=>{const row=statfsSync(live);return Number(row.bavail)*Number(row.bsize);};
 let least=Number.POSITIVE_INFINITY;
 for(let attempt=0;attempt<3&&least>64*1048576;attempt++){
  const before=free();
  const snapshot=await capture(live,fresh("state"),{entries:["probe"]});
  least=Math.min(least,before-free());
  expect(readFileSync(join(snapshot.root,"probe")).length).toBe(big.length);
 }
 expect(least).toBeLessThan(64*1048576);
},60_000);

test("a build inside the snapshot compiles the captured bytes although the live sources are rewritten mid-build, and the sources it consumed are exactly the snapshot rows",async()=>{
 const cargo=registeredToolExecutableV1("cargo"),live=fresh("cargo"),state=fresh("state"),signals=fresh("signals"),started=join(signals,"started"),release=join(signals,"release");
 write(live,"Cargo.toml",'[package]\nname="snap"\nversion="0.1.0"\nedition="2021"\nbuild="build.rs"\n[[bin]]\nname="snap"\npath="🧩️é/main.rs"\n[workspace]\n');
 write(live,"build.rs",'fn main(){let s=std::env::var("SNAP_STARTED").unwrap();let r=std::env::var("SNAP_RELEASE").unwrap();std::fs::write(&s,"1").unwrap();for _ in 0..600{if std::path::Path::new(&r).exists(){break;}std::thread::sleep(std::time::Duration::from_millis(50));}println!("cargo::rerun-if-env-changed=SNAP_STARTED");}\n');
 write(live,"🧩️é/main.rs",'mod part;\nfn main(){print!("{}|{}",part::NAME,include_str!("../data/ü.txt"));}\n');
 write(live,"🧩️é/part.rs",'pub const NAME:&str="original";\n');
 write(live,"data/ü.txt","grüezi");
 const run=(args:string[],cwd:string,extra:Record<string,string>={})=>new Promise<{status:number|null;stdout:string;stderr:string}>(resolve=>{
  const child=spawn(cargo,args,{cwd,env:{...process.env,CARGO_BUILD_RUSTC:"rustc",CARGO_BUILD_RUSTC_WRAPPER:"",CARGO_BUILD_BUILD_DIR:args.includes("--target-dir")?args[args.indexOf("--target-dir")+1]!:join(sandbox,"gen"+counter++),...extra}});let stdout="",stderr="";
  child.stdout.on("data",chunk=>{stdout+=chunk;});child.stderr.on("data",chunk=>{stderr+=chunk;});child.on("close",status=>resolve({status,stdout,stderr}));
 });
 const lock=await run(["generate-lockfile","--offline"],live);
 expect(lock.status,lock.stderr).toBe(0);
 writeFileSync(join(signals,"o-release"),"1");
 const oracle=await run(["run","--locked","--offline","--quiet","--target-dir",join(sandbox,"oracle"+counter++)],live,{SNAP_STARTED:join(signals,"o-started"),SNAP_RELEASE:join(signals,"o-release")});
 expect(oracle.status,oracle.stderr+oracle.stdout).toBe(0);
 expect(oracle.stdout).toBe("original|grüezi");
 const snapshot=await capture(live,state),target=join(sandbox,"target"+counter++);
 const building=run(["build","--locked","--offline","--quiet","--target-dir",target],snapshot.root,{SNAP_STARTED:started,SNAP_RELEASE:release});
 for(let waited=0;!existsSync(started)&&waited<600;waited++)await new Promise(resolve=>setTimeout(resolve,50));
 expect(existsSync(started)).toBe(true);
 write(live,"🧩️é/part.rs",'pub const NAME:&str="edited";\n');write(live,"data/ü.txt","tschüss");write(live,"🧩️é/main.rs",'fn main(){print!("rewritten");}\n');
 writeFileSync(release,"1");
 const built=await building;
 expect(built.status,built.stderr).toBe(0);
 const executable=join(target,"debug","snap"+(process.platform==="win32"?".exe":"")),ran=Bun.spawnSync([executable]);
 expect(ran.stdout.toString()).toBe(oracle.stdout);
 const depinfo=readFileSync(join(target,"debug","snap.d"),"utf8"),consumed=new Map<string,string>();
 for(const match of depinfo.matchAll(/^(?:[^\s:]+): (.+)$/gmu))for(const path of match[1]!.split(/(?<!\\) /u).map(part=>part.replaceAll("\\ "," ")))if(path.startsWith(snapshot.root+"/"))consumed.set(path.slice(snapshot.root.length+1),"");
 expect([...consumed.keys()].sort()).toEqual(["build.rs","data/ü.txt","🧩️é/main.rs","🧩️é/part.rs"].sort());
 const rows=await readCheckoutSnapshotRowsV1(snapshot.root),sources=[...consumed.keys()].map(path=>[path,rows.get(path)!] as const);
 const proof=await checkoutSnapshotProvesV1(snapshot.root,sources);
 expect(proof).toEqual({proved:4,missing:[],differing:[]});
 expect((await checkoutSnapshotProvesV1(snapshot.root,[["🧩️é/part.rs","0".repeat(64)]])).differing).toEqual(["🧩️é/part.rs"]);
 expect((await verifyCheckoutSnapshotV1(snapshot.root)).changed).toEqual([]);
 const after=await capture(live,state);
 expect(after.digest).not.toBe(snapshot.digest);
},180_000);

test("a directory is a snapshot of a workspace by its place alone",()=>{
 const state="/m/.cache",origin="/m";
 expect(checkoutSnapshotOfV1("/m/.cache/s/0123456789ab","/m",state)).toBe(true);
 expect(checkoutSnapshotOfV1("/m/.cache/s/0123456789ab/sub","/m",state)).toBe(false);
 expect(checkoutSnapshotOfV1("/m/.cache/s/xyz","/m",state)).toBe(false);
 expect(checkoutSnapshotOfV1("/other/.cache/s/0123456789ab","/m",state)).toBe(false);
 expect(checkoutSnapshotOfV1("/m","/m",state)).toBe(false);
});
