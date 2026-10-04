import intrinsicCorpus from "../../../../🎒️pack/🌱️value/🧫️fixtures/🎞️intrinsic-media/🔣️.json";
import { test, expect } from "bun:test";
import { Database } from "bun:sqlite";
import Ajv from "ajv/dist/2020";
import fixture from "../../🧫️fixtures/🛬️controlled/🔣️.json";
import schema from "../../🧬️schema/🛬️controlled/🔣️.json";
import Ajv7 from "ajv";
import borrowedKeys from "../../🧫️fixtures/🛬️controlled/🔗️borrowed-keys.json";
import borrowedKeySchema from "../../🧬️schema/🛬️controlled/🔗️borrowed-keys.json";
import { ValueError } from "../../../⚠️refusal/🟦️.ts";

test("controlled native constructor neutral corpus independently validates and preserves ordered values",()=>{
  expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
  const database=new Database(":memory:");
  database.exec("CREATE TABLE entry(id INTEGER PRIMARY KEY,label TEXT NOT NULL,optional INTEGER);CREATE TABLE sample(entry INTEGER NOT NULL,ordinal INTEGER NOT NULL,value INTEGER NOT NULL);");
  const insert=database.query("INSERT INTO entry VALUES(?,?,?)"),sample=database.query("INSERT INTO sample VALUES(?,?,?)");
  for(const [index,entry]of fixture.entries.entries()){
    insert.run(index+1,entry.label,entry.optional===null?null:entry.optional?1:0);
    for(const [ordinal,value]of entry.samples.entries())sample.run(index+1,ordinal,value);
  }
  for(const[index,entry]of fixture.entries.entries()){
    const row=database.query("SELECT label,optional FROM entry WHERE id=?").get(index+1) as {label:string;optional:number|null};
    expect(row.label).toBe(entry.label);
    expect(row.optional===null?null:row.optional===1).toBe(entry.optional);
    expect((database.query("SELECT value FROM sample WHERE entry=? ORDER BY ordinal").all(index+1) as {value:number}[]).map(x=>x.value)).toEqual(entry.samples);
  }
  database.close();
});

test("unordered set corpus matches independent SQLite membership without promising iteration order",()=>{
  expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
  expect(new Set([...fixture.hashSets,...fixture.dropOwners].map(row=>row.id)).size).toBe(fixture.hashSets.length+fixture.dropOwners.length);
  const database=new Database(":memory:");
  database.exec("CREATE TABLE member(value INTEGER PRIMARY KEY CHECK(value >= 0 AND value <= 4294967295));");
  for(const row of fixture.hashSets){
    database.exec("DELETE FROM member;");
    const input=row.input as unknown;
    const values=Array.isArray(input)?input:[];
    const accepted=Array.isArray(input)&&values.every(value=>typeof value==="number"&&Number.isInteger(value)&&value>=0&&value<=4294967295);
    expect(accepted).toBe(row.accepted);
    if(accepted){for(const value of values)database.query("INSERT OR IGNORE INTO member VALUES(?)").run(value);expect((database.query("SELECT value FROM member ORDER BY value").all() as {value:number}[]).map(row=>row.value)).toEqual(row.members);}
  }
  database.close();
});

test("custom bridge defaults are portable and independently normalized by SQLite JSON",()=>{
  expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
  expect(new Set(fixture.customDefaults.map(row=>row.id)).size).toBe(fixture.customDefaults.length);
  const database=new Database(":memory:");
  for(const row of fixture.customDefaults){
    const defaultValue=row.mode==="controlled"?"":null;
    const result=database.query("SELECT json_set(?, '$.payload', json(?)) AS value").get(JSON.stringify({payload:defaultValue}),JSON.stringify("payload" in row.input?row.input.payload:defaultValue)) as {value:string};
    expect(JSON.parse(result.value).payload).toEqual(row.plainPayload);
    expect(row.controlledAccepted).toBe(row.mode==="controlled");
  }
  database.close();
});

test("intrinsic IEEE words and octets agree with independent DataView and Buffer",()=>{
  expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
  const view=new DataView(new ArrayBuffer(8));
  for(const word of fixture.floatWords){const bits=BigInt(`0x${word}`);view.setBigUint64(0,bits,false);expect(view.getBigUint64(0,false)).toBe(bits);expect(Buffer.from(view.buffer).toString("hex")).toBe(word);}
  expect([...Buffer.from(fixture.octets)]).toEqual(fixture.octets);
});

