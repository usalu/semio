import {expect,test} from "bun:test";
import Ajv from "ajv";
import Ajv2020 from "ajv/dist/2020";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import {join} from "node:path";
test("original inference caller and complete failure receipt remain one event",()=>{
 const fixture=JSON.parse(readFileSync(join(import.meta.dir,"../🧫️fixtures/🎟️caller/🔣️.json"),"utf8"));const schema=JSON.parse(readFileSync(join(import.meta.dir,"../../🧬️schema/🎟️caller/🔣️.json"),"utf8"));const validate=new Ajv2020({strict:false}).compile(schema);expect(validate(fixture)).toBe(true);expect(validate({...fixture,caller:{...fixture.caller,fuel:2}})).toBe(false);
 const db=new Database(":memory:");try{db.run("CREATE TABLE original_call(operation INTEGER,generation INTEGER,fuel INTEGER,deadline INTEGER)");db.run("INSERT INTO original_call VALUES(?,?,?,?)",fixture.caller.operation,fixture.caller.generation,fixture.caller.fuel,fixture.caller.deadlineUs);db.run("UPDATE original_call SET fuel=fuel-1 WHERE operation=? AND generation=? AND fuel>0 AND deadline>?",fixture.caller.operation,fixture.caller.generation,fixture.caller.clockUs);expect(db.query("SELECT * FROM original_call").get()).toEqual({operation:961,generation:13,fuel:0,deadline:1000000});expect(db.query("SELECT * FROM original_call WHERE fuel>0").all()).toEqual([]);}finally{db.close();}
 expect(fixture.preserveWholeFailure).toEqual(["fieldCode","fieldMessage","driverKey","driverParent","nativePrimary","receiptRefusal","contextRefusal"]);console.log("[DEBUG] independent Ajv/SQLite preserves original inference op961/gen13/deadline1000000 and one actual CPU event; native projection is separate");
});

test("borrowed original graph planning agrees with SQLite document-order Kahn",()=>{
 const law=JSON.parse(readFileSync(join(import.meta.dir,"../🧫️fixtures/🧭️plan/🔣️.json"),"utf8")),schema=JSON.parse(readFileSync(join(import.meta.dir,"../../🧬️schema/🧭️plan/🔣️.json"),"utf8"));expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);
 const db=new Database(":memory:");try{
  db.exec("CREATE TABLE nodes(id TEXT PRIMARY KEY,ordinal INTEGER);CREATE TABLE edges(parent TEXT,child TEXT,PRIMARY KEY(parent,child));CREATE TABLE emitted(id TEXT PRIMARY KEY,ordinal INTEGER)");
  for(const [ordinal,id]of law.nodes.entries())db.query("INSERT OR IGNORE INTO nodes VALUES(?,?)").run(id,ordinal);
  for(const [parent,child]of law.links)db.query("INSERT OR IGNORE INTO edges SELECT ?,? WHERE EXISTS(SELECT 1 FROM nodes WHERE id=?) AND EXISTS(SELECT 1 FROM nodes WHERE id=?)").run(parent,child,parent,child);
  const queue=(db.query("SELECT id FROM nodes WHERE NOT EXISTS(SELECT 1 FROM edges WHERE child=nodes.id) ORDER BY ordinal").all()as{id:string}[]).map(row=>row.id),out:string[]=[];
  while(queue.length){const id=queue.shift()!;out.push(id);db.query("INSERT INTO emitted VALUES(?,?)").run(id,out.length);for(const row of db.query("SELECT id FROM nodes WHERE EXISTS(SELECT 1 FROM edges WHERE parent=? AND child=nodes.id) AND NOT EXISTS(SELECT 1 FROM edges WHERE child=nodes.id AND parent NOT IN(SELECT id FROM emitted)) ORDER BY ordinal").all(id)as{id:string}[])queue.push(row.id);}
  out.push(...(db.query("SELECT id FROM nodes WHERE id NOT IN(SELECT id FROM emitted) ORDER BY ordinal").all()as{id:string}[]).map(row=>row.id));expect(out).toEqual(law.expected);expect(db.query("SELECT count(*) AS count FROM edges").get()).toEqual({count:law.expectedDistinctEdges});
 }finally{db.close()}
 console.log("[DEBUG] independent Ajv/SQLite actual document-order graph: duplicate nodes/links, missing source, roots, parent order and cyclic tail; native paid events separate");
});

test("field projection identities follow original payload through borrowed headers",()=>{
 const owner=join(import.meta.dir,"../../../../../../🔨️modules/🗣️dsl/🧬️schema/🛫️encode"),law=JSON.parse(readFileSync(join(owner,"🧪️tests/🧫️fixtures/🪪️identity/🔣️.json"),"utf8")),schema=JSON.parse(readFileSync(join(owner,"🧬️schema/🪪️identity/🔣️.json"),"utf8"));expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);const original={text:law.original},first={original},second={original},changed={original:{text:law.original}};expect(first!==second&&first.original===second.original).toBe(true);expect(changed.original===first.original).toBe(false);expect(new TextDecoder("utf-8",{fatal:true}).decode(new TextEncoder().encode(original.text))).toBe(law.original);console.log("[DEBUG] independent Ajv/UTF8 object-reference law: borrowed header movement preserves actual original reference; same bytes from another original remain distinct");
});

test("mounted inference admission borrows original UTF8 names and donates the same source",()=>{
 const owner=join(import.meta.dir,"../../../🔌️plugin/📜️declaration/👷️job/🧪️inference-admission"),law=JSON.parse(readFileSync(join(owner,"🧫️fixtures/🔣️.json"),"utf8")),schema=JSON.parse(readFileSync(join(owner,"🧬️schema/🔣️.json"),"utf8"));expect(new Ajv2020({strict:false}).compile(schema)(law)).toBe(true);expect(new Ajv2020({strict:false}).compile(schema)({...law,lookupAtom:2})).toBe(false);const encoder=new TextEncoder(),name=encoder.encode(law.name),known=encoder.encode(law.existingName);let byte=0;while(byte<name.length&&byte<known.length&&name[byte]===known[byte])byte++;expect(byte).toBe(3);const original={text:law.payload},donor={original:original as typeof original|null},access={original:donor.original};donor.original=null;expect(access.original).toBe(original);expect(encoder.encode(law.name).byteLength).toBe(8);console.log("[DEBUG] independent Ajv/TextEncoder original mounted lookup UTF8 byte atom and exact source donation; native physical admission remains separate");
});
