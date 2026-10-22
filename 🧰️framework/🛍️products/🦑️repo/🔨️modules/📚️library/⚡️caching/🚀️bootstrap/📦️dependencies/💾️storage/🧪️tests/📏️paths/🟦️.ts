import{withBuildTestInvocationV1}from"../../../../../../../../../../🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts";
import {test,expect} from "bun:test";
import Ajv from "ajv";
import {createHash} from "node:crypto";
import {join} from "node:path";
import {appendFileSync,chmodSync,lstatSync,mkdirSync,mkdtempSync,readFileSync,renameSync,rmSync,symlinkSync,unlinkSync,utimesSync,writeFileSync} from "node:fs";
import {javascriptInstallSchemaV1 as installSchema} from "../../🧱️schemas/🟦️.ts";
import corpus from "./🔣️.json";

type Scan=(root:string,control:unknown,check:()=>void)=>Promise<{version:1;entries:number;directories:number;files:number;links:number;maximumPathUnits:number;sha256:string}>;
type Tree=readonly Readonly<{path:string;kind:"directory"|"file"|"link";target?:string}>[];

const python=`import os,sys,json,hashlib,stat
r=json.loads(sys.stdin.read())["root"]; rows=[]; counts={"entries":0,"directories":0,"files":0,"links":0,"maximumPathUnits":0}; work=[0]
def walk(p,q):
 s=os.lstat(p); k="link" if stat.S_ISLNK(s.st_mode) else "directory" if stat.S_ISDIR(s.st_mode) else "file" if stat.S_ISREG(s.st_mode) else "foreign"
 counts["entries"]+=1; counts["maximumPathUnits"]=max(counts["maximumPathUnits"],len(p.encode("utf-16-le"))//2); counts[{"link":"links","directory":"directories","file":"files"}[k]]+=1; rows.append(json.dumps([q,k],ensure_ascii=False,separators=(",",":"))+"\\n"); work[0]+=1+len(q)
 if k=="directory":
  names=sorted(os.listdir(p),key=lambda n:n.encode("utf-8")); work[0]+=len(names)+1
  for n in names: walk(os.path.join(p,n),n if q=="." else q+"/"+n)
walk(r,"."); print(json.dumps(dict(version=1,sha256=hashlib.sha256("".join(rows).encode()).hexdigest(),work=work[0],**counts)))`;

const build=(root:string,tree:Tree)=>{for(const row of tree){const path=join(root,row.path);if(row.kind==="directory")mkdirSync(path);else if(row.kind==="file")writeFileSync(path,"retained");else symlinkSync(row.target!,path);}};

