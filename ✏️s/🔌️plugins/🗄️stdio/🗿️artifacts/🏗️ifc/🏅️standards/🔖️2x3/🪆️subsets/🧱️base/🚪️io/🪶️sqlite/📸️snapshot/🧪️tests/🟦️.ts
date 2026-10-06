/** 🏭️ IFC2x3 exact decimal fields and EDM data remain independently queryable. */
import {Database} from "bun:sqlite";
import {expect,test} from "bun:test";
import f from "../🧫️fixtures/🔣️.json";
import type {Ifc2x3Snapshot,Part21Value} from "../../../../🧬️schema/📸️snapshot/🟦️.ts";
import type {Ifc2x3Artifact,Ifc2x3Diff,Ifc2x3Mutation} from "@semio-tech/stdio-ifc";
import {IFC2X3_SQLITE_SCHEMA,ifc2x3SnapshotToSqliteDatabase,ifc2x3SnapshotFromSqliteDatabase} from "../🟦️.ts";
import {exportSqliteDatabase,importSqliteDatabase} from "@semio-tech/framework";
function fixture():Ifc2x3Snapshot{return{schema:f.schema,document:{header:{fileDescription:[{kind:"list",values:[{kind:"str",value:"ViewDefinition [CoordinationView, StructuralAnalysisView, FMHandOverView]"}]},{kind:"typed",typeName:"HEADER_CUSTOM",values:[]}],fileName:[{kind:"ref",value:0n}],fileSchema:[{kind:"list",values:[{kind:"str",value:"IFC2X3"},{kind:"str",value:"IFC2X3"}]}]},instances:[{id:0n,entities:[{typeName:"IFCWALL",arguments:[{kind:"unset"},{kind:"derived"},...f.integerWords.map(word=>({kind:"int" as const,value:BigInt(word)})),...f.decimals.map(decimal=>({kind:"real" as const,value:structuredClone(decimal)})),{kind:"str",value:f.text},{kind:"enum",value:"UNKNOWN"},{kind:"ref",value:BigInt(f.instanceWords[1]!)},{kind:"list",values:[]},{kind:"typed",typeName:"IFCLENGTHMEASURE",values:[{kind:"int",value:1n},{kind:"list",values:[{kind:"derived"}]}]}]},{typeName:"IFCROOT",arguments:[{kind:"ref",value:0n}]}]},{id:BigInt(f.instanceWords[1]!),entities:[{typeName:"TARGET",arguments:[{kind:"ref",value:0n}]}]}]},edmPreamble:structuredClone(f.edmPreamble)};}
test("IFC2x3 independent exact decimal and optional EDM fields",async()=>{expect(IFC2X3_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../🗄️.sql",import.meta.url)).text());const input=fixture(),db=Database.deserialize(await exportSqliteDatabase(await ifc2x3SnapshotToSqliteDatabase(input)));try{expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);expect((db.query("SELECT kind FROM ifc2x3_value GROUP BY kind ORDER BY kind").all()as{kind:string}[]).map(row=>row.kind)).toEqual([...f.valueKinds].sort());expect(db.query("SELECT decimal_negative,decimal_coefficient,decimal_scale,decimal_exponent FROM ifc2x3_value WHERE kind='real' ORDER BY id").all()).toEqual(f.decimals.map(d=>({decimal_negative:d.negative?1:0,decimal_coefficient:d.coefficient,decimal_scale:d.scale,decimal_exponent:d.exponent??null})));expect(await ifc2x3SnapshotFromSqliteDatabase(await importSqliteDatabase(db.serialize()))).toEqual(input);db.query("UPDATE ifc2x3_edm_preamble SET options='SQL_EDIT'").run();input.edmPreamble!.options="SQL_EDIT";expect(await ifc2x3SnapshotFromSqliteDatabase(await importSqliteDatabase(db.serialize()))).toEqual(input);}finally{db.close();}const absent=fixture();delete absent.edmPreamble;expect(await ifc2x3SnapshotFromSqliteDatabase(await ifc2x3SnapshotToSqliteDatabase(absent))).toEqual(absent);});
test("IFC2x3 artifact/diff/mutation carry exact decimal and identity state",async()=>{const artifact:Ifc2x3Artifact=fixture(),diff:Ifc2x3Diff={header:artifact.document.header,upsertedInstances:artifact.document.instances,removedInstances:[18446744073709551615n],instanceOrder:[0n,18446744073709551615n],edmPreamble:null},mutation:Ifc2x3Mutation={mutation:"upsertInstance",instance:diff.upsertedInstances![1]!};if(mutation.mutation!=="upsertInstance")throw Error("mutation");expect(mutation.instance.id).toBe(18446744073709551615n);expect(await ifc2x3SnapshotFromSqliteDatabase(await ifc2x3SnapshotToSqliteDatabase(artifact))).toEqual(artifact);expect(diff.instanceOrder).toEqual([0n,18446744073709551615n]);});
test("IFC2x3 independent corrupt ownership decimal width and optional EDM rejected",async()=>{const bytes=await exportSqliteDatabase(await ifc2x3SnapshotToSqliteDatabase(fixture()));for(const sql of["UPDATE ifc2x3_entity_argument SET value_id=999 WHERE id=1","UPDATE ifc2x3_value SET reference_instance_id='00' WHERE kind='ref'","UPDATE ifc2x3_instance SET instance_id='0' WHERE id=2","UPDATE ifc2x3_document SET edm_preamble_id=NULL","UPDATE ifc2x3_file_name_argument SET ordinal=1 WHERE id=1","UPDATE ifc2x3_value SET decimal_scale=4294967296 WHERE kind='real'","UPDATE ifc2x3_value SET decimal_exponent=2147483648 WHERE kind='real'","UPDATE ifc2x3_value SET decimal_negative=2 WHERE kind='real'","UPDATE ifc2x3_list_element SET value_id=list_value_id WHERE id=1"]){const db=Database.deserialize(bytes);try{db.run("PRAGMA ignore_check_constraints=ON");db.run(sql);await expect(ifc2x3SnapshotFromSqliteDatabase(await importSqliteDatabase(db.serialize()))).rejects.toThrow();}finally{db.close();}}});
test("IFC2x3 cancellation and budgets bound exact decimal validation and deep graphs",async()=>{const input=fixture(),database=await ifc2x3SnapshotToSqliteDatabase(input);await expect(ifc2x3SnapshotToSqliteDatabase(input,{maxRows:1})).rejects.toThrow();await expect(ifc2x3SnapshotFromSqliteDatabase(database,{maxValueBytes:1})).rejects.toThrow();input.document.instances[0]!.entities[0]!.arguments=[{kind:"real",value:{negative:false,coefficient:"1".repeat(100000),scale:4294967295}}];const cancel=new AbortController();let large=false;await expect(ifc2x3SnapshotToSqliteDatabase(input,{signal:cancel.signal,onProgress:p=>{if(p.phase==="projectSnapshot"&&p.total===100000){large=true;cancel.abort();}}})).rejects.toThrow();expect(large).toBe(true);let value:Part21Value={kind:"unset"};for(let i=0;i<1200;i++)value={kind:"list",values:[value]};input.document.instances[0]!.entities[0]!.arguments=[value];const restored=await ifc2x3SnapshotFromSqliteDatabase(await ifc2x3SnapshotToSqliteDatabase(input));let cursor=restored.document.instances[0]!.entities[0]!.arguments[0]!,depth=0;while(cursor.kind==="list"){depth++;cursor=cursor.values[0]!;}expect(depth).toBe(1200);const cycle:Part21Value={kind:"list",values:[]};cycle.values.push(cycle);input.document.instances[0]!.entities[0]!.arguments=[cycle];await expect(ifc2x3SnapshotToSqliteDatabase(input)).rejects.toThrow(/cyclic/);});

import Ajv from "ajv";
import cohort from "../../../../../../../../../../📇️registry/🧬️contract/📐️part21/🧫️fixtures/🚦️sqlite-cohort/🔣️.json";


test("Part21 full header and retirement-name neutral fields remain independently interpretable",async()=>{
 const input=fixture();input.schema="";
 input.document.header={fileDescription:structuredClone(cohort.headerCase.fileDescription)as Part21Value[],fileName:[],fileSchema:structuredClone(cohort.headerCase.fileSchema)as Part21Value[]};
 const physical=Database.deserialize(await exportSqliteDatabase(await ifc2x3SnapshotToSqliteDatabase(input)));
 try{
  expect(physical.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});
  expect(await ifc2x3SnapshotFromSqliteDatabase(await importSqliteDatabase(physical.serialize()))).toEqual(input);
  const row=cohort.retirementCase;
  const counts=physical.query("SELECT length(CAST(? AS BLOB))+length(CAST(? AS BLOB))+length(CAST(? AS BLOB))+8 AS bytes").get(row.name,row.typedName,row.value)as{bytes:number};
  expect(counts.bytes).toBe(row.expectedReleasedBytes);
  expect(cohort.unsignedWords.map(BigInt)).toEqual([0n,18446744073709551615n]);
  expect(cohort.binary64Words.map(word=>{const view=new DataView(new ArrayBuffer(8));view.setBigUint64(0,BigInt(`0x${word}`),true);return view.getBigUint64(0,true).toString(16).padStart(16,"0");})).toEqual(cohort.binary64Words);
 }finally{physical.close();}
});
test("Part21 shared cohort keeps exact decimal authority through IFC2x3 semantic files",async()=>{
 
 const input=fixture();input.schema="";input.document.instances[0]!.entities[0]!.arguments=[{kind:"ref",value:18446744073709551615n},{kind:"str",value:cohort.literalText},{kind:"enum",value:"UNKNOWN"},{kind:"int",value:-9223372036854775808n},...cohort.decimals.map(value=>({kind:"real" as const,value:{...value}})),{kind:"list",values:[]},{kind:"typed",typeName:"",values:[]},{kind:"unset"},{kind:"derived"}];
 const bytes=await exportSqliteDatabase(await ifc2x3SnapshotToSqliteDatabase(input));expect([...bytes.subarray(0,16)]).toEqual([...new TextEncoder().encode("SQLite format 3\0")]);const db=Database.deserialize(bytes);
 try{expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
 expect((db.query("SELECT DISTINCT kind FROM ifc2x3_value ORDER BY kind").all() as {kind:string}[]).map(row=>row.kind)).toEqual([...cohort.valueKinds].sort());
 expect(db.query("SELECT decimal_negative,decimal_coefficient,decimal_scale,decimal_exponent FROM ifc2x3_value WHERE kind='real' ORDER BY id").all()).toEqual(cohort.decimals.map(d=>({decimal_negative:d.negative?1:0,decimal_coefficient:d.coefficient,decimal_scale:d.scale,decimal_exponent:d.exponent??null})));
 expect(await ifc2x3SnapshotFromSqliteDatabase(await importSqliteDatabase(db.serialize()))).toEqual(input);
 }finally{db.close();}
});
test("Part21 shared cohort bounds real Unicode ownership without decimal expansion",async()=>{
 const input=fixture();input.document.instances[0]!.entities[0]!.arguments=[{kind:"str",value:cohort.longText.unit.repeat(cohort.longText.repeat)},{kind:"real",value:{...cohort.decimals[1]!}}];
 const database=await ifc2x3SnapshotToSqliteDatabase(input);expect(await ifc2x3SnapshotFromSqliteDatabase(database)).toEqual(input);
 for(const phase of ["projectSnapshot","reconstructSnapshot"] as const){let reached=false;const options={onProgress:(p:{phase:string,completed:number,total:number})=>{if(p.phase===phase&&p.total>=16384&&p.completed>=16384&&p.completed<p.total){reached=true;throw Error("cohort canceled")}}};
 await expect(phase==="projectSnapshot"?ifc2x3SnapshotToSqliteDatabase(input,options):ifc2x3SnapshotFromSqliteDatabase(database,options)).rejects.toThrow();expect(reached).toBe(true);
 }
});

test("IFC2x3 literal owned coefficients and independent positive SQL surrogates preserve full state",async()=>{
 
 const input=fixture();input.schema=cohort.nativeOwned.schema;
 input.document.header={fileDescription:structuredClone(cohort.headerCase.fileDescription)as Part21Value[],fileName:[],fileSchema:structuredClone(cohort.headerCase.fileSchema)as Part21Value[]};
 const coefficients=[...cohort.nativeOwned.decimalCoefficients,...cohort.decimalOwnedExtras.map(value=>value.coefficient),cohort.literalText];
 input.document.instances[0]!.entities[0]!.arguments=coefficients.map(coefficient=>({kind:"real" as const,value:{negative:true,coefficient,scale:4294967295,exponent:-2147483648}}));
 const physical=Database.deserialize(await exportSqliteDatabase(await ifc2x3SnapshotToSqliteDatabase(input)));
 try{
  const queried=physical.query("SELECT hex(CAST(decimal_coefficient AS BLOB)) AS coefficient,decimal_negative,decimal_scale,decimal_exponent FROM ifc2x3_value WHERE kind='real' ORDER BY id").all();
  expect(queried).toEqual(coefficients.map(coefficient=>({coefficient:Buffer.from(coefficient).toString("hex").toUpperCase(),decimal_negative:1,decimal_scale:4294967295,decimal_exponent:-2147483648})));
  physical.run("PRAGMA foreign_keys=OFF");const offset=cohort.nativeOwned.surrogateOffset;
  physical.run(`UPDATE ifc2x3_document SET id=id+${offset},header_id=header_id+${offset},edm_preamble_id=edm_preamble_id+${offset}`);
  physical.run(`UPDATE ifc2x3_header SET id=id+${offset}`);physical.run(`UPDATE ifc2x3_edm_preamble SET id=id+${offset}`);
  for(const table of["ifc2x3_file_description_argument","ifc2x3_file_name_argument","ifc2x3_file_schema_argument"])physical.run(`UPDATE ${table} SET header_id=header_id+${offset}`);
  physical.run(`UPDATE ifc2x3_instance SET document_id=document_id+${offset}`);
  expect(physical.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(physical.query("PRAGMA foreign_key_check").all()).toEqual([]);
  expect(await ifc2x3SnapshotFromSqliteDatabase(await importSqliteDatabase(physical.serialize()))).toEqual(input);
  physical.run("UPDATE ifc2x3_value SET decimal_coefficient='1x' WHERE kind='real'");
  input.document.instances[0]!.entities[0]!.arguments.forEach(value=>{if(value.kind==="real")value.value.coefficient="1x";});
  expect(await ifc2x3SnapshotFromSqliteDatabase(await importSqliteDatabase(physical.serialize()))).toEqual(input);
 }finally{physical.close();}
});

test("IFC2x3 native backing and semantic cell limits have closed independent roles",async()=>{
 const vector=await Bun.file(new URL("../🧫️fixtures/💰️backing/🔣️.json",import.meta.url)).json();
 expect(vector["forecast"]).toEqual("unpaidBorrowedBound");expect(vector["allocation"]).toEqual("fullConcreteBufferBeforeConstruction");expect(vector["retirement"]).toEqual("noCumulativeRefund");expect(vector["inlineRoot"]).toEqual("noBackingRequest");expect(vector["semanticTables"]).toEqual(12);expect(vector["limits"]["semantic"]).toEqual("maxValueBytes");expect(vector["limits"]["backing"]).toEqual("maxAllocationBytes");expect(vector["limits"]["file"]).toEqual("maxFileBytes");expect(vector["nativeRoles"]["owned"]).toEqual("maxAllocationBytes");expect(vector["nativeRoles"]["semantic"]).toEqual("maxValueBytes");expect(vector["nativeRoles"]["preflight"]).toEqual("paidFrontiersOnly");expect(vector["nativeRoles"]["decimal"]).toEqual("literalUtf8Coefficient");
 expect(vector.inlineRoot).toBe("noBackingRequest");expect(vector.nativeRoles.decimal).toBe("literalUtf8Coefficient");
 const db=new Database(":memory:");try{
  db.run("CREATE TABLE role(kind TEXT PRIMARY KEY,limit_name TEXT NOT NULL)");for(const[kind,limit]of Object.entries(vector.limits))db.run("INSERT INTO role VALUES(?,?)",[kind,String(limit)]);
  expect(db.query("SELECT kind,limit_name FROM role ORDER BY kind").all()).toEqual([{kind:"backing",limit_name:"maxAllocationBytes"},{kind:"file",limit_name:"maxFileBytes"},{kind:"semantic",limit_name:"maxValueBytes"}]);
  db.run("CREATE TABLE admission(request INTEGER NOT NULL CHECK(request>=0),retired INTEGER NOT NULL)");for(const request of vector.ledger.requests)db.run("INSERT INTO admission VALUES(?,1)",[request]);
  expect(db.query("SELECT sum(request) AS spent FROM admission").get()).toEqual({spent:vector.ledger.expectedSpent});expect(vector.ledger.exact).toBe(vector.ledger.expectedSpent);expect(vector.ledger.refused).toBe(vector.ledger.expectedSpent-1);
  db.exec(IFC2X3_SQLITE_SCHEMA);expect(db.query("SELECT count(*) AS count FROM sqlite_schema WHERE type='table' AND name LIKE 'ifc2x3_%'").get()).toEqual({count:vector.semanticTables});
 }finally{db.close();}
});

test("IFC2x3 subset history snapshots have independent complete relational witnesses",async()=>{
 for(const[folder,id]of[["🤝️cv20","cv20"],["🧮️sav","sav"],["🏢️cobie","cobie"]]){
  const root=new URL("../../../../../"+folder+"/🧫️fixtures/🧬️mutations/📸️set-snapshot/📋️rename-file/",import.meta.url);
  const before=await Bun.file(new URL("📸️snapshot/⬅️before/🔣️.json",root)).json()as Ifc2x3Snapshot;
  const after=await Bun.file(new URL("📸️snapshot/➡️after/🔣️.json",root)).json()as Ifc2x3Snapshot;
  const mutation=await Bun.file(new URL("🦠️mutation/🔣️.json",root)).json();
  expect(mutation).toEqual({SetSnapshot:{snapshot:after}});
  expect(await Bun.file(new URL("🎯️outcome/🔣️.json",root)).json()).toEqual({status:"applied"});
  for(const[side,input]of[["⬅️before",before],["➡️after",after]]as const){
   const text=await Bun.file(new URL("📸️snapshot/"+side+"/🗣️.dsl.semio",root)).text();
   expect(text.startsWith("semio stdio.ifc.2x3.dsl v1\n")).toBe(true);
   const physical=Database.deserialize(await exportSqliteDatabase(await ifc2x3SnapshotToSqliteDatabase(input)));
   try{
    expect(physical.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});
    expect(physical.query("PRAGMA foreign_key_check").all()).toEqual([]);
    expect(physical.query("SELECT count(*) AS count FROM sqlite_schema WHERE type='table' AND name LIKE 'ifc2x3_%'").get()).toEqual({count:12});
    const name=id+(side==="⬅️before"?"-before.ifc":"-after.ifc");
    expect(physical.query("SELECT v.string_value AS name FROM ifc2x3_file_name_argument AS a JOIN ifc2x3_value AS v ON v.id=a.value_id ORDER BY a.ordinal").all()).toEqual([{name}]);
    expect(await ifc2x3SnapshotFromSqliteDatabase(await importSqliteDatabase(physical.serialize()))).toEqual(input);
    if(side==="⬅️before"){
     physical.query("UPDATE ifc2x3_value SET string_value=? WHERE id=(SELECT value_id FROM ifc2x3_file_name_argument WHERE ordinal=0)").run(id+"-after.ifc");
     expect(await ifc2x3SnapshotFromSqliteDatabase(await importSqliteDatabase(physical.serialize()))).toEqual(after);
    }
   }finally{physical.close();}
  }
 }
});

test("IFC2x3 removal-result references preserve full owner through independently edited files",async()=>{
 const corpus=await Bun.file(new URL("../🧫️fixtures/🧩️intermediate/🔣️.json",import.meta.url)).json();
 
 const {default:Ajv}=await import("ajv");
 for(const case_ of corpus.cases){
  const input=fixture();input.schema=corpus.schema;input.document.instances=input.document.instances.filter(instance=>instance.id!==BigInt(case_.removedId));
  const physical=Database.deserialize(await exportSqliteDatabase(await ifc2x3SnapshotToSqliteDatabase(input)));
  try{
   expect(physical.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(physical.query("PRAGMA foreign_key_check").all()).toEqual([]);
   expect(physical.query("SELECT instance_id FROM ifc2x3_instance ORDER BY ordinal").all()).toEqual(case_.remainingIds.map((instance_id:string)=>({instance_id})));
   expect(physical.query("SELECT v.reference_instance_id AS literal,e.instance_id AS resolved FROM ifc2x3_value AS v LEFT JOIN ifc2x3_instance AS e ON e.id=v.reference_instance_row_id WHERE v.kind='ref' ORDER BY v.id").all()).toEqual(case_.references);
   expect(await ifc2x3SnapshotFromSqliteDatabase(await importSqliteDatabase(physical.serialize()))).toEqual(input);
   physical.query("UPDATE ifc2x3_value SET reference_instance_id=? WHERE kind='ref' AND reference_instance_row_id IS NULL").run(corpus.editReference);
   function edit(value:Part21Value){if(value.kind==="ref"&&value.value===BigInt(case_.removedId))value.value=BigInt(corpus.editReference);else if(value.kind==="list"||value.kind==="typed")value.values.forEach(edit);}
   [...input.document.header.fileDescription,...input.document.header.fileName,...input.document.header.fileSchema,...input.document.instances.flatMap(instance=>instance.entities.flatMap(entity=>entity.arguments))].forEach(edit);
   expect(await ifc2x3SnapshotFromSqliteDatabase(await importSqliteDatabase(physical.serialize()))).toEqual(input);
   console.log("[DEBUG] IFC2x3 full intermediate reference owner",case_.removedId);
  }finally{physical.close();}
 }
});
test("IFC2x3 complete positive SQL surrogates preserve literal native identities",async()=>{
 const corpus=await Bun.file(new URL("../🧫️fixtures/🧩️intermediate/🔣️.json",import.meta.url)).json(),input=fixture(),physical=Database.deserialize(await exportSqliteDatabase(await ifc2x3SnapshotToSqliteDatabase(input)));
 try{
  physical.exec("PRAGMA foreign_keys=OFF");
  for(const{name}of physical.query("SELECT name FROM sqlite_schema WHERE type='table'").all()as{name:string}[]){
   const key=(physical.query("PRAGMA table_info("+name+")").all()as{name:string;pk:number}[]).find(column=>column.pk===1)!.name;
   const links=physical.query("PRAGMA foreign_key_list("+name+")").all()as{from:string}[];
   physical.exec("UPDATE "+name+" SET "+[key+"="+key+"+"+corpus.surrogateOffset,...links.filter(link=>link.from!==key).map(link=>link.from+"="+link.from+"+"+corpus.surrogateOffset)].join(","));
  }
  expect(physical.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(physical.query("PRAGMA foreign_key_check").all()).toEqual([]);
  expect(await ifc2x3SnapshotFromSqliteDatabase(await importSqliteDatabase(physical.serialize()))).toEqual(input);console.log("[DEBUG] IFC2x3 full shifted-surrogate owner",corpus.surrogateOffset);
 }finally{physical.close();}
});

test("IFC2x3 resolved reference witnesses agree exactly with literal native words",async()=>{
 const bytes=await exportSqliteDatabase(await ifc2x3SnapshotToSqliteDatabase(fixture()));
 for(const sql of["UPDATE ifc2x3_value SET reference_instance_row_id=NULL WHERE kind='ref'","UPDATE ifc2x3_value SET reference_instance_row_id=(SELECT id FROM ifc2x3_instance WHERE instance_id='18446744073709551615') WHERE kind='ref' AND reference_instance_id='0'"]){
  const physical=Database.deserialize(bytes);try{physical.exec("PRAGMA ignore_check_constraints=ON");physical.exec(sql);await expect(ifc2x3SnapshotFromSqliteDatabase(await importSqliteDatabase(physical.serialize()))).rejects.toThrow();}finally{physical.close();}
 }
});

import NormSemanticAjv from "ajv/dist/2020.js";
import normSemanticContract from "../🧫️fixtures/🎛️semantic.json";

import {ifc2x3SnapshotToSqliteDatabase as normSemanticProject,ifc2x3SnapshotFromSqliteDatabase as normSemanticRestore} from "../🟦️.ts";
import {exportSqliteDatabase as normSemanticExport,importSqliteDatabase as normSemanticImport} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import {independentSqliteExtent as normIndependentExtent} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️tests/🔮️semantic-extent/🟦️.ts";
test("IFC2X3 complete independent cells preserve copied limits and required empty metadata",async()=>{
 expect(normSemanticContract["schemaBytes"]).toEqual(4006);expect(normSemanticContract["tableWidths"]).toEqual({"ifc2x3_document":4,"ifc2x3_edm_preamble":17,"ifc2x3_entity_argument":4,"ifc2x3_entity_type":4,"ifc2x3_file_description_argument":4,"ifc2x3_file_name_argument":4,"ifc2x3_file_schema_argument":4,"ifc2x3_header":1,"ifc2x3_instance":4,"ifc2x3_list_element":4,"ifc2x3_typed_argument":4,"ifc2x3_value":12});expect(normSemanticContract["cases"]).toEqual([{"id":"originalSourceFull","rows":58,"valueBytes":1847},{"id":"emptyEntitiesRetainHeader","rows":17,"valueBytes":587}]);
 for(const item of normSemanticContract.cases){const source=fixture();if(item.id==="emptyEntitiesRetainHeader"){source.document.instances=[];}const database=await normSemanticProject(source),bytes=await normSemanticExport(database);expect(normIndependentExtent(bytes)).toEqual({rows:item.rows,valueBytes:item.valueBytes,schemaBytes:normSemanticContract.schemaBytes,tableWidths:normSemanticContract.tableWidths});const limits={maxRows:item.rows,maxValueBytes:item.valueBytes,maxSchemaBytes:normSemanticContract.schemaBytes,maxTables:12,maxColumns:17};expect(await normSemanticProject(source,limits)).toEqual(database);expect(await normSemanticRestore(await normSemanticImport(bytes),limits)).toEqual(source);
  for(const short of[{...limits,maxRows:item.rows-1},{...limits,maxValueBytes:item.valueBytes-1},{...limits,maxSchemaBytes:limits.maxSchemaBytes-1},{...limits,maxTables:11},{...limits,maxColumns:16}]){await expect(normSemanticProject(source,short)).rejects.toThrow();await expect(normSemanticRestore(database,short)).rejects.toThrow();}
 }console.log("[DEBUG] IFC2X3 complete independent SQLite cells and metadata preserve full and empty semantic limits");
});
