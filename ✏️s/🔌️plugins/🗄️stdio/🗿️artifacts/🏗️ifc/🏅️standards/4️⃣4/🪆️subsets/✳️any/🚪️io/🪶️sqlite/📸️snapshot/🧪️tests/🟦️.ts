/** 🏗️ IFC4 header/entity values are domain-owned and preserve their IEEE words. */
import {Database} from "bun:sqlite";
import {expect,test} from "bun:test";
import f from "../🧫️fixtures/🔣️.json";
import type {IfcSnapshot,IfcValue} from "../../../../🧬️schema/📸️snapshot/🟦️.ts";
import type {IfcArtifact,IfcDiff,IfcMutation} from "@semio-tech/stdio-ifc";
import {IFC_SQLITE_SCHEMA,ifcSnapshotToSqliteDatabase,ifcSnapshotFromSqliteDatabase} from "../🟦️.ts";
import {exportSqliteDatabase,importSqliteDatabase} from "@semio-tech/framework";
function fixture():IfcSnapshot{return{schema:f.schema,header:{fileDescription:[{kind:"aggregate",value:[{kind:"string",value:"description"}]},{kind:"typedValue",value:{name:"HEADER_CUSTOM",items:[]}}],fileName:[{kind:"reference",value:0n},{kind:"real",value:{bits:0x7ff0000000000001n}}],fileSchema:[{kind:"aggregate",value:[{kind:"string",value:"IFC4"},{kind:"string",value:"IFC4"}]}]},entities:[{id:0n,name:"IFCWALL",args:[{kind:"unset"},{kind:"derived"},...f.integerWords.map(word=>({kind:"integer" as const,value:BigInt(word)})),...f.binary64Words.map(word=>({kind:"real" as const,value:{bits:BigInt("0x"+word)}})),{kind:"string",value:f.text},{kind:"enum",value:"UNKNOWN"},{kind:"reference",value:BigInt(f.instanceWords[1]!)},{kind:"aggregate",value:[]},{kind:"typedValue",value:{name:"IFCLENGTHMEASURE",items:[{kind:"integer",value:1n},{kind:"aggregate",value:[{kind:"derived"}]}]}}],complex:[{name:"IFCROOT",args:[{kind:"reference",value:0n}]}]},{id:BigInt(f.instanceWords[1]!),name:"TARGET",args:[{kind:"reference",value:0n}],complex:[]}]};}
test("IFC4 independent complete typed header/value graph and scalar edit",async()=>{expect(IFC_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../🗄️.sql",import.meta.url)).text());const input=fixture(),db=Database.deserialize(await exportSqliteDatabase(await ifcSnapshotToSqliteDatabase(input)));try{expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);expect((db.query("SELECT kind FROM ifc_value GROUP BY kind ORDER BY kind").all()as{kind:string}[]).map(row=>row.kind)).toEqual([...f.valueKinds].sort());expect(db.query("SELECT COUNT(*) n FROM ifc_file_name_argument").get()).toEqual({n:2});expect(await ifcSnapshotFromSqliteDatabase(await importSqliteDatabase(db.serialize()))).toEqual(input);db.query("UPDATE ifc_entity SET name='SQL_EDIT' WHERE instance_id='18446744073709551615'").run();input.entities[1]!.name="SQL_EDIT";expect(await ifcSnapshotFromSqliteDatabase(await importSqliteDatabase(db.serialize()))).toEqual(input);}finally{db.close();}});
test("IFC4 owned artifact/diff/mutation consumers preserve exact words",async()=>{const artifact:IfcArtifact=fixture(),diff:IfcDiff={entities:{modified:[{id:artifact.entities[1]!.id,diff:{args:{added:[{index:0,value:{kind:"real",value:{bits:0xfff8000000000001n}}}]}}}]}},mutation:IfcMutation={mutation:"setEntityArg",id:artifact.entities[1]!.id,index:0,value:diff.entities!.modified![0]!.diff.args!.added![0]!.value};if(mutation.mutation!=="setEntityArg")throw Error("mutation");artifact.entities[1]!.args[mutation.index]=mutation.value;expect(await ifcSnapshotFromSqliteDatabase(await ifcSnapshotToSqliteDatabase(artifact))).toEqual(artifact);expect(diff.entities!.modified![0]!.id).toBe(18446744073709551615n);});
test("IFC4 independent corrupt graph and IEEE fields are rejected",async()=>{const bytes=await exportSqliteDatabase(await ifcSnapshotToSqliteDatabase(fixture()));for(const sql of["UPDATE ifc_argument SET value_id=999 WHERE id=1","UPDATE ifc_value SET reference_instance_id='00' WHERE kind='reference'","UPDATE ifc_entity SET instance_id='0' WHERE id=2","UPDATE ifc_file_name_argument SET ordinal=8 WHERE id=1","UPDATE ifc_value SET real_class='finite' WHERE kind='real'","UPDATE ifc_aggregate_element SET value_id=aggregate_value_id WHERE id=1"]){const db=Database.deserialize(bytes);try{db.run("PRAGMA ignore_check_constraints=ON");db.run(sql);await expect(ifcSnapshotFromSqliteDatabase(await importSqliteDatabase(db.serialize()))).rejects.toThrow();}finally{db.close();}}});
test("IFC4 bounds and cancellation precede large scalar and deep forest traversal",async()=>{const input=fixture(),database=await ifcSnapshotToSqliteDatabase(input);await expect(ifcSnapshotToSqliteDatabase(input,{maxValueBytes:1})).rejects.toThrow();await expect(ifcSnapshotFromSqliteDatabase(database,{maxValueBytes:1})).rejects.toThrow();const cycle:IfcValue={kind:"aggregate",value:[]};cycle.value.push(cycle);input.entities[0]!.args=[cycle];await expect(ifcSnapshotToSqliteDatabase(input)).rejects.toThrow(/cyclic/);input.entities[0]!.args=[{kind:"string",value:"文".repeat(40000)}];const cancel=new AbortController();let large=false;await expect(ifcSnapshotToSqliteDatabase(input,{signal:cancel.signal,onProgress:p=>{if(p.phase==="projectSnapshot"&&p.completed>10){large=true;cancel.abort();}}})).rejects.toThrow();expect(large).toBe(true);let value:IfcValue={kind:"unset"};for(let i=0;i<1200;i++)value={kind:"aggregate",value:[value]};input.entities[0]!.args=[value];const deep=await ifcSnapshotFromSqliteDatabase(await ifcSnapshotToSqliteDatabase(input));let cursor=deep.entities[0]!.args[0]!,depth=0;while(cursor.kind==="aggregate"){depth++;cursor=cursor.value[0]!;}expect(depth).toBe(1200);});


test("IFC4 native storage owns full value roots with distinct concrete backing",async()=>{
 const vector=await Bun.file(new URL("../🧫️fixtures/💰️backing/🔣️.json",import.meta.url)).json();
 const {default:Ajv2020}=await import("ajv/dist/2020");expect(vector["nativeConstruction"]["header"]).toEqual("threeOrderedValueRoots");expect(vector["nativeConstruction"]["values"]).toEqual("flatExclusiveForest");expect(vector["nativeConstruction"]["typedValue"]).toEqual("orderedItems");expect(vector["nativeConstruction"]["root"]).toEqual("inlineNoBacking");expect(vector["nativeConstruction"]["forecast"]).toEqual("paidFrontiersOnly");expect(vector["limits"]["semantic"]).toEqual("maxValueBytes");expect(vector["limits"]["backing"]).toEqual("maxAllocationBytes");expect(vector["limits"]["file"]).toEqual("maxFileBytes");expect(vector["nativeRoles"]["owned"]).toEqual("maxAllocationBytes");expect(vector["nativeRoles"]["semantic"]).toEqual("maxValueBytes");expect(vector["nativeRoles"]["preflight"]).toEqual("paidFrontiersOnly");expect(vector["nativeRoles"]["nonfinite"]).toEqual("retainedBinary64");
 expect(vector.nativeConstruction).toEqual({header:"threeOrderedValueRoots",values:"flatExclusiveForest",typedValue:"orderedItems",root:"inlineNoBacking",forecast:"paidFrontiersOnly"});
 expect(vector.limits).toEqual({semantic:"maxValueBytes",backing:"maxAllocationBytes",file:"maxFileBytes"});
 const db=new Database(":memory:");try{db.exec("CREATE TABLE storage_law(kind TEXT PRIMARY KEY,slots INTEGER NOT NULL CHECK(slots>=0));INSERT INTO storage_law VALUES('inlineRoot',0),('typedItems',2);");expect(db.query("SELECT kind,slots FROM storage_law ORDER BY slots").all()).toEqual([{kind:"inlineRoot",slots:0},{kind:"typedItems",slots:2}]);}finally{db.close();}
});


test("IFC4 native admission roles distinguish full backing from semantic bytes",async()=>{
 const vector=await Bun.file(new URL("../🧫️fixtures/💰️backing/🔣️.json",import.meta.url)).json();expect(vector.nativeRoles).toEqual({owned:"maxAllocationBytes",semantic:"maxValueBytes",preflight:"paidFrontiersOnly",nonfinite:"retainedBinary64"});
 const db=new Database(":memory:");try{db.exec("CREATE TABLE admission(stage TEXT PRIMARY KEY,kind TEXT NOT NULL);INSERT INTO admission VALUES('nativeFields','maxAllocationBytes'),('semanticCells','maxValueBytes');");expect(db.query("SELECT stage,kind FROM admission ORDER BY stage").all()).toEqual([{stage:"nativeFields",kind:"maxAllocationBytes"},{stage:"semanticCells",kind:"maxValueBytes"}]);}finally{db.close();}
});

test("IFC4 removal-result references preserve full owner through independently edited files",async()=>{
 const corpus=await Bun.file(new URL("../🧫️fixtures/🧩️intermediate/🔣️.json",import.meta.url)).json();
 
 const {default:Ajv}=await import("ajv");
 for(const case_ of corpus.cases){
  const input=fixture();input.schema=corpus.schema;input.entities=input.entities.filter(entity=>entity.id!==BigInt(case_.removedId));
  const physical=Database.deserialize(await exportSqliteDatabase(await ifcSnapshotToSqliteDatabase(input)));
  try{
   expect(physical.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(physical.query("PRAGMA foreign_key_check").all()).toEqual([]);
   expect(physical.query("SELECT instance_id FROM ifc_entity ORDER BY ordinal").all()).toEqual(case_.remainingIds.map((instance_id:string)=>({instance_id})));
   expect(physical.query("SELECT v.reference_instance_id AS literal,e.instance_id AS resolved FROM ifc_value AS v LEFT JOIN ifc_entity AS e ON e.id=v.reference_entity_id WHERE v.kind='reference' ORDER BY v.id").all()).toEqual(case_.references);
   expect(await ifcSnapshotFromSqliteDatabase(await importSqliteDatabase(physical.serialize()))).toEqual(input);
   physical.query("UPDATE ifc_value SET reference_instance_id=? WHERE kind='reference' AND reference_entity_id IS NULL").run(corpus.editReference);
   function edit(value:IfcValue){if(value.kind==="reference"&&value.value===BigInt(case_.removedId))value.value=BigInt(corpus.editReference);else if(value.kind==="aggregate")value.value.forEach(edit);else if(value.kind==="typedValue")value.value.items.forEach(edit);}
   [...input.header.fileDescription,...input.header.fileName,...input.header.fileSchema,...input.entities.flatMap(entity=>[...entity.args,...entity.complex.flatMap(part=>part.args)])].forEach(edit);
   expect(await ifcSnapshotFromSqliteDatabase(await importSqliteDatabase(physical.serialize()))).toEqual(input);
   console.log("[DEBUG] IFC4 full intermediate reference owner",case_.removedId);
  }finally{physical.close();}
 }
});
test("IFC4 complete positive SQL surrogates preserve literal native identities",async()=>{
 const corpus=await Bun.file(new URL("../🧫️fixtures/🧩️intermediate/🔣️.json",import.meta.url)).json(),input=fixture(),physical=Database.deserialize(await exportSqliteDatabase(await ifcSnapshotToSqliteDatabase(input)));
 try{
  physical.exec("PRAGMA foreign_keys=OFF");
  for(const{name}of physical.query("SELECT name FROM sqlite_schema WHERE type='table'").all()as{name:string}[]){
   const key=(physical.query("PRAGMA table_info("+name+")").all()as{name:string;pk:number}[]).find(column=>column.pk===1)!.name;
   const links=physical.query("PRAGMA foreign_key_list("+name+")").all()as{from:string}[];
   physical.exec("UPDATE "+name+" SET "+[key+"="+key+"+"+corpus.surrogateOffset,...links.filter(link=>link.from!==key).map(link=>link.from+"="+link.from+"+"+corpus.surrogateOffset)].join(","));
  }
  expect(physical.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(physical.query("PRAGMA foreign_key_check").all()).toEqual([]);
  expect(await ifcSnapshotFromSqliteDatabase(await importSqliteDatabase(physical.serialize()))).toEqual(input);console.log("[DEBUG] IFC4 full shifted-surrogate owner",corpus.surrogateOffset);
 }finally{physical.close();}
});

test("IFC4 resolved reference witnesses agree exactly with literal native words",async()=>{
 const bytes=await exportSqliteDatabase(await ifcSnapshotToSqliteDatabase(fixture()));
 for(const sql of["UPDATE ifc_value SET reference_entity_id=NULL WHERE kind='reference'","UPDATE ifc_value SET reference_entity_id=(SELECT id FROM ifc_entity WHERE instance_id='18446744073709551615') WHERE kind='reference' AND reference_instance_id='0'"]){
  const physical=Database.deserialize(bytes);try{physical.exec("PRAGMA ignore_check_constraints=ON");physical.exec(sql);await expect(ifcSnapshotFromSqliteDatabase(await importSqliteDatabase(physical.serialize()))).rejects.toThrow();}finally{physical.close();}
 }
});

import NormSemanticAjv from "ajv/dist/2020.js";
import normSemanticContract from "../🧫️fixtures/🎛️semantic.json";

import {ifcSnapshotToSqliteDatabase as normSemanticProject,ifcSnapshotFromSqliteDatabase as normSemanticRestore} from "../🟦️.ts";
import {exportSqliteDatabase as normSemanticExport,importSqliteDatabase as normSemanticImport} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import {independentSqliteExtent as normIndependentExtent} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️tests/🔮️semantic-extent/🟦️.ts";
test("IFC4 complete independent cells preserve copied limits and required empty metadata",async()=>{
 expect(normSemanticContract["schemaBytes"]).toEqual(3250);expect(normSemanticContract["tableWidths"]).toEqual({"ifc_aggregate_element":4,"ifc_argument":5,"ifc_complex_type":4,"ifc_document":3,"ifc_entity":5,"ifc_file_description_argument":4,"ifc_file_name_argument":4,"ifc_file_schema_argument":4,"ifc_header":1,"ifc_typed_argument":4,"ifc_value":11});expect(normSemanticContract["cases"]).toEqual([{"id":"originalSourceFull","rows":65,"valueBytes":1869},{"id":"emptyEntitiesRetainHeader","rows":18,"valueBytes":456}]);
 for(const item of normSemanticContract.cases){const source=fixture();if(item.id==="emptyEntitiesRetainHeader"){source.entities=[];}const database=await normSemanticProject(source),bytes=await normSemanticExport(database);expect(normIndependentExtent(bytes)).toEqual({rows:item.rows,valueBytes:item.valueBytes,schemaBytes:normSemanticContract.schemaBytes,tableWidths:normSemanticContract.tableWidths});const limits={maxRows:item.rows,maxValueBytes:item.valueBytes,maxSchemaBytes:normSemanticContract.schemaBytes,maxTables:11,maxColumns:11};expect(await normSemanticProject(source,limits)).toEqual(database);expect(await normSemanticRestore(await normSemanticImport(bytes),limits)).toEqual(source);
  for(const short of[{...limits,maxRows:item.rows-1},{...limits,maxValueBytes:item.valueBytes-1},{...limits,maxSchemaBytes:limits.maxSchemaBytes-1},{...limits,maxTables:10},{...limits,maxColumns:10}]){await expect(normSemanticProject(source,short)).rejects.toThrow();await expect(normSemanticRestore(database,short)).rejects.toThrow();}
 }console.log("[DEBUG] IFC4 complete independent SQLite cells and metadata preserve full and empty semantic limits");
});
