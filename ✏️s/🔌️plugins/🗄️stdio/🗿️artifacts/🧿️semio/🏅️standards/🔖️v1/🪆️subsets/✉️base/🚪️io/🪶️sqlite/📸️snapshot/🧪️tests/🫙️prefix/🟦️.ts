/** 🫙️ Neutral original prefix corpus and independent SQLite text identity. */
import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import Ajv from "ajv/dist/2020";
import corpus from "../../🧫️fixtures/🫙️prefix/🔣️.json";
import schema from "../../🧬️schema/🫙️prefix/🔣️.json";
import forest from "../../🧫️fixtures/🫙️prefix/🌳️forest.json";
import forestSchema from "../../🧬️schema/🫙️prefix/🌳️forest.json";
import projection from "../../🧫️fixtures/🫙️prefix/🧮️projection.json";
import projectionSchema from "../../🧬️schema/🫙️prefix/🧮️projection.json";
test("Graph properties retain actual original source indices and shared value pages",()=>{
 const root=new URL("../../../../../../🕸️graph/🚪️io/🪶️sqlite/📸️snapshot/",import.meta.url);expect(readFileSync(new URL("🦀️.rs",root),"utf8").includes("projection::project(self")).toBe(true);const source=readFileSync(new URL("🫙️projection/🦀️.rs",root),"utf8");expect({names:source.includes("NameIndex"),tree:source.includes("TreeScratch"),values:source.includes("visit_tree(&entry.value,VALUES,None,tree,out)"),original:source.includes("project_owned")}).toEqual({names:true,tree:true,values:true,original:true});
});
test("Table cells share the actual original Value traversal",()=>{
 const root=new URL("../../../../../../📊️table/🚪️io/🪶️sqlite/📸️snapshot/",import.meta.url);expect(readFileSync(new URL("🦀️.rs",root),"utf8").includes("projection::project(self")).toBe(true);const source=readFileSync(new URL("🫙️projection/🦀️.rs",root),"utf8");expect({sharedTree:source.includes("TreeScratch"),original:source.includes("project_owned"),cells:source.includes("visit_tree(value,VALUES,None,tree,out)")}).toEqual({sharedTree:true,original:true,cells:true});
});
test("Value projection retains original scalar tree pages across both real traversals",()=>{
 const root=new URL("../../../../../../🔢️value/🚪️io/🪶️sqlite/📸️snapshot/",import.meta.url);expect(readFileSync(new URL("🦀️.rs",root),"utf8")).toContain("projection::project(self");const tree=readFileSync(new URL("🫙️projection/🌳️tree/🦀️.rs",root),"utf8");expect(tree).toContain("struct TreeScratch");expect(tree).toContain("PagedList<Frame");expect(tree).toContain("reserve_one(remaining)");expect(tree).not.toContain("Vec<&");expect(tree).not.toContain(".pop(");
});
test("Flow projection retains original scalar ordinals for authored names",()=>{
 const root=new URL("../../../../../../🌊️flow/🚪️io/🪶️sqlite/📸️snapshot/",import.meta.url);const source=readFileSync(new URL("🦀️.rs",root),"utf8");expect(source.includes("projection::project(self")).toBe(true);const prefix=readFileSync(new URL("🫙️projection/🦀️.rs",root),"utf8");expect({scratch:prefix.includes("struct Scratch"),originalNames:prefix.includes("NameIndex"),borrowed:prefix.includes("Vec<&")||prefix.includes("Vec<(&")}).toEqual({scratch:true,originalNames:true,borrowed:false});const names=readFileSync(new URL("../../🫙️projection/🪪️names/🦀️.rs",import.meta.url),"utf8");expect(names.includes("Vec<usize>")).toBe(true);
});
test("original projection corpus retains real table identities and fields",()=>{
 expect(new Ajv({strict:true}).compile(projectionSchema)(projection)).toBe(true);const db=new Database(":memory:");try{
  db.exec(readFileSync(new URL("../../../../../../🔤️text/🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql",import.meta.url),"utf8"));db.run("INSERT INTO semio_text_document VALUES(1,?)",[projection.schema]);db.run("INSERT INTO semio_text_run VALUES(1,1,0,?,?)",[projection.language,projection.content]);db.run("INSERT INTO semio_text_mark VALUES(1,1,0,?,?)",[projection.kind,projection.href]);
  expect(db.query("SELECT language,content FROM semio_text_run").get()).toEqual({language:projection.language,content:projection.content});expect(db.query("SELECT kind,href FROM semio_text_mark").get()).toEqual({kind:projection.kind,href:projection.href});for(const[table,count]of Object.entries(projection.expectedRows))expect(db.query(`SELECT count(*) AS n FROM ${table}`).get()).toEqual({n:count});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
 }finally{db.close();}
 const source=readFileSync(new URL("../../🫙️projection/🦀️.rs",import.meta.url),"utf8");expect(source).toContain("project_owned");expect(source).toContain("RowWriter::census");expect(source).toContain("RowWriter::new_into");
 for(const id of projection.domains){const domain=corpus.domains.find(domain=>domain.id===id)!;const source=readFileSync(new URL("../../../../../../"+domain.path+"/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs",import.meta.url),"utf8");expect(source).toContain("projection::project_rows_owned");}
});
test("original value forest has independent ordered SQLite variants and exact fields",()=>{
 expect(new Ajv({strict:true}).compile(forestSchema)(forest)).toBe(true);
 const db=new Database(":memory:");try{
  db.exec(readFileSync(new URL("../../../../../../🔢️value/🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql",import.meta.url),"utf8"));
  const cells=[[],[],[1],[null,forest.integerLexeme],[null,null,forest.floatLexeme],[null,null,null,forest.text],[null,null,null,null,Uint8Array.from(forest.octets)],[],[null,null,null,null,null,1]];
  for(const[index,kind]of forest.kinds.entries()){const fields=Array<unknown>(6).fill(null);for(const[column,value]of cells[index]!.entries())fields[column]=value;db.run("INSERT INTO semio_value_value VALUES(?,?,?,?,?,?,?,?)",[index+1,kind,...fields] as (string|number|null|Uint8Array)[]);}
  db.run("INSERT INTO semio_value_value(id,kind,boolean_value) VALUES(10,'bool',0)");db.run("INSERT INTO semio_value_value(id,kind) VALUES(11,'null')");
  db.run("INSERT INTO semio_value_document VALUES(1,?,1)",[forest.schema]);db.run("INSERT INTO semio_value_node VALUES(1,1,0,?,10)",[forest.nodeId]);
  for(let ordinal=0;ordinal<8;ordinal++)db.run("INSERT INTO semio_value_map_entry VALUES(?,1,?,?,?)",[ordinal+1,ordinal,forest.key,ordinal+2]);
  db.run("INSERT INTO semio_value_list_element VALUES(1,8,0,11)");
  expect((db.query("SELECT kind FROM semio_value_value WHERE id<=9 ORDER BY id").all() as {kind:string}[]).map(row=>row.kind)).toEqual(forest.kinds);
  expect(db.query("SELECT integer_lexeme AS integerLexeme FROM semio_value_value WHERE id=4").get()).toEqual({integerLexeme:forest.integerLexeme});
  expect(db.query("SELECT float_lexeme AS floatLexeme FROM semio_value_value WHERE id=5").get()).toEqual({floatLexeme:forest.floatLexeme});
  expect(db.query("SELECT string_value AS text FROM semio_value_value WHERE id=6").get()).toEqual({text:forest.text});
  expect(Array.from((db.query("SELECT bytes_value AS octets FROM semio_value_value WHERE id=7").get() as {octets:Uint8Array}).octets)).toEqual(forest.octets);
  expect((db.query("SELECT member_key AS key FROM semio_value_map_entry ORDER BY ordinal").all() as {key:string}[]).map(row=>row.key)).toEqual(Array(8).fill(forest.key));
  expect(db.query("SELECT native_id AS nodeId FROM semio_value_node WHERE id=(SELECT target_node_id FROM semio_value_value WHERE id=9)").get()).toEqual({nodeId:forest.nodeId});
  expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
 }finally{db.close();}
});
test("native prefix corpus preserves authored text independently",()=>{
 const validate=new Ajv({strict:true}).compile(schema);expect(validate(corpus)).toBe(true);expect(new Set(corpus.domains.map(x=>x.id)).size).toBe(18);
 const db=new Database(":memory:");try{db.run("CREATE TABLE original_prefix(field TEXT PRIMARY KEY,value TEXT NOT NULL)");for(const [key,value]of Object.entries(corpus.flow))db.query("INSERT INTO original_prefix VALUES(?,?)").run(key,value);for(const[key,value]of Object.entries(corpus.flow))expect(db.query("SELECT value FROM original_prefix WHERE field=?").get(key)).toEqual({value});}finally{db.close();}
});
for(const domain of corpus.domains)test("original typed prefix source "+domain.id,()=>{
 const source=readFileSync(new URL("../../../../../../"+domain.path+"/🚪️io/🪶️sqlite/📸️snapshot/🛬️native/🦀️.rs",import.meta.url),"utf8");expect({domain:domain.id,borrowedBinary:source.includes("fn binary_into("),borrowedText:source.includes("fn document_into("),actualWrapper:source.includes("workspace::decode("),ungrantedGuard:source.includes("Owned::new(")}).toEqual({domain:domain.id,borrowedBinary:true,borrowedText:true,actualWrapper:true,ungrantedGuard:false});
});
for(const domain of corpus.domains)test("original relational prefix source "+domain.id,()=>{
 const base=new URL("../../../../../../"+domain.path+"/🚪️io/🪶️sqlite/📸️snapshot/",import.meta.url);const db=new Database(":memory:");try{db.exec(readFileSync(new URL("🗄️.sql",base),"utf8"));expect((db.query("SELECT count(*) AS n FROM sqlite_schema WHERE type='table'").get() as {n:number}).n).toBeGreaterThan(0);}finally{db.close();}
 const source=readFileSync(new URL("💰️reconstruction/🦀️.rs",base),"utf8");expect(source).toContain("struct Prefix");expect(source).toContain("SchemaValidationStorage");expect(source).toContain("reconstruct_projected");expect(source).not.toContain("Owned::new(");expect(source).not.toContain("reconstruct_text(control,");
});
test("recursive value forest retains actual variant and link owners",()=>{
 const source=readFileSync(new URL("../../../../../../🔢️value/🚪️io/🪶️sqlite/📸️snapshot/💰️reconstruction/🌳️forest/🦀️.rs",import.meta.url),"utf8");expect(source).toContain("struct ValueForestStorage");expect(source).toContain("reconstruct_value_forest_into");expect(source).not.toContain("Owned::new(");expect(source).not.toContain("Vec<&");
});
test("relational envelope retains the exact selected child and original recipient",()=>{
 const f=JSON.parse(readFileSync(new URL("../../🧫️fixtures/🫙️prefix/✉️envelope.json",import.meta.url),"utf8")) as {schema:string;subset:string;selectedDocument:number};const validate=new Ajv({strict:true}).compile(JSON.parse(readFileSync(new URL("../../🧬️schema/🫙️prefix/✉️envelope.json",import.meta.url),"utf8")));expect(validate(f)).toBe(true);const db=new Database(":memory:");try{db.exec(readFileSync(new URL("../../🗄️.sql",import.meta.url),"utf8"));db.exec(readFileSync(new URL("../../../../../../🌊️flow/🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql",import.meta.url),"utf8"));db.run("INSERT INTO semio_flow_document VALUES(1,?)",[corpus.flow.schema]);db.run("INSERT INTO semio_base_document(id,schema,subset,flow_document_id) VALUES(1,?,?,?)",[f.schema,f.subset,f.selectedDocument]);expect(db.query("SELECT schema,subset,flow_document_id AS selectedDocument FROM semio_base_document").get()).toEqual({schema:f.schema,subset:f.subset,selectedDocument:f.selectedDocument});}finally{db.close();}
 const source=readFileSync(new URL("../../💰️reconstruction/🦀️.rs",import.meta.url),"utf8");expect(source).toContain("struct Prefix");expect(source).toContain("SchemaValidationStorage");expect(source).toContain("NativeDecodeRetirementRecipient");expect(source).toContain("with_retirement_child");expect(source).not.toContain("Owned::new(");
});
test("link pin original variants preserve authored metadata and expose controlled custody",()=>{
 const owner=resolve(import.meta.dir,...Array(14).fill(".."),"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store");const f=JSON.parse(readFileSync(resolve(owner,"🔗️link/🧫️fixtures/♻️retirement/🔣️.json"),"utf8")) as {pins:unknown[]};const shape=JSON.parse(readFileSync(resolve(owner,"🔗️link/🧬️schema/🔣️.json"),"utf8")),blob=JSON.parse(readFileSync(resolve(owner,"📦️blob/🧬️schema/🔣️.json"),"utf8"));const pinShape=shape.$defs.LinkPin;pinShape.oneOf[2].properties.blob=blob;delete blob.$schema;const validate=new Ajv({strict:true}).compile(pinShape);for(const pin of f.pins)expect(validate(pin)).toBe(true);const db=new Database(":memory:");try{db.exec("CREATE TABLE pins(id INTEGER PRIMARY KEY,value TEXT NOT NULL)");for(const[index,pin]of f.pins.entries())db.run("INSERT INTO pins VALUES(?,?)",[index,JSON.stringify(pin)]);expect((db.query("SELECT value FROM pins ORDER BY id").all() as {value:string}[]).map(row=>JSON.parse(row.value))).toEqual(f.pins);}finally{db.close();}
 const source=readFileSync(resolve(owner,"🔗️link/🧬️schema/🦀️.rs"),"utf8");expect(source.slice(source.indexOf("/// 📌️"),source.indexOf("pub enum LinkPin"))).toContain("semio_framework_value::RetireOwned");
});

