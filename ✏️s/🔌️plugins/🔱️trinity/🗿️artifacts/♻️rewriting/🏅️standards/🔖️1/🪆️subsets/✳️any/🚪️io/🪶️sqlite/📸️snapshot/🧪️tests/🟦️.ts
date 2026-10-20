import {parse as parseGraphql,print as printGraphql,buildASTSchema,coerceInputValue,Kind} from "graphql";
import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import {semioSchemaAjvV1} from "@semio-tech/framework/schema/oracles/validator";
import * as owner from "../../../../../../../../🟦️.ts";
import {exportSqliteDatabase,importSqliteDatabase,type SqliteDatabase} from "@semio-tech/framework/io/sqlite-snapshot";
import {readdirSync} from "node:fs";
test("Rewriting actual document retirement vectors own typed parent and every mutation",async()=>{const api=await import("../../../../🧬️schema/♻️retirement/🧪️tests/🔬️document-retirement/🟦️.ts");expect(()=>api.testRewritingDocumentRetirementOracle()).not.toThrow();});

import {parseRewritingDiff} from "../../../../🧬️schema/🔺️diff/🟦️.ts";
import { frameworkSourcePath, frameworkSourceUrl } from "@semio-tech/framework/test/source-text";


const fixture=JSON.parse(readFileSync(new URL("./🧫️fixtures/🔣️.json",import.meta.url),"utf8"));
const sql=readFileSync(new URL("../🗄️.sql",import.meta.url),"utf8");
function value(input:any):unknown{switch(input.variant){case"null":return{kind:"null"};case"bool":return{kind:"bool",value:input.value};case"number":return{kind:"number",value:{bits:BigInt("0x"+input.bits)}};case"string":return{kind:"string",value:input.value};case"array":return{kind:"array",values:input.elements.map(value)};case"object":return{kind:"object",values:Object.fromEntries(input.members.map((row:any)=>[row.key,value(row.value)]))};default:throw Error("fixture variant");}}
export function specimen(word=fixture.binary64Bits[0]):unknown{const typed=JSON.parse(JSON.stringify({workingGraph:fixture.nativeCase.workingGraph,lhs:fixture.nativeCase.lhs,rhs:fixture.nativeCase.rhs}),(key,value)=>key==="bits"?BigInt("0x"+value):value);for(const parameter of typed.rhs.parameters)if(parameter.default.kind==="number")parameter.default.value.bits=BigInt("0x"+word);return{...typed,parameterBindings:Object.fromEntries(fixture.nativeCase.parameterBindings.map((row:any)=>[row.key,value(row.value)])),ruleLayout:Object.fromEntries(fixture.nativeCase.ruleLayout.map((row:any)=>[row.key,{x:{bits:BigInt("0x"+word)},y:{bits:BigInt("0x"+word)}}]))};}
test("Rewriting actual typed map delta delegates full scalar words to owning parsers",()=>{const number={kind:"number",value:{bits:0x7ff0000000000001n}};const point={x:{bits:0x8000000000000000n},y:{bits:0x7ff0000000000000n}};const input={parameterBindings:{entries:{"":{precondition:"any",operation:{kind:"set",value:number}}}},ruleLayout:{entries:{"":{precondition:"any",operation:{kind:"set",value:point}}}}};expect(parseRewritingDiff(input).parameterBindings).toEqual(input.parameterBindings as any);expect(parseRewritingDiff(input).ruleLayout).toEqual(input.ruleLayout as any);});
test("Rewriting canonical parent retains literal authored strings, six variants and exact words",()=>{for(const word of fixture.binary64Bits){const input=specimen(word);expect(owner.parseRewritingArtifact(input) as unknown).toEqual(input);}});
test("Rewriting shared handcrafted schema is independently valid SQLite",()=>{const db=new Database(":memory:");try{db.exec(sql);expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);for(const [table,width]of Object.entries(fixture.tableWidths))expect(db.query("PRAGMA table_info("+table+")").all().length).toBe(Number(width));}finally{db.close();}});
test("Rewriting actual declared JSON schema admits exact owned scalar words",()=>{const ajv=semioSchemaAjvV1({allErrors:true});for(const url of JSON.parse(readFileSync(new URL("./🧫️fixtures/🌳️typed/🔣️.json",import.meta.url),"utf8")).schemaDocuments)ajv.addSchema(JSON.parse(readFileSync(new URL(url,import.meta.url),"utf8")));const schema=JSON.parse(readFileSync(new URL("../../../../🧬️schema/🔣️.json",import.meta.url),"utf8"));const validate=ajv.compile(schema);const wire=JSON.parse(JSON.stringify(specimen(),(_,v)=>typeof v==="bigint"?v.toString(16).padStart(16,"0"):v));expect(validate(wire)).toBe(true);});
test("Rewriting published semantic facade retains complete parent through independent SQLite",async()=>{const api=owner as unknown as {rewritingSnapshotToSqliteDatabase:(v:unknown)=>Promise<unknown>;rewritingSnapshotFromSqliteDatabase:(v:unknown)=>Promise<unknown>};expect(typeof api.rewritingSnapshotToSqliteDatabase).toBe("function");const input=specimen();expect(await api.rewritingSnapshotFromSqliteDatabase(await api.rewritingSnapshotToSqliteDatabase(input))).toEqual(input);});

