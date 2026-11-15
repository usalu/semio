/** 🩹️ A vanished build input closure leaf is a miss that is repaired, never a permanent failure. */
import{expect,test}from"bun:test";
import{mkdirSync,mkdtempSync,readdirSync,realpathSync,rmSync,writeFileSync,existsSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{brotliCompressSync}from"node:zlib";
import{createHash}from"node:crypto";
import{closureMissingErrorV1,closureMissingLineV1,closureOwnersV1,closureReferencesOfTextV1,closuresOfFragmentsV1,obtainClosureLeafV1,purgeClosureReferencesV1,readClosureLeafV1,settleClosureLeafV1}from"../🟨️.mjs";

const KEY="b".repeat(64),OTHER="c".repeat(64);
const put=(file:string,text:string)=>writeFileSync(file,brotliCompressSync(Buffer.from(text)));

test("records that name the missing leaf are removed with their fleet marker, others stay; a dry run removes nothing",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"closure-repair-"));
 try{
  const np=join(root,"inference/selections/"+"a".repeat(64)+"/np"),nodes=join(root,"derived-nodes/n");mkdirSync(np,{recursive:true});mkdirSync(nodes,{recursive:true});
  put(join(np,"p1.json"),'{"inputs":["SEMIO_SOURCE_CLOSURE_'+KEY+'"]}');writeFileSync(join(np,"p1.json.fleet"),"x");
  put(join(np,"p2.json"),'{"inputs":["SEMIO_SOURCE_CLOSURE_'+OTHER+'"]}');
  writeFileSync(join(np,"p3.json"),'plain text naming '+KEY);
  put(join(nodes,"n1"),'{"compact":{"digest":"'+KEY+'"}}');put(join(nodes,"n2"),'{"compact":{"digest":"'+OTHER+'"}}');
  expect(await purgeClosureReferencesV1({root,key:KEY,dryRun:true})).toBe(3);expect(readdirSync(np).length).toBe(4);
  expect(await purgeClosureReferencesV1({root,key:KEY})).toBe(3);
  expect(readdirSync(np).sort()).toEqual(["p2.json"]);expect(readdirSync(nodes)).toEqual(["n2"]);expect(existsSync(join(np,"p1.json.fleet"))).toBe(false);
  expect(await purgeClosureReferencesV1({root,key:KEY})).toBe(0);
  await expect(purgeClosureReferencesV1({root,key:"nope"})).rejects.toThrow("closure key");
 }finally{rmSync(root,{recursive:true,force:true});}
});

const sha=(bytes:Uint8Array|string)=>createHash("sha256").update(bytes).digest("hex");
const leafBytes=Buffer.from('{"files":["a.ts"]}'),LEAF=sha(leafBytes),NS="build-input-closures" as const;
const withDirectory=<T>(run:(directory:(namespace:string)=>string,base:string)=>Promise<T>)=>async()=>{
 const base=mkdtempSync(join(realpathSync(tmpdir()),"closure-obtain-"));
 try{mkdirSync(join(base,NS),{recursive:true});return await run(namespace=>join(base,namespace),base);}finally{rmSync(base,{recursive:true,force:true});}
};

test("a leaf on this machine is present, an absent one is fetched from the fleet and verified by its name, a wrong, empty, failing or missing answer is only missing",withDirectory(async directory=>{
 const put=async(namespace:string,key:string,bytes:Uint8Array)=>{writeFileSync(join(directory(namespace),key+".json"),bytes);},asks:string[]=[];
 const fetch=(bytes:Uint8Array|undefined|"throw")=>async(_namespace:string,key:string)=>{asks.push(key);if(bytes==="throw")throw Error("network");return bytes;};
 const call=(key:string,fetcher?:ReturnType<typeof fetch>)=>obtainClosureLeafV1({directory:directory as never,namespace:NS,key,fetch:fetcher as never,put:put as never});
 expect(await call(LEAF,fetch(undefined))).toBe("missing");
 expect(await call(LEAF,fetch(Buffer.from("other")))).toBe("missing");
 expect(await call(LEAF,fetch(Buffer.alloc(0)))).toBe("missing");
 expect(await call(LEAF,fetch("throw"))).toBe("missing");
 expect(await call(LEAF)).toBe("missing");
 expect(existsSync(join(directory(NS),LEAF+".json"))).toBe(false);
 expect(await call(LEAF,fetch(leafBytes))).toBe("refetched");
 expect(readdirSync(directory(NS))).toEqual([LEAF+".json"]);
 asks.length=0;expect(await call(LEAF,fetch(leafBytes))).toBe("present");expect(asks).toEqual([]);
 await expect(call("nope")).rejects.toThrow("closure key");
}));

