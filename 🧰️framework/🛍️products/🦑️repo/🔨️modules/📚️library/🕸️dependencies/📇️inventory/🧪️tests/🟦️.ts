/** 📇️ Contract: the dependency inventory reads Cargo, package.json, go.mod, pyproject and csproj manifests of a synthetic repository into one baseline, classifies oracle declarations and audits the Bun lock against manifests; the corpus is language-agnostic, `@iarna/toml` and JSON model the Cargo, pyproject and package.json inventories independently, and `🐍️oracle.py` models Go, .NET, lock audit, classification and the test domain. */
import {expect,test} from "bun:test";
import Ajv from "ajv";
import {parse} from "@iarna/toml";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {DEPENDENCY_ECOSYSTEMS,dependencyClassifyOracleEntry,dependencyFreezeCurrentThirdParty,dependencyIsCompositionManifest,dependencyJsLockWorkspaceMismatches,dependencyParseGoModule,dependencyTestDomain} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

type Row=Record<string,any>;
const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1(),clean=(value:unknown):any=>JSON.parse(JSON.stringify(value));
const materialize=(files:Readonly<Record<string,string|undefined>>):string=>{
 const root=mkdtempSync(join(tmpdir(),"inventory-"));
 const all:Record<string,string|undefined>={...files,[corpus.taxonomyPath]:JSON.stringify(corpus.taxonomy)};
 for(const[path,text]of Object.entries(all)){if(text===undefined)continue;mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}
 return root;
};
const real=(files:Readonly<Record<string,string|undefined>>):Row[]=>{const root=materialize(files);try{return clean(dependencyFreezeCurrentThirdParty(root));}finally{rmSync(root,{recursive:true,force:true});}};
const testOwned=(path:string):boolean=>path.startsWith(corpus.taxonomy.testDomainPath+"/")||path.split("/").slice(0,-1).some(part=>Object.values(corpus.taxonomy).includes(part));
const kindOf=(section:string,procMacro:boolean):string=>section==="dev-dependencies"?"test-runner":section==="build-dependencies"||procMacro?"production-build":"production-runtime";
const internalRust=(name:string,table:any,workspace:Row):boolean=>name.startsWith("semio-")||name.startsWith("semio_")||name==="ports"||/^db(?:_[a-z0-9]+)*$/.test(name)||(typeof table==="object"&&table!==null&&"path" in table)||(typeof table==="object"&&table!==null&&table.workspace===true&&typeof workspace[name]==="object"&&"path" in workspace[name]);
const versionOf=(table:any,workspace:Row,name:string):string=>{
 const own=typeof table==="string"?table:table?.version;
 if(typeof table==="object"&&table?.workspace===true){const shared=workspace[name];return typeof shared==="string"?shared:shared?.version??"*";}
 return own??"*";
};
const rows=(entries:[string,string,string,string,string][]):Row[]=>{
 const byKey=new Map<string,Row>();
 for(const[eco,name,version,kind,user]of entries){
  const declared=testOwned(user)&&(kind==="production-runtime"||kind==="production-build")?"test-runner":kind,key=eco+":"+name,found=byKey.get(key);
  if(found){if(!found.kinds.includes(declared))found.kinds.push(declared);if(!found.users.includes(user))found.users.push(user);if(!found.declarations.some((row:Row)=>row.user===user&&row.version===version&&row.kind===declared))found.declarations.push({user,version,kind:declared});continue;}
  byKey.set(key,{ecosystem:eco,name,version,kinds:[declared],users:[user],productionReachable:false,declarations:[{user,version,kind:declared}]});
 }
 for(const row of byKey.values()){row.declarations.sort((a:Row,b:Row)=>a.user.localeCompare(b.user)||a.kind.localeCompare(b.kind)||a.version.localeCompare(b.version));row.kinds.sort();row.users.sort();row.productionReachable=row.kinds.some((kind:string)=>kind==="production-runtime"||kind==="production-build");}
 return[...byKey.values()].sort((a,b)=>a.ecosystem===b.ecosystem?a.name.localeCompare(b.name):a.ecosystem.localeCompare(b.ecosystem));
};
const modelCargoJsPython=(files:Readonly<Record<string,string|undefined>>):Row[]=>{
 const found:[string,string,string,string,string][]=[],skipped=(path:string)=>path.split("/").some(part=>["compose","node_modules",".git","target"].includes(part)),paths=Object.keys(files).filter(path=>!skipped(path)).sort();
 const workspace:Row={};
 for(const path of paths)if(path==="Cargo.toml")Object.assign(workspace,((files[path]?parse(files[path]!):{})as{workspace?:{dependencies?:Row}}).workspace?.dependencies??{});
 const names=new Set<string>();
 for(const path of paths)if(path.endsWith("package.json")){try{const name=JSON.parse(files[path]!).name;if(name)names.add(name);}catch{}}
 for(const path of paths){
  const text=files[path]!;
  if(path.endsWith("Cargo.toml")&&text){
   const document:Row=parse(text),procMacro=document.lib?.["proc-macro"]===true,tables:[string,Row][]=[];
   for(const section of["dependencies","dev-dependencies","build-dependencies"]){if(document[section])tables.push([section,document[section]]);for(const target of Object.values<Row>(document.target??{}))if(target[section])tables.push([section,target[section]]);}
   for(const[section,table]of tables)for(const[name,value]of Object.entries<any>(table))if(!internalRust(name,value,workspace))found.push(["rust",name,versionOf(value,workspace,name),kindOf(section,procMacro&&section==="dependencies"),path]);
  }
  if(path.endsWith("package.json")){
   let document:Row;try{document=JSON.parse(text);}catch{continue;}
   for(const[section,kind]of[["dependencies","production-runtime"],["peerDependencies","production-runtime"],["optionalDependencies","production-runtime"],["devDependencies","repository-tooling"]]as const)for(const[name,version]of Object.entries<any>(document[section]??{}))if(!(names.has(name)||name.startsWith("@semio-tech/")||/^(workspace|file|link):/.test(String(version))))found.push(["js",name,String(version),kind,path]);
  }
  if(path.endsWith("pyproject.toml")&&text){
   const document:Row=parse(text),groups:[string,string[]][]=[["production-runtime",document.project?.dependencies??[]]];
   for(const[group,list]of Object.entries<string[]>({...(document.project?.["optional-dependencies"]??{}),...(document["dependency-groups"]??{})}))if(["dev","test","lint","docs"].includes(group))groups.push(["test-runner",list]);
   for(const[kind,list]of groups)for(const requirement of list){const match=/^([A-Za-z0-9._-]+)\s*(.*)$/.exec(requirement)!;if(!match[1]!.startsWith("semio"))found.push(["python",match[1]!,match[2]!.trim()||"*",kind,path]);}
  }
 }
 return rows(found);
};