const project=(input:unknown,options:Record<string,unknown>={})=>owner.rewritingSnapshotToSqliteDatabase(input as owner.RewritingArtifact,options);
const restore=(database:SqliteDatabase,options:Record<string,unknown>={})=>owner.rewritingSnapshotFromSqliteDatabase(database,options);
async function oracle(input:unknown):Promise<Database>{return Database.deserialize(Buffer.from(await exportSqliteDatabase(await project(input))));}
async function edited(db:Database):Promise<SqliteDatabase>{return importSqliteDatabase(db.serialize());}
for(const word of fixture.binary64Bits)test("Rewriting independently reserialized complete IEEE word "+word,async()=>{const input=specimen(word),db=await oracle(input);try{expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);const row=(db.query("SELECT x_ieee754_bits,x_numeric_class,x FROM rewriting_layout LIMIT 1") as unknown as {safeIntegers(on:boolean):{get():{x_ieee754_bits:bigint;x_numeric_class:string;x:number|null}}}).safeIntegers(true).get();expect(BigInt.asUintN(64,row.x_ieee754_bits)).toBe(BigInt("0x"+word));const view=new DataView(new ArrayBuffer(8));view.setBigUint64(0,BigInt("0x"+word));const number=view.getFloat64(0);if(Number.isNaN(number))expect(row.x).toBeNull();else if(Object.is(number,-0))expect(row.x_numeric_class).toBe("finite");db.exec("UPDATE jack_document SET name='independently edited literal'");const expected=owner.parseRewritingArtifact(input);expected.workingGraph.name="independently edited literal";expect(await restore(await edited(db))).toEqual(expected);}finally{db.close();}});
test("Rewriting complete independent SQL entity census",async()=>{const db=await oracle(specimen());try{for(const [table,count]of Object.entries(fixture.tableRowCounts))expect((db.query("SELECT COUNT(*) AS count FROM "+table).get() as {count:number}).count).toBe(Number(count));}finally{db.close();}});
const malformed=[
"INSERT INTO rewriting_document VALUES(999,1)",
"UPDATE rewriting_layout SET document_id=999 WHERE id=1",
"UPDATE rewriting_layout SET map_key=(SELECT map_key FROM rewriting_layout WHERE id=1) WHERE id=2",
"UPDATE rewriting_binding SET map_key=(SELECT map_key FROM rewriting_binding WHERE id=1) WHERE id=2",
"UPDATE rewriting_value SET variant='unknown' WHERE id=1",
"UPDATE rewriting_boolean SET value=2 WHERE id=1",
"DELETE FROM rewriting_number WHERE id=1",
"UPDATE rewriting_array_element SET ordinal=99 WHERE id=1",
"UPDATE rewriting_array_element SET value_id=array_id WHERE id=1",
"UPDATE rewriting_binding SET value_id=(SELECT value_id FROM rewriting_binding WHERE id=1) WHERE id=2",
"UPDATE rewriting_object_member SET ordinal=99 WHERE id=1",
"UPDATE rewriting_object_member SET map_key=(SELECT map_key FROM rewriting_object_member WHERE id=1) WHERE id=2",
"INSERT INTO rewriting_value VALUES(999,'null')",
"UPDATE rewriting_layout SET x_numeric_class='nan',x=42",
"UPDATE rewriting_number SET value_numeric_class='finite',value=42"
];
for(const mutation of malformed)test("Rewriting refuses independent malformed owner SQL: "+mutation,async()=>{const db=await oracle(specimen());try{db.exec(mutation);expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});await expect(restore(await edited(db))).rejects.toThrow();}finally{db.close();}});
test("Rewriting admits independently renumbered structural surrogate keys",async()=>{const input=specimen(),db=await oracle(input);try{db.exec("PRAGMA foreign_keys=OFF");for(const table of Object.keys(fixture.tableRowCounts)){db.exec("UPDATE "+table+" SET id=id+1000");for(const row of db.query("PRAGMA foreign_key_list("+table+")").all() as {from:string}[])if(row.from!=="id")db.exec("UPDATE "+table+" SET "+row.from+"="+row.from+"+1000");}expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);const database=await edited(db);expect(await restore(database)).toEqual(owner.parseRewritingArtifact(input));expect(await owner.validateRewritingSnapshotSqliteDialect(owner.parseRewritingArtifact(input),{artifactKind:"s.trinity.rewriting",standard:"1",subset:"*"},database)).toEqual([]);}finally{db.close();}});
test("Rewriting exact row/schema frontier precedes semantic ownership",async()=>{const input=specimen(),database=await project(input,{maxRows:89});expect(database.tables.reduce((n,t)=>n+t.rows.length,0)).toBe(89);await expect(project(input,{maxRows:88})).rejects.toThrow();await expect(restore(database,{maxRows:88})).rejects.toThrow();const bytes=Math.max(Buffer.byteLength(sql),database.tables.reduce((n,t)=>n+Buffer.byteLength(t.sql)+Buffer.byteLength(t.name),0));await expect(project(input,{maxSchemaBytes:bytes-1})).rejects.toThrow();expect(await restore(database,{maxSchemaBytes:bytes})).toEqual(owner.parseRewritingArtifact(input));});
test("Rewriting iterative retained property depth and interior cancellation",async()=>{let value:owner.PropertyValue={kind:"null"};for(let i=0;i<1024;i++)value={kind:"array",values:[value]};const input=owner.parseRewritingArtifact(specimen());input.parameterBindings={deep:value};const database=await project(input),output=await restore(database);let depth=0,node=output.parameterBindings.deep!;while(node.kind==="array"){depth++;node=node.values[0]!;}expect(depth).toBe(1024);for(const phase of["projectSnapshot","reconstructSnapshot"]){let observed=0;const controller=new AbortController();const options={signal:controller.signal,onProgress:(p:{phase:string;completed:number})=>{if(p.phase===phase&&p.completed>=256){observed=p.completed;controller.abort();}return true;}};await expect(phase==="projectSnapshot"?project(input,options):restore(database,options)).rejects.toThrow();expect(observed).toBeGreaterThanOrEqual(256);}});
test("Rewriting declared JSON word boundary is the actual published parent",()=>{const api=owner as unknown as {parseRewritingJsonValue:(v:unknown)=>unknown;rewritingToJsonValue:(v:owner.RewritingArtifact)=>unknown};expect(typeof api.parseRewritingJsonValue).toBe("function");for(const word of fixture.binary64Bits){const input=owner.parseRewritingArtifact(specimen(word)),wire=api.rewritingToJsonValue(input);expect(api.parseRewritingJsonValue(JSON.parse(JSON.stringify(wire)))).toEqual(input);}});
test("Rewriting committed snapshots use the closed canonical declared JSON scalar schema",()=>{const root=new URL("../../../../🧪️tests/🧫️fixtures/🧬️mutations/",import.meta.url),ajv=semioSchemaAjvV1({allErrors:true});for(const url of JSON.parse(readFileSync(new URL("./🧫️fixtures/🌳️typed/🔣️.json",import.meta.url),"utf8")).schemaDocuments)ajv.addSchema(JSON.parse(readFileSync(new URL(url,import.meta.url),"utf8")));const validate=ajv.compile(JSON.parse(readFileSync(new URL("../../../../🧬️schema/🔣️.json",import.meta.url),"utf8")));const files=readdirSync(root,{recursive:true}).map(String).filter(p=>p.endsWith("/📸️snapshot/⬅️before/🔣️.json")||p.endsWith("/📸️snapshot/➡️after/🔣️.json"));expect(files.length).toBe(18);expect(files).toContain('🫳️drag-rule-nodes/🫳️moves/📸️snapshot/⬅️before/🔣️.json');expect(files).toContain('🫳️drag-rule-nodes/🫳️moves/📸️snapshot/➡️after/🔣️.json');for(const path of files)expect(validate(JSON.parse(readFileSync(new URL(path,root),"utf8")))).toBe(true);});

