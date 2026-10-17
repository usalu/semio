import {expect,test} from "bun:test";
import Ajv from "ajv";
import Ajv2020 from "ajv/dist/2020";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import {join} from "node:path";
test("original stepped scalar DAG agrees with independent recursive SQLite projection",()=>{
 const law=JSON.parse(readFileSync(join(import.meta.dir,"../🧫️fixtures/🪜️stepped-driver/🔣️.json"),"utf8")),schema=JSON.parse(readFileSync(join(import.meta.dir,"../../🧬️schema/🪜️stepped-driver/🔣️.json"),"utf8"));expect(new Ajv2020({strict:false}).compile(schema)(law)).toBe(true);
 const db=new Database(":memory:");try{db.exec("CREATE TABLE nodes(id TEXT PRIMARY KEY,weight INTEGER);CREATE TABLE edges(parent TEXT,child TEXT)");for(const node of law.nodes){db.run("INSERT INTO nodes VALUES(?,?)",node.key,node.weight);for(const parent of node.parents)db.run("INSERT INTO edges VALUES(?,?)",parent,node.key);}const rows=db.query("WITH RECURSIVE ancestors(child,parent) AS (SELECT id,id FROM nodes UNION ALL SELECT ancestors.child,edges.parent FROM ancestors JOIN edges ON edges.child=ancestors.parent) SELECT child AS id,sum(weight) AS value FROM ancestors JOIN nodes ON nodes.id=ancestors.parent GROUP BY child ORDER BY child").all()as{id:string;value:number}[];expect(Object.fromEntries(rows.map(row=>[row.id,row.value]))).toEqual(law.expected);}finally{db.close()}
 expect(law.minimumComputeSteps).toBe(law.nodes.length*law.slices);
});
test("original inferred value ordinals agree with independent SQLite cells",()=>{
 const root=join(import.meta.dir,"../../🧺️values"),law=JSON.parse(readFileSync(join(root,"🧪️tests/🧫️fixtures/🔣️.json"),"utf8")),schema=JSON.parse(readFileSync(join(root,"🧬️schema/🔣️.json"),"utf8"));expect(new Ajv2020({strict:false}).compile(schema)(law)).toBe(true);
 const db=new Database(":memory:");try{db.run("CREATE TABLE cells(source INTEGER PRIMARY KEY,value INTEGER)");for(let source=0;source<law.nodes;source++)db.run("INSERT INTO cells VALUES(?,NULL)",[source]);for(const row of law.outputs)db.run("UPDATE cells SET value=? WHERE source=? AND value IS NULL",row.value,row.source);expect(db.query("SELECT source,value FROM cells WHERE value IS NOT NULL ORDER BY source").all()).toEqual([{source:0,value:3},{source:2,value:11}]);expect(db.query("SELECT count(*) AS count FROM cells").get()).toEqual({count:law.nodes});}finally{db.close()}
});
test("original inference caller and complete failure receipt remain one event",()=>{
 const fixture=JSON.parse(readFileSync(join(import.meta.dir,"../🧫️fixtures/🎟️caller/🔣️.json"),"utf8"));const schema=JSON.parse(readFileSync(join(import.meta.dir,"../../🧬️schema/🎟️caller/🔣️.json"),"utf8"));const validate=new Ajv2020({strict:false}).compile(schema);expect(validate(fixture)).toBe(true);expect(validate({...fixture,caller:{...fixture.caller,fuel:2}})).toBe(false);
 const db=new Database(":memory:");try{db.run("CREATE TABLE original_call(operation INTEGER,generation INTEGER,fuel INTEGER,deadline INTEGER)");db.run("INSERT INTO original_call VALUES(?,?,?,?)",fixture.caller.operation,fixture.caller.generation,fixture.caller.fuel,fixture.caller.deadlineUs);db.run("UPDATE original_call SET fuel=fuel-1 WHERE operation=? AND generation=? AND fuel>0 AND deadline>?",fixture.caller.operation,fixture.caller.generation,fixture.caller.clockUs);expect(db.query("SELECT * FROM original_call").get()).toEqual({operation:961,generation:13,fuel:fixture.caller.fuel-1,deadline:1000000});expect(db.query("SELECT count(*) AS count FROM original_call WHERE fuel>0").get()).toEqual({count:1});}finally{db.close();}
 expect(fixture.preserveWholeFailure).toEqual(["fieldCode","fieldMessage","driverKey","driverParent","nativePrimary","receiptRefusal","contextRefusal"]);
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
});

test("field projection identities follow original payload through borrowed headers",()=>{
 const owner=join(import.meta.dir,"../../../../../../🔨️modules/🗣️dsl/🧬️schema/🛫️encode"),law=JSON.parse(readFileSync(join(owner,"🧪️tests/🧫️fixtures/🪪️identity/🔣️.json"),"utf8")),schema=JSON.parse(readFileSync(join(owner,"🧬️schema/🪪️identity/🔣️.json"),"utf8"));expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);const original={text:law.original},first={original},second={original},changed={original:{text:law.original}};expect(first!==second&&first.original===second.original).toBe(true);expect(changed.original===first.original).toBe(false);expect(new TextDecoder("utf-8",{fatal:true}).decode(new TextEncoder().encode(original.text))).toBe(law.original);
});

test("mounted inference admission borrows original UTF8 names and donates the same source",()=>{
 const owner=join(import.meta.dir,"../../../🔌️plugin/📜️declaration/👷️job/🧪️inference-admission"),law=JSON.parse(readFileSync(join(owner,"🧫️fixtures/🔣️.json"),"utf8")),schema=JSON.parse(readFileSync(join(owner,"🧬️schema/🔣️.json"),"utf8"));expect(new Ajv2020({strict:false}).compile(schema)(law)).toBe(true);expect(new Ajv2020({strict:false}).compile(schema)({...law,lookupAtom:2})).toBe(false);const encoder=new TextEncoder(),name=encoder.encode(law.name),known=encoder.encode(law.existingName);let byte=0;while(byte<name.length&&byte<known.length&&name[byte]===known[byte])byte++;expect(byte).toBe(3);const original={text:law.payload},donor={original:original as typeof original|null},access={original:donor.original};donor.original=null;expect(access.original).toBe(original);expect(encoder.encode(law.name).byteLength).toBe(8);
});


test("original inference identity admits the same source and refuses every foreign axis",()=>{
 const law=JSON.parse(readFileSync(join(import.meta.dir,"../🧫️fixtures/🎟️caller/🔣️.json"),"utf8")),schema=JSON.parse(readFileSync(join(import.meta.dir,"../../🧬️schema/🎟️caller/🔣️.json"),"utf8"));expect(new Ajv2020({strict:false}).compile(schema)(law)).toBe(true);
 const db=new Database(":memory:");try{
  db.run("CREATE TABLE source(parent TEXT,recipient TEXT,clock TEXT,binding INTEGER,deadline INTEGER)");const original=law.sourceIdentity.original;db.run("INSERT INTO source VALUES(?,?,?,?,?)",original.parent,original.recipient,original.clock,original.clockBinding,original.deadlineUs);
  for(const row of law.sourceIdentity.cases){const input=row.identity;const found=db.query("SELECT count(*) AS matched FROM source WHERE parent=? AND recipient=? AND clock=? AND binding=? AND deadline=?").get(input.parent,input.recipient,input.clock,input.clockBinding,input.deadlineUs)as{matched:number};expect(found.matched===1).toBe(row.accepted);}
 }finally{db.close()}
});