test("native encoder keeps original output and admission children through cancellation",()=>{
 const fixture=JSON.parse(readFileSync(new URL("../../🧫️fixtures/🫙️prefix/🛫️encoding.json",import.meta.url),"utf8"));
 const schema=JSON.parse(readFileSync(new URL("../../🧬️schema/🫙️prefix/🛫️encoding.json",import.meta.url),"utf8"));
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
 const db=new Database(":memory:");try{
  const input=fixture.unit.repeat(fixture.repeat);const hex=(value:string)=>(db.query("SELECT lower(hex(CAST(? AS BLOB))) AS value").get(value) as {value:string}).value;
  const expected=fixture.preamble+"schema="+hex(fixture.schema)+"\nruns=[["+hex(fixture.language)+","+hex(input)+",[]]]";
  const independent=fixture.preamble+"schema="+Buffer.from(fixture.schema).toString("hex")+"\nruns=[["+Buffer.from(fixture.language).toString("hex")+","+Buffer.from(input).toString("hex")+",[]]]";
  expect(expected).toBe(independent);expect(expected.length).toBeGreaterThan(262144);
 }finally{db.close();}
 const source=readFileSync(new URL("../../🛫️native/🦀️.rs",import.meta.url),"utf8");expect(source).toContain("struct EncodingPrefix");expect(source).toContain("owned_workspace");expect(source).toContain("with_retirement_child");expect(source).toContain("output:Some(&mut prefix.bytes)");expect(source).not.toContain("let mut bytes=native.allocate_vec");
});