test("Rewriting typed domain contract closes rule and composed graph semantics",async()=>{
 const vector=await Bun.file(new URL("./🧫️fixtures/🌳️typed/🔣️.json",import.meta.url)).json();const schema=await Bun.file(new URL("../../../../🧬️schema/📸️snapshot/🌳️typed/🔣️.json",import.meta.url)).json();const ajv=semioSchemaAjvV1({allErrors:true});
 for(const url of vector.schemaDocuments)ajv.addSchema(JSON.parse(readFileSync(new URL(url,import.meta.url),"utf8")));const validate=ajv.compile(schema);
 expect(validate(vector.snapshot)).toBe(true);expect(validate({...vector.snapshot,lhsJson:"{}"})).toBe(false);expect(validate({...vector.snapshot,lhs:"{}"})).toBe(false);expect(validate({...vector.snapshot,lhs:{pattern:{leftVar:"a",leftKind:"Piece",unknown:0}}})).toBe(false);expect(validate({...vector.snapshot,rhs:{...vector.snapshot.rhs,parameters:[{name:"caption",kind:"json",default:{kind:"null"}}]}})).toBe(false);
 const db=new Database(":memory:");try{db.exec(await Bun.file(new URL("../🌳️typed/🗄️.sql",import.meta.url)).text());db.run("INSERT INTO rewriting_pattern VALUES(1,'lhs',0,?,?,?,?,?,?)",[...["leftVar","leftKind","edgeVar","edgeKind","rightVar","rightKind"].map(key=>vector.snapshot.lhs.pattern[key]??null)]);db.run("INSERT INTO rewriting_lhs VALUES(1,1,?)",[vector.snapshot.lhs.whereClause??null]);for(const [ordinal,assignment]of vector.snapshot.rhs.set.entries())db.run("INSERT INTO rewriting_assignment VALUES(?,1,?,?,?)",[ordinal+1,ordinal,assignment.var,assignment.prop]);for(const [ordinal,parameter]of vector.snapshot.rhs.parameters.entries())db.run("INSERT INTO rewriting_parameter VALUES(?,1,?,?,?)",[ordinal+1,ordinal,parameter.name,parameter.kind]);expect(db.query("SELECT p.left_var,p.edge_var,p.right_var,l.where_clause FROM rewriting_lhs l JOIN rewriting_pattern p ON p.id=l.pattern_id").get()).toEqual(vector.oracle.pattern);expect(db.query("SELECT ordinal,var,prop FROM rewriting_assignment ORDER BY ordinal").all()).toEqual(vector.oracle.assignments);expect(db.query("SELECT ordinal,name,kind FROM rewriting_parameter ORDER BY ordinal").all()).toEqual(vector.oracle.parameters);expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(vector.snapshot.workingGraph.content.target.dialect).toEqual({artifactKind:"s.stdio.semio",standard:"v1",subset:"graph"});expect(vector.snapshot.workingGraph.query).toBe(vector.oracle.query);expect(vector.snapshot.parameterBindings.literal.value).toBe(vector.oracle.literal);}finally{db.close()}
});

test("Rewriting published JSON boundary owns normalized rule and working graph fields",async()=>{
 const vector=await Bun.file(new URL("./🧫️fixtures/🌳️typed/🔣️.json",import.meta.url)).json();const typed=JSON.parse(JSON.stringify(vector.snapshot),(key,value)=>key==="bits"?BigInt("0x"+value):value);expect(owner.parseRewritingJsonValue(vector.snapshot)).toEqual(typed);
});


test("Rewriting published normalized SQL owns complete typed parent and inline Jack manifest",async()=>{
 const vector=await Bun.file(new URL("./🧫️fixtures/🌳️typed/🔣️.json",import.meta.url)).json(),typed=owner.parseRewritingJsonValue(vector.snapshot),database=await project(typed);
 expect(database.tables.length).toBe(30);
 const db=Database.deserialize(await exportSqliteDatabase(database));
 try{
  expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
  expect(db.query("SELECT p.left_var,p.edge_var,p.right_var,l.where_clause FROM rewriting_lhs l JOIN rewriting_pattern p ON p.id=l.pattern_id").get()).toEqual(vector.oracle.pattern);
  expect(db.query("SELECT ordinal,var,prop FROM rewriting_assignment ORDER BY ordinal").all()).toEqual(vector.oracle.assignments);
  expect(db.query("SELECT ordinal,name,kind FROM rewriting_parameter ORDER BY ordinal").all()).toEqual(vector.oracle.parameters);
  expect((db.query("SELECT c.child_id,c.artifact_kind,c.standard,c.subset FROM rewriting_document d JOIN jack_document j ON j.id=d.working_graph_id JOIN jack_content_child c ON c.document_id=j.id").get() as any)).toEqual({child_id:typed.workingGraph.content.childId,artifact_kind:"s.stdio.semio",standard:"v1",subset:"graph"});
  expect(await restore(await importSqliteDatabase(db.serialize()))).toEqual(typed);
 }finally{db.close();}
});

