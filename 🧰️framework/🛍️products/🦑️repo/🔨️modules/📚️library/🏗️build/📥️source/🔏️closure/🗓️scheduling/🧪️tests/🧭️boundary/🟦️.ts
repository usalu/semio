/** 🧭️ Contract: no file of the build or the caching module can silently join every key. Every member of both modules is classified (scheduling, mixed, keyed) and a new member fails until the corpus names it; keyed code reaches a scheduling domain only through its ports (a ratchet for the domains that still have such imports); an edit of any file of any domain moves no keyed identity, of a keyed file it does; the command closure of a Rust compile really leaves the files of the new domains. Measured by a TypeScript scanner and checked against an independent Python oracle. */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {existsSync,readFileSync,readdirSync} from "node:fs";
import {dirname,join,normalize,relative,resolve} from "node:path";
import Ajv from "ajv";
import {buildSourceClosureIdentityV1,type BuildSourceClosureV1} from "../../../🟦️.ts";
import {buildSchedulingDomainOfV1,buildSchedulingNativePrefixesV1,buildSchedulingPrefixesV1,buildSchedulingV1,splitBuildSourceClosureV1} from "../../🟦️.ts";
import declared from "../../🔣️.json";
import corpus from "./🔣️.json";
import corpusSchema from "./🧬️schema/🔣️.json";

const checkout=process.env.SEMIO_TEST_CHECKOUT??resolve(import.meta.dir,corpus.checkoutFromTests),library="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/",oracleFile=join(import.meta.dir,"🐍️.py");
const sha=(value:string)=>createHash("sha256").update(value).digest("hex");
const walk=(path:string):string[]=>readdirSync(join(checkout,path),{withFileTypes:true}).filter(row=>row.name!=="node_modules").flatMap(row=>row.isDirectory()?walk(path+"/"+row.name):[path+"/"+row.name]);
const files=["🏗️build","⚡️caching"].flatMap(module=>walk(library+module));
const known=new Set(files),prefixes=buildSchedulingPrefixesV1();
const covered=(file:string,list:readonly string[]=prefixes)=>list.some(prefix=>file.startsWith(prefix));
const SPEC=/\.(?:ts|mjs|js|cjs|mts|tsx)$/u;
const importsOf=(file:string):string[]=>{
 if(!SPEC.test(file)||file.includes("/🧪️tests/"))return [];
 let found:string[];
 try{found=new Bun.Transpiler({loader:"ts"}).scan(readFileSync(join(checkout,file),"utf8")).imports.map(row=>row.path).filter(path=>path.startsWith("./")||path.startsWith("../"));}catch{return [];}
 const out:string[]=[];
 for(const specifier of found){const path=normalize(join(dirname(file),specifier));const hit=[path,path.replace(/\.js$/u,".ts"),path.replace(/\.mjs$/u,".d.mts")].find(candidate=>known.has(candidate));if(hit!==undefined)out.push(hit);}
 return out;
};
const edges=new Map(files.filter(file=>SPEC.test(file)).map(file=>[file,importsOf(file)] as const));
const violations=(id:string):string[]=>{
 const domain=buildSchedulingV1().domains.find(row=>row.id===id)!,ports=new Set(domain.ports),inside=(file:string)=>covered(file,domain.prefixes),bad=new Set<string>();
 for(const [file,targets] of edges){if(inside(file))continue;for(const target of targets)if(inside(target)&&!ports.has(target))bad.add(file+" -> "+target);}
 return [...bad].sort();
};
const memberOf=(file:string)=>{const parts=file.slice(library.length).split("/");return parts[0]+"/"+(parts.length>2?parts[1]:".");};
const statuses=():Record<string,string>=>{
 const count=new Map<string,[number,number]>();
 for(const file of files){const name=memberOf(file),row=count.get(name)??[0,0];row[covered(file)?0:1]++;count.set(name,row);}
 return Object.fromEntries([...count].map(([name,[inside,outside]])=>[name,outside===0?"scheduling":inside===0?"keyed":"mixed"]));
};
const oracle=(()=>{const run=spawnSync("python3",["-I",oracleFile,checkout],{input:JSON.stringify(declared),encoding:"utf8",maxBuffer:1<<26});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout) as {members:Record<string,string>;violations:Record<string,string[]>};})();

