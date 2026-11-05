import {test,expect} from "bun:test";
import Ajv from "ajv";
import ts from "typescript";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import vectors from "./🔣️.json" with {type:"json"};

const load=()=>{
 const source=readFileSync(new URL("../../🟦️.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("fence.ts",source,ts.ScriptTarget.Latest,true),names=["sameJavascriptDependencyParentIdentityV1","requireJavascriptDependencyPreservationSourceUnchangedV1"],optional=tree.statements.find((n):n is ts.FunctionDeclaration=>ts.isFunctionDeclaration(n)&&n.name?.text==="javascriptDependencyPreservationMismatchV1");
 const nodes=names.map(name=>{const node=tree.statements.find((n):n is ts.FunctionDeclaration=>ts.isFunctionDeclaration(n)&&n.name?.text===name);if(!node)throw Error(name);return node;});if(optional)nodes.unshift(optional);
 const schema=JSON.parse(readFileSync(new URL("../../🧬️schema/📥️install.json",import.meta.url),"utf8"));
 return {schema,run:new Function("installSchema",ts.transpileModule(nodes.map(n=>n.getText(tree).replace(/^export\s/u,"")).join("\n")+";return requireJavascriptDependencyPreservationSourceUnchangedV1;",{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText)(schema)};
};
test("dependency Source fence mismatch gives bounded neutral DATA and preserves rejection",()=>{
 const {schema,run}=load(),valid=new Ajv({strict:true}).compile(schema.$defs.SourceFenceDiagnostic),db=new Database(":memory:"),stamp=JSON.stringify(["1","2","16832","1","0","10","11"]);
 try{for(const row of vectors){
  const original={rows:[{sourceDirectory:"/workspace/p/node_modules",kind:"link",stamp,target:"/store/current/node_modules"}],parents:[{path:"/workspace/p",identity:stamp}],ancestors:[{path:"/workspace",identity:JSON.stringify(["1","3","16832"])}]},current=structuredClone(original),expected=new Map(original.rows.map(r=>[r.sourceDirectory,r])),parents=new Map(original.parents.map(r=>[r.path,r.identity])),ancestors=new Map(original.ancestors.map(r=>[r.path,r.identity]));
  if(row.mode==="endpoint-time"){const s=JSON.parse(current.rows[0]!.stamp);s[5]="99";current.rows[0]!.stamp=JSON.stringify(s);}
  if(row.mode==="endpoint-target")current.rows[0]!.target="/store/peer/node_modules";
  if(row.mode==="parent-device"){const s=JSON.parse(current.parents[0]!.identity);s[0]="9";current.parents[0]!.identity=JSON.stringify(s);}
  if(row.mode==="ancestor-inode"){const s=JSON.parse(current.ancestors[0]!.identity);s[1]="9";current.ancestors[0]!.identity=JSON.stringify(s);}
  if(row.mode==="endpoint-count")current.rows=[];
  if(row.mode==="endpoint-order")current.rows[0]!.sourceDirectory="/workspace/peer/node_modules";
  if(row.mode==="bounded-path")current.rows[0]!.sourceDirectory="/workspace/"+"p".repeat(300);
  if(row.mode==="all-fields"){current.rows[0]={sourceDirectory:"/workspace/peer/node_modules",kind:"file",target:"/store/peer/node_modules",stamp:JSON.stringify(["9","9","9","9","9","99","99"])};}
  let error:any;try{run(current,expected,parents,ancestors);}catch(e){error=e;}
  expect(error?.message,row.id).toBe("Exact current Store Source fence changed");
  expect(error?.cause,row.id).toEqual({version:1,purpose:"current-store-source-fence-mismatch",section:row.section,index:0,path:row.path,fields:row.fields});
  expect(valid(error.cause),row.id).toBe(true);
  expect(valid({...error.cause,authority:true}),row.id).toBe(false);
  expect(JSON.stringify(error.cause)).not.toContain("/store/");
  if(row.mode==="endpoint-time"){const changed=db.query("SELECT group_concat(CASE a.key WHEN 0 THEN 'dev' WHEN 1 THEN 'ino' WHEN 2 THEN 'mode' WHEN 3 THEN 'nlink' WHEN 4 THEN 'size' WHEN 5 THEN 'mtimeNs' WHEN 6 THEN 'ctimeNs' END) fields FROM json_each(?1) a JOIN json_each(?2) b ON a.key=b.key WHERE a.value<>b.value").get(original.rows[0]!.stamp,current.rows[0]!.stamp) as {fields:string};expect(changed.fields.split(",")).toEqual(error.cause.fields);}
  expect(()=>run(original,expected,parents,ancestors)).not.toThrow();
  const parentTime=structuredClone(original),s=JSON.parse(parentTime.parents[0]!.identity);s[5]="99";parentTime.parents[0]!.identity=JSON.stringify(s);expect(()=>run(parentTime,expected,parents,ancestors)).not.toThrow();
 }}finally{db.close();}
});