test("Rewriting inline manifest owns all seven declared value type families",async()=>{
 const vector=await Bun.file(new URL("./🧫️fixtures/🌳️typed/🔣️.json",import.meta.url)).json(),typed=owner.parseRewritingJsonValue(vector.snapshot);
 expect(typed.workingGraph.manifest.nodeKinds.length).toBe(1);
 const properties=typed.workingGraph.manifest.nodeKinds[0]!.properties;
 expect(properties.map(value=>value.valueType.kind)).toEqual(["boolean","integer","decimal","text","list","any","schema"]);
 expect(properties.map(value=>value.kind)).toEqual(["data","data","data","data","data","data","derived"]);
 const db=Database.deserialize(await exportSqliteDatabase(await project(typed)));
 try{
  expect(db.query("SELECT p.name,p.property_kind,v.variant,p.expression FROM jack_node_property p JOIN jack_value_type v ON v.id=p.value_type_id ORDER BY p.ordinal").all()).toEqual(vector.manifestOracle.properties);
  expect(db.query("SELECT s.schema FROM jack_node_property p JOIN jack_value_type_schema s ON s.value_type_id=p.value_type_id").get()).toEqual({schema:vector.manifestOracle.schema});
  expect(db.query("SELECT v.variant FROM jack_node_property p JOIN jack_value_type_list l ON l.value_type_id=p.value_type_id JOIN jack_value_type v ON v.id=l.child_type_id").get()).toEqual({variant:"text"});
  expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
  expect(await restore(await importSqliteDatabase(db.serialize()))).toEqual(typed);
 }finally{db.close();}
});

test("Rewriting committed mutation children are exact typed graphs whose edges join held nodes",()=>{
 const root=new URL("../../../../🧪️tests/🧫️fixtures/🧬️mutations/",import.meta.url),files=readdirSync(root,{recursive:true}).map(String).filter(p=>p.endsWith("/📸️snapshot/⬅️before/🪆️child/🔣️.json")||p.endsWith("/📸️snapshot/➡️after/🪆️child/🔣️.json"));
 expect(files.length).toBe(18);
 const ajv=semioSchemaAjvV1({allErrors:true}),validate=ajv.compile(JSON.parse(readFileSync(new URL("../../../../🧬️schema/📸️snapshot/🌳️typed/🪆️child/🔣️.json",import.meta.url),"utf8")));
 for(const path of files){const child=JSON.parse(readFileSync(new URL(path,root),"utf8"));expect(validate(child)).toBe(true);const ids=new Set(child.nodes.map((node:any)=>node.id.value));for(const edge of child.edges){expect(ids.has(edge.source.value)).toBe(true);expect(ids.has(edge.target.value)).toBe(true);}}
});
test("Rewriting rule layout closed ownership contract keeps literal sorted slots and words",async()=>{
 const input=JSON.parse(readFileSync(new URL("./🧫️fixtures/🗂️layout/🔣️.json",import.meta.url),"utf8"));
 
 expect(input["contractId"]).toEqual("rewriting-rule-layout-owned-slots");expect(input["authority"]).toEqual("literalSortedContiguousSlots");expect(input["budget"]).toEqual("cumulativeCallerAllocation");
 for(const example of input.cases){const source=owner.parseRewritingArtifact(specimen());source.ruleLayout=Object.fromEntries(example.members.map((row:any)=>[row.key,{x:{bits:BigInt("0x"+row.xBits)},y:{bits:BigInt("0x"+row.yBits)}}]));const db=await oracle(source);try{const rows=(db.query("SELECT map_key,x_ieee754_bits,y_ieee754_bits FROM rewriting_layout ORDER BY map_key COLLATE BINARY") as any).safeIntegers(true).all();expect(rows.map((row:any)=>[row.map_key,BigInt.asUintN(64,row.x_ieee754_bits).toString(16).padStart(16,"0"),BigInt.asUintN(64,row.y_ieee754_bits).toString(16).padStart(16,"0")])).toEqual(example.expected);expect((await restore(await edited(db))).ruleLayout).toEqual(source.ruleLayout);}finally{db.close();}}
});

test("Rewriting composed working slot declares the actual Semio child owner",()=>{
 const vector=JSON.parse(readFileSync(new URL("./🧫️fixtures/🌳️typed/🔣️.json",import.meta.url),"utf8"));
 const parent=vector.snapshot.workingGraph,child=vector.childSnapshot;
 const db=new Database(":memory:");
 try{
  db.exec("CREATE TABLE slot(name TEXT PRIMARY KEY,kind TEXT NOT NULL); CREATE TABLE retained_child(slot TEXT NOT NULL REFERENCES slot(name),child_id TEXT NOT NULL,artifact_id TEXT NOT NULL,kind TEXT NOT NULL,standard TEXT NOT NULL,subset TEXT NOT NULL,PRIMARY KEY(slot,child_id));");
  for(const source of ["../../../../🧬️schema/🔣️.json","../../../../🧬️schema/📸️snapshot/🔣️.json"]){
   const declared=JSON.parse(readFileSync(new URL(source,import.meta.url),"utf8")).properties.workingGraph;
   expect(declared["x-semio-child-kind"]).toBe(parent.content.target.dialect.artifactKind);
   db.run("INSERT OR REPLACE INTO slot VALUES('workingGraph',?)",[declared["x-semio-child-kind"]]);
   const address=parent.content;
   db.run("INSERT OR REPLACE INTO retained_child VALUES('workingGraph',?,?,?,?,?)",[address.childId,address.target.artifactId,address.target.dialect.artifactKind,address.target.dialect.standard,address.target.dialect.subset]);
   const row=db.query("SELECT c.child_id,c.artifact_id,c.kind,c.standard,c.subset FROM retained_child c JOIN slot s ON s.name=c.slot AND s.kind=c.kind").get() as any;
   expect(row).toEqual({child_id:address.childId,artifact_id:address.target.artifactId,kind:"s.stdio.semio",standard:"v1",subset:"graph"});
   expect(child.schema).toBe(row.kind+"."+row.subset);
   expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
  }
 }finally{db.close();}
});

