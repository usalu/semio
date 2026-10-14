import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv/dist/2020";
import fixture from "./🔣️.json" with {type:"json"};
import schema from "../../../🧬️schema/🚪️admission.json" with {type:"json"};
import {closeBuildOwnerAdmissionV1,readBuildOwnerRegistryV1,readBuildGraphSelectionV1} from "../🟦️.ts";

test("catalog activation requires actual consumer tasks, direct input custody or intentional all-owner admission",async()=>{
 const registry=readBuildOwnerRegistryV1(fixture.registry),oracle=new Ajv({strict:false}).compile(schema),db=new Database(":memory:");
 try{db.exec("CREATE TABLE consumer(root TEXT);CREATE TABLE task(owner TEXT,target TEXT);CREATE TABLE owner(name TEXT,root TEXT);INSERT INTO consumer VALUES ('registry');INSERT INTO owner VALUES ('app','app'),('registry','registry'),('provider','provider');");
 for(const row of fixture.cases){const selection=readBuildGraphSelectionV1(row.selection),result=await closeBuildOwnerAdmissionV1(selection,registry,async(owner,targets,inputs)=>({project:{name:owner.name,targets:owner.name==="app"?{build:{dependsOn:row.dependsOn}}:owner.name==="registry"?Object.fromEntries(row.registryTargets.map(target=>[target,{}])):{}},files:[owner.manifest,...owner.name==="app"&&targets.length?["registry/api.ts"]:[],...owner.name==="provider"&&inputs.length?["provider/publication.ts"]:[]],dependencies:owner.name==="app"&&targets.length?["registry"]:[],dependencyInputs:owner.name==="app"&&targets.length&&row.inputs.length?{provider:row.inputs}:{}}));
 expect(oracle(result),JSON.stringify(oracle.errors)).toBe(true);expect(result.catalogProviders.map(provider=>provider.sourcePath),row.name).toEqual(row.expectedProviders);if(row.name==="imported consumer Source"){expect(result.owners).toContain("registry");expect(result.files).toContain("registry/api.ts");expect(result.targets.registry).toBeUndefined();}
 db.exec("DELETE FROM task");for(const[name,targets]of Object.entries(result.targets))for(const target of targets)db.query("INSERT INTO task VALUES (?,?)").run(name,target);
 const selected=db.query("SELECT DISTINCT consumer.root FROM consumer JOIN owner ON owner.root=consumer.root OR owner.root LIKE consumer.root||'/%' JOIN task ON task.owner=owner.name").all();expect(selection.kind==="all"||selected.length>0||row.inputs.includes("{workspaceRoot}/provider/publication.ts")).toBe(row.expectedProviders.length>0);
 }
 console.log("[DEBUG] Actual catalog admission matches portable selection vectors, independent Ajv and SQLite task/root join while preserving imported consumer Source");
 }finally{db.close();}
});