test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
test("the ecosystems the freeze tracks are the five shipped ones",()=>{expect([...DEPENDENCY_ECOSYSTEMS]).toEqual(["rust","js","go","python","dotnet"]);});
for(const row of corpus.freeze){
 const defect=(row as Row).defect!==undefined,run=defect?test.failing:test;
 run("freeze: "+row.name,()=>{expect(real(row.files)).toEqual(row.expected);});
 if(row.oracle==="ts")test("independent TOML/JSON model: "+row.name,()=>{expect(modelCargoJsPython(row.files)).toEqual(row.expected);});
}
for(const row of corpus.freezeErrors)test("freeze fails: "+row.name,()=>{expect(()=>real(row.files)).toThrow(row.message);});
for(const row of corpus.classify)test("classify: "+row.name,()=>{
 const entry=clean(row.entry);dependencyClassifyOracleEntry(entry,row.oracleIds??undefined,dependencyTestDomain(corpus.taxonomy));expect(entry).toEqual(row.expected);
});
for(const row of corpus.testDomain)test("test domain: "+row.name,()=>{
 if(row.expected===null)expect(()=>dependencyTestDomain(row.taxonomy)).toThrow("[dependencies]");
 else expect(clean(dependencyTestDomain(row.taxonomy))).toEqual(row.expected);
});
for(const row of corpus.composition)test("composition: "+row.path,()=>{expect(dependencyIsCompositionManifest(row.path)).toBe(row.expected);});
for(const row of corpus.goModule)test("go.mod: "+row.name,()=>{expect(clean(dependencyParseGoModule(row.content))).toEqual(row.expected);});
for(const row of corpus.lockMismatches)test("lock audit: "+row.name,()=>{expect(clean(dependencyJsLockWorkspaceMismatches(new Map(row.manifests as [string,unknown][]),row.workspaces))).toEqual(row.expected);});
test("the independent Python model reaches the same verdicts",()=>{
 if(!python)return;
 const result=Bun.spawnSync([python,"-I",join(here,"🐍️oracle.py"),join(here,"🔣️.json")]);
 expect(result.stderr.toString()).toBe("");expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});expect(result.exitCode).toBe(0);
});