test("Rewriting committed sparse deltas own typed fields and exact inverse assets",()=>{
 const root=new URL("../../../../🧪️tests/🧫️fixtures/🧬️mutations/",import.meta.url),ajv=semioSchemaAjvV1({allErrors:true});
 const vector=JSON.parse(readFileSync(new URL("./🧫️fixtures/🌳️typed/🔣️.json",import.meta.url),"utf8"));
 for(const url of vector.schemaDocuments)ajv.addSchema(JSON.parse(readFileSync(new URL(url,import.meta.url),"utf8")));
 ajv.addSchema(JSON.parse(readFileSync(new URL("../../../../🧬️schema/🔣️.json",import.meta.url),"utf8")));
 ajv.addSchema(JSON.parse(readFileSync(frameworkSourceUrl("🔨️modules/📡️replication/🎮️mutation/🗂️map/🧬️schema/🔣️.json"),"utf8")));
 const validate=ajv.compile(JSON.parse(readFileSync(new URL("../../../../🧬️schema/🔺️diff/🔣️.json",import.meta.url),"utf8")));
 const files=readdirSync(root,{recursive:true}).map(String).filter(path=>path.endsWith("/🔺️diff/🔣️.json"));
 expect(files.length).toBe(9);
 const db=new Database(":memory:");
 try{
  db.exec("CREATE TABLE delta(case_name TEXT PRIMARY KEY,role TEXT NOT NULL,payload TEXT NOT NULL CHECK(json_valid(payload)));");
  for(const path of files){
   const diff=JSON.parse(readFileSync(new URL(path,root),"utf8"));
   expect(validate(diff)).toBe(true);
   expect(Object.keys(diff).sort()).toEqual(["lhs","parameterBindings","rhs","ruleLayout","workingGraph"]);
   const after=JSON.parse(readFileSync(new URL(path.replace("/🔺️diff/🔣️.json","/📸️snapshot/➡️after/🔣️.json"),root),"utf8")),before=JSON.parse(readFileSync(new URL(path.replace("/🔺️diff/🔣️.json","/📸️snapshot/⬅️before/🔣️.json"),root),"utf8"));
   for(const role of ["lhs","rhs","workingGraph"]){if(diff[role]!==null){db.run("INSERT INTO delta VALUES(?,?,?)",[path,role,JSON.stringify(diff[role])]);expect({...before[role],...Object.fromEntries(Object.entries(diff[role]).filter(([,value])=>value!==null))}).toEqual(after[role]);}}
  }
  for(const row of db.query("SELECT case_name,role,payload FROM delta ORDER BY case_name COLLATE BINARY").all() as {case_name:string;role:string;payload:string}[]){
   const after=JSON.parse(readFileSync(new URL(row.case_name.replace("/🔺️diff/🔣️.json","/📸️snapshot/➡️after/🔣️.json"),root),"utf8")),before=JSON.parse(readFileSync(new URL(row.case_name.replace("/🔺️diff/🔣️.json","/📸️snapshot/⬅️before/🔣️.json"),root),"utf8"));expect({...before[row.role],...Object.fromEntries(Object.entries(JSON.parse(row.payload)).filter(([,value])=>value!==null))}).toEqual(after[row.role]);
  }
 }finally{db.close();}
});

test("Rewriting published mutation union accepts all nine typed committed inputs",()=>{
 const root=new URL("../../../../🧪️tests/🧫️fixtures/🧬️mutations/",import.meta.url),ajv=semioSchemaAjvV1({allErrors:true}),read=(relative:string)=>JSON.parse(readFileSync(new URL(relative,import.meta.url),"utf8"));
 for(const relative of read("./🧫️fixtures/🌳️typed/🔣️.json").schemaDocuments)ajv.addSchema(read(relative));
 ajv.addSchema(read("../../../../🧬️schema/🔣️.json"));
 const declared=read("../../../../🧬️schema/🧬️mutations/🔣️.json");
 const schemas=readdirSync(new URL("../../../../🧬️schema/🧬️mutations",import.meta.url),{recursive:true}).map(String).filter(path=>path.endsWith("/🧬️schema/🔣️.json"));
 expect(schemas.length).toBe(9);for(const path of schemas)ajv.addSchema(read("../../../../🧬️schema/🧬️mutations/"+path));
 const validate=ajv.compile(declared),files=readdirSync(root,{recursive:true}).map(String).filter(path=>path.endsWith("/🦠️mutation/🔣️.json"));
 expect(files.length).toBe(9);
 for(const path of files){const input=JSON.parse(readFileSync(new URL(path,root),"utf8"));expect(validate(input)).toBe(true);expect(validate({...input,legacyBody:"{}"})).toBe(false);expect(JSON.parse(JSON.stringify(input))).toEqual(input);}
});