test("reading an absent leaf obtains it and reads again, and stays a typed closure-missing error that is never an ENOENT",async()=>{
 const enoent=()=>Object.assign(Error("ENOENT: lstat 'x'"),{code:"ENOENT"});let present=false;
 const read=()=>{if(!present)throw enoent();return"value";};
 expect(await readClosureLeafV1({namespace:NS,key:LEAF,read,obtain:async()=>{present=true;return"refetched";}})).toBe("value");
 present=false;
 const failure=await readClosureLeafV1({namespace:NS,key:LEAF,read,obtain:async()=>"missing",projects:()=>["a","b"]}).catch(error=>error);
 expect(failure.code).toBe("closure-missing");expect(failure.message).not.toContain("ENOENT:");expect(failure.message).toBe("closure-missing:"+LEAF+" "+NS+" leaf is not on this machine");
 expect(failure.missingClosure).toBe(LEAF);expect(failure.missingProjects).toEqual(["a","b"]);expect(failure.cause.code).toBe("ENOENT");
 await expect(readClosureLeafV1({namespace:NS,key:LEAF,read:()=>{throw Object.assign(Error("EACCES"),{code:"EACCES"});}})).rejects.toThrow("EACCES");
 expect(closureMissingErrorV1({namespace:NS,key:LEAF}).missingProjects).toEqual([]);
});

test("the leaves a text names are its closure tokens, each once, with their namespace",()=>{
 const lock=sha("lock");
 const found=closureReferencesOfTextV1('{"a":{"env":"SEMIO_SOURCE_CLOSURE_'+LEAF+'"},"b":"SEMIO_SOURCE_CLOSURE_'+LEAF+'","c":"SEMIO_CARGO_LOCK_'+lock+'","d":"SEMIO_SOURCE_CLOSURE_short"}');
 expect([...found]).toEqual([[LEAF,NS],[lock,"build-lock-closures"]]);
});

test("the leaves of brotli fragments are read once each (memo) and the owners that name them come from the owner table",async()=>{
 const other=sha("o"),docs:Record<string,Buffer>={f1:brotliCompressSync(Buffer.from('{"node":{"env":"SEMIO_SOURCE_CLOSURE_'+LEAF+'"}}')),f2:brotliCompressSync(Buffer.from('{"node":{"env":"SEMIO_SOURCE_CLOSURE_'+LEAF+'","x":"SEMIO_SOURCE_CLOSURE_'+other+'"}}')),f3:brotliCompressSync(Buffer.from("{}"))};
 const reads:string[]=[],memo=new Map(),read=async(name:string)=>{reads.push(name);return docs[name];};
 const named=await closuresOfFragmentsV1({names:["f1","f2","f3","gone"],read,memo});
 expect([...named.keys()].sort()).toEqual([LEAF,other].sort());
 const owners=closureOwnersV1(named,{b:{fragment:"f2"},a:{fragment:"f1"},c:{fragment:"f3"},d:{fragment:"f1"}});
 expect(owners.get(LEAF)).toEqual({namespace:NS,projects:["a","b","d"]});expect(owners.get(other)).toEqual({namespace:NS,projects:["b"]});
 reads.length=0;await closuresOfFragmentsV1({names:["f1","f2","f3"],read,memo});expect(reads).toEqual([]);
});

test("an absent leaf is settled in the request: refetched when the fleet has it, else its projects are marked stale (re-derived, or asked of the fleet when heavy), one exact line each",async()=>{
 const lines:string[]=[],purged:string[]=[],say=(line:string)=>lines.push(line),projects=["a","b","c"];
 expect(closureMissingLineV1({key:LEAF,count:3,outcome:"refetched"})).toBe("[graph] closure-missing "+LEAF+" named by 3 projects: refetched");
 expect(await settleClosureLeafV1({key:LEAF,namespace:NS,projects,obtain:async()=>"refetched",purge:async key=>{purged.push(key);},say})).toBe("refetched");
 expect(purged).toEqual([]);
 expect(await settleClosureLeafV1({key:LEAF,namespace:NS,projects,obtain:async()=>"missing",purge:async key=>{purged.push(key);},heavy:()=>false,say})).toBe("re-derived");
 expect(purged).toEqual([LEAF]);
 expect(await settleClosureLeafV1({key:LEAF,namespace:NS,projects,obtain:async()=>"missing",purge:async key=>{purged.push(key);},heavy:names=>names.includes("c"),say})).toBe("asked of the fleet");
 expect(lines).toEqual(["refetched","re-derived","asked of the fleet"].map(outcome=>"[graph] closure-missing "+LEAF+" named by 3 projects: "+outcome));
});