test("the corpus is closed by its schema and every enforced or indebted domain exists",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,extra:1})).toBe(false);
 const ids=buildSchedulingV1().domains.map(row=>row.id);
 expect([...corpus.enforced,...Object.keys(corpus.debt)].sort()).toEqual([...ids].sort());
 for(const id of corpus.introduced)expect(corpus.enforced.includes(id),id).toBe(true);
});

test("Guard: every member of the build and the caching module is classified, so a new member cannot silently fall into every key",()=>{
 const actual=statuses();
 expect(actual).toEqual(corpus.members);
 expect(oracle.members).toEqual(actual);
 const unclassified=Object.keys(actual).filter(name=>!(name in corpus.members));
 expect(unclassified,"add each new member to the corpus as scheduling, mixed or keyed, and declare a scheduling domain when it only schedules, stores or reports").toEqual([]);
});

test("Law: keyed code reaches a scheduling domain only through its ports; the domains that still have other imports may not get more",()=>{
 for(const id of corpus.enforced){expect(violations(id),id).toEqual([]);expect(oracle.violations[id],id).toEqual([]);}
 for(const [id,count] of Object.entries(corpus.debt)){
  expect(violations(id).length,id).toBeLessThanOrEqual(count);
  expect(oracle.violations[id]!.length,id).toBeLessThanOrEqual(count);
 }
});

const file=(path:string,text:string)=>({path,sha256:sha(text),base64:Buffer.from(text).toString("base64"),imports:{}});

test("Law: an edit of any file of any scheduling domain moves 0 keyed identities, an edit of a keyed file moves exactly the identity that holds it",()=>{
 const domains=buildSchedulingV1().domains,entry="entry.ts",keyedFiles=["keyed-a.ts","keyed-b.ts"];
 const inDomains=domains.flatMap(domain=>[domain.ports[0]!,domain.prefixes[0]!+"deep/x.json"]);
 const make=(edit?:string):BuildSourceClosureV1=>({version:1,root:"/r",entry,files:[entry,...keyedFiles,...inDomains].map(path=>file(path,path===edit?"edited":"base"))});
 const identity=(snapshot:BuildSourceClosureV1,ids:readonly string[])=>buildSourceClosureIdentityV1(splitBuildSourceClosureV1(snapshot,ids).keyed);
 const ids=domains.map(row=>row.id),before=identity(make(),ids);
 for(const path of inDomains){expect(buildSchedulingDomainOfV1(path),path).toBeDefined();expect(identity(make(path),ids),path).toBe(before);}
 for(const path of keyedFiles)expect(identity(make(path),ids),path).not.toBe(before);
 expect(identity(make(keyedFiles[0]),ids)).not.toBe(identity(make(keyedFiles[1]),ids));
 for(const domain of domains){
  const own=identity(make(domain.ports[0]),[domain.id]),base=identity(make(),[domain.id]);
  expect(own,domain.id+" named alone").toBe(base);
  const other=domains.find(row=>row.id!==domain.id)!;
  expect(identity(make(other.ports[0]),[domain.id]),domain.id+" does not hide "+other.id).not.toBe(base);
 }
});

test("Law: the Rust compile view leaves every file of the new domains, which the walk of the real cargo script closure shows",async()=>{
 const native=buildSchedulingNativePrefixesV1(),added=declared.domains.filter(row=>corpus.introduced.includes(row.id)).flatMap(row=>row.prefixes);
 expect(added.length).toBeGreaterThan(0);
 for(const prefix of added)expect(native.includes(prefix),prefix).toBe(true);
 const root=join(checkout,library,"🏗️build","🕸️graph","📥️inference")+"/";
 const inference=await import(join(root,"🟨️.mjs")) as any;await inference.libraryBootstrap;
 const scripts=await import(join(root,"🧭️script-inputs","🟨️.mjs")) as any;
 const raw=(await scripts.relativeScriptInputs([join(checkout,corpus.cargoScript)],checkout,inference.createScriptInputCache({workspaceRoot:checkout}))).filter((row:unknown)=>typeof row==="string").map((row:string)=>row.slice("{workspaceRoot}/".length)) as string[];
 const leaving=raw.filter(path=>covered(path,added));
 expect(leaving.length,"files of the new domains in the real closure").toBeGreaterThanOrEqual(corpus.minimumLeaving);
 for(const path of leaving)expect(existsSync(join(checkout,path)),path).toBe(true);
 expect(relative(checkout,join(checkout,corpus.cargoScript)).length).toBeGreaterThan(0);
},180000);