test("Rewriting GraphQL body inputs own full typed rules and composed parent",()=>{
 const read=(relative:string)=>readFileSync(new URL(relative,import.meta.url),"utf8");
 const roots=["../../../../🧬️schema/🔗️.graphql","../../../../🧬️schema/🧬️mutations/🔗️.graphql","../../../../../../../../../🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔗️.graphql",frameworkSourcePath("🔨️modules/🕸️graph/🛂️manifest/🌱️value/🧬️schema/🔗️.graphql"),frameworkSourcePath("🔨️modules/🌱️value/🧬️schema/🔗️.graphql"),frameworkSourcePath("🔨️modules/🧬️schema/🗿️artifact-reference/🔗️.graphql"),frameworkSourcePath("🔨️modules/🧬️schema/🗿️artifact-reference/🪆️child/🧬️schema/🔗️.graphql"),frameworkSourcePath("🔨️modules/🚪️io/🧬️schema/🔗️.graphql")];
 const documents=roots.map(relative=>parseGraphql(read(relative)));
 const types=new Map(documents.flatMap(document=>document.definitions).filter((node:any)=>node.name).map((node:any)=>[node.name.value,node]));
 for(const [slug,type,field,payload]of [["👈️edit-lhs","EditLhsInput","newLhs","LhsInput"],["👉️edit-rhs","EditRhsInput","newRhs","RhsInput"],["🖼️edit-working-graph","EditWorkingGraphInput","newWorkingGraph","JackSnapshotInput"]]){
  const leaf:any=parseGraphql(read("../../../../🧬️schema/🧬️mutations/"+slug+"/🔗️.graphql")).definitions[0];
  expect(leaf.fields.map((item:any)=>item.name.value)).toEqual([field]);
  expect(printGraphql(leaf.fields[0].type)).toBe(payload+"!");
  const aggregate:any=types.get(type);expect(printGraphql(aggregate)).toBe(printGraphql(leaf));
 }
 const definitions=documents.flatMap(document=>document.definitions).filter(node=>node.kind==="InputObjectTypeDefinition"||node.kind==="EnumTypeDefinition"||node.kind==="ScalarTypeDefinition");
 const graph=buildASTSchema({kind:Kind.DOCUMENT,definitions});
 const vector=JSON.parse(read("./🧫️fixtures/🌳️typed/🔣️.json"));
 const property=(value:any):any=>{switch(value.kind){case"null":return{null:"NULL"};case"bool":return{boolean:value.value};case"number":return{number:value.value};case"string":return{string:value.value};case"array":return{array:value.values.map(property)};case"object":return{object:Object.entries(value.values).map(([key,value])=>({key,value:property(value)}))};default:throw Error("fixture property family");}};
 const valueType=(value:any):any=>value.kind==="list"?{list:valueType(value.of)}:value.kind==="schema"?{schema:value.of}:{scalar:value.kind.toUpperCase()};
 const kind=(value:any)=>({...value,properties:value.properties.map((item:any)=>({...item,kind:item.kind.toUpperCase(),valueType:valueType(item.valueType)}))});
 const parent={...vector.snapshot.workingGraph,manifest:{nodeKinds:vector.snapshot.workingGraph.manifest.nodeKinds.map(kind),edgeKinds:vector.snapshot.workingGraph.manifest.edgeKinds.map(kind),portKinds:vector.snapshot.workingGraph.manifest.portKinds.map(kind)}};
 const rhs={...vector.snapshot.rhs,set:vector.snapshot.rhs.set.map((item:any)=>({...item,value:property(item.value)})),parameters:vector.snapshot.rhs.parameters.map((item:any)=>({...item,kind:item.kind.toUpperCase(),default:property(item.default)}))};
 const accepted=(name:string,input:unknown)=>{const output=coerceInputValue(input,graph.getType(name) as any);expect(output).toBeDefined();expect(output).toEqual(input);};
 accepted("EditLhsInput",{newLhs:vector.snapshot.lhs});accepted("EditRhsInput",{newRhs:rhs});accepted("EditWorkingGraphInput",{newWorkingGraph:parent});
 for(const sample of[{kind:"null"},{kind:"bool",value:false},{kind:"number",value:{bits:"7ff8000000000011"}},{kind:"string",value:"quote\\\" NUL\u0000世界😀"},{kind:"array",values:[{kind:"null"}]},{kind:"object",values:{"":{kind:"bool",value:true}}}])accepted("PropertyValueInput",property(sample));
 for(const name of["PropertyValueInput","ValueTypeInput"])expect((graph.getType(name) as any).isOneOf).toBe(true);
 for(const [name,input]of[["EditLhsInput",{newLhs:"{}"}],["EditRhsInput",{newRhs:"{}"}],["EditWorkingGraphInput",{newWorkingGraph:"{}"}],["PropertyValueInput",{string:"literal",number:{bits:"0000000000000000"}}],["ValueTypeInput",{scalar:"TEXT",schema:"literal"}]] as const){expect(coerceInputValue(input,graph.getType(name) as any)).toBeUndefined();}

 const mutationRoot=new URL("../../../../🧪️tests/🧫️fixtures/🧬️mutations/",import.meta.url);
 const mutations=readdirSync(mutationRoot,{recursive:true}).map(String).filter(path=>path.endsWith("/🦠️mutation/🔣️.json"));expect(mutations.length).toBe(9);
 expect((graph.getType("RewriteRuleMutationInput") as any).isOneOf).toBe(true);expect(Object.keys((graph.getType("RewriteRuleMutationInput") as any).getFields()).length).toBe(9);
 for(const path of mutations){
  const raw=JSON.parse(readFileSync(new URL(path,mutationRoot),"utf8")),{mutation,...input}=raw;
  if(input.newValue)input.newValue=property(input.newValue);
  if(input.newRhs)input.newRhs={...input.newRhs,set:input.newRhs.set.map((item:any)=>({...item,value:property(item.value)})),parameters:input.newRhs.parameters.map((item:any)=>({...item,kind:item.kind.toUpperCase(),default:property(item.default)}))};
  if(input.newWorkingGraph)input.newWorkingGraph={...input.newWorkingGraph,manifest:{nodeKinds:input.newWorkingGraph.manifest.nodeKinds.map(kind),edgeKinds:input.newWorkingGraph.manifest.edgeKinds.map(kind),portKinds:input.newWorkingGraph.manifest.portKinds.map(kind)}};
  const type=mutation[0].toUpperCase()+mutation.slice(1)+"Input";
  const leaf:any=parseGraphql(read("../../../../🧬️schema/🧬️mutations/"+path.split("/")[0]+"/🔗️.graphql")).definitions[0];expect(printGraphql(types.get(type) as any)).toBe(printGraphql(leaf));
  accepted(type,input);accepted("RewriteRuleMutationInput",{[mutation]:input});
 }
 expect(coerceInputValue({editLhs:{newLhs:vector.snapshot.lhs},editRhs:{newRhs:rhs}},graph.getType("RewriteRuleMutationInput") as any)).toBeUndefined();

 expect((graph.getType("JackSnapshotInput") as any).getFields().camera.type.toString()).toBe("CameraInput!");
 expect(Object.keys((graph.getType("ManifestNodeKindInput") as any).getFields())).toEqual(["name","properties","portKinds"]);
 const artifact=parseGraphql(read("../../../../../../../../../🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔗️.graphql"));
 const artifactTypes=artifact.definitions.filter((node:any)=>node.name).map((node:any)=>node.name.value);
 expect(artifactTypes).toEqual(["JackArtifact"]);
 const artifactRoot:any=artifact.definitions[0],snapshotRoot:any=types.get("JackSnapshot");
 expect(artifactRoot.fields.map((field:any)=>[field.name.value,printGraphql(field.type)])).toEqual(snapshotRoot.fields.map((field:any)=>[field.name.value,printGraphql(field.type)]));
 expect(read("../../../../../../../../../🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔗️.graphql")).toContain('# import Manifest, Camera from');
});