test("wide nullable frontier corpus agrees with independent SQLite JSON cardinality",()=>{
  const database=new Database(":memory:");
  const input=JSON.stringify(Array(fixture.limits.nullableSlots).fill(null));
  const row=database.query("SELECT json_array_length(?) AS count,(SELECT count(*) FROM json_each(?) WHERE type='null') AS nulls").get(input,input) as {count:number;nulls:number};
  expect(row.count).toBe(fixture.limits.nullableSlots);expect(row.nulls).toBe(row.count);
  expect(fixture.limits.interiorCancellationUnit).toBeLessThan(row.count);
  database.close();
});

test("owned Binary32 intrinsic bridge corpus agrees with independent DataView and Buffer words",()=>{
 const view=new DataView(new ArrayBuffer(4));
 for(const word of fixture.binary32Words){view.setUint32(0,Number.parseInt(word,16),false);expect(view.getUint32(0,false)).toBe(Number.parseInt(word,16));expect(Buffer.from(view.buffer).toString("hex")).toBe(word);}
});

test("NaN Binary32 bridge corpus preserves independent IEEE sign and payload positions",()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 const narrow=new DataView(new ArrayBuffer(4)),wide=new DataView(new ArrayBuffer(8));
 for(const row of fixture.binary32NanBridges){narrow.setUint32(0,Number.parseInt(row.word,16),false);wide.setBigUint64(0,BigInt("0x"+row.binary64Word),false);const bits32=BigInt(narrow.getUint32(0,false)),bits64=wide.getBigUint64(0,false);expect(bits64>>63n).toBe(bits32>>31n);expect((bits64>>52n)&2047n).toBe(2047n);expect((bits64&4503599627370495n)>>29n).toBe(bits32&8388607n);expect(Buffer.from(wide.buffer).toString("hex")).toBe(row.binary64Word);}
});


test("controlled native output neutral corpus preserves independent JSON and SQLite values",async()=>{
 const fixture=(await import("../../🧫️fixtures/🛫️controlled/🔣️.json")).default,shape=(await import("../../🧬️schema/🛫️controlled/🔣️.json")).default;expect(new Ajv({strict:true}).validate(shape,fixture)).toBe(true);
 expect(Object.keys(fixture.entries[0]!)).toEqual(fixture.fieldOrder);const db=new Database(":memory:");db.exec("CREATE TABLE encoded_entry(id INTEGER PRIMARY KEY,label TEXT NOT NULL,optional INTEGER);CREATE TABLE encoded_sample(parent INTEGER NOT NULL,ordinal INTEGER NOT NULL,value INTEGER NOT NULL);CREATE TABLE word(value INTEGER NOT NULL)");
 for(const [index,row]of fixture.entries.entries()){db.query("INSERT INTO encoded_entry VALUES(?,?,?)").run(index,row.label,row.optional===null?null:row.optional?1:0);for(const[ordinal,value]of row.samples.entries())db.query("INSERT INTO encoded_sample VALUES(?,?,?)").run(index,ordinal,value);expect(JSON.parse(JSON.stringify(row))).toEqual(row);expect(db.query("SELECT label AS label,optional AS optional FROM encoded_entry WHERE id=?").get(index)).toEqual({label:row.label,optional:row.optional===null?null:row.optional?1:0});expect((db.query("SELECT value FROM encoded_sample WHERE parent=? ORDER BY ordinal").all(index) as {value:number}[]).map(row=>row.value)).toEqual(row.samples);}
 for(const word of fixture.integerWords)db.query("INSERT INTO word VALUES(?)").run(BigInt(word));expect((db.query("SELECT CAST(value AS TEXT) AS word FROM word ORDER BY rowid").all() as {word:string}[]).map(row=>row.word)).toEqual(fixture.integerWords);
 db.query("INSERT INTO encoded_entry VALUES(?,?,?)").run(99,"x".repeat(fixture.largeTextBytes),null);expect(db.query("SELECT length(label) AS extent FROM encoded_entry WHERE id=99").get()).toEqual({extent:fixture.largeTextBytes});expect(fixture.cancelAfter).toBeLessThan(fixture.collectionItems);for(const row of fixture.representations){expect(JSON.parse(JSON.stringify(row.output))).toEqual(row.output);const result=db.query("SELECT json(?) AS value").get(JSON.stringify(row.output)) as {value:string};expect(JSON.parse(result.value)).toEqual(row.output);}expect(new Set(fixture.representations.map(row=>row.id)).size).toBe(fixture.representations.length);expect(Buffer.from(fixture.path.octets).toString("utf8")).toBe(fixture.path.lossy);expect(db.query("SELECT json_array_length(?) AS count").get(JSON.stringify(Array(fixture.frontier.nullableSlots).fill(null)))).toEqual({count:fixture.frontier.nullableSlots});expect(fixture.frontier.typedDepth).toBeGreaterThan(fixture.frontier.typedLimit);db.close();
});

