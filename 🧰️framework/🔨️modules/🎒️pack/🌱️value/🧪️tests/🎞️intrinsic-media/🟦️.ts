/** 🎞️ Independent SQL materialization retains neutral intrinsic occurrences and exact words. */
import{test,expect}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import fixture from"../../🧫️fixtures/🎞️intrinsic-media/🔣️.json";
import schema from"../../🧬️schema/🔣️.json";
import depthFixture from"../../🧫️fixtures/🎞️intrinsic-media/🌲️depth/🔣️.json";
type Literal={kind:string;value?:unknown};

test("closed intrinsic media corpus survives independent SQLite file materialization",()=>{
 const validator=new Ajv({strict:true});
 const database=new Database(":memory:");database.exec("CREATE TABLE node(id INTEGER PRIMARY KEY,parent INTEGER,ordinal INTEGER NOT NULL,name BLOB,kind TEXT NOT NULL,literal BLOB);CREATE UNIQUE INDEX occurrence ON node(parent,ordinal)");let next=0;
 function write(value:Literal,parent:number|null,ordinal:number,name:string|null):void{
  const id=next++,literal=value.kind==="float"?Buffer.from(value.value as string,"hex"):value.kind==="bytes"?Buffer.from(value.value as string,"hex"):value.kind==="null"||value.kind==="array"||value.kind==="object"?null:Buffer.from(String(value.value));
  database.query("INSERT INTO node VALUES(?,?,?,?,?,?)").run(id,parent,ordinal,name===null?null:Buffer.from(name),value.kind,literal);
  if(value.kind==="array")(value.value as Literal[]).forEach((child,ordinal)=>write(child,id,ordinal,null));
  if(value.kind==="object")(value.value as {key:string;value:Literal}[]).forEach((child,ordinal)=>write(child.value,id,ordinal,child.key));
 }
 write(fixture.value,null,0,null);const file=database.serialize();expect(Buffer.from(file).subarray(0,16).toString()).toBe("SQLite format 3\0");const reopened=Database.deserialize(file);
 try{
  type Row={id:number;name:Uint8Array|null;kind:string;literal:Uint8Array|null};
  function read(row:Row):Literal{
   const{kind,literal}=row;if(kind==="null")return{kind};
   if(kind==="array"||kind==="object"){const children=reopened.query("SELECT id,name,kind,literal FROM node WHERE parent=? ORDER BY ordinal").all(row.id) as Row[];return{kind,value:children.map(child=>kind==="array"?read(child):{key:Buffer.from(child.name!).toString(),value:read(child)})};}
   const text=literal===null?"":Buffer.from(literal).toString();return{kind,value:kind==="bool"?text==="true":kind==="float"||kind==="bytes"?Buffer.from(literal!).toString("hex"):text};
  }
  const produced=read(reopened.query("SELECT id,name,kind,literal FROM node WHERE parent IS NULL").get() as Row);expect(validator.validate(schema,produced)).toBe(true);expect(produced).toEqual(fixture.value);
  for(const word of fixture.binary64Words){const bytes=Buffer.from(word,"hex"),view=new DataView(bytes.buffer,bytes.byteOffset,bytes.byteLength);expect(view.getBigUint64(0,false).toString(16).padStart(16,"0")).toBe(word);}
  expect(BigInt("18446744073709551615")).toBe((1n<<64n)-1n);expect(BigInt("-9223372036854775808")).toBe(-(1n<<63n));
 }finally{reopened.close();database.close();}
});

test("intrinsic depth counts semantic array edges through the declared upper frontier",()=>{
 const database=new Database(":memory:");database.exec("CREATE TABLE chain(id INTEGER PRIMARY KEY,parent INTEGER,kind TEXT NOT NULL,word BLOB)");
 try{for(const row of depthFixture.cases){database.exec("DELETE FROM chain");const insert=database.query("INSERT INTO chain VALUES(?,?,?,?)");for(let edge=0;edge<=row.edges;edge++)insert.run(edge,edge===0?null:edge-1,edge===row.edges?"float":"array",edge===row.edges?Buffer.from(depthFixture.leafWord,"hex"):null);const file=database.serialize(),reopened=Database.deserialize(file);try{const measured=reopened.query("WITH RECURSIVE depth(id,n) AS (SELECT id,0 FROM chain WHERE parent IS NULL UNION ALL SELECT child.id,depth.n+1 FROM chain child JOIN depth ON child.parent=depth.id) SELECT MAX(n) AS edges FROM depth").get() as {edges:number};expect(measured.edges).toBe(row.edges);expect(measured.edges<=row.maxDepth).toBe(row.accepted);const leaf=reopened.query("SELECT word FROM chain WHERE kind='float'").get() as {word:Uint8Array};expect(Buffer.from(leaf.word).toString("hex")).toBe(depthFixture.leafWord);}finally{reopened.close();}}}finally{database.close();}
});
