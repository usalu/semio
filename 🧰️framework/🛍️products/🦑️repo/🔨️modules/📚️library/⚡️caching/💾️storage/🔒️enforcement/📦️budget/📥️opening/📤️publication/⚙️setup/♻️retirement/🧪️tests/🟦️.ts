import {expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import * as ts from "typescript";
import {readFileSync} from "node:fs";
import fixture from "./🔣️.json" with {type:"json"};
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {javascriptInstallSchemaV1} from "../../../../../../../../🚀️bootstrap/📦️dependencies/💾️storage/🧱️schemas/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";

import {materializeSchemaV1,schemaViewV1} from "../../../../../../../../🚀️bootstrap/📦️dependencies/🔌️managers/🧩️materialize/🟦️.ts";
import {packageManagerRegistryV1} from "../../../../../../../../🚀️bootstrap/📦️dependencies/🔌️managers/📚️catalog/🟦️.ts";

const selectedSchema=materializeSchemaV1(schema,schemaViewV1(javascriptInstallSchemaV1.$defs.ManagerProtocol.manager.descriptor)) as typeof schema;
const wire={...selectedSchema,$defs:{...selectedSchema.$defs,SelectedRetirementSnapshot:javascriptInstallSchemaV1.$defs.SelectedRetirementSnapshot,SelectedRetirementResult:javascriptInstallSchemaV1.$defs.SelectedRetirementResult}};
function sourceFunctions(names:string[],selected=selectedSchema):Record<string,(...args:any[])=>any>{
 const source=readFileSync(new URL("../🟦️.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("retirement.ts",source,ts.ScriptTarget.Latest,true),nodes=names.map(name=>{const node=tree.statements.find((row):row is ts.FunctionDeclaration=>ts.isFunctionDeclaration(row)&&row.name?.text===name);expect(node).toBeDefined();return node!;}),ports={validateJsonSchemaSubset,retirementWireSchema:wire,retirementProtocol:selected.$defs.Protocol.const,retirementSchema:schema,materializeSchemaV1,schemaViewV1};
 return new Function(...Object.keys(ports),ts.transpileModule(nodes.map(row=>row.getText(tree)).join("\n")+";return{"+names.join(",")+"};",{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText)(...Object.values(ports));
}

test("obsolete driver chooses closed no-inbound DATA and refuses unknown graphs cycles and busy results",()=>{
 const api=sourceFunctions(["readRepositoryDependencyRetirementPlanV1","selectRepositoryDependencyRetirementCandidateV1"]),validate=new Ajv({strict:true}).compile({...wire,$ref:"#/$defs/AcquiredPlan"}),db=new Database(":memory:");
 const sql="WITH n AS(SELECT value v,json_extract(value,'$.prefix') p,json_extract(value,'$.generation') g,json_extract(value,'$.status') s FROM json_each(?1,'$.candidates')),l AS(SELECT json_extract(value,'$.generation') g FROM json_each(?1,'$.current.scopes')),e AS(SELECT n.g s,json_extract(r.value,'$.generation') t FROM n,json_each(n.v,'$.ownerReferences') r),candidate AS(SELECT p FROM n WHERE (s='inactive' AND NOT EXISTS(SELECT 1 FROM e WHERE e.t=n.g)) OR (s='unqualified' AND json_array_length(n.v,'$.roles')=0) ORDER BY p LIMIT 1),bad AS(SELECT (SELECT count(*) FROM n)<>(SELECT count(DISTINCT p) FROM n) OR (SELECT count(g) FROM n)<>(SELECT count(DISTINCT g) FROM n) OR (SELECT count(*) FROM json_each(?1,'$.generations.names'))<>(SELECT count(*) FROM n) OR EXISTS(SELECT 1 FROM json_each(?1,'$.generations.names') r WHERE NOT EXISTS(SELECT 1 FROM n WHERE n.p=r.value)) OR NOT EXISTS(SELECT 1 FROM l) OR EXISTS(SELECT 1 FROM n WHERE s='unqualified' AND (json_array_length(v,'$.roles')<>0 OR g IS NOT NULL OR json_extract(v,'$.receiptFingerprint') IS NOT NULL OR json_array_length(v,'$.ownerReferences')<>0)) OR EXISTS(SELECT 1 FROM l WHERE NOT EXISTS(SELECT 1 FROM n WHERE n.g=l.g AND n.s='active')) OR EXISTS(SELECT 1 FROM n WHERE s='active' AND NOT EXISTS(SELECT 1 FROM l WHERE l.g=n.g)) OR EXISTS(SELECT 1 FROM e WHERE NOT EXISTS(SELECT 1 FROM n WHERE n.g=e.t) AND (EXISTS(SELECT 1 FROM n WHERE n.g=e.s AND n.s='active') OR EXISTS(SELECT 1 FROM n WHERE n.p=substr(e.t,1,20)))) OR EXISTS(SELECT 1 FROM e JOIN n ON n.g=e.s WHERE n.s='active' AND NOT EXISTS(SELECT 1 FROM l WHERE l.g=e.t)) invalid) SELECT NOT invalid AND (EXISTS(SELECT 1 FROM candidate) OR NOT EXISTS(SELECT 1 FROM n WHERE s<>'active')) admitted,(SELECT p FROM candidate) prefix FROM bad";
 try{for(const row of fixture.cases){
  const plan:any=structuredClone(fixture.plan),wrapped:any={acquired:true,value:plan};
  if(row.mode==="reversed"){plan.candidates.reverse();plan.generations.names.reverse();}
  if(row.mode==="after-first"){plan.candidates.shift();plan.generations.names.shift();}
  if(row.mode==="done"){plan.candidates=plan.candidates.slice(-1);plan.generations.names=plan.generations.names.slice(-1);}
  if(row.mode==="empty"){plan.candidates=plan.candidates.slice(-1);plan.candidates.push({prefix:"f".repeat(20),generation:null,status:"unqualified",receiptFingerprint:null,rootStamp:"empty",roles:[],ownerReferences:[],payloadQualified:false});plan.generations.names=plan.candidates.map((value:any)=>value.prefix);}
  if(row.mode==="unknown")plan.candidates[1].ownerReferences=[{scope:"missing",generation:"f".repeat(64)}];
  if(row.mode==="cycle")plan.candidates[1].ownerReferences=[{scope:"root",generation:"a".repeat(64)}];
  if(row.mode==="current-references-old")plan.candidates[2].ownerReferences=[{scope:"old",generation:"a".repeat(64)}];
  if(row.mode==="unqualified")plan.candidates[0].status="unqualified";
  if(row.mode==="duplicate")plan.candidates.push(plan.candidates[0]);
  if(row.mode==="missing-current")plan.current.scopes[0].generation="f".repeat(64);
  if(row.mode==="active-mismatch")plan.candidates[0].status="active";
  if(row.mode==="empty-current")plan.current.scopes=[];
  if(row.mode==="busy")wrapped.acquired=false;
  if(row.mode==="extra")wrapped.callback=true;
  let accepted=true,result:string|null=null;try{const actual=api.readRepositoryDependencyRetirementPlanV1!(wrapped);result=api.selectRepositoryDependencyRetirementCandidateV1!(actual)?.prefix??null;}catch{accepted=false;}
  const reference=db.query(sql).get(JSON.stringify(plan))as{admitted:number;prefix:string|null};
  expect(accepted,row.mode).toBe(row.accepted);expect(accepted,row.mode).toBe(Boolean(validate(wrapped)&&reference.admitted));if(accepted){expect(result).toBe(row.expected);expect(result).toBe(reference.prefix);}
 }}finally{db.close();}
});

test("driver settlement DATA cannot change exact selection funding or effect kind",()=>{
 const read=sourceFunctions(["readRepositoryDependencyRetirementSettlementV1"]).readRepositoryDependencyRetirementSettlementV1!,validate=new Ajv({strict:true}).compile({...wire,$ref:"#/$defs/AcquiredSettlement"}),db=new Database(":memory:"),artifact=fixture.plan.artifact,baseSelection={kind:"inactive-generation",prefix:"a".repeat(20),generation:"a".repeat(64),receiptFingerprint:"fp-a",rootStamp:"root-a",currentBindingsSha256:fixture.plan.current.sha256};
 const modes=["observe","retire","empty","busy","journal","prefix","root","bindings","generation","receipt","operation","whole","effects","unretired","extra"];
 try{for(const mode of modes){
  const operation=mode==="retire"||mode==="unretired"?"retire":"observe",selection:any=structuredClone(baseSelection),snapshot:any={version:1,purpose:"current-selected-dependency-payload",kind:selection.kind,prefix:selection.prefix,generation:selection.generation,receiptFingerprint:selection.receiptFingerprint,rootStamp:selection.rootStamp,inventorySha256:"b".repeat(64),rosterSha256:"c".repeat(64),currentBindingsSha256:selection.currentBindingsSha256,entries:3,directories:1,files:1,links:1,logicalBytes:"4"};
  if(mode==="empty"){delete selection.generation;delete selection.receiptFingerprint;selection.kind="empty-generation";snapshot.kind=selection.kind;snapshot.generation=null;snapshot.receiptFingerprint=null;snapshot.entries=0;snapshot.directories=0;snapshot.files=0;snapshot.links=0;snapshot.logicalBytes="0";}
  const result:any={version:1,purpose:"current-selected-dependency-maintenance",operation,selected:snapshot,retired:operation==="retire",wholeAllocationRetired:false,publicationRetirementDispatches:0,journalSha256:artifact.journalSha256,journalBytes:artifact.journalBytes},wrapped:any={acquired:true,value:result};
  if(mode==="busy")wrapped.acquired=false;if(mode==="journal")result.journalSha256="f".repeat(64);if(mode==="prefix")snapshot.prefix="f".repeat(20);if(mode==="root")snapshot.rootStamp="changed";if(mode==="bindings")snapshot.currentBindingsSha256="f".repeat(64);if(mode==="generation")snapshot.generation="f".repeat(64);if(mode==="receipt")snapshot.receiptFingerprint="changed";if(mode==="operation")result.operation="retire";if(mode==="whole")result.wholeAllocationRetired=true;if(mode==="effects")result.publicationRetirementDispatches=1;if(mode==="unretired")result.retired=false;if(mode==="extra")wrapped.callback=true;
  let accepted=true;try{expect(read(wrapped,operation,selection,artifact)).toBe(result);}catch{accepted=false;}
  const oracle=db.query("SELECT json_extract(?1,'$.operation')=?2 AND json_extract(?1,'$.retired')=(?2='retire') AND json_extract(?1,'$.journalSha256')=json_extract(?4,'$.journalSha256') AND json_extract(?1,'$.journalBytes')=json_extract(?4,'$.journalBytes') AND json_extract(?1,'$.selected.kind')=json_extract(?3,'$.kind') AND json_extract(?1,'$.selected.prefix')=json_extract(?3,'$.prefix') AND json_extract(?1,'$.selected.rootStamp')=json_extract(?3,'$.rootStamp') AND json_extract(?1,'$.selected.currentBindingsSha256')=json_extract(?3,'$.currentBindingsSha256') AND json_extract(?1,'$.selected.generation') IS json_extract(?3,'$.generation') AND json_extract(?1,'$.selected.receiptFingerprint') IS json_extract(?3,'$.receiptFingerprint') accepted").get(JSON.stringify(result),operation,JSON.stringify(selection),JSON.stringify(artifact))as{accepted:number};
  expect(accepted,mode).toBe(["observe","retire","empty"].includes(mode));expect(accepted).toBe(Boolean(validate(wrapped)&&oracle.accepted));
 }}finally{db.close();}
});

test("production driver calls genuine owners with a fresh parent for every phase",async()=>{
 const owner=await import("../🟦️.ts");expect(typeof owner.retireObsoleteRepositoryDependencyGenerationsV1).toBe("function");const source=readFileSync(new URL("../🟦️.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("driver.ts",source,ts.ScriptTarget.Latest,true),node=tree.statements.find((row):row is ts.FunctionDeclaration=>ts.isFunctionDeclaration(row)&&row.name?.text==="retireObsoleteRepositoryDependencyGenerationsV1")!,calls:string[]=[];const visit=(value:ts.Node)=>{if(ts.isCallExpression(value))calls.push(value.expression.getText(tree));ts.forEachChild(value,visit);};visit(node);
 expect(node.parameters).toHaveLength(2);expect(calls.filter(name=>name==="observeRepositoryDependencyMaintenanceParentV1")).toHaveLength(3);expect(calls.filter(name=>name==="planSelectedRepositoryDependencyGenerationsV1")).toHaveLength(1);expect(calls.filter(name=>name==="maintainSelectedRepositoryDependencyGenerationV1")).toHaveLength(2);expect(node.getText(tree)).toContain("parent:beforeObserve.parent");expect(node.getText(tree)).toContain("parent:beforeRetire.parent");expect(node.getText(tree)).not.toContain("maximumWorkspaceBytes");expect(ts.transpileModule(source,{reportDiagnostics:true,compilerOptions:{target:ts.ScriptTarget.ES2022}}).diagnostics?.filter(row=>row.category===ts.DiagnosticCategory.Error)??[]).toEqual([]);
});


test("never-ready and capsule roles remain physical locators with current graph closure",()=>{
 const api=sourceFunctions(["selectRepositoryDependencyRetirementCandidateV1"]),db=new Database(":memory:");
 try{for(const row of fixture.unreadyCases){const plan:any=structuredClone(fixture.plan),metadata=row.mode.startsWith("metadata"),capsule=row.mode.startsWith("capsule")||metadata,candidate:any={prefix:capsule?".p00000000":"0".repeat(20),generation:capsule&&!metadata?"d".repeat(64):null,status:metadata?"unready-metadata":capsule?"retained-unready":"unready",receiptFingerprint:capsule&&!metadata?"capsule-fingerprint":null,rootStamp:"fresh",roles:metadata?["r"]:capsule?["i","p","r"]:["package.json","w"],ownerReferences:[],payloadQualified:false};
 if(row.mode==="capsule-same-current")candidate.generation=plan.current.scopes[0].generation;
 if(row.mode==="metadata-with-payload")candidate.roles.push("p");if(row.mode==="unready-with-ready")candidate.roles.push("bindings.json");if(row.mode==="capsule-with-references")candidate.ownerReferences.push({scope:".",generation:"c".repeat(64)});if(row.mode==="capsule-bad-prefix")candidate.prefix="0".repeat(20);
 if(row.mode.startsWith("dangling")||row.mode==="prefix-collision"){const source=plan.candidates[row.mode==="dangling-active"?2:1];source.ownerReferences.push({scope:"missing",generation:"f".repeat(64)});if(row.mode==="prefix-collision")candidate.prefix="f".repeat(20);}else plan.candidates.push(candidate);
 if(row.mode==="prefix-collision")plan.candidates.push(candidate);plan.generations.names=plan.candidates.map((value:any)=>value.prefix);
 let accepted=true,result:string|null=null;try{result=api.selectRepositoryDependencyRetirementCandidateV1!(plan)?.prefix??null;}catch{accepted=false;}
 const oracle=db.query("WITH n AS(SELECT value v,json_extract(value,'$.prefix') p,json_extract(value,'$.generation') g,json_extract(value,'$.status') s FROM json_each(?1,'$.candidates')), ready AS(SELECT * FROM n WHERE s IN ('active','inactive')), live AS(SELECT json_extract(value,'$.generation') g FROM json_each(?1,'$.current.scopes')), e AS(SELECT ready.p p,ready.s s,json_extract(value,'$.generation') t FROM ready,json_each(ready.v,'$.ownerReferences')),bad AS(SELECT (SELECT count(*) FROM n)<>(SELECT count(DISTINCT p) FROM n) OR (SELECT count(g) FROM ready)<>(SELECT count(DISTINCT g) FROM ready) OR NOT EXISTS(SELECT 1 FROM live) OR EXISTS(SELECT 1 FROM live WHERE NOT EXISTS(SELECT 1 FROM ready WHERE ready.g=live.g AND ready.s='active')) OR EXISTS(SELECT 1 FROM ready WHERE substr(g,1,20)<>p OR json_type(v,'$.receiptFingerprint')<>'text' OR ((s='active')<>EXISTS(SELECT 1 FROM live WHERE live.g=ready.g))) OR EXISTS(SELECT 1 FROM n WHERE s NOT IN ('active','inactive') AND (json_array_length(v,'$.ownerReferences')<>0 OR CASE WHEN s IN ('retained-unready','unready-metadata') THEN length(p)<>10 OR substr(p,1,2)<>'.p' OR substr(p,3) GLOB '*[^a-f0-9]*' ELSE length(p)<>20 OR p GLOB '*[^a-f0-9]*' END OR CASE WHEN s='retained-unready' THEN g IS NULL OR json_type(v,'$.receiptFingerprint')<>'text' OR NOT EXISTS(SELECT 1 FROM json_each(v,'$.roles') WHERE value='p') WHEN s='unready-metadata' THEN g IS NOT NULL OR json_type(v,'$.receiptFingerprint')<>'null' OR EXISTS(SELECT 1 FROM json_each(v,'$.roles') WHERE value NOT IN ('i','r')) ELSE g IS NOT NULL OR json_type(v,'$.receiptFingerprint')<>'null' OR EXISTS(SELECT 1 FROM json_each(v,'$.roles') WHERE value='bindings.json') END)) OR EXISTS(SELECT 1 FROM e WHERE NOT EXISTS(SELECT 1 FROM ready WHERE ready.g=e.t) AND (e.s='active' OR EXISTS(SELECT 1 FROM n WHERE n.p=substr(e.t,1,20)))) OR EXISTS(SELECT 1 FROM e WHERE e.s='active' AND NOT EXISTS(SELECT 1 FROM live WHERE live.g=e.t)) invalid), eligible AS(SELECT p FROM n WHERE s IN ('unready','retained-unready','unready-metadata') OR s='inactive' AND NOT EXISTS(SELECT 1 FROM e WHERE e.t=n.g) ORDER BY p LIMIT 1) SELECT NOT invalid admitted,(SELECT p FROM eligible) prefix FROM bad").get(JSON.stringify(plan))as{admitted:number;prefix:string|null};expect(accepted,row.mode).toBe(row.accepted);expect(accepted).toBe(Boolean(oracle.admitted));if(accepted){expect(result,row.mode).toBe(row.prefix);expect(result).toBe(oracle.prefix);}
 }}finally{db.close();}
});

test("retirement payload roles follow the selected descriptor and independent SQLite",()=>{
 const materialize=sourceFunctions(["materializeRepositoryDependencyRetirementSchemaV1"]).materializeRepositoryDependencyRetirementSchemaV1!,db=new Database(":memory:");
 try{for(const row of fixture.managerRoleCases){
  const manager=packageManagerRegistryV1.get(row.manager),resolved=materialize(manager.descriptor),roles=resolved.$defs.Protocol.const.unreadyPayloadRoles;
  const oracle=db.query("SELECT json_array('node_modules','w','patches','package.json',(SELECT json_extract(value,'$.name') FROM json_each(?1,'$.locks') WHERE json_extract(value,'$.role')='primary'),json_extract(?1,'$.configurations.scope[0]')) roles").get(JSON.stringify(manager.descriptor))as{roles:string};
  expect(roles).toEqual(row.roles);expect(roles).toEqual(JSON.parse(oracle.roles));expect(JSON.stringify(resolved)).not.toContain("{{");
  const api=sourceFunctions(["selectRepositoryDependencyRetirementCandidateV1"],resolved),plan:any=structuredClone(fixture.plan),candidate={prefix:"0".repeat(20),generation:null,status:"unready",receiptFingerprint:null,rootStamp:"current",roles:roles.slice(-2),ownerReferences:[]};
  plan.candidates.push(candidate);plan.generations.names.push(candidate.prefix);expect(api.selectRepositoryDependencyRetirementCandidateV1!(plan)?.prefix).toBe(candidate.prefix);
  candidate.roles=["unknown-manager-payload"];expect(()=>api.selectRepositoryDependencyRetirementCandidateV1!(plan)).toThrow();
 }}finally{db.close();}
});