test("Rewriting Proto field roles retain typed bodies and complete Jack manifests",()=>{
 const read=(relative:string)=>readFileSync(new URL(relative,import.meta.url),"utf8");
 for(const [slug,name,field,type]of [["👈️edit-lhs","EditLhs","new_lhs","semio.s.trinity.rewriting.artifact.Lhs"],["👉️edit-rhs","EditRhs","new_rhs","semio.s.trinity.rewriting.artifact.Rhs"],["🖼️edit-working-graph","EditWorkingGraph","new_working_graph","semio.s.trinity.jack.snapshot.JackSnapshot"]]){
  const source=read("../../../../🧬️schema/🧬️mutations/"+slug+"/🛰️.proto"),body=source.match(new RegExp("message\\s+"+name+"\\s*\\{([^}]+)\\}"))![1]!;
  expect(body.trim().replace(/\s+/g," ")).toBe(type+" "+field+" = 1;");
  expect(read("../../../../🧬️schema/🧬️mutations/🛰️.proto")).toContain(type+" "+field+" = 1;");
 }
 const jack=read("../../../../../../../../../🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🛰️.proto");
 const fields=(name:string)=>jack.match(new RegExp("message\\s+"+name+"\\s*\\{([^}]+)\\}"))![1]!.trim().replace(/\s+/g," ");
 expect(fields("Camera")).toBe("semio.framework.value.Binary64Word x = 1; semio.framework.value.Binary64Word y = 2; semio.framework.value.Binary64Word zoom = 3;");
 expect(fields("ManifestNodeKind")).toBe("string name = 1; repeated ManifestProperty properties = 2; repeated string port_kinds = 3;");
 expect(fields("ManifestEdgeKind")).toBe("string name = 1; repeated ManifestProperty properties = 2;");
 expect(fields("ManifestPortKind")).toBe("string name = 1; string direction = 2; repeated ManifestProperty properties = 3;");
 expect(fields("ManifestProperty")).toContain("semio.framework.value.ValueType value_type = 3;");
 const value=read(frameworkSourcePath("🔨️modules/🌱️value/🧬️schema/🛰️.proto"));
 expect(value).toContain("message Binary64Word { fixed64 bits = 1; }");
 const db=new Database(":memory:",{safeIntegers:true});
 try{db.exec("CREATE TABLE exact_word(bits INTEGER NOT NULL) STRICT");
  for(const hex of fixture.binary64Bits){const bits=BigInt("0x"+hex),bytes=new DataView(new ArrayBuffer(8));bytes.setBigUint64(0,bits,true);db.run("INSERT INTO exact_word VALUES(?)",[BigInt.asIntN(64,bytes.getBigUint64(0,true))]);const row=db.query("SELECT bits FROM exact_word ORDER BY rowid DESC LIMIT 1").get() as {bits:bigint};expect(BigInt.asUintN(64,row.bits)).toBe(bits);expect(bytes.getBigUint64(0,true).toString(16).padStart(16,"0")).toBe(hex);}
 }finally{db.close();}
});


test("Rewriting retained child capture contract preserves exact publication and release authority",async()=>{
 const base=new URL("../../../../🧪️tests/🧫️fixtures/📸️snapshot/🪆️child/👁️capture/",import.meta.url);
 const contract=await Bun.file(new URL("🔣️.json",base)).json();
 
 const vector=await Bun.file(new URL("./🧫️fixtures/🌳️typed/🔣️.json",import.meta.url)).json();
 const graph=structuredClone(vector.childSnapshot),before=structuredClone(graph),db=new Database(":memory:");
 try{
  db.exec("PRAGMA foreign_keys=ON;CREATE TABLE entry(id INTEGER PRIMARY KEY,slot TEXT NOT NULL,child_id TEXT NOT NULL,kind TEXT NOT NULL,standard TEXT NOT NULL,subset TEXT NOT NULL,owner_schema TEXT NOT NULL,label TEXT NOT NULL);CREATE TABLE live(slot TEXT PRIMARY KEY,entry_id INTEGER NOT NULL REFERENCES entry(id));CREATE TABLE capture(id INTEGER PRIMARY KEY,entry_id INTEGER NOT NULL REFERENCES entry(id));");
  for(const [index,owner]of contract.owners.entries())db.run("INSERT INTO entry VALUES(?,?,?,?,?,?,?,?)",[index+1,contract.slot,contract.childId,contract.dialect.artifactKind,contract.dialect.standard,contract.dialect.subset,vector.childSnapshot.schema,owner.label]);
  db.run("INSERT INTO live VALUES(?,1)",[contract.slot]);
  db.run("INSERT INTO capture SELECT 1,entry_id FROM live WHERE slot=?",[contract.slot]);
  expect(contract.ownerSchema).toBe(vector.childSnapshot.schema);db.run("UPDATE live SET entry_id=2 WHERE slot=?",[contract.slot]);
  const captured=db.query("SELECT e.label FROM entry e JOIN capture c ON c.entry_id=e.id").get() as {label:string};expect(captured.label).toBe(contract.owners[0].label);
  expect(graph).toEqual(before);expect(db.query("SELECT COUNT(*) n FROM capture WHERE entry_id=1").get()).toEqual({n:contract.blockedCaptureCount});
  for(const refusal of contract.refusals){const slot=refusal==="missingSlot"?"absent":contract.slot,id=refusal==="missingChild"?"absent":contract.childId,kind=refusal==="wrongDialect"?"wrong":contract.dialect.artifactKind,ownerSchema=refusal==="wrongType"?"wrong":contract.ownerSchema;const found=db.query("SELECT e.id FROM entry e JOIN live l ON l.entry_id=e.id WHERE l.slot=? AND e.child_id=? AND e.kind=? AND e.owner_schema=?").get(slot,id,kind,ownerSchema);expect(found).toBeNull();}
  expect(()=>db.run("DELETE FROM entry WHERE id=1")).toThrow();db.run("DELETE FROM capture WHERE id=1");expect(db.query("SELECT COUNT(*) n FROM capture WHERE entry_id=1").get()).toEqual({n:contract.releasedCaptureCount});
  db.run("DELETE FROM entry WHERE id=1");expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
  expect(db.query("SELECT label FROM entry").all()).toEqual([{label:contract.owners[1].label}]);expect(graph).toEqual(before);
 }finally{db.close()}
});
test("Rewriting exact twenty-eight scenario roster owns all eighteen closed typed inline payloads",async()=>{
 const {parseFeature}=await import("@semio-tech/framework/test/gherkin");
 const read=(relative:string)=>JSON.parse(readFileSync(new URL(relative,import.meta.url),"utf8"));
 const feature=parseFeature(readFileSync(new URL("../../../../🧪️tests/♻️mutate-rewrite-1/🥒️.feature",import.meta.url),"utf8"));
 const kinds=["edit-working-graph","edit-lhs","edit-rhs","change-parameter-binding","remove-parameter-binding","change-rule-layout-point","remove-rule-layout-point","drag-rule-nodes","set-rule-layout-points"];
 expect(feature.errors).toEqual([]);
 expect(feature.scenarios.map(row=>row.id)).toEqual(["mutate","inverse","spec-vector"].flatMap(group=>kinds.map(kind=>group+"-"+kind)).concat("identity-round-trip"));
 const ajv=semioSchemaAjvV1({allErrors:true});
 for(const relative of read("./🧫️fixtures/🌳️typed/🔣️.json").schemaDocuments)ajv.addSchema(read(relative));
 ajv.addSchema(read("../../../../🧬️schema/🔣️.json"));
 const schemas=readdirSync(new URL("../../../../🧬️schema/🧬️mutations",import.meta.url),{recursive:true}).map(String).filter(path=>path.endsWith("/🧬️schema/🔣️.json"));
 expect(schemas.length).toBe(9);for(const path of schemas)ajv.addSchema(read("../../../../🧬️schema/🧬️mutations/"+path));
 const validate=ajv.compile(read("../../../../🧬️schema/🧬️mutations/🔣️.json"));
 const inline=feature.scenarios.filter(row=>row.id.startsWith("mutate-")||row.id.startsWith("inverse-"));
 expect(inline.length).toBe(18);
 const invalid=inline.flatMap(row=>{const payloads=row.steps.filter(step=>step.docString!==undefined);expect(payloads.length).toBe(1);return validate(JSON.parse(payloads[0]!.docString!))?[]:[row.id]});
 expect(invalid).toEqual([]);
 const childSchema=ajv.compile(read("../../../../🧬️schema/📸️snapshot/🌳️typed/🪆️child/🔣️.json"));
 const declaredChild="shared://♻️mutate-rewrite-1/🪆️content/🔣️.snapshot.json";
 for(const row of inline){
  expect(row.steps.some(step=>step.text==="the real derived child "+declaredChild)).toBe(true);
  const child=read("../../../../🧪️tests/🧫️fixtures/♻️mutate-rewrite-1/🪆️content/🔣️.snapshot.json");expect(childSchema(child)).toBe(true);
  const payload=JSON.parse(row.steps.find(step=>step.docString!==undefined)!.docString!);
  if(payload.mutation==="editWorkingGraph"){
   const uri="shared://♻️mutate-rewrite-1/🔁️core-only/🪆️content/🔣️.snapshot.json";
   expect(row.steps.some(step=>step.text==="the replacement working child "+uri)).toBe(true);
   expect(payload.newWorkingGraph.content).toEqual({childId:"nakagin-core-only-content",target:{artifactId:"nakagin-core-only-content",dialect:{artifactKind:"s.stdio.semio",standard:"v1",subset:"graph"}}});
   expect(payload.newWorkingGraph.camera).toEqual({x:{bits:"0000000000000000"},y:{bits:"0000000000000000"},zoom:{bits:"3ff0000000000000"}});
   const replacement=read("../../../../🧪️tests/🧫️fixtures/♻️mutate-rewrite-1/🔁️core-only/🪆️content/🔣️.snapshot.json");expect(childSchema(replacement)).toBe(true);expect(replacement).toEqual({schema:"s.stdio.semio.graph",nodes:[],edges:[]});
  }
 }
});


