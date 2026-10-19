import{test,expect,spyOn}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import ts from"typescript";
import{readFileSync,readdirSync,mkdtempSync,mkdirSync,realpathSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import corpus from"./🔣️.json"with{type:"json"};
import contract from"./🧬️schema.json"with{type:"json"};
import storeSchema from"../../🧬️schema/🔣️.json"with{type:"json"};
import budgetSchema from"../../../../🧬️schema/🔣️.json"with{type:"json"};
import{repositoryCacheOrdinaryMaintenanceV1}from"../../../../🟦️.ts";
import{withBuildTestInvocationV1}from"@semio-tech/process/testing/invocation";
import{createIssuedResourceControlV1,scriptInvocationEnvironment}from"../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import*as leases from"@semio-tech/process/leases";
import{captureOwnedCommandResultV1}from"@semio-tech/process/execution";

const sitePattern=/\bwait\s*:\s*false\b|\bwithRepositoryCacheStoreMaintenanceV1\s*\(/u,oracleProgram=`import json,os,re,sys
spec=json.loads(sys.argv[1]);pattern=re.compile(r"\\bwait\\s*:\\s*false\\b|\\bwithRepositoryCacheStoreMaintenanceV1\\s*\\(");rows={}
for root in spec["roots"]:
 for directory,names,files in os.walk(root):
  names[:]=[name for name in names if name not in spec["excluded"]]
  for file in files:
   if not file.endswith((".ts",".mts",".mjs")):continue
   path=os.path.join(directory,file)
   with open(path,encoding="utf8") as handle:count=len(pattern.findall(handle.read()))
   if count:rows[path.replace(os.sep,"/")]=count
print(json.dumps(rows,sort_keys=True))`;

function countSites(path:string,text:string):number{
 if(!sitePattern.test(text))return 0;
 const file=ts.createSourceFile(path,text,ts.ScriptTarget.Latest,true,path.endsWith(".mjs")?ts.ScriptKind.JS:ts.ScriptKind.TS);let sites=0;
 const visit=(node:ts.Node):void=>{
  if(ts.isCallExpression(node)&&ts.isIdentifier(node.expression)&&node.expression.text==="withRepositoryCacheStoreMaintenanceV1")sites++;
  if((ts.isPropertyAssignment(node)||ts.isPropertySignature(node))&&node.name.getText(file)==="wait"){const falsy=ts.isPropertyAssignment(node)?node.initializer.kind===ts.SyntaxKind.FalseKeyword:!!node.type&&ts.isLiteralTypeNode(node.type)&&node.type.literal.kind===ts.SyntaxKind.FalseKeyword;if(falsy)sites++;}
  ts.forEachChild(node,visit);
 };
 visit(file);return sites;
}
function productionFiles(root:string,excluded:ReadonlySet<string>):string[]{
 const files:string[]=[],pending=[root];
 while(pending.length){const directory=pending.pop()!;for(const entry of readdirSync(directory,{withFileTypes:true})){if(excluded.has(entry.name)||entry.isSymbolicLink())continue;const path=join(directory,entry.name).replaceAll("\\","/");if(entry.isDirectory())pending.push(path);else if(/\.(?:ts|mts|mjs)$/u.test(path))files.push(path);}}
 return files;
}
function productionSites():Record<string,number>{
 const rows:Record<string,number>={},excluded=new Set(corpus.excludedSegments);
 for(const root of corpus.roots)for(const path of productionFiles(root,excluded)){const sites=countSites(path,readFileSync(path,"utf8"));if(sites)rows[path]=sites;}
 return Object.fromEntries(Object.entries(rows).sort(([left],[right])=>left<right?-1:left>right?1:0));
}

test("queued cache maintenance corpus is closed and the ordinary request agrees with Ajv and SQLite",()=>{
 const validate=new Ajv({strict:true}).compile(contract);expect(validate(corpus)).toBe(true);
 const request=repositoryCacheOrdinaryMaintenanceV1();expect<unknown>(request).toEqual(corpus.request);expect(budgetSchema.$defs.ordinaryMaintenance.const).toEqual(corpus.request);
 expect(new Ajv({strict:true}).validate(storeSchema.definitions.QueuedMaintenanceRequest,request)).toBe(true);expect(Object.isFrozen(request)).toBe(true);
 const database=new Database(":memory:");
 try{
  database.exec("create table queued(id text primary key,path text unique);create table nonwaiting(path text primary key,sites integer,kind text)");
  for(const row of corpus.queued)database.query("insert into queued values(?,?)").run(row.id,row.path);
  for(const row of corpus.nonwaiting)database.query("insert into nonwaiting values(?,?,?)").run(row.path,row.sites,row.kind);
  expect(database.query("select count(*) rows from queued q join nonwaiting n on n.path=q.path where n.kind not in ('inherited','opportunistic')").get()).toEqual({rows:0});
  expect(database.query("select count(*) rows,sum(sites) sites from nonwaiting").get()).toEqual({rows:corpus.nonwaiting.length,sites:corpus.nonwaiting.reduce((sum,row)=>sum+row.sites,0)});
  expect(database.query("select count(*) rows from (select 1 from json_each(?1) where json_extract(value,'$.maximumWaitMilliseconds') between 1 and 3600000)").get(JSON.stringify([request]))).toEqual({rows:1});
 }finally{database.close();}
});

test("every queued caller of whole-image maintenance carries its declared queued admission",()=>{
 for(const row of corpus.queued){const text=readFileSync(row.path,"utf8");for(const token of row.tokens)expect(text.includes(token),row.id+" lacks "+token).toBe(true);}
});

test("nonwaiting cache maintenance exists only where the declared roster explains it",()=>withBuildTestInvocationV1(async original=>{
 const declared=Object.fromEntries(corpus.nonwaiting.map(row=>[row.path,row.sites])),actual=productionSites();expect(actual).toEqual(Object.fromEntries(Object.entries(declared).sort(([left],[right])=>left<right?-1:left>right?1:0)));
 const python=Bun.which("python3");if(!python)throw Error("Independent Python scanning oracle required");
 const result=await captureOwnedCommandResultV1(python,["-I","-c",oracleProgram,JSON.stringify({roots:corpus.roots,excluded:corpus.excludedSegments})],process.cwd(),{control:createIssuedResourceControlV1(original,"fixture:nonwaiting-oracle",0),env:scriptInvocationEnvironment(original,process.env),output:{maximumBytes:1048576,maximumLines:4096}});
 expect(result.status,result.stderr).toBe(0);expect(JSON.parse(result.stdout)).toEqual(actual);
}),120000);

const heldProgram=`import sqlite3,sys
db=sqlite3.connect(sys.argv[1],timeout=0)
try:
 db.execute("BEGIN EXCLUSIVE");print("free");db.rollback()
except sqlite3.OperationalError as error:
 if "locked" not in str(error):raise
 print("held")
finally:db.close()`;

test("a concurrent shared holder makes queued maintenance wait with progress and cancellation while opportunistic maintenance defers",()=>withBuildTestInvocationV1(async original=>{
 const python=Bun.which("python3");if(!python)throw Error("Independent Python SQLite oracle required");
 for(const scenario of corpus.scenarios){
  const parent=mkdtempSync(join(realpathSync(tmpdir()),"qm-")),directory=join(parent,"l"),abort=new AbortController(),resource="native-fixed-image:"+join(parent,"i"),path=leases.resourceLeasePathV1(directory,resource),waits:string[]=[],events:string[]=[];mkdirSync(directory);
  const control=(name:string,signal?:AbortSignal)=>createIssuedResourceControlV1(original,"test:queued-"+name,60000,signal),oracle=async(expected:string)=>{const result=await captureOwnedCommandResultV1(python,["-I","-c",heldProgram,path],process.cwd(),{control:createIssuedResourceControlV1(original,"fixture:queued-oracle",0),env:scriptInvocationEnvironment(original,process.env),output:{maximumBytes:4096,maximumLines:32}});expect(result.status,result.stderr).toBe(0);expect(result.stdout.trim()).toBe(expected);};
  const shared=(name:string)=>{const actual=control(name);return leases.acquireQueuedResourceLease({directory,resource,mode:"shared",owner:name,signal:actual.signal,control:actual,sharedCohort:true});};
  const holder=await shared("holder");let holderReleased=false;const releaseHolder=()=>{if(!holderReleased){holderReleased=true;holder.release();}};
  try{
   const maintenance=control("maintenance",abort.signal),queued=scenario.requester==="queued",started=performance.now();
   if(!queued){await expect(leases.acquireResourceLease({directory,resource,mode:"exclusive",signal:maintenance.signal,control:maintenance,timeoutMs:0})).rejects.toThrow("Resource lease timed out: "+resource);expect(performance.now()-started).toBeLessThan(1000);await oracle("held");expect(scenario.expect).toBe("not-acquired");continue;}
   let settled=false;const pending=leases.acquireQueuedResourceLease({directory,resource,mode:"exclusive",owner:"maintenance",signal:maintenance.signal,control:maintenance,onWait:async event=>{waits.push(event.resource);}}).then(lease=>{settled=true;events.push("maintenance");return lease;});void pending.catch(()=>{});
   await Bun.sleep(1200);expect(settled).toBe(false);expect(waits.length).toBeGreaterThan(0);expect(waits.every(row=>row===resource)).toBe(true);await oracle("held");
   if(scenario.event==="release"){releaseHolder();const lease=await pending;expect(lease.mode).toBe("exclusive");await oracle("held");lease.release();await oracle("free");}
   else if(scenario.event==="cancel"){abort.abort(Object.freeze(Error("Owned queued maintenance cancellation")));await expect(pending).rejects.toThrow(/abort|cancel/iu);expect(maintenance.signal.aborted).toBe(true);expect(settled).toBe(false);await oracle("held");releaseHolder();await oracle("free");}
   else{
    const late=shared("late-reader").then(lease=>{events.push("late-reader");return lease;});void late.catch(()=>{});await Bun.sleep(600);expect(events).toEqual([]);releaseHolder();const lease=await pending;expect(events).toEqual(["maintenance"]);lease.release();const reader=await late;expect(events).toEqual(["maintenance","late-reader"]);reader.release();await oracle("free");
   }
  }finally{releaseHolder();rmSync(parent,{recursive:true,force:true});}
 }
}),180000);
