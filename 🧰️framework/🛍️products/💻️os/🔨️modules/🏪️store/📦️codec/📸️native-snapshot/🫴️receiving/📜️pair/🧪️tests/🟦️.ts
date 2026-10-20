import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import fixture from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";

/** 📜️ Both received roots retain the published document identity without replacing it with an active editor. */
test("original Pack and SPR pair source laws use exact neutral document identity",()=>{
 const validate=new Ajv({strict:true}).compile(schema);expect(validate(fixture)).toBe(true);
 const db=new Database(":memory:");try{const row=db.query("SELECT json_extract(?, '$.id') AS id, json_extract(?, '$.schema') AS schema, json_array_length(?, '$.edits') AS edits, json_array_length(?, '$.transitions') AS transitions").get(...Array(4).fill(JSON.stringify(fixture.document))) as {id:string;schema:string;edits:number;transitions:number};expect(row).toEqual({id:fixture.document.id,schema:fixture.document.schema,edits:0,transitions:0});
  for(const changed of [{...fixture.document,id:"open-editor"},{...fixture.document,schema:"different.revision"}])expect(validate({...fixture,document:changed})).toBe(false);
  expect(Buffer.from(row.id).equals(new TextEncoder().encode(fixture.document.id))).toBe(true);
 }finally{db.close()}
 console.log("[DEBUG] original pair strictSchema=true actualNativeInputs=true independentSQLite=true exactUnicodeIdentity=true editorSubstitutionRefused=true");
});

import {Blake3Hasher} from "@semio-tech/framework-hash";
import vectors from "@semio-tech/framework-hash/testing/fixtures/blake3.json";

/** 🔏️ Native Pair genesis digest uses the same bounded first-party hash with independent BLAKE3-team vectors. */
test("original Pack digest bounded input chunks match independent published vectors",()=>{
 for(const row of vectors.vectors){const bytes=Uint8Array.from({length:row.length},(_,index)=>index%251);for(const quantum of [1,3,4096]){const hasher=new Blake3Hasher();for(let at=0;at<bytes.length;at+=quantum)hasher.update(bytes.subarray(at,Math.min(bytes.length,at+quantum)));expect(Buffer.from(hasher.digest()).toString("hex")).toBe(row.hash);}}
 console.log("[DEBUG] original Pair digest bounded1_3_4096=true firstPartyHasher=true independentBlake3TeamVectors=true");
});