test("Rewriting member membership identity remains separate from its exact addressed target",async()=>{
 const contract=JSON.parse(readFileSync(new URL("../../../../🧪️tests/🧫️fixtures/📸️snapshot/🪆️child/🧵️lifetime/🔣️.json",import.meta.url),"utf8"));
 expect(contract.childId).toBe("logical !@/\0引用😀");expect(contract.childId).not.toBe(contract.target.artifactId);
 const ajv=semioSchemaAjvV1({allErrors:true});ajv.addSchema(JSON.parse(readFileSync(new URL("../../../../🧬️schema/📸️snapshot/🌳️typed/🪆️child/🔣️.json",import.meta.url),"utf8")));
 expect(contract["contractId"]).toEqual("rewriting-full-child-frame-lifetime");expect(contract["slot"]).toEqual("workingGraph");expect(contract["target"]["dialect"]["artifactKind"]).toEqual("s.stdio.semio");expect(contract["target"]["dialect"]["standard"]).toEqual("v1");expect(contract["target"]["dialect"]["subset"]).toEqual("graph");expect(contract["retainedTailBytes"]).toEqual(65536);expect(contract["borrowedCaptureAllocationBytes"]).toEqual(0);expect(contract["closeGrants"]).toEqual([0,1]);expect(contract["refusals"]).toEqual(["wrongParent","missingSlot","missingTarget","wrongDialect"]);expect(contract["intrinsicKinds"]).toEqual(["bool","bytes","float","int","list","map","null","ref","str"]);expect(contract["childId"]).toEqual("logical !@/\u0000引用😀");
 const db=new Database(":memory:");try{db.exec("CREATE TABLE member (parent TEXT NOT NULL,slot TEXT NOT NULL,child_id TEXT NOT NULL,target TEXT NOT NULL UNIQUE,PRIMARY KEY(parent,slot,child_id));");db.run("INSERT INTO member VALUES (?,?,?,?)",["parent",contract.slot,contract.childId,contract.target.artifactId]);expect(db.query("SELECT target FROM member WHERE parent=? AND slot=? AND child_id=?").get("parent",contract.slot,contract.childId)).toEqual({target:contract.target.artifactId});expect(()=>db.run("INSERT INTO member VALUES (?,?,?,?)",["other","other","other",contract.target.artifactId])).toThrow();}finally{db.close();}
 const plugin=readFileSync(frameworkSourceUrl("🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🏪️stores/📥️staging/🚪️ingress/🦀️.rs"),"utf8"),declaration=plugin.indexOf("fn declared_child_reference(");
 const admission=plugin.slice(declaration,plugin.indexOf("\n    }\n",declaration));expect(admission).toContain("fields.artifact_id");expect(admission).toContain("fields.child_id == child_id");expect(admission).not.toContain("artifact_id: child_id.clone()");
 const store=readFileSync(frameworkSourceUrl("🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🦀️.rs"),"utf8"),syncStart=store.indexOf("pub async fn sync_member<P: ArtifactRefs>"),sync=store.slice(syncStart,store.indexOf("\n    }\n",syncStart));
 expect(sync).not.toContain("child.child_id != child.target.artifact_id");expect(sync).toContain("next_owns.insert(child.target.artifact_id");
});
