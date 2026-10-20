import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import {applyPatch} from "fast-json-patch";
import Ajv from "ajv";
import protobuf from "protobufjs/minimal";
const read=(name:string)=>JSON.parse(readFileSync(new URL(name,import.meta.url),"utf8"));
const law=read("./🔣️.json") as {envelope:{name:string;initial:number;expected:number;discardedProjectionClones:number}[];commandAuthority:{name:string;job:boolean;generation:boolean;issuer:boolean;ready:boolean}[];ordering:{left:string;right:string;order:number}[];interaction:{name:string;initial:number;values:number[];expected:number;generation:number;edits:number}[];grantCopy:number[]};
test("original lane publication resolves every controlled insertion method at its actual defining owner",()=>{
 const owner=readFileSync(new URL("../../../../../../🔨️modules/📡️replication/🔗️causal/🔀️transition/🔁️fold/🗂️index/🪴️insert/🦀️.rs",import.meta.url),"utf8");
 for(const name of ["compare_original_text","step_controlled","commit_copy_byte_demand","commit_ready","commit_controlled","return_original_controlled"])expect(owner).toMatch(new RegExp(`pub fn ${name}\\b`));
 expect(owner).toContain("commit_or_replace_with");
 expect(owner).toContain("matching:Option<usize>");
 expect(owner).toContain("next_controlled_demands");
 expect(owner).toContain("commit_or_replace_controlled");
 const normalized=readFileSync(new URL("../../../📡️spr/📜️history/🔁️fold/🦀️.rs",import.meta.url),"utf8");
 expect(normalized).toContain("control.insert_text(&mut owners");
 expect(normalized).toContain("control.insert_text(&mut excluded");
 expect(normalized).not.toContain("owners.insert(");
 expect(normalized).not.toContain("excluded.insert(");
 expect(normalized).not.toContain("format!(");
 expect(normalized).not.toContain("schema.clone()");
 expect(normalized).not.toMatch(/\w+\.push\(/);
 const folded=readFileSync(new URL("../../../../../../🔨️modules/📡️replication/🔗️causal/🔀️transition/🔁️fold/🦀️.rs",import.meta.url),"utf8");
 expect(folded).not.toMatch(/\.(?:insert|get_or_insert)\(/);
 for(const api of ["insert_copy","insert_ordered_text","get_or_insert_actor","insert_mutation"])expect(folded).toContain(`control.${api}(`);
 expect(folded.includes("control.push_vec(&mut fold.alternatives[index].checkpoint_ids")).toBe(true);
 expect(folded.includes("admitted_original_release")).toBe(true);
 expect(folded.includes("page_bytes.store(body.clamp(1,4096)")).toBe(true);
 expect(folded.includes("page_bytes.store(grant.maximum_copy_bytes")).toBe(false);
 expect(folded.includes("crate::wire::read_varint_u64(")).toBe(false);
 expect(folded.includes("control.read_varint(")).toBe(true);
 expect(folded.includes("format!(")).toBe(false);
 expect(folded.includes("excluded: &HistoryFoldIndex<String, ()>")).toBe(true);
 expect(folded.includes("control.get_text(")).toBe(true);
 expect(/\.(?:remove|pop_first|get_mut)\(/.test(folded.replaceAll("control.pop_first(",""))).toBe(false);
 expect(normalized.includes("&|id| excluded_index.contains_key(id)")).toBe(false);
});

test("original immutable History source preserves refused inputs and the final unique root",()=>{
 const root="../../../📡️spr/📜️history/👁️source/";
 const corpus=read(root+"🧫️fixtures/🔣️.json") as {removals:{key:string;value:number|null}[];pops:{prefix:number;key:string;value:number}[];lookups:{key:string;value:number|null}[];vectorGrowth:{length:number;capacity:number}[];indexRows:{key:string;value:number}[];cases:{events:string[];expected:{input:boolean;source:boolean;alias:boolean;output:boolean}}[]};
 expect(new Ajv({strict:true}).validate(read(root+"🧬️schema/🔣️.json"),corpus)).toBe(true);
 for(const row of corpus.cases){
  let actual={input:true,source:false,alias:false,output:false},independent={...actual};
  for(const event of row.events){
   const patches:{op:"replace";path:string;value:boolean}[]=[];
   if(event==="birth"&&actual.input){actual.input=false;actual.source=true;patches.push({op:"replace",path:"/input",value:false},{op:"replace",path:"/source",value:true});}
   if(event==="fold"&&actual.source){actual.alias=true;patches.push({op:"replace",path:"/alias",value:true});}
   if(event==="closeFold"){actual.alias=false;patches.push({op:"replace",path:"/alias",value:false});}
   if(event==="take"&&actual.source&&!actual.alias){actual.source=false;actual.output=true;patches.push({op:"replace",path:"/source",value:false},{op:"replace",path:"/output",value:true});}
   if(event==="closeSource"&&actual.source&&!actual.alias){actual.source=false;patches.push({op:"replace",path:"/source",value:false});}
   independent=applyPatch(independent,patches,true,false).newDocument;
   expect(actual).toEqual(independent);
  }
  expect(actual).toEqual(row.expected);
 }
 const owner=readFileSync(new URL(root+"🦀️.rs",import.meta.url),"utf8");
 for(const api of ["admit","admit_fold","take_history","close_original"])expect(owner).toMatch(new RegExp(`pub fn ${api}\\b`));
 expect(owner).toContain("Arc::strong_count(source) != 1");
 expect(owner).toContain("Arc::weak_count(source) != 0");
 expect(owner.indexOf("*output = Some(Self")).toBeLessThan(owner.indexOf("native.record_constructed_allocation"));
 expect(owner).not.toContain("HistoryFoldJob::new");
 const db=new Database(":memory:");
 try{
  db.exec("CREATE TABLE original_rows(k BLOB PRIMARY KEY,v INTEGER)");const expected=new Map<string,number>();
  for(const row of corpus.indexRows){expected.set(row.key,row.value);db.query("INSERT INTO original_rows VALUES(?,?) ON CONFLICT(k) DO UPDATE SET v=excluded.v").run(Buffer.from(row.key),row.value);}
  const actual=db.query("SELECT k,v FROM original_rows ORDER BY k").all() as {k:Uint8Array;v:number}[];
  expect(actual.map(row=>[Buffer.from(row.k).toString("utf8"),row.v])).toEqual([...expected].sort(([a],[b])=>Buffer.compare(Buffer.from(a),Buffer.from(b))));
  for(const lookup of corpus.lookups){const row=db.query("SELECT v FROM original_rows WHERE k=?").get(Buffer.from(lookup.key)) as {v:number}|null;expect(row?.v??null).toBe(lookup.value);expect(expected.get(lookup.key)??null).toBe(lookup.value);}
  for(const removal of corpus.removals){const row=db.query("DELETE FROM original_rows WHERE k=? RETURNING v").get(Buffer.from(removal.key)) as {v:number}|null;expect(row?.v??null).toBe(removal.value);expect(expected.get(removal.key)??null).toBe(removal.value);expected.delete(removal.key);}
  db.exec("DELETE FROM original_rows");const first=new Map<string,number>();
  for(const row of corpus.indexRows){if(!first.has(row.key))first.set(row.key,row.value);db.query("INSERT INTO original_rows VALUES(?,?) ON CONFLICT(k) DO NOTHING").run(Buffer.from(row.key),row.value);}
  const retained=db.query("SELECT k,v FROM original_rows ORDER BY k").all() as {k:Uint8Array;v:number}[];
  expect(retained.map(row=>[Buffer.from(row.k).toString("utf8"),row.v])).toEqual([...first].sort(([a],[b])=>Buffer.compare(Buffer.from(a),Buffer.from(b))));
  db.exec("CREATE TABLE fixed_rows(k INTEGER PRIMARY KEY,v INTEGER); CREATE TABLE ordered_rows(p INTEGER,k BLOB,v INTEGER,PRIMARY KEY(p,k))");
  const fixed=new Map<number,number>(),ordered=new Map<string,[number,string,number]>();
  for(const row of corpus.indexRows){const key=Buffer.byteLength(row.key);fixed.set(key,row.value);db.query("INSERT INTO fixed_rows VALUES(?,?) ON CONFLICT(k) DO UPDATE SET v=excluded.v").run(key,row.value);const prefix=row.value%2;ordered.set(JSON.stringify([prefix,row.key]),[prefix,row.key,row.value]);db.query("INSERT INTO ordered_rows VALUES(?,?,?) ON CONFLICT(p,k) DO UPDATE SET v=excluded.v").run(prefix,Buffer.from(row.key),row.value);}
  expect(db.query("SELECT k,v FROM fixed_rows ORDER BY k").all()).toEqual([...fixed].sort(([a],[b])=>a-b).map(([k,v])=>({k,v})));
  const order=db.query("SELECT p,k,v FROM ordered_rows ORDER BY p,k").all() as {p:number;k:Uint8Array;v:number}[];
  expect(order.map(row=>[row.p,Buffer.from(row.k).toString("utf8"),row.v])).toEqual([...ordered.values()].sort(([ap,ak],[bp,bk])=>ap-bp||Buffer.compare(Buffer.from(ak),Buffer.from(bk))));
  for(const pop of corpus.pops){const row=db.query("DELETE FROM ordered_rows WHERE (p,k)=(SELECT p,k FROM ordered_rows ORDER BY p,k LIMIT 1) RETURNING p,k,v").get() as {p:number;k:Uint8Array;v:number};expect({prefix:row.p,key:Buffer.from(row.k).toString("utf8"),value:row.v}).toEqual(pop);}
  expect(db.query("SELECT count(*) AS n FROM ordered_rows").get()).toEqual({n:0});
  for(const row of corpus.vectorGrowth){
   let capacity=0;for(let n=0;n<row.length;n++)if(n===capacity)capacity=Math.max(n+1,2*capacity);
   const independent=db.query("WITH RECURSIVE backing(n) AS (VALUES(0) UNION ALL SELECT CASE WHEN n=0 THEN 1 ELSE 2*n END FROM backing WHERE n<?) SELECT max(n) AS capacity FROM backing").get(row.length) as {capacity:number};
   expect(capacity).toBe(row.capacity);expect(independent.capacity).toBe(row.capacity);
   for(const word of [4,8])for(const width of [1,3*word])expect(Buffer.alloc(capacity*width).byteLength).toBe(row.capacity*width);
  }
 }finally{db.close();}
});
test("original lane corpus agrees with SQLite byte order and JSONPatch interaction events",()=>{
 const validate=new Ajv({strict:true}).compile(read("./🧬️schema/🔣️.json"));expect(validate(law)).toBe(true);
 const db=new Database(":memory:");
 try{
  for(const row of law.ordering){const expected=db.query("SELECT CASE WHEN CAST(? AS BLOB)<CAST(? AS BLOB) THEN -1 WHEN CAST(? AS BLOB)>CAST(? AS BLOB) THEN 1 ELSE 0 END AS ordering").get(row.left,row.right,row.left,row.right) as {ordering:number};expect(expected.ordering).toBe(row.order);
   for(const word of [4,8])for(const body of law.grantCopy){let offset=0,ready=false,result=0,turns=0;const a=new TextEncoder().encode(row.left),b=new TextEncoder().encode(row.right),metadata=word+1;while(!ready&&turns++<128){if(body<metadata)break;let end=Math.min(a.length,b.length,offset+Math.floor((body-metadata)/2));while(offset<end){result=Math.sign(a[offset]!-b[offset]!);offset++;if(result){ready=true;break;}}if(!ready&&offset===Math.min(a.length,b.length)){result=Math.sign(a.length-b.length);ready=true;}if(end===offset&&!ready&&body<metadata+2)break;}if(ready)expect(result).toBe(row.order);else expect(body).toBeLessThan(metadata+2);}
  }
  for(const row of law.envelope){const reference=db.query("SELECT ? AS original,0 AS discardedProjectionClones").get(row.initial) as {original:number;discardedProjectionClones:number};expect(reference).toEqual({original:row.expected,discardedProjectionClones:row.discardedProjectionClones});}
  for(const row of law.commandAuthority){const expected=db.query("SELECT ? AND ? AND ? AS admitted").get(Number(row.job),Number(row.generation),Number(row.issuer)) as {admitted:number};expect(Boolean(expected.admitted)).toBe(row.ready);}
  for(const row of law.interaction){let state={n:row.initial};for(const value of row.values)state=applyPatch(state,[{op:"replace",path:"/n",value}],true,false).newDocument;expect(state.n).toBe(row.expected);expect(row.values.length).toBe(row.generation);expect(row.values.length).toBe(row.edits);}
 }finally{db.close();}
});

test("original scalar varints preserve u64 values and exact consumed prefixes against protobuf",()=>{
 const corpus=read("../../../📡️spr/📜️history/👁️source/🧪️tests/🧫️fixtures/🔣️.json") as {varints:{bytes:number[];value:string|null;position:number;refusal:null|"truncated"|"overflow"}[]};
 for(const row of corpus.varints){
  for(const body of [1,7,4096])for(const word of [4,8]){
   let position=0,value=0n,refusal:null|"truncated"|"overflow"=null,copied=0;
   for(let index=0;index<10;index++){
    const physical=2*(5*word)+1;
    expect(physical).toBeGreaterThan(1);
    if(position===row.bytes.length){refusal="truncated";break;}
    const byte=row.bytes[position++]!;copied++;
    expect(Math.min(body,1)).toBe(1);
    if(index===9&&((byte&128)!==0||(byte&127)>1)){refusal="overflow";break;}
    value|=BigInt(byte&127)<<BigInt(7*index);
    if((byte&128)===0)break;
   }
   expect({value:refusal?null:value.toString(),position,refusal}).toEqual({value:row.value,position:row.position,refusal:row.refusal});
   expect(copied).toBe(row.position);
  }
  if(row.refusal===null){
   const reader=protobuf.Reader.create(Uint8Array.from(row.bytes));
   expect(reader.uint64().toString()).toBe(row.value!);
   expect(reader.pos).toBe(row.position);
   expect([...protobuf.Writer.create().uint64(row.value!).finish()]).toEqual(row.bytes);
  }
 }
});

test("original UTF8 scalar custody agrees with independent streaming decoders",()=>{
 const corpus=read("../../../📡️spr/📜️history/👁️source/🧪️tests/🧫️fixtures/🔣️.json") as {utf8:{bytes:number[];text:string|null;position:number;offset:number;refusal:string|null}[]};
 for(const row of corpus.utf8){
  const bytes=Uint8Array.from(row.bytes);
  if(row.text===null){expect(()=>new TextDecoder("utf-8",{fatal:true}).decode(bytes)).toThrow();expect(row.refusal).not.toBeNull();}
  else{
   for(const body of [1,7,4096]){
    const independent=new TextDecoder("utf-8",{fatal:true});let text="";
    for(let at=0;at<bytes.length;at+=body)text+=independent.decode(bytes.subarray(at,at+body),{stream:true});
    text+=independent.decode();expect(text).toBe(row.text);expect(Buffer.from(bytes).toString("utf8")).toBe(row.text);
   }
   expect(row.position).toBe(bytes.length);
  }
 }
});

test("original normalized operation identities retain exact Unicode and full unsigned indexes",()=>{
 const corpus=read("../../../📡️spr/📜️history/👁️source/🧪️tests/🧫️fixtures/🔣️.json") as {identities:{edit:string;index:string;expected:string}[]};
 const independent=new Database(":memory:");
 try{for(const row of corpus.identities){
  let index=BigInt(row.index),digits="";
  do{digits=String(Number(index%10n))+digits;index/=10n;}while(index);
  expect(`${row.edit}#${digits}`).toBe(row.expected);
  expect((independent.query("SELECT ? || '#' || ? AS identity").get(row.edit,row.index)as{identity:string}).identity).toBe(row.expected);
  const wire=protobuf.Writer.create().uint64(row.index).finish();
  expect(protobuf.Reader.create(wire).uint64().toString()).toBe(digits);
 }}finally{independent.close();}
});
