/** 🚧️ Law: the parent of a read never walks closures, never runs TypeScript resolution, never derives a closure entry, never resolves the closure references of an owner and never infers an owner while a farm exists. Every function that does is called in a process marked as the parent of a read and must throw the refusal of the guard; the references of an owner are asked of a child, and a source that does not parse in the child withholds the owner in the parent without a derivation.
 * @see ../../../../../👁️observation/🚧️read-parent/🟨️.mjs */
import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";
import{join}from"node:path";
import{markReadParentV1}from"../../../../../👁️observation/🚧️read-parent/🟨️.mjs";
import{askOwnerRefsV1,inferProjectBodyV1,recordHeaderDigestV1,refsFailureV1}from"../../🟨️.mjs";
import{deriveClosureFactV1,resolutionConfigsV1}from"../../../🧭️script-inputs/🟨️.mjs";
import{resolveClosureRefsV1}from"../../../🧱️project-assembly/🟨️.mjs";

const source=readFileSync(join(import.meta.dir,"..","..","🟨️.mjs"),"utf8");
const refused=async(call:()=>unknown,kind:string)=>{
 let caught:any;try{await call();}catch(error){caught=error;}
 expect(caught?.code,kind).toBe("graph.read-parent-computes");expect(caught.kind).toBe(kind);
};

test("in a process marked as the parent of a read every walk, derive, resolve and infer function refuses; with the farm gone the guard lets them run into their own work",async()=>{
 let farm=true;const unmark=markReadParentV1(()=>farm);
 await refused(()=>inferProjectBodyV1({workspaceRoot:"/w"} as never,{name:"p",root:"p"} as never,undefined),"infer");
 await refused(()=>deriveClosureFactV1({path:"/x",kind:"file"} as never,{} as never),"walk");
 await refused(()=>resolutionConfigsV1({} as never,{} as never,"/w",[]),"resolve");
 await refused(()=>resolveClosureRefsV1({name:"p"} as never,"/w",{} as never),"refs");
 farm=false;
 let caught:any;try{await inferProjectBodyV1({workspaceRoot:"/w"} as never,{name:"p",root:"p"} as never,undefined);}catch(error){caught=error;}
 expect(caught?.code,"no farm: the body runs and fails on its own missing environment, not on the guard").not.toBe("graph.read-parent-computes");
 unmark();
});

const job={name:"p",root:"p"};
const farmOf=(replies:(object|undefined)[],alive:(()=>boolean)=()=>true)=>{const asked:object[]=[];return{asked,exists:alive,observe:async(each:object)=>{asked.push(each);return replies.shift();}};};
const holds={verdict:"holds",headerDigest:"d".repeat(64),refs:{path:"/r",digest:"x",size:1,headerDigest:"d".repeat(64),resolver:1}};

test("the references of an owner come from a child that holds the record the parent holds; a failure is asked again and refused after three tries; no farm answers undefined",async()=>{
 const good=farmOf([holds]);
 expect((await askOwnerRefsV1({farm:good as never,job,headerDigest:"d".repeat(64)}))?.refs).toBe(holds.refs);
 expect(good.asked[0]).toMatchObject({observe:true,refs:true});
 const flaky=farmOf([{verdict:"holds",headerDigest:"d".repeat(64),refs:{failed:"entry stale"}},holds]);
 expect((await askOwnerRefsV1({farm:flaky as never,job,headerDigest:"d".repeat(64)}))?.refs).toBe(holds.refs);
 const broken=farmOf([1,2,3].map(()=>({verdict:"holds",headerDigest:"d".repeat(64),refs:{failed:"entry stale"}})));
 let caught:any;try{await askOwnerRefsV1({farm:broken as never,job:{...job,name:"@semio-tech/x"},headerDigest:"d".repeat(64)});}catch(error){caught=error;}
 expect(caught?.code).toBe("graph.farm-job-failed");expect(caught.message).toContain("@semio-tech/x");expect(caught.message).toContain("entry stale");
 expect(await askOwnerRefsV1({farm:farmOf([holds],()=>false) as never,job,headerDigest:"d".repeat(64)}),"the farm is gone").toBeUndefined();
 expect(await askOwnerRefsV1({farm:farmOf([undefined]) as never,job,headerDigest:"d".repeat(64)}),"the farm ended while asking").toBeUndefined();
});

test("a source that does not parse is reported by the child as data and failures are named, never thrown across the farm",()=>{
 const unparseable=refsFailureV1(Object.assign(Error("bad json"),{code:"BUILD_SOURCE_UNPARSEABLE",source:"/w/p/x.json",sha256:"s".repeat(64)}));
 expect(unparseable).toEqual({unparseable:{source:"/w/p/x.json",sha256:"s".repeat(64),message:"bad json"}});
 expect(refsFailureV1(Error("entry   stale\nnow"))).toEqual({failed:"entry stale now"});
 expect(recordHeaderDigestV1("/no/such/record.json")).toBeUndefined();
});

test("the read marks its parent while the farm exists, starts the children at once, asks a child for every verdict without a time limit, primes the shared closures in a child and never resolves references in the parent while a farm exists",()=>{
 expect(source).toContain("immediate:true");
 expect(source).toContain("withReadParentScopeV1(readScope=>emojiProjectJsonNodesBodyV1(");
 expect(source).toContain("readScope.arm(()=>farm.exists())");
 expect(source).toContain("unmarkParent?.();await farm?.close()");
 expect(source).toContain("verdictFinal:farm!==undefined");
 expect(source).toContain("verdict:observeJob===undefined?undefined:");
 expect(source).not.toContain("verdictWithinV1(farm.observe(");
 expect(source).toContain("await primeSharedClosuresInChildV1({workspaceRoot,storeDirectory");
 expect(source).toContain("askOwnerRefsV1({farm,job:observeJob,headerDigest})");
 expect(source).toContain('assertNotReadParentV1("infer", name)');
});
