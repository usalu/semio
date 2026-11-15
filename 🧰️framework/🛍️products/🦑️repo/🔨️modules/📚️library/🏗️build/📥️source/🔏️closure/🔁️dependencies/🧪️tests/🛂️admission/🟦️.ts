/** 🛂️ Contract: a move of the private Nx projection sources while the Original plans is adopted by a fresh process when only code moved, ends the command (naming the files in both languages) when a contract file moved or the identity was already handed to an Nx process, and never hides a fresh process that fails. */
import{expect,test}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import admissionSchema from"../../🧬️schema/🔣️.json";
import fixture from"./🔣️.json";
import{dependencyReadmissionCodeV1,projectionAdmissionActionV1,projectionAdmissionV1,projectionFilesTextV1,withDependencyReadmissionV1}from"../../🟦️.ts";

const revisions=(...rows:string[])=>rows.map(path=>({path,sha256:"a".repeat(64)}));
const fake=(options:{moves:string[][];fromMoves?:string[][];fail?:string;identity?:string}={moves:[]})=>{
 const calls={changes:0,from:0,probe:0,identity:0},moves=[...options.moves],from=[...(options.fromMoves??[])];
 const api={
  nxBuildDependencyImplementationChangesV1:()=>{calls.changes++;return moves.shift()??[];},
  nxBuildDependencyImplementationV1:()=>{calls.identity++;return"1".repeat(64);},
  nxBuildDependencyImplementationChangesFromV1:()=>{calls.from++;return from.shift()??[];},
  nxBuildDependencyImplementationProbeV1:()=>{calls.probe++;if(options.fail)throw new Error(options.fail);return{identity:options.identity??"2".repeat(64),revisions:revisions("p.mjs")};},
 };
 return{api,calls};
};
const run=(api:ReturnType<typeof fake>["api"],language?:string)=>{const said:string[]=[];return{gate:projectionAdmissionV1(api,{environment:language?{SEMIO_LANGUAGE:language}:{},write:text=>said.push(text)}),said};};

test("the schema is closed and every fixture case validates",()=>{
 const validate=new Ajv({strict:true}).compile({...admissionSchema.$defs.Case,$defs:admissionSchema.$defs});
 for(const row of fixture.cases)expect(validate(row),row.name).toBe(true);
 expect(Object.keys(admissionSchema.$defs.Protocol.const.texts.en).sort()).toEqual(Object.keys(admissionSchema.$defs.Protocol.const.texts.de).sort());
});

test("the action of a move equals an independent SQL oracle",()=>{
 const database=new Database(":memory:");
 try{for(const row of fixture.cases){
  const oracle=database.query("SELECT CASE WHEN json_array_length(?1)=0 THEN 'keep' WHEN EXISTS(SELECT 1 FROM json_each(?1) WHERE value LIKE '%.json') THEN 'restart' ELSE 'adopt' END action").get(JSON.stringify(row.changes))as{action:string};
  expect(oracle.action,row.name).toBe(row.action);expect(projectionAdmissionActionV1(row.changes),row.name).toBe(row.action);
 }}finally{database.close();}
});

test("unchanged sources pass without a fresh process and hand out the loaded identity",()=>{
 const{api,calls}=fake({moves:[[],[]]}),{gate,said}=run(api);
 gate.admit();expect(gate.identity()).toBe("1".repeat(64));gate.admit();
 expect(calls).toEqual({changes:2,from:0,probe:0,identity:1});expect(said).toEqual([]);
});

test("a code-only move before the identity is adopted by one fresh process, announced in both languages without a restart",()=>{
 const{api,calls}=fake({moves:[["x/🟨️.mjs"]]}),{gate,said}=run(api);
 gate.admit();
 expect(calls.probe).toBe(1);expect(gate.adopted()).toBe(true);expect(gate.identity()).toBe("2".repeat(64));expect(calls.identity).toBe(0);
 expect(said).toHaveLength(1);expect(said[0]).toContain("x/🟨️.mjs");expect(said[0]).toContain("no restart");expect(said[0]).toContain("kein Neustart");
 gate.admit();expect(calls.probe).toBe(1);
});

test("the language of the invocation selects the only language of the announcement",()=>{
 const{api}=fake({moves:[["x/🟨️.mjs"]]}),{gate,said}=run(api,"de");
 gate.admit();expect(said[0]).toContain("kein Neustart");expect(said[0]).not.toContain("no restart");
});

test("after the identity was handed out every move ends the command, naming the files",()=>{
 const{api,calls}=fake({moves:[[],["x/🟨️.mjs"]]}),{gate,said}=run(api);
 gate.admit();gate.identity();
 let thrown:unknown;try{gate.admit();}catch(error){thrown=error;}
 expect(dependencyReadmissionCodeV1(thrown)).toBe(76);expect(calls.probe).toBe(0);
 const message=(thrown as Error).message;expect(message).toContain("x/🟨️.mjs");expect(message).toContain("starts again");expect(message).toContain("startet");expect(said.join("")).toContain("x/🟨️.mjs");
});

test("a moved contract file ends the command at once and never starts a fresh process",()=>{
 const{api,calls}=fake({moves:[["a/🟨️.mjs","a/🧬️schema/📥️pin.json"]]}),{gate}=run(api);
 let thrown:unknown;try{gate.admit();}catch(error){thrown=error;}
 expect(dependencyReadmissionCodeV1(thrown)).toBe(76);expect(calls.probe).toBe(0);expect((thrown as Error).message).toContain("📥️pin.json");
});

test("a fresh process that fails, or reports sources that moved again, ends the command",()=>{
 const failing=fake({moves:[["x/🟨️.mjs"]],fail:"boom"}),first=run(failing.api);
 let thrown:unknown;try{first.gate.admit();}catch(error){thrown=error;}
 expect(dependencyReadmissionCodeV1(thrown)).toBe(76);expect(first.said.join("")).toContain("boom");expect(first.said.join("")).toContain("konnte");
 const moving=fake({moves:[["x/🟨️.mjs"]],fromMoves:[["y/🟨️.mjs"]]}),second=run(moving.api);
 thrown=undefined;try{second.gate.admit();}catch(error){thrown=error;}
 expect(dependencyReadmissionCodeV1(thrown)).toBe(76);expect((thrown as Error).message).toContain("y/🟨️.mjs");expect(second.gate.adopted()).toBe(false);
});

test("a long list names the declared number of files and counts the rest in both languages",()=>{
 const files=["a","b","c","d","e","f"];
 expect(projectionFilesTextV1("en",files)).toBe("a, b, c, d, and 2 more");expect(projectionFilesTextV1("de",files)).toBe("a, b, c, d, und 2 weitere");
});

test("the handoff of the Nx step to a fresh process says why in both languages",async()=>{
 const failure={name:"OwnedCommandFailure",outcome:{version:1,status:76,signal:null}};
 await expect(withDependencyReadmissionV1(async()=>{throw failure;},{})).rejects.toThrow(/fresh process admission[\s\S]*frische Prozess-Zulassung/u);
 await expect(withDependencyReadmissionV1(async()=>{throw failure;},{SEMIO_LANGUAGE:"de"})).rejects.not.toThrow(/fresh process admission/u);
});
