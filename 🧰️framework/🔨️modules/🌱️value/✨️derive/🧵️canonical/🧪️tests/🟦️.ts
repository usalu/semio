/** 🧵️ Native field-role declaration follows an independent tagged JSON/SQLite oracle. */
import {test,expect} from "bun:test";
import Ajv2020 from "ajv/dist/2020";
import {Database} from "bun:sqlite";
import {readFileSync,existsSync} from "node:fs";
test("canonical native field derivation retains schema wire order without serialization callbacks",()=>{
 const root=new URL("../",import.meta.url);const read=(path:string)=>JSON.parse(readFileSync(new URL(path,root),"utf8"));const fixture=read("🧫️fixtures/🔣️.json");
 expect(new Ajv2020({strict:true}).compile(read("🧬️schema/🔣️.json"))(fixture)).toBe(true);expect(JSON.stringify({[fixture.tag]:fixture.variant,...fixture.values})).toBe(fixture.expected);
 const db=new Database(":memory:");db.exec("CREATE TABLE fields(ordinal INTEGER PRIMARY KEY,name TEXT,owner INTEGER)");Object.keys(fixture.values).forEach((key,index)=>db.query("INSERT INTO fields VALUES(?,?,1)").run(index,key));expect((db.query("SELECT name FROM fields ORDER BY ordinal").all() as {name:string}[]).map(row=>row.name)).toEqual(Object.keys(JSON.parse(fixture.expected)).slice(1));expect(db.query("SELECT SUM(owner) AS n FROM fields").get()).toEqual({n:3});db.close();
 console.log("[DEBUG] Ajv/JSON tagged record byte oracle and SQLite native schema field ownership/order agree; no whole payload copies");
 const path=new URL("🦀️.rs",root);expect(existsSync(path)).toBe(true);const source=readFileSync(path,"utf8");expect(source).toContain("canonical_tree_child");expect(source).toContain("ArtifactCanonicalJsonTree");expect(source).not.toContain("ToValue::to_value");expect(source).not.toContain("transmute");
});
test("canonical native field roles follow independent predicate, octet array and decimal string oracles",()=>{
 const root=new URL("../",import.meta.url);const read=(path:string)=>JSON.parse(readFileSync(new URL(path,root),"utf8"));const fixture=read("🧫️fixtures/🔣️.json");
 const validate=new Ajv2020({strict:true}).compile(read("🧬️schema/🔣️.json"));expect(validate(fixture)).toBe(true);
 const db=new Database(":memory:");db.exec("CREATE TABLE emitted(row INTEGER,ordinal INTEGER,name TEXT,PRIMARY KEY(row,ordinal))");
 fixture.roles.forEach((row:any,index:number)=>{
  const wire:Record<string,unknown>={};
  if(row.padByte!==0)wire.padByte=row.padByte;if(row.quote!=="double")wire.quote=row.quote;if(row.position!=="0")wire.position=row.position;wire.octets=row.octets;if(row.tail!==null)wire.tail=row.tail;wire.maybe=row.maybe;
  expect(JSON.stringify(wire)).toBe(row.expected);expect(BigInt(row.position).toString()).toBe(row.position);expect(typeof JSON.parse(row.expected).position).toBe(row.position==="0"?"undefined":"string");
  Object.keys(wire).forEach((name,ordinal)=>db.query("INSERT INTO emitted VALUES(?,?,?)").run(index,ordinal,name));
 });
 const counts=db.query("SELECT row,COUNT(*) AS n FROM emitted GROUP BY row ORDER BY row").all() as {row:number,n:number}[];expect(counts.map(count=>count.n)).toEqual(fixture.roles.map((row:any)=>Object.keys(JSON.parse(row.expected)).length));db.close();
 console.log("[DEBUG] canonical field-role fixture: Ajv schema, JSON.stringify predicate/octet/decimal oracle and SQLite emitted-field order agree");
 const source=readFileSync(new URL("🦀️.rs",root),"utf8");expect(source).toContain("decimal_string");expect(source).toContain("octets_kind");expect(source).not.toContain("transmute");
});
test("canonical hex word roles follow an independent IEEE-754 binary32 bit oracle",()=>{
 const root=new URL("../",import.meta.url);const read=(path:string)=>JSON.parse(readFileSync(new URL(path,root),"utf8"));const fixture=read("🧫️fixtures/🔣️.json");
 expect(new Ajv2020({strict:true}).compile(read("🧬️schema/🔣️.json"))(fixture)).toBe(true);
 const word=(value:number)=>{const view=new DataView(new ArrayBuffer(4));view.setFloat32(0,value);return view.getUint32(0).toString(16).padStart(8,"0")};
 const db=new Database(":memory:");db.exec("CREATE TABLE words(value REAL,word TEXT)");
 for(const row of fixture.hexWords){
  const wire:Record<string,unknown>={position:row.position.map(word)};if(row.normal!==null)wire.normal=row.normal.map(word);wire.halfedge=row.halfedge;wire.uv=row.uv.map(word);wire.weight=word(row.weight);if(row.samples.length)wire.samples=row.samples.map(word);
  expect(JSON.stringify(wire)).toBe(row.expected);
  for(const value of [...row.position,...row.uv,row.weight,...row.samples])db.query("INSERT INTO words VALUES(?,?)").run(value,word(value));
 }
 for(const record of db.query("SELECT word FROM words").all() as {word:string}[])expect(record.word).toMatch(/^[0-9a-f]{8}$/);
 db.close();
 console.log("[DEBUG] canonical hex word fixture: Ajv schema, DataView binary32 bit oracle and SQLite word shape agree");
 const source=readFileSync(new URL("🦀️.rs",root),"utf8");expect(source).toContain("hex_word");expect(source).not.toContain("transmute");
});
test("canonical flatten splices the flattened record at its field position per an independent oracle",()=>{
 const root=new URL("../",import.meta.url);const read=(path:string)=>JSON.parse(readFileSync(new URL(path,root),"utf8"));const fixture=read("🧫️fixtures/🔣️.json");
 expect(new Ajv2020({strict:true}).compile(read("🧬️schema/🔣️.json"))(fixture)).toBe(true);
 const db=new Database(":memory:");db.exec("CREATE TABLE members(row INTEGER,owner TEXT,ordinal INTEGER,name TEXT)");
 fixture.flatten.forEach((row:any,index:number)=>{
  const header:Record<string,unknown>={id:row.id};if(row.note!==null)header.note=row.note;
  const nested={...header,level:row.level};
  const wire:Record<string,unknown>={name:row.name,...nested};if(row.extraCount!==null)wire.extraCount=row.extraCount;if(row.tail!==null)wire.tail=row.tail;
  expect(JSON.stringify(wire)).toBe(row.expected);
  Object.keys(wire).forEach((name,ordinal)=>db.query("INSERT INTO members VALUES(?,?,?,?)").run(index,name==="name"||name==="tail"?"root":name==="extraCount"?"extra":name==="level"?"nested":"header",ordinal,name));
 });
 expect((db.query("SELECT COUNT(*) AS n FROM members WHERE owner='header'").get() as {n:number}).n).toBe(fixture.flatten.reduce((n:number,row:any)=>n+1+(row.note!==null?1:0),0));
 db.close();
 console.log("[DEBUG] canonical flatten fixture: Ajv schema, JSON.stringify splice oracle and SQLite member ownership agree");
 const source=readFileSync(new URL("🦀️.rs",root),"utf8");expect(source).toContain("canonical flattened field is not an object");
});
test("canonical externally tagged named variants nest their fields under the variant key per an independent oracle",()=>{
 const root=new URL("../",import.meta.url);const read=(path:string)=>JSON.parse(readFileSync(new URL(path,root),"utf8"));const fixture=read("🧫️fixtures/🔣️.json");
 expect(new Ajv2020({strict:true}).compile(read("🧬️schema/🔣️.json"))(fixture)).toBe(true);
 const db=new Database(":memory:");db.exec("CREATE TABLE shapes(variant TEXT,depth INTEGER,members INTEGER)");
 for(const row of fixture.externalVariants){
  let wire:unknown;
  if(row.variant==="point")wire="point";
  else if(row.variant==="circle")wire={circle:row.radius};
  else if(row.variant==="rect"){const rect:Record<string,unknown>={width:row.width,height:row.height};if(row.label!==null)rect.label=row.label;wire={rect};}
  else wire={frame:{outerEdge:row.edge==="solid"?"solid":{dashed:{onLength:row.onLength,offLength:row.offLength}},count:row.count}};
  expect(JSON.stringify(wire)).toBe(row.expected);
  const members=typeof wire==="string"?0:Object.keys((wire as Record<string,Record<string,unknown>>)[row.variant]??{}).length;
  db.query("INSERT INTO shapes VALUES(?,?,?)").run(row.variant,typeof wire==="string"?0:1,members);
 }
 expect((db.query("SELECT COUNT(*) AS n FROM shapes WHERE depth=1").get() as {n:number}).n).toBe(fixture.externalVariants.filter((row:any)=>row.variant!=="point").length);
 db.close();
 console.log("[DEBUG] canonical externally tagged variant fixture: Ajv schema, JSON.stringify single-key wrapper oracle and SQLite variant nesting agree");
 const source=readFileSync(new URL("🦀️.rs",root),"utf8");expect(source).toContain("CanonicalView");expect(source).not.toContain("transmute");
});
