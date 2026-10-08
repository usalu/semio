import refusalFixture from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/⚠️refusal/🧫️fixtures/🔣️.json";
const canceledKind=refusalFixture.cases.find(item=>item.id==="canceled-projection")!.expectedKind;
import {Database} from "bun:sqlite";
import {expect,test} from "bun:test";
import Ajv from "ajv";
import {buildSchema,graphqlSync} from "graphql";
import schema from "../../../../🧬️schema/📸️snapshot/🔣️.json";
import artifactSchema from "../../../../🧬️schema/🔣️.json";
import diffSchema from "../../../../🧬️schema/🔺️diff/🔣️.json";
import fixture from "../🧫️fixtures/🔣️.json";
import type {Mp3Snapshot} from "../../../../🧬️schema/📸️snapshot/🟦️.ts";
import {parseMp3Snapshot} from "../../../../🧬️schema/📸️snapshot/🟦️.ts";
import {parseMp3Diff} from "../../../../🧬️schema/🔺️diff/🟦️.ts";
import {MP3_SQLITE_SCHEMA,mp3SnapshotToSqliteDatabase,mp3SnapshotFromSqliteDatabase} from "../🟦️.ts";
import {exportSqliteDatabase,importSqliteDatabase} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

const emptyTrailer=()=>({title:"",artist:"",album:"",year:"",comment:"",track:null,genre:null});
test("MP3 semantic metadata exposes independent joined text and named trailer edits",async()=>{
 expect(new Ajv({strict:false}).validate(schema,fixture)).toBe(true);expect(MP3_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../🗄️.sql",import.meta.url)).text());const snapshot=structuredClone(fixture) as unknown as Mp3Snapshot;const database=await mp3SnapshotToSqliteDatabase(snapshot);expect(await mp3SnapshotFromSqliteDatabase(database)).toEqual(snapshot);const db=Database.deserialize(await exportSqliteDatabase(database));try{
 expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(db.query("SELECT f.frame_identifier,f.content_kind,v.value FROM mp3_id3v2_frame f JOIN mp3_id3_text_value v ON v.frame_id=f.id WHERE f.ordinal=0 AND v.ordinal=0").get()).toEqual({frame_identifier:"TIT2",content_kind:"text",value:"Café"});
 db.run("UPDATE mp3_id3_text_value SET value='edited 世界' WHERE frame_id=1 AND ordinal=0");db.run("UPDATE mp3_audio_frame SET emphasis=42 WHERE ordinal=1");db.run("UPDATE mp3_audio_payload_octet SET octet=17 WHERE frame_id=1 AND ordinal=2");db.run("UPDATE mp3_id3v1_tag SET title='Edited' WHERE id=1");if(snapshot.id3v2!.frames[0]!.content.kind!=="text")throw Error("neutral text role");snapshot.id3v2!.frames[0]!.content.values[0]="edited 世界";snapshot.frames[1]!.header.emphasis=42;snapshot.frames[0]!.payload[2]=17;snapshot.id3v1!.title="Edited";expect(await mp3SnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).toEqual(snapshot);
 }finally{db.close();}console.log("[DEBUG] Independent SQLite edits logical ID3 text/title rows; no ID3 framing/padding or encoding marker cells");
});
test("MP3 preserves absent and semantically empty optional metadata",async()=>{for(const present of [false,true]){const snapshot:Mp3Snapshot={schema:"",frames:[],id3v2:present?{frames:[]}:null,id3v1:present?emptyTrailer():null};expect(await mp3SnapshotFromSqliteDatabase(await importSqliteDatabase(await exportSqliteDatabase(await mp3SnapshotToSqliteDatabase(snapshot))))).toEqual(snapshot);}});
test("MP3 canonical parser and sparse typed tag changes agree with independent schema",()=>{
 expect<unknown>(parseMp3Snapshot(fixture)).toEqual(fixture);expect(new Ajv({strict:false}).validate(artifactSchema,fixture)).toBe(true);const ajv=new Ajv({strict:false}).addSchema(schema);for(const diff of [{},{id3v1:null,id3v2:fixture.id3v2,frames:[]}]){expect(ajv.validate(diffSchema,diff)).toBe(true);expect<unknown>(parseMp3Diff(diff)).toEqual(diff);}
 for(const edit of [(s:Mp3Snapshot)=>s.id3v1!.track=0,(s:Mp3Snapshot)=>s.id3v1!.genre=255,(s:Mp3Snapshot)=>s.frames[0]!.payload[0]=-1,(s:Mp3Snapshot)=>s.frames[0]!.header.layer=256]){const invalid=structuredClone(fixture) as unknown as Mp3Snapshot;edit(invalid);expect(new Ajv({strict:false}).validate(schema,invalid)).toBe(false);expect(()=>parseMp3Snapshot(invalid)).toThrow();}
});
test("MP3 refuses relational and semantic role corruption and supports cancellation",async()=>{
 const snapshot=fixture as unknown as Mp3Snapshot;const database=await mp3SnapshotToSqliteDatabase(snapshot);for(const sql of ["UPDATE mp3_id3v2_frame SET tag_id=99 WHERE id=1","UPDATE mp3_id3v2_frame SET ordinal=99 WHERE id=1","UPDATE mp3_id3v2_frame SET content_kind='opaque' WHERE id=1","UPDATE mp3_id3v2_frame SET language='eng' WHERE id=1","UPDATE mp3_audio_frame SET mpeg_version_id=256 WHERE id=1","UPDATE mp3_audio_frame SET protection_bit=2 WHERE id=1","UPDATE mp3_audio_payload_octet SET frame_id=99 WHERE id=1","UPDATE mp3_audio_payload_octet SET ordinal=99 WHERE id=1","UPDATE mp3_id3v1_tag SET track=0 WHERE id=1","DELETE FROM mp3_id3v2_tag"]){const db=Database.deserialize(await exportSqliteDatabase(database));try{db.run("PRAGMA ignore_check_constraints=ON");db.run(sql);await expect(mp3SnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow();}finally{db.close();}}
 await expect(mp3SnapshotToSqliteDatabase(snapshot,{maxRows:1})).rejects.toThrow();await expect(mp3SnapshotFromSqliteDatabase(database,{maxValueBytes:0})).rejects.toThrow();const large=structuredClone(snapshot);large.frames[0]!.payload=Array(2000).fill(255);const cancel=new AbortController();let reached=false;await expect(mp3SnapshotToSqliteDatabase(large,{signal:cancel.signal,onProgress:event=>{if(event.completed>=256){reached=true;cancel.abort();}}})).rejects.toHaveProperty("kind",canceledKind);expect(reached).toBe(true);
});
test("MP3 independent GraphQL returns typed metadata content",async()=>{
 const sdl=await Bun.file(new URL("../../../../🧬️schema/📸️snapshot/🔗️.graphql",import.meta.url)).text();const result=graphqlSync({schema:buildSchema("enum StateClass{ARTIFACT} directive @state(class:StateClass!) on FIELD_DEFINITION "+sdl+" type Query{snapshot:Mp3Snapshot!}"),source:"{snapshot{schema id3v2{frames{id content{... on Id3Text{kind values} ... on Id3Opaque{kind bytes}}}} id3v1{title artist album year comment track genre} frames{header{mpegVersionId layer protectionBit bitrateIndex sampleRateIndex padding privateBit channelMode modeExtension copyright original emphasis} payload}}}",rootValue:{snapshot:fixture},typeResolver:value=>"Id3"+(value.kind.charAt(0).toUpperCase()+value.kind.slice(1))});expect(result.errors).toBeUndefined();expect<unknown>(result.data?.snapshot).toEqual(fixture);
});

import Ajv2020 from "ajv/dist/2020.js";
import controls from "../🧫️fixtures/🎛️control.json";

import {independentSqliteExtent} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️tests/🔮️semantic-extent/🟦️.ts";
test("MP3 complete literal and optional tag extents agree with independent SQLite at copied limits",async()=>{
 expect(controls["largeOctetBytes"]).toEqual(196608);expect(controls["cancelAt"]).toEqual(256);expect(controls["maxOwnedBytes"]).toEqual(65536);
 for(const item of controls.semanticCells.cases){const source=structuredClone(fixture) as unknown as Mp3Snapshot;if(item.id==="absent"){source.frames=[];source.id3v1=null;source.id3v2=null;}else if(item.id==="presentEmpty"){source.frames=[];source.id3v2!.frames=[];source.id3v1=emptyTrailer();}const database=await mp3SnapshotToSqliteDatabase(source);console.log("[DEBUG] MP3 independent logical SQLite extent",item.id,JSON.stringify(independentSqliteExtent(await exportSqliteDatabase(database))));expect(independentSqliteExtent(await exportSqliteDatabase(database))).toEqual({rows:item.rows,valueBytes:item.valueBytes,schemaBytes:controls.semanticCells.schemaBytes,tableWidths:controls.semanticCells.tableWidths});const limits={maxRows:item.rows,maxValueBytes:item.valueBytes,maxSchemaBytes:controls.semanticCells.schemaBytes,maxTables:8,maxColumns:15};expect(await mp3SnapshotToSqliteDatabase(source,limits)).toEqual(database);
  for(const restricted of [{...limits,maxRows:item.rows-1},{...limits,maxValueBytes:item.valueBytes-1},{...limits,maxSchemaBytes:limits.maxSchemaBytes-1},{...limits,maxTables:7},{...limits,maxColumns:14}])await expect(mp3SnapshotToSqliteDatabase(source,restricted)).rejects.toThrow();
 }console.log("[DEBUG] independent SQLite measures every MP3 header, tag field and ordered INTEGER octet role");
});
