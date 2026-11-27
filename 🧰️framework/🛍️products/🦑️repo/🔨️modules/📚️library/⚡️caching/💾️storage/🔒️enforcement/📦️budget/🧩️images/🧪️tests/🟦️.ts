/** 🧩️ Tests for the split of a partition into several images: the schema bound of 32 GiB per image, the re-layout plan and the physical move below a fake image lifecycle, all checked against an independent Python model. */
import{test,expect}from"bun:test";
import{mkdirSync,mkdtempSync,readFileSync,readdirSync,rmSync,writeFileSync,existsSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{validateJsonSchemaSubset}from"../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import{nativeImageShardFoldV1,nativeImageShardHomeLayoutV1,nativeImageShardLayoutsV1,readNativeImageShardLedgerV1,requireNativeImageShardRuleV1,type NativeImageShardLayoutV1}from"../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🧩️shard/🟦️.ts";
import budgetSchema from"../../🧬️schema/🔣️.json"with{type:"json"};
import policy from"../../🔣️policy.json"with{type:"json"};
import cases from"./🔣️.json"with{type:"json"};
import{relayoutRepositoryPartitionV1,repositoryPartitionImageOfV1,repositoryRelayoutPlanV1,type RepositoryPartitionImageV1}from"../🟦️.ts";
const GiB=1073741824,rule=requireNativeImageShardRuleV1(policy.partitions.rust.imageShards);
const python=(source:string,input:unknown):unknown=>{const run=Bun.spawnSync(["python3","-I","-c",source,JSON.stringify(input)]);expect(run.exitCode,run.stderr.toString()).toBe(0);return JSON.parse(run.stdout.toString());};
const mutate=(path:readonly(string|number)[],value:unknown)=>{const copy=structuredClone(policy) as Record<string,unknown>;if(path.length===0)return copy;let at=copy as Record<string|number,unknown>;for(const key of path.slice(0,-1))at=at[key] as Record<string|number,unknown>;at[path[path.length-1]!]=value;return copy;};

test("no declared image exceeds 32 GiB: the schema refuses every larger image, and the independent model agrees on every case",()=>{
 expect(budgetSchema.$defs.imageCap.const.maximumImageBytes).toBe(32*GiB);
 expect(validateJsonSchemaSubset(budgetSchema,policy)).toEqual([]);
 const rows=cases.bounds.map(row=>{let policyCopy=mutate(row.path,row.value);const extra=(row as{extra?:{path:(string|number)[];value:unknown}}).extra;if(extra){const at=structuredClone(policyCopy) as Record<string,unknown>;let node=at as Record<string|number,unknown>;for(const key of extra.path.slice(0,-1))node=node[key] as Record<string|number,unknown>;node[extra.path[extra.path.length-1]!]=extra.value;policyCopy=at;}return{id:row.id,policy:policyCopy,valid:row.valid};});
 const model=`import json,sys
G=2**35
def ok(p):
    for v in p["partitions"].values():
        s=v.get("imageShards")
        if not(s and s["enabled"]) and v["capacityBytes"]>G:return False
        if s and (s.get("maximumImageBytes",0)>G or any(r["needBytes"]>G for r in s["roots"])):return False
    for v in p["elastic"].values():
        if any(v[k]>G for k in("maximumBytes","minimumBytes","floorBytes","needBytes")):return False
    return True
print(json.dumps({r["id"]:ok(r["policy"]) for r in json.loads(sys.argv[1])}))`;
 const independent=python(model,rows) as Record<string,boolean>;
 for(const row of rows){expect(validateJsonSchemaSubset(budgetSchema,row.policy).length===0,row.id).toBe(row.valid);expect(independent[row.id],row.id).toBe(row.valid);}
});
test("the declared Rust partition is split: every image is at most 32 GiB and each sub-area has its own image",()=>{
 expect(policy.elastic.rust.maximumBytes).toBeLessThanOrEqual(32*GiB);expect(policy.partitions.rust.imageShards.enabled).toBe(true);
 expect(policy.partitions.rust.imageShards.roots.map(root=>root.prefix)).toEqual(["u/l","u","" ]);
 for(const[,row]of Object.entries(policy.partitions))if(!("imageShards"in row&&(row as{imageShards:{enabled:boolean}}).imageShards.enabled))expect(row.capacityBytes).toBeLessThanOrEqual(32*GiB);
});
test("the re-layout plan: the cases are answered by the implementation and by the independent model",()=>{
 const model=`import json,sys,re,hashlib
d=json.loads(sys.argv[1]);rule=d["rule"];A=".relayout-";out={}
def ident(k):return k if re.fullmatch(r"[a-z0-9]{1,12}",k) else "x"+hashlib.sha256(k.encode()).hexdigest()[:11]
for c in d["plans"]:
    steps=[]
    for i,r in enumerate(rule["roots"]):
        names=c["children"].get(r["prefix"],[]);asides={n[len(A):] for n in names if n.startswith(A)}
        keys=sorted({n for n in names if not n.startswith(A)}|asides)
        for k in keys:
            if not re.match(r["pattern"],k):continue
            steps.append({"id":ident(k),"key":k,"root":i,"path":(r["prefix"]+"/"+k) if r["prefix"] else k,"action":"attach" if ident(k) in c["live"] and k not in asides else "migrate"})
    out[c["id"]]=steps
print(json.dumps(out))`;
 const independent=python(model,{rule,plans:cases.plans}) as Record<string,unknown>;
 for(const row of cases.plans){
  const live=new Map(row.live.map(id=>[id,{id,key:id,root:0,path:"",capacityBytes:GiB}]));
  const plan=repositoryRelayoutPlanV1(rule,prefix=>(row.children as Record<string,string[]>)[prefix]??[],live);
  expect(plan,row.id).toEqual(row.steps as never);expect(independent[row.id],row.id).toEqual(row.steps);
 }
});
function fixture(){
 const directory=mkdtempSync(join(tmpdir(),"semio-relayout-")),home=nativeImageShardHomeLayoutV1(directory),mounted=new Set<string>([home.mountPoint]);
 const put=(path:string,text:string)=>{const file=join(home.mountPoint,path);mkdirSync(join(file,".."),{recursive:true});writeFileSync(file,text);};
 put("u/l/pdf7c0a/t/a.bin","lane-a");put("u/l/p4fa76d/t/b.bin","lane-b");put("u/sealed/s1","sealed");put("u/w/w1","work");put("u/ballast/keep","home");put("cargo/home/registry/c1","crate");put("components/runs/r1","run");put("b/p/keep","home");
 const lifecycle={
  async create(layout:NativeImageShardLayoutV1,capacityBytes:number){mkdirSync(layout.directory,{recursive:true});writeFileSync(layout.image,"");writeFileSync(layout.receipt,JSON.stringify({capacityBytes}));},
  async attach(layout:NativeImageShardLayoutV1){mkdirSync(layout.mountPoint,{recursive:true});mounted.add(layout.mountPoint);},
  async detach(layout:NativeImageShardLayoutV1){mounted.delete(layout.mountPoint);},
  async retire(layout:NativeImageShardLayoutV1){mounted.delete(layout.mountPoint);rmSync(layout.directory,{recursive:true,force:true});}
 };
 const run=(reserveBytes=0)=>relayoutRepositoryPartitionV1({directory,rule,platform:"darwin",lifecycle:lifecycle as never,lock:body=>body(),hostFreeBytes:()=>200*GiB,reserveBytes,now:()=>1,mounted:path=>mounted.has(path)});
 return{directory,home,put,run,mounted};
}
test("the re-layout gives every declared sub-area its own image at its logical path, keeps every file where it was and is repeatable",async()=>{
 const{directory,home,run}=fixture();
 try{
  const first=await run();
  expect(first.status).toBe("done");expect(first.steps.map(step=>step.result)).toEqual(Array(6).fill("born"));expect(first.movedBytes).toBe(["lane-a","lane-b","sealed","work","crate","run"].reduce((sum,text)=>sum+text.length,0));
  const read=(path:string)=>readFileSync(join(home.mountPoint,path),"utf8");
  expect([read("u/l/pdf7c0a/t/a.bin"),read("u/l/p4fa76d/t/b.bin"),read("u/sealed/s1"),read("u/w/w1"),read("cargo/home/registry/c1"),read("components/runs/r1"),read("u/ballast/keep"),read("b/p/keep")]).toEqual(["lane-a","lane-b","sealed","work","crate","run","home","home"]);
  expect(readdirSync(join(home.mountPoint,"u/l")).sort()).toEqual(["p4fa76d","pdf7c0a"]);expect(readdirSync(home.mountPoint).some(name=>name.startsWith(".relayout-"))).toBe(false);
  const live=nativeImageShardFoldV1(readNativeImageShardLedgerV1(directory));expect([...live.keys()].sort()).toEqual(["cargo","components","p4fa76d","pdf7c0a","sealed","w"]);
  for(const record of live.values())expect(record.capacityBytes).toBeLessThanOrEqual(32*GiB);
  const layouts=nativeImageShardLayoutsV1(directory,live);expect(layouts.map(layout=>layout.id)).toEqual(["home","cargo","components","p4fa76d","pdf7c0a","sealed","w"]);
  const images=layouts.map(layout=>({...layout,logicalPath:layout.mountPoint.slice(home.mountPoint.length+1)})) as RepositoryPartitionImageV1[];
  expect(repositoryPartitionImageOfV1(images,"u/l/pdf7c0a/t/a.bin").id).toBe("pdf7c0a");expect(repositoryPartitionImageOfV1(images,"u/ballast/keep").id).toBe("home");expect(repositoryPartitionImageOfV1(images,"cargo/home/x").id).toBe("cargo");
  const again=await run();expect(again.steps.map(step=>step.result)).toEqual(Array(6).fill("attached"));expect(again.movedBytes).toBe(0);
 }finally{rmSync(directory,{recursive:true,force:true});}
});
test("a volume below its reserve stops the move cleanly and a repeat resumes it; an unmounted home image is left alone",async()=>{
 const{directory,home,run,mounted}=fixture();
 try{
  const stopped=await run(1000*GiB);expect(stopped.status).toBe("space");expect(stopped.movedBytes).toBe(0);
  expect(existsSync(join(home.mountPoint,"u/l/.relayout-p4fa76d/t/b.bin"))).toBe(true);
  const resumed=await run();expect(resumed.status).toBe("done");expect(readFileSync(join(home.mountPoint,"u/l/p4fa76d/t/b.bin"),"utf8")).toBe("lane-b");expect(existsSync(join(home.mountPoint,"u/l/.relayout-p4fa76d"))).toBe(false);
  mounted.delete(home.mountPoint);expect((await run()).status).toBe("unmounted");
 }finally{rmSync(directory,{recursive:true,force:true});}
});