/** 📏️ Pins the installed path scan to an independent Python walk and to its closed mutation table: every change to a directory between its listing and its close is refused. */
test("installed dependency path scan refuses every directory change between listing and close",async()=>{
 const ts=await import("typescript"),source=readFileSync(new URL("../../🟦️.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("dependency-path.ts",source,ts.ScriptTarget.Latest,true),fn=tree.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="scanJavascriptDependencyInstalledPathsV1");expect(fn).toBeDefined();
 const code=ts.transpileModule(fn!.getText(tree),{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText,routing=await import("../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"),leases=await import("../../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts"),descriptors=await import("@semio-tech/framework-filesystem/observation/descriptor"),capture=await import("../../../../../../../../../../🔨️modules/🏃️process/🎛️owned-execution/📥️capture/🟦️.ts"),artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Original dependency path artifact root required");
 const validate=new Ajv({strict:true}).compile(corpus.schema),make=(stat:unknown,read:unknown)=>new Function("installSchema","createHash","lstatSync","withArtifactReadDirectoryV1","join","requireIssuedResourceControlV1","checkResourceLeaseControl",code+";return scanJavascriptDependencyInstalledPathsV1;")(installSchema,createHash,stat,read,join,routing.requireIssuedResourceControlV1,leases.checkResourceLeaseControl)as Scan;
 await withBuildTestInvocationV1(async original=>{
  const control=routing.createIssuedResourceControlV1(original,"dependency-path:fence",0),oracle=async(root:string)=>{const child=await capture.captureOwnedCommandResultV1("python3",["-c",python],process.cwd(),{control,input:{version:1,text:JSON.stringify({root})},output:{maximumBytes:4096,maximumLines:2}});expect(child.status).toBe(0);return JSON.parse(child.stdout)as Record<string,unknown>&{sha256:string;work:number};};
  const failures:string[]=[];
  for(const row of corpus.cases){
   expect(validate(row),row.id).toBe(true);const root=mkdtempSync(join(artifacts,"pf-")),absolute=(path:string)=>path==="."?root:join(root,path);
   try{
    build(root,corpus.trees.nested as Tree);const before=await oracle(root);let fired=false,calls=0;
    const mutate=()=>{const path=absolute(row.mutation.path);switch(row.mutation.effect){case"add-file":writeFileSync(join(path,"added"),"x");break;case"remove-entry":unlinkSync(path);break;case"replace-directory":renameSync(path,path+".moved");mkdirSync(path);break;case"file-to-link":unlinkSync(path);symlinkSync("elsewhere",path);break;case"touch":utimesSync(path,new Date(Date.now()+5000),new Date(Date.now()+5000));break;case"chmod":chmodSync(path,0o700);break;case"append":appendFileSync(path,"x");break;case"none":break;}},fire=()=>{if(!fired){fired=true;mutate();}};
    const stat=(path:string,options:unknown)=>{if(row.event.kind==="leaf-checked"&&path===absolute(row.event.path)&&++calls===2)fire();return(lstatSync as any)(path,options);},read=async(path:string,checkpoint:()=>void,body:unknown)=>{const value=await(descriptors.withArtifactReadDirectoryV1 as any)(path,checkpoint,body);if(row.event.kind==="listed"&&path===absolute(row.event.path))fire();return value;};
    let accepted=true,result:Awaited<ReturnType<Scan>>|undefined;try{result=await make(stat,read)(root,control,()=>routing.requireIssuedResourceControlV1(control));}catch{accepted=false;}
    if(accepted!==row.accepted)failures.push(row.id+": expected "+(row.accepted?"accepted":"refused"));
    if(row.event.kind!=="none"&&!fired)failures.push(row.id+": event never fired");
    if(accepted&&JSON.stringify([result!.sha256,result!.entries,result!.directories,result!.files,result!.links,result!.maximumPathUnits])!==JSON.stringify([before.sha256,before.entries,before.directories,before.files,before.links,before.maximumPathUnits]))failures.push(row.id+": result differs from the Python walk");
   }finally{rmSync(root,{recursive:true,force:true});}
  }
  expect(failures).toEqual([]);
 });
},120000);

/** ⏱️ Publishes one bounded progress frame per scan work batch and still answers cancellation inside every continuation. */
test("installed dependency path scan publishes bounded progress frames and honours cancellation",async()=>{
 const ts=await import("typescript"),source=readFileSync(new URL("../../🟦️.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("dependency-path.ts",source,ts.ScriptTarget.Latest,true),fn=tree.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="scanJavascriptDependencyInstalledPathsV1")!,
 code=ts.transpileModule(fn.getText(tree),{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText,routing=await import("../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"),leases=await import("../../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts"),descriptors=await import("@semio-tech/framework-filesystem/observation/descriptor"),capture=await import("../../../../../../../../../../🔨️modules/🏃️process/🎛️owned-execution/📥️capture/🟦️.ts"),artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Original dependency path artifact root required");
 const policy=installSchema.$defs.InstalledPathProtocol.const,scan=new Function("installSchema","createHash","lstatSync","withArtifactReadDirectoryV1","join","requireIssuedResourceControlV1","checkResourceLeaseControl",code+";return scanJavascriptDependencyInstalledPathsV1;")(installSchema,createHash,lstatSync,descriptors.withArtifactReadDirectoryV1,join,routing.requireIssuedResourceControlV1,leases.checkResourceLeaseControl)as Scan;
 expect(policy.scanProgressWorkUnits).toBeGreaterThan(policy.progressWorkUnits);expect(policy.scanProgressWorkUnits).toBeLessThanOrEqual(65536);
 const root=mkdtempSync(join(artifacts,"pp-"));
 try{
  for(let directory=0;directory<25;directory++){mkdirSync(join(root,"d"+directory));for(let file=0;file<100;file++)writeFileSync(join(root,"d"+directory,"file-"+file+".js"),"x");}
  await withBuildTestInvocationV1(async original=>{
   const control=routing.createIssuedResourceControlV1(original,"dependency-path:frames",0),child=await capture.captureOwnedCommandResultV1("python3",["-c",python],process.cwd(),{control,input:{version:1,text:JSON.stringify({root})},output:{maximumBytes:4096,maximumLines:2}}),oracle=JSON.parse(child.stdout)as {sha256:string;work:number;entries:number},write=process.stderr.write;let frames=0;
   process.stderr.write=((chunk:unknown,...rest:unknown[])=>{if(String(chunk).includes('"resource":"javascript-installed-paths:'))frames++;return(write as any).call(process.stderr,chunk,...rest);})as typeof process.stderr.write;
   try{const result=await scan(root,control,()=>routing.requireIssuedResourceControlV1(control));expect(result.sha256).toBe(oracle.sha256);expect(result.entries).toBe(oracle.entries);}finally{process.stderr.write=write;}
   const expected=Math.floor(oracle.work/policy.scanProgressWorkUnits);expect(frames).toBeGreaterThanOrEqual(Math.max(1,expected-1));expect(frames).toBeLessThanOrEqual(expected+1);
   const stop=new AbortController(),marker=Object.freeze(Error("Expected installed path cancellation")),stopped=routing.createIssuedResourceControlV1(original,"dependency-path:cancel",0,stop.signal);let seen=0;
   const counting=((path:string,options:unknown)=>{if(++seen===500)stop.abort(marker);return(lstatSync as any)(path,options);});
   const cancelling=new Function("installSchema","createHash","lstatSync","withArtifactReadDirectoryV1","join","requireIssuedResourceControlV1","checkResourceLeaseControl",code+";return scanJavascriptDependencyInstalledPathsV1;")(installSchema,createHash,counting,descriptors.withArtifactReadDirectoryV1,join,routing.requireIssuedResourceControlV1,leases.checkResourceLeaseControl)as Scan;
   let failure:unknown;try{await cancelling(root,stopped,()=>routing.requireIssuedResourceControlV1(stopped));}catch(error){failure=error;}
   expect(failure).toBe(marker);expect(seen).toBeLessThan(530);
  });
 }finally{rmSync(root,{recursive:true,force:true});}
},120000);
