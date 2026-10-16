import {test,expect} from "bun:test";
import {createHash} from "node:crypto";
import {existsSync,mkdtempSync,mkdirSync,writeFileSync,readFileSync,rmSync} from "node:fs";
import {join,dirname,resolve} from "node:path";
import {pathToFileURL} from "node:url";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "../🧬️schema/🔣️.json";
import corpus from "./🔣️.json";

/** 📦️ Complete declared compilation closure DATA agrees with independent actual Node bytes and SQL inventory. */
test("neutral completed Cargo retention keeps dependency generated and resource closure",async()=>{
 const owner=resolve(import.meta.dir,"../🟦️.ts");expect(existsSync(owner),"actual neutral retained-compilation owner").toBe(true);if(!existsSync(owner))throw Error("Actual completed evidence owner required");const api=await import(pathToFileURL(owner).href),sink=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!sink)throw Error("Ticket generated DATA sink required");const root=mkdtempSync(join(sink,"retention-")),database=new Database(":memory:");try{database.exec("create table expected(path text primary key,sha text,custody text)");const digest=(value:Uint8Array|string)=>createHash("sha256").update(value).digest("hex");for(const row of corpus.files){const path=join(root,row.path);mkdirSync(dirname(path),{recursive:true});writeFileSync(path,row.text);database.query("insert into expected values(?,?,?)").run(path,digest(readFileSync(path)),row.custody);}const origin={version:1,workspace:root,mode:"prototype",manifest:join(root,"Cargo.toml"),cwd:root,buildDirectory:join(root,"paid/build"),targetDirectory:join(root,"paid/target"),cargoHome:join(root,"paid/cargo")};expect(new Ajv({strict:true}).validate({...schema,$ref:"#/definitions/Origin"},origin)).toBe(true);const files=corpus.files.map(row=>({path:join(root,row.path),kind:"file",sha256:digest(readFileSync(join(root,row.path)))})),observation={version:1,manifest:origin.manifest,cwd:root,buildDirectory:origin.buildDirectory,status:0,cancelled:false,units:[{message:{},depInfo:[],inputs:files.slice(0,6),artifacts:files.slice(6).map(({path,sha256})=>({path,sha256}))}],invocationInputs:corpus.absent.map(path=>({path:join(root,path),sha256:null})),buildResources:[],compilerResources:[]},actual=api.completedCargoClosureRosterV1(observation,origin) as readonly {path:string;sha256:string|null;custody:string;kind:string}[];expect(actual.filter(row=>row.kind!=="absent").map(({path,sha256,custody})=>({path,sha:sha256,custody}))).toEqual(database.query("select * from expected order by path").all());expect(actual.filter(row=>row.kind==="absent").map(row=>row.path)).toEqual(corpus.absent.map(path=>join(root,path)));expect(Object.isFrozen(actual)&&actual.every(Object.isFrozen)).toBe(true);
 }finally{database.close();rmSync(root,{recursive:true,force:true});}
});

/** 🧊️ Public schema DATA cannot change the private origin/path contract after module load. */
test("neutral completed Cargo retention resists public schema DATA mutation",async()=>{
 const owner=resolve(import.meta.dir,"../🟦️.ts"),api=await import(pathToFileURL(owner).href),limit=schema.definitions.Path.maxLength,enumBefore=[...schema.definitions.Origin.properties.mode.enum],root=resolve(process.cwd()),origin={version:1,workspace:root,mode:"prototype",manifest:join(root,"Cargo.toml"),cwd:root,buildDirectory:join(root,"build"),targetDirectory:join(root,"target"),cargoHome:join(root,"cargo")};try{schema.definitions.Path.maxLength=0;schema.definitions.Origin.properties.mode.enum.splice(0,2,"invalid");expect(api.readCompletedCargoOriginV1(origin)).toEqual(origin);}finally{schema.definitions.Path.maxLength=limit;schema.definitions.Origin.properties.mode.enum.splice(0,schema.definitions.Origin.properties.mode.enum.length,...enumBefore);}
});

/** 🔐️ A source path beneath a build root still cannot masquerade as an actual completed output. */
test("neutral completed Cargo retention distinguishes actual output claims from mutable inputs",async()=>{
 const api=await import(pathToFileURL(resolve(import.meta.dir,"../🟦️.ts")).href),root=resolve(process.cwd()),origin={version:1,workspace:root,mode:"prototype",manifest:join(root,"Cargo.toml"),cwd:root,buildDirectory:join(root,"build"),targetDirectory:join(root,"target"),cargoHome:join(root,"cargo")},sha="a".repeat(64),input=join(origin.buildDirectory,"mutable-source.rs"),artifact=join(origin.targetDirectory,"dependency.rlib"),observation={version:1,manifest:origin.manifest,cwd:root,buildDirectory:origin.buildDirectory,status:0,cancelled:false,units:[{message:{},depInfo:[],inputs:[{path:input,kind:"file",sha256:sha}],artifacts:[{path:artifact,sha256:sha}]}],invocationInputs:[],buildResources:[],compilerResources:[]};expect(api.completedCargoClosureRosterV1(observation,origin)).toHaveLength(2);expect(api.completedCargoOutputClaimsV1(observation,origin).map((row:{path:string})=>row.path)).toEqual([artifact]);
});