test("controlled enum field identities survive generated-binding names in independent SQLite JSON",async()=>{
 const fixture=(await import("../../🧫️fixtures/🛫️controlled/🔣️.json")).default,schema=(await import("../../🧬️schema/🛫️controlled/🔣️.json")).default;
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);expect(new Set(fixture.bindingHygiene.map(row=>row.id)).size).toBe(fixture.bindingHygiene.length);
 const uninhabited=new Ajv({strict:true}).compile(false);expect(new Set(fixture.emptyEnums.map(row=>row.id)).size).toBe(fixture.emptyEnums.length);for(const row of fixture.emptyEnums){expect(row.inhabited).toBe(false);for(const input of row.inputs)expect(uninhabited(input)).toBe(false);}
 const database=new Database(":memory:");
 for(const row of fixture.bindingHygiene){const output=row.id==="external"?{Named:row.fields}:row.id==="internal"?{kind:"Named",...row.fields}:{kind:"Named",payload:row.fields};expect(output).toEqual(row.output);const result=database.query("SELECT json(?) AS value").get(JSON.stringify(output)) as {value:string};expect(JSON.parse(result.value)).toEqual(row.output);const pointer=row.id==="external"?'$.Named':row.id==="internal"?'$':'$.payload';expect(database.query("SELECT count(*) AS count FROM json_each(?,?)").get(JSON.stringify(row.output),pointer)).toEqual({count:Object.keys(row.fields).length+(row.id==="internal"?1:0)});}
 database.close();
});

test("scalar owner-named defaults retain independent SQLite JSON value semantics", () => {
  expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
  const database = new Database(":memory:");
  try {
    const items = database.query("SELECT value FROM json_each(?, '$.items') ORDER BY key").all(JSON.stringify(fixture.scalarOwnerConstInput)) as { value: number }[];
    expect(items.map(row => row.value)).toEqual(fixture.scalarOwnerConstInput.items);
    for (const row of fixture.scalarOwnerDefaultInputs) {
      const oracle = database.query("SELECT coalesce(json_extract(?, '$.count'), 1) AS count").get(JSON.stringify(row.input)) as { count: number };
      expect(oracle).toEqual(row.expected);
    }
  } finally { database.close(); }
});

test("borrowed long-key corpus retains literal UTF-8 membership through independent SQLite", () => {
  expect(new Ajv7({strict:true}).compile(borrowedKeySchema)(borrowedKeys)).toBe(true);
  expect(borrowedKeys.expectedKind).toBe(new ValueError("canceled","owned cancellation").kind);
  const key=borrowedKeys.unit.repeat(borrowedKeys.repeat),bytes=new TextEncoder().encode(key);
  expect(bytes.length).toBe(140000);
  expect(bytes.length).toBeGreaterThan(borrowedKeys.maximumOwnedBytes);
  const database=new Database(":memory:");
  try {
    database.run("CREATE TABLE entry(ordinal INTEGER PRIMARY KEY,key TEXT NOT NULL)");
    for(const row of borrowedKeys.cases){
      database.run("DELETE FROM entry");
      for(let i=0;i<row.entries;i++)database.query("INSERT INTO entry VALUES(?,?)").run(i,key);
      expect(database.query("SELECT count(*) AS total,count(DISTINCT key) AS kinds FROM entry").get()).toEqual({total:row.entries,kinds:1});
      expect((database.query("SELECT key,length(CAST(key AS BLOB)) AS bytes FROM entry ORDER BY ordinal").all()as{key:string;bytes:number}[])).toEqual(Array.from({length:row.entries},()=>({key,bytes:bytes.length})));
    }
  } finally { database.close(); }
});


