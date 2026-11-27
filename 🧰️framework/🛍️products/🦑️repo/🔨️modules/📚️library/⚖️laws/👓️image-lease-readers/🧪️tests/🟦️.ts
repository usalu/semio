/** 👓️ Tests for the image-lease-readers law: the policy is schema-valid, the verdicts equal an independent Python implementation on a corpus and on the whole repository, a file with one leased and one unleased read fails call by call, and no call opens a path built below the mount of a cache image outside a shared lease unless it is documented. */
import{expect,test}from"bun:test";
import{spawnSync}from"node:child_process";
import{existsSync,mkdirSync,mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{dirname,join,resolve}from"node:path";
import{fileURLToPath}from"node:url";
import Ajv from"ajv";
import{testScratchRootV1}from"../../../../../../../🔨️modules/🏃️process/🧪️testing/🏖️sandbox/🟦️.ts";
import{testPythonV1}from"../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import policyDocument from"../📒️allowlist/🔣️.json"with{type:"json"};
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import fixtures from"./🧫️fixtures/🔣️.json"with{type:"json"};
import{imageLeaseRefusalV1,judgeImageLeaseReadersV1,judgeImageLeaseRepositoryV1,listImageLeaseLawFilesV1,namesImageMountV1,readImageLeaseLawPolicyV1,unleasedImageSitesV1,type ImageLeaseVerdictV1}from"../🟦️.ts";
const here=dirname(fileURLToPath(import.meta.url)),repository=resolve(here,"../../../../../../../.."),oracle=join(here,"🐍️oracle","🐍️.py"),allowlist=join(here,"..","📒️allowlist","🔣️.json");
const python=()=>{const found=testPythonV1();if(found===null)throw Error("Python required by the image lease oracle");return found;};
type OracleVerdict={named:string[];unrouted:string[];sites:string[];stale:string[]};
const ask=(args:readonly string[])=>{const run=spawnSync(python(),["-I",oracle,...args],{encoding:"utf8",maxBuffer:67108864});if(run.status!==0)throw run.error??Error(run.stderr);return JSON.parse(run.stdout) as unknown;};
const shape=(verdict:ImageLeaseVerdictV1):OracleVerdict=>({named:[...verdict.named],unrouted:[...verdict.unrouted],sites:verdict.sites.map(site=>site.path+":"+site.line+":"+site.opener),stale:[...verdict.stale]});
test("the policy is valid against its schema with a third-party validator, closed, and refuses malformed documents",()=>{
 const ajv=new(Ajv as unknown as new(options:object)=>{validate(schema:object,value:unknown):boolean})({strict:false,allErrors:true});
 expect(ajv.validate(schema,policyDocument)).toBe(true);
 const policy=readImageLeaseLawPolicyV1(policyDocument);expect(policy.entries.length).toBeGreaterThan(0);
 expect(()=>readImageLeaseLawPolicyV1({...policyDocument,extra:1})).toThrow("Valid image lease law policy required");
 expect(()=>readImageLeaseLawPolicyV1({...policyDocument,entries:[{path:"a.ts",site:"x()",class:"nonsense",reason:{en:"x",de:"y"}}]})).toThrow("Declared allowlist class required");
 expect(()=>readImageLeaseLawPolicyV1({...policyDocument,entries:[{path:"a.ts",class:policyDocument.classes[0],reason:{en:"x",de:"y"}}]})).toThrow("Valid image lease law policy required");
 expect(()=>readImageLeaseLawPolicyV1({...policyDocument,entries:[{path:"a.ts",site:"x()",class:policyDocument.classes[0],reason:{en:"x"}}]})).toThrow("Valid image lease law policy required");
 for(const row of policy.entries){expect(row.reason.en.length).toBeGreaterThan(10);expect(row.reason.de.length).toBeGreaterThan(10);}
});
const corpusPolicy=readImageLeaseLawPolicyV1({...fixtures.policy,"$schema":"x"});
test("every corpus case is named and judged call by call as declared and equals the independent oracle, lines included",()=>{
 const oracleAnswers=ask(["cases",join(here,"🧫️fixtures","🔣️.json")]) as OracleVerdict[];
 fixtures.cases.forEach((row,index)=>{
  expect(namesImageMountV1(row.source,corpusPolicy),row.name).toBe(row.named);
  const verdict=judgeImageLeaseReadersV1({sources:new Map([["x.ts",row.source]]),policy:corpusPolicy});
  expect(verdict.sites.map(site=>site.opener),row.name).toEqual(row.unleased);
  expect(shape(verdict),row.name).toEqual(oracleAnswers[index]!);
 });
});
test("a documented site passes, a stale entry is found and the refusal names the call sites in both languages",()=>{
 const source="const root=join(partitions.graph.directory,\"m\");\nconst there=existsSync(root);\nconst names=readdirSync(root);";
 const documented={...corpusPolicy,entries:[{path:"x.ts",site:"existsSync(root)",class:"custody",reason:{en:"Only asks whether the mount exists.",de:"Fragt nur, ob der Mount existiert."}}]};
 const sources=new Map([["x.ts",source],["z.ts","const a=1;"]]);
 const verdict=judgeImageLeaseReadersV1({sources,policy:documented});
 expect(verdict.sites.map(site=>site.opener+"@"+site.line)).toEqual(["readdirSync@3"]);expect(verdict.stale).toEqual([]);
 const stale=judgeImageLeaseReadersV1({sources,policy:{...documented,entries:[...documented.entries,{path:"x.ts",site:"statSync(root)",class:"custody",reason:{en:"Routed now.",de:"Jetzt geführt."}},{path:"gone.ts",site:"readdirSync(x)",class:"custody",reason:{en:"Gone.",de:"Weg."}}]}});
 expect(stale.stale).toEqual(["gone.ts::readdirSync(x)","x.ts::statSync(root)"]);
 expect(imageLeaseRefusalV1(verdict)).toBe("Calls that open a path below the mount of a cache image outside its shared lease: x.ts:3 readdirSync / Aufrufe, die einen Pfad unter dem Mount eines Cache-Images ausserhalb dessen geteilter Lease öffnen: x.ts:3 readdirSync");
 expect(unleasedImageSitesV1("y.ts","const a=1;",corpusPolicy)).toEqual([]);
});
test("no call of the repository opens a path built below the mount of a cache image outside a shared lease or a documented site, and every documented site still needs its entry",()=>{
 const policy=readImageLeaseLawPolicyV1(policyDocument);expect(existsSync(join(repository,policy.roots[0]!))).toBe(true);
 const files=listImageLeaseLawFilesV1(repository,policy);expect(files.length).toBeGreaterThan(1000);
 const verdict=judgeImageLeaseRepositoryV1(repository,policy);
 expect(verdict.named.length).toBeGreaterThan(10);
 expect(verdict.sites,imageLeaseRefusalV1(verdict)).toEqual([]);expect(verdict.stale).toEqual([]);
 expect(shape(verdict)).toEqual(ask(["repository",repository,allowlist]) as OracleVerdict);
},120000);
test("a scratch repository fails on a file with one leased and one unleased read, the fixed file and a test file pass",()=>{
 const policy=readImageLeaseLawPolicyV1(policyDocument),directory=mkdtempSync(join(testScratchRootV1("image-lease-law"),"repo-"));
 try{
  const source=(name:string,text:string)=>{const path=join(directory,"🧰️framework","new",name);mkdirSync(dirname(path),{recursive:true});writeFileSync(path,text);};
  const mount="const root=join(partitions.graph.directory,\"m\",\"generations\");\n";
  source("mixed.ts",mount+"export async function names(input:Input){\n return withNativeImageSharedReadV1(input,()=>readdirSync(root));\n}\nexport const stamp=()=>statSync(join(root,\"current.json\")).mtimeMs;\n");
  source("fixed.ts",mount+"export async function names(input:Input){\n return withNativeImageSharedReadV1(input,()=>readdirSync(root));\n}\nexport const stamp=(input:Input)=>withNativeImageSharedReadV1(input,()=>statSync(join(root,\"current.json\")).mtimeMs);\n");
  mkdirSync(join(directory,"🧰️framework","new","🧪️tests"),{recursive:true});writeFileSync(join(directory,"🧰️framework","new","🧪️tests","fixture.ts"),"readdirSync(\".cache/b/g/m\")");
  const verdict=judgeImageLeaseRepositoryV1(directory,policy);
  expect(verdict.named).toEqual(["🧰️framework/new/fixed.ts","🧰️framework/new/mixed.ts"]);
  expect(verdict.unrouted).toEqual(["🧰️framework/new/mixed.ts"]);expect(verdict.sites.map(site=>site.line+":"+site.opener)).toEqual(["5:statSync"]);
  expect(imageLeaseRefusalV1(verdict)).toContain("🧰️framework/new/mixed.ts:5 statSync");
  expect(shape(verdict)).toEqual(ask(["repository",directory,allowlist]) as OracleVerdict);
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("a scratch repository with the old dedupe files fails the law on the walker and on the command, the fixed files pass, and the oracle agrees",()=>{
 const policy=readImageLeaseLawPolicyV1(policyDocument),directory=mkdtempSync(join(testScratchRootV1("image-lease-law"),"repo-"));
 try{
  const source=(folder:string,text:string)=>{const path=join(directory,"🧰️framework","dedupe",folder,"🟦️.ts");mkdirSync(dirname(path),{recursive:true});writeFileSync(path,text);};
  const walker=(reader:boolean)=>"type Request=Readonly<{base:string;"+(reader?"reader:Reader;":"")+"minimumBytes:number}>;\n"
   +"async function* files(base:string"+(reader?",reader:Reader":"")+"){\n const stack=[base];\n while(stack.length){\n  const directory=stack.pop()!;\n  const children="+(reader?"await reader.run(()=>readdir(directory))":"await readdir(directory)")+";\n  for(const child of children){const path=join(directory,child);"+(reader?"yield await reader.run(()=>lstat(path));":"yield await lstat(path);")+"}\n }\n}\n"
   +"export async function dedupe(request:DedupeRequestV1){\n const found=[];\n for await(const file of files(request.base"+(reader?",request.reader":"")+"))found.push(file.path);\n for(const path of found)"+(reader?"await request.reader.run(()=>unlink(path));":"await unlink(path);")+"\n}\n";
  const command=(reader:boolean)=>"export async function run(context:Context,budgets:CachePartitionBudgetsV1,leaseDirectory:string){\n const partition=budgets.tools,base=join(context.workspace,partition.directory,\"m\");\n"
   +(reader?" const reader=createNativeImageSharedReaderV1({leaseDirectory,imageDirectory:join(context.workspace,partition.directory),signal});\n try{\n  await reader.run(()=>stat(base));\n  await dedupeTreeV1({base,reader},ports);\n }finally{await reader.close();}\n":" await stat(base);\n await dedupeTreeV1({base},ports);\n")+"}\n";
  source("old-walker",walker(false));source("old-command",command(false));source("fixed-walker",walker(true));source("fixed-command",command(true));
  const verdict=judgeImageLeaseRepositoryV1(directory,policy),shown=verdict.sites.map(site=>site.path.split("/").slice(-2)[0]+":"+site.opener);
  expect(verdict.unrouted.map(path=>path.split("/").slice(-2)[0])).toEqual(["old-command","old-walker"]);
  expect(shown.filter(row=>row.startsWith("old-command"))).toEqual(["old-command:stat","old-command:dedupeTreeV1"]);
  expect(shown.filter(row=>row.startsWith("old-walker"))).toEqual(["old-walker:readdir","old-walker:lstat","old-walker:unlink"]);
  expect(shown.filter(row=>row.startsWith("fixed")),"the fixed files pass").toEqual([]);
  expect(verdict.named.length).toBe(4);
  expect(shape(verdict)).toEqual(ask(["repository",directory,allowlist]) as OracleVerdict);
 }finally{rmSync(directory,{recursive:true,force:true});}
});
