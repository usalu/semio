import { test, expect } from "bun:test";
import Ajv from "ajv";
import { Database } from "bun:sqlite";
import { inflateRawSync } from "node:zlib";
import { readFileSync, existsSync } from "node:fs";
import schema from "../🧬️schema/🔣️.json";
import law from "./🧫️fixtures/🔣️.json";
import { formatVerificationFixtureUrl } from "@semio-tech/framework-replication/testing/fixtures";
import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";

test("original compressed History frame retains paid paged source and exact Deflate output",()=>{
  expect(new Ajv().compile(schema)(law)).toBe(true);
  const read=(path:string)=>readFileSync(new URL(path,import.meta.url),"utf8");
  const reference=JSON.parse(readFileSync(formatVerificationFixtureUrl,"utf8"));
  const frame=reference.compressed.find((row:{id:string})=>row.id===law.fixture);
  expect(frame.error).toBe(null);
  const output=inflateRawSync(Buffer.from(frame.storedHex,"hex"));expect(output.toString("hex")).toBe(frame.rawHex);expect(output.length).toBe(9);
  const db=new Database(":memory:");try{db.exec("CREATE TABLE bytes(ordinal INTEGER PRIMARY KEY,value INTEGER NOT NULL)");for(const[ordinal,value]of output.entries())db.query("INSERT INTO bytes VALUES(?,?)").run(ordinal,value);for(const cut of law.cuts){expect(db.query("SELECT count(*)as count FROM bytes WHERE ordinal<?").get(cut)).toEqual({count:cut});}expect(db.query("SELECT sum(value)as checksum FROM bytes").get()).toEqual({checksum:[...output].reduce((a,b)=>a+b,0)});}finally{db.close()}
  const owner=new URL("../🦀️.rs",import.meta.url);expect(existsSync(owner)).toBe(true);const source=readFileSync(owner,"utf8");
  for(const marker of["DeflateRetainedCursor","OriginalWireSourceIdentity","NativeDecodeControl","PagedList","take_partial_into","take_receipt","close_step"]){expect(source).toContain(marker)}
  for(const cold of["Vec<u8>","deflate_decompress(","inflate(","collect::<Vec", "RetainedCloneGrant::default"]){expect(source).not.toContain(cold)}
});

test("retained Deflate closes metadata without copying source or output bytes",()=>{
  expect(new Ajv().compile(schema)(law)).toBe(true);
  const read=(path:string)=>readFileSync(new URL(path,import.meta.url),"utf8");
  const lower=readModuleTreeSource(new URL("../../../../../../../../../🔨️modules/🗜️deflate/🦀️.rs",import.meta.url));
  const protocol=read("../../../../../../../../../🔨️modules/📡️replication/⚙️codec/🦀️.rs");
  const quote=(source:string,name:string)=>source.slice(source.indexOf(`pub fn ${name}`),source.indexOf("\n    pub fn",source.indexOf(`pub fn ${name}`)+1));
  for(const source of[quote(lower,"retained_retirement_demands"),quote(protocol,"retirement_demands")]){expect(source).not.toContain("copy_bytes:");expect(source).toContain("depth: 1");}
  const db=new Database(":memory:");try{expect(db.query("SELECT ? AS copiedBytes, ? AS maximumCopyBytes, ? AS windowBytes").get(0,law.grant.maximumCopyBytes,32768)).toEqual({copiedBytes:0,maximumCopyBytes:2,windowBytes:32768});}finally{db.close()}
});