test("borrowed object index has explicit full-key scratch and ordered identity authority", () => {
  type Index={storage:string;requests:string;probes:string;cases:{id:string;keys:string[];unique:boolean}[];wide:{prefix:string;count:number;duplicateOrdinal:number}};
  const index=(borrowedKeys as unknown as {indexAuthority?:Index}).indexAuthority;
  expect(index,"closed neutral exact borrowed-key storage facet").toBeDefined();
  if(!index)throw Error("borrowed-key index authority is absent");
  expect(index.storage).toBe("indexedOwnedSlots");expect(index.requests).toBe("fullConcreteReplacement");expect(index.probes).toBe("boundedFullUtf8Equality");
  expect(new Ajv7({strict:true}).compile(borrowedKeySchema)(borrowedKeys)).toBe(true);
  const database=new Database(":memory:");
  try {
    database.run("CREATE TABLE member(ordinal INTEGER PRIMARY KEY,key TEXT NOT NULL COLLATE BINARY)");
    const cases=[...index.cases,{id:"wide",keys:Array.from({length:index.wide.count},(_,i)=>index.wide.prefix+i),unique:true}];
    cases.push({id:"distantDuplicate",keys:[...cases.at(-1)!.keys,cases.at(-1)!.keys[index.wide.duplicateOrdinal]!],unique:false});
    for(const row of cases){database.run("DELETE FROM member");for(const[ordinal,key]of row.keys.entries())database.query("INSERT INTO member VALUES(?,?)").run(ordinal,key);
      const oracle=database.query("SELECT count(*)=count(DISTINCT key) AS unique_keys FROM member").get()as{unique_keys:number};expect(Boolean(oracle.unique_keys),row.id).toBe(row.unique);
      expect(database.query("SELECT ordinal,key,hex(CAST(key AS BLOB)) AS word FROM member ORDER BY ordinal").all()).toEqual(row.keys.map((key,ordinal)=>({ordinal,key,word:Buffer.from(key).toString("hex").toUpperCase()})));
    }
  }finally{database.close();}
});


test("canonical intrinsic retirement declares complete actual owner release without new backing",()=>{
 const contract=(borrowedKeys as unknown as {canonicalRetirement?:unknown}).canonicalRetirement;
 expect(contract,"canonical all-nine retirement contract").toEqual({requestBytes:0,releasedBytes:"completeBorrowedCapacityCensus",sourceCorpus:"intrinsic-media-wire-v1",depth:256,longBranch:{unit:"文🌠",repeat:20000,utf8Bytes:140000}});
 expect(new Ajv7({strict:true}).compile(borrowedKeySchema)(borrowedKeys)).toBe(true);
 expect(intrinsicCorpus.contract).toBe((contract as {sourceCorpus:string}).sourceCorpus);
 const validate=new Ajv7({strict:true}).compile(borrowedKeySchema);expect(validate({...borrowedKeys,canonicalRetirement:{...borrowedKeys.canonicalRetirement,requestBytes:1}})).toBe(false);expect(validate({...borrowedKeys,canonicalRetirement:{...borrowedKeys.canonicalRetirement,opaqueOwner:true}})).toBe(false);
 const word=new DataView(new ArrayBuffer(8));word.setBigUint64(0,0xfff800000000002an);expect(Number.isNaN(word.getFloat64(0))).toBe(true);expect(word.getBigUint64(0)).toBe(0xfff800000000002an);
 expect(Buffer.byteLength("文🌠".repeat(20000))).toBe(140000);
 const database=new Database(":memory:");try{database.run("CREATE TABLE intrinsic(ordinal INTEGER PRIMARY KEY,kind TEXT NOT NULL)");const kinds:string[]=[];type Node={kind:string;value?:unknown};const visit=(value:Node)=>{kinds.push(value.kind);if(value.kind==="array")for(const child of value.value as Node[])visit(child);if(value.kind==="object")for(const member of value.value as {key:string;value:Node}[])visit(member.value);};visit(intrinsicCorpus.value as Node);for(const[i,kind]of kinds.entries())database.query("INSERT INTO intrinsic VALUES(?,?)").run(i,kind);expect(database.query("SELECT kind FROM intrinsic ORDER BY ordinal").all()).toEqual(kinds.map(kind=>({kind})));expect(database.query("SELECT DISTINCT kind FROM intrinsic ORDER BY kind").all()).toEqual(["array","bool","bytes","float","int","null","object","string","uint"].map(kind=>({kind})));expect(database.query("SELECT count(DISTINCT kind) AS count FROM intrinsic").get()).toEqual({count:9});}finally{database.close();}
});
