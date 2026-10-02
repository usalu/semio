/** 📸️ Cross-language point-buffer baselines checked with an independent schema engine. */
import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import Ajv from "ajv";
import laws from "../../🧫️fixtures/🪶️sqlite/🔣️.json";
import * as owner from "../../🟦️.ts";
test("SQLite remodeling authored semantic tables execute in an independent engine",()=>{const db=new Database(":memory:");try{db.run(readFileSync(new URL("../../🪶️sqlite/🗄️.sql",import.meta.url),"utf8"));expect(db.query("SELECT count(*) AS n FROM sqlite_schema WHERE type='table'").get()).toEqual({n:35});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(db.query<{name:string},[]>("PRAGMA table_info(remodel_float_sample)").all().map(row=>row.name)).toEqual(["id","buffer_id","ordinal","sample","sample_bits","sample_class"]);}finally{db.close()}});
test("SQLite remodeling inline buffers follow the independent tagged schema",()=>{const admits=new Ajv({strict:true}).compile(laws.bufferSchema);for(const buffer of laws.inlineBuffers){expect(admits(buffer)).toBe(true);const snapshot=JSON.parse(owner.remodelingSnapshotToJsonText(owner.defaultRemodelingSnapshot()));const json={...snapshot,results:{...snapshot.results,sparse:{points:buffer,colors:null}}};expect(owner.decodeRemodelingSnapshot(json).results.sparse?.points).toEqual({kind:"inline",values:buffer.values.map(value=>{const bytes=Buffer.alloc(4);bytes.writeFloatBE(value);return{bits:bytes.readUInt32BE()}})});}});
test("SQLite remodeling durable chunks contain literal octets",()=>{const snapshot=JSON.parse(owner.remodelingSnapshotToJsonText(owner.defaultRemodelingSnapshot()));const json={...snapshot,durableArtifacts:{"literal\u0000😀":{kind:"sparse",mime:null,width:0,height:0,chunks:laws.byteChunks}}};expect(owner.decodeRemodelingSnapshot(json).durableArtifacts["literal\u0000😀"]?.chunks).toEqual(laws.byteChunks.map(value=>Uint8Array.from(value)));});
test("SQLite remodeling declares its owned public relational entry",()=>{expect(typeof Reflect.get(owner,"remodelingSnapshotToSqliteDatabase")).toBe("function");});

test("SQLite remodeling declared JSON admits every exact IEEE word",()=>{for(const bits of laws.binary64Words){const snapshot=JSON.parse(owner.remodelingSnapshotToJsonText(owner.defaultRemodelingSnapshot()));const restored=owner.decodeRemodelingSnapshot({...snapshot,streams:[{id:"s",syncOffsetMs:{bits}}]});expect(restored.streams[0]?.syncOffsetMs).toEqual({bits:BigInt("0x"+bits)});expect(JSON.parse(owner.remodelingSnapshotToJsonText(restored)).streams[0].syncOffsetMs).toEqual({bits});}for(const bits of laws.binary32Words){const snapshot=JSON.parse(owner.remodelingSnapshotToJsonText(owner.defaultRemodelingSnapshot()));const restored=owner.decodeRemodelingSnapshot({...snapshot,results:{...snapshot.results,sparse:{points:{kind:"inline",values:[{bits}]},colors:null}}});expect(restored.results.sparse?.points).toEqual({kind:"inline",values:[{bits:Number.parseInt(bits,16)}]});}});
test("SQLite remodeling retained content references own full unsigned64 decimal words",()=>{for(const count of laws.unsigned64Decimals){const snapshot=JSON.parse(owner.remodelingSnapshotToJsonText(owner.defaultRemodelingSnapshot()));const restored=owner.decodeRemodelingSnapshot({...snapshot,results:{...snapshot.results,sparse:{points:{kind:"content",contentId:"literal\u0000😀",chunkCount:count},colors:null}}});expect(restored.results.sparse?.points).toEqual({kind:"content",contentId:"literal\u0000😀",chunkCount:BigInt(count)});}});

import {exportSqliteDatabase,importSqliteDatabase} from '../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts';

test("SQLite remodeling owns every semantic table through an independent physical engine",async()=>{const snapshot=owner.decodeRemodelingSnapshot(laws.snapshotJson),database=await owner.remodelingSnapshotToSqliteDatabase(snapshot),bytes=await exportSqliteDatabase(database),oracle=Database.deserialize(bytes);try{expect(oracle.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(oracle.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(oracle.query("SELECT count(*) AS n FROM remodel_float_sample").get()).toEqual({n:9});expect(oracle.query("SELECT chunk_count FROM remodel_float_buffer WHERE storage='content'").get()).toEqual({chunk_count:"18446744073709551615"});expect(oracle.query("SELECT kind FROM remodel_durable_artifact").get()).toEqual({kind:"arbitrary retained native string"});expect(oracle.query("SELECT hex(octets) AS octets FROM remodel_durable_chunk ORDER BY ordinal").all()).toEqual([{octets:"00017F80FF"},{octets:""}]);const restored=await owner.remodelingSnapshotFromSqliteDatabase(await importSqliteDatabase(oracle.serialize()));expect(restored).toEqual(snapshot);for(const table of database.tables)expect(table.rows.length).toBeGreaterThan(0);}finally{oracle.close()}},15000);
test("SQLite remodeling accepts consistent edited surrogate aliases and independent IEEE words",async()=>{const snapshot=owner.decodeRemodelingSnapshot(laws.snapshotJson),bytes=await exportSqliteDatabase(await owner.remodelingSnapshotToSqliteDatabase(snapshot)),oracle=Database.deserialize(bytes);try{oracle.run("PRAGMA foreign_keys=OFF");oracle.run("UPDATE remodel_document SET id=21");for(const table of["remodel_calibration","remodel_parameters","remodel_results","remodel_asset","remodel_durable_artifact","remodel_stream","remodel_ground_control_point"])oracle.run("UPDATE "+table+" SET document_id=21");oracle.run("UPDATE remodel_float_sample SET sample=NULL,sample_bits=4286653253,sample_class='nan' WHERE ordinal=0");expect(oracle.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(oracle.query("PRAGMA foreign_key_check").all()).toEqual([]);const restored=await owner.remodelingSnapshotFromSqliteDatabase(await importSqliteDatabase(oracle.serialize()));expect(restored.results.sparse?.points).toEqual({kind:"inline",values:[{bits:4286653253},...laws.binary32Words.slice(1).map(bits=>({bits:Number.parseInt(bits,16)}))]});}finally{oracle.close()}},15000);
test("SQLite remodeling refuses independently edited orphan and IEEE mismatches",async()=>{const snapshot=owner.decodeRemodelingSnapshot(laws.snapshotJson),bytes=await exportSqliteDatabase(await owner.remodelingSnapshotToSqliteDatabase(snapshot));for(const sql of["UPDATE remodel_camera_distortion SET camera_id=999","UPDATE remodel_float_sample SET sample_class='finite' WHERE ordinal=6","UPDATE remodel_frame SET ordinal=1","UPDATE remodel_watertight_report SET mesh_id=999 WHERE mesh_id IS NOT NULL"]){const oracle=Database.deserialize(bytes);try{oracle.run("PRAGMA foreign_keys=OFF");oracle.run(sql);expect(oracle.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});await expect(owner.remodelingSnapshotFromSqliteDatabase(await importSqliteDatabase(oracle.serialize()))).rejects.toThrow();}finally{oracle.close()}}},15000);

import documentSchema from "../../../🔣️.json";
import {remodelingSnapshotFromDslText} from "../../../../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🟦️.ts";
test("SQLite remodeling exact document words obey the independent authored JSON schema",()=>{
 const validator=new Ajv({strict:false,validateFormats:false}).compile(documentSchema);
 const wire=JSON.parse(owner.remodelingSnapshotToJsonText(owner.decodeRemodelingSnapshot(laws.snapshotJson)));
 expect(validator(wire)).toBe(true);
 for(const chunkCount of laws.unsigned64Decimals){const candidate=structuredClone(wire);candidate.results.sparse.points={kind:"content",contentId:"literal",chunkCount};expect(validator(candidate)).toBe(true)}
 for(const chunkCount of ["18446744073709551616","-1","00","+1","1.0"]){const candidate=structuredClone(wire);candidate.results.sparse.points={kind:"content",contentId:"literal",chunkCount};expect(validator(candidate)).toBe(false)}
 for(const signed of laws.signed64Decimals){const candidate=structuredClone(wire);candidate.results.mesh.watertight.eulerCharacteristic=signed;expect(validator(candidate)).toBe(true)}
 for(const signed of ["9223372036854775808","-9223372036854775809","-0","00"]){const candidate=structuredClone(wire);candidate.results.mesh.watertight.eulerCharacteristic=signed;expect(validator(candidate)).toBe(false)}
});
test("SQLite remodeling projection and restoration obey exact domain row admission",async()=>{
 const snapshot=owner.decodeRemodelingSnapshot(laws.snapshotJson),database=await owner.remodelingSnapshotToSqliteDatabase(snapshot),count=database.tables.reduce((sum,table)=>sum+table.rows.length,0);
 expect(await owner.remodelingSnapshotToSqliteDatabase(snapshot,{maxRows:count})).toEqual(database);
 await expect(owner.remodelingSnapshotToSqliteDatabase(snapshot,{maxRows:count-1})).rejects.toThrow();
 expect(await owner.remodelingSnapshotFromSqliteDatabase(database,{maxRows:count})).toEqual(snapshot);
 await expect(owner.remodelingSnapshotFromSqliteDatabase(database,{maxRows:count-1})).rejects.toThrow();
 const abort=new AbortController();abort.abort();await expect(owner.remodelingSnapshotToSqliteDatabase(snapshot,{signal:abort.signal})).rejects.toThrow();await expect(owner.remodelingSnapshotFromSqliteDatabase(database,{signal:abort.signal})).rejects.toThrow();
});
test("SQLite remodeling declared Text retains primitive exact words and signed64 values",()=>{
 const text='semio remodeling.remodeling.dsl v1\nschema="literal" id="retained"\nparams { ingest { min-sharpness=nan32_7f800001 } }\nresults { qc { reprojection-rms-px=nan64_7ff0000000000001 watertight { euler-characteristic=-9223372036854775808 genus=9223372036854775807 signed-volume=-0 } } }';
 const restored=remodelingSnapshotFromDslText(text);expect(restored.params.ingest.minSharpness).toEqual({bits:0x7f800001});expect(restored.results.qc?.reprojectionRmsPx).toEqual({bits:0x7ff0000000000001n});expect(restored.results.qc?.watertight?.eulerCharacteristic).toBe(-9223372036854775808n);expect(restored.results.qc?.watertight?.genus).toBe(9223372036854775807n);expect(restored.results.qc?.watertight?.signedVolume).toEqual({bits:0x8000000000000000n});
});

test("SQLite remodeling declared mutation schemas share exact words and raw octets",()=>{
 const schemaRoot=new URL("../../../",import.meta.url),schemas=[['⏱️change-stream-sync',{mutation:'changeStreamSync',id:'s',newSyncOffsetMs:{bits:'7ff0000000000001'}}],['📦append-content',{mutation:'appendContent',contentId:'c',kind:'sparse',mime:null,width:0,height:0,first:'18446744073709551615',chunks: laws.byteChunks}],['🔪remove-content',{mutation:'removeContent',contentId:'c',from:'18446744073709551615'}]] as const;
 const ajv=new Ajv({strict:false,validateFormats:false});ajv.addSchema(documentSchema);
 for(const[slug,value]of schemas){const json=JSON.parse(readFileSync(new URL('🧬️mutations/'+slug+'/🧬️schema/🔣️.json',schemaRoot),'utf8'));const valid=ajv.compile(json);expect(valid(value)).toBe(true)}
});
test("SQLite remodeling declared Text owns tagged buffers octets and literal references",()=>{
 const text='semio remodeling.remodeling.dsl v1\nschema="literal" id="retained"\nassets={"__proto__"{child_id="local" target { artifact-id="" artifact-kind="literal!@/" standard="" subset="*" }}}\nresults { sparse { points { inline values=[nan32_7f800001 -0] } colors="AAF/gP8=" } dense { positions { content content-id="unresolved" chunk-count=18446744073709551615 } } }';
 const restored=remodelingSnapshotFromDslText(text);expect(restored.results.sparse?.points).toEqual({kind:'inline',values:[{bits:0x7f800001},{bits:0x80000000}]});expect(restored.results.sparse?.colors).toEqual(Uint8Array.from(Buffer.from('AAF/gP8=','base64')));expect(restored.results.dense?.positions).toEqual({kind:'content',contentId:'unresolved',chunkCount:18446744073709551615n});expect(Object.hasOwn(restored.assets,'__proto__')).toBe(true);expect(restored.assets['__proto__']).toEqual({childId:'local',target:{artifactId:'',dialect:{artifactKind:'literal!@/',standard:'',subset:'*'}}});
});
test("SQLite remodeling native Text binary32 widening is exact and refuses lost payload",()=>{
 for(const value of laws.nativeTextWordCases){const oracle=new DataView(new ArrayBuffer(8));oracle.setBigUint64(0,BigInt('0x'+value.widenedBinary64Bits));expect(Number.isNaN(oracle.getFloat64(0))).toBe(true);const restored=remodelingSnapshotFromDslText('semio remodeling.remodeling.dsl v1\nschema="literal" id="x"\nparams { ingest { min-sharpness=nan64_'+value.widenedBinary64Bits+' } }');expect(restored.params.ingest.minSharpness).toEqual({bits:Number.parseInt(value.binary32Bits,16)})}
 expect(()=>remodelingSnapshotFromDslText('semio remodeling.remodeling.dsl v1\nschema="literal" id="x"\nparams { ingest { min-sharpness=nan64_7ff0000020000001 } }')).toThrow('binary32');
});

import {readdirSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {join} from 'node:path';
test("SQLite remodeling every committed snapshot fixture owns the canonical document domain",()=>{
 const root=fileURLToPath(new URL('../../../../🧫️fixtures/🧬️mutations/',import.meta.url));let count=0;const errors:string[]=[];
 const validator=new Ajv({strict:false,validateFormats:false}).compile(documentSchema);
 for(const slug of readdirSync(root))for(const name of readdirSync(join(root,slug)))for(const direction of ['⬅️before','➡️after']){const file=join(root,slug,name,'📸️snapshot',direction,'🔣️.json'),wire=JSON.parse(readFileSync(file,'utf8'));count++;try{const value=owner.decodeRemodelingSnapshot(wire);expect(validator(JSON.parse(owner.remodelingSnapshotToJsonText(value)))).toBe(true)}catch(error){errors.push(slug+'/'+name+'/'+direction+': '+String(error))}}
 expect(count).toBe(272);expect(errors.slice(0,8)).toEqual([]);
});
import {decodeRemodelingMutation,applyRemodelingMutation} from '../../../🧬️mutations/🟦️.ts';
import {decodeRemodelingDiff,applyRemodelingDiff} from '../../../🔺️diff/🟦️.ts';
import {existsSync} from 'node:fs';
test("SQLite remodeling committed mutation and diff replay preserves typed snapshots",()=>{
 const root=fileURLToPath(new URL('../../../../🧫️fixtures/🧬️mutations/',import.meta.url));let count=0;const errors:string[]=[];
 for(const slug of readdirSync(root))for(const name of readdirSync(join(root,slug))){const dir=join(root,slug,name),read=(path:string)=>JSON.parse(readFileSync(join(dir,path),'utf8'));count++;try{const before=owner.decodeRemodelingSnapshot(read('📸️snapshot/⬅️before/🔣️.json')),after=owner.decodeRemodelingSnapshot(read('📸️snapshot/➡️after/🔣️.json')),mutation=decodeRemodelingMutation(read('🦠️mutation/🔣️.json'));expect(applyRemodelingMutation(before,mutation)).toEqual(after);if(existsSync(join(dir,'🔺️diff/🔣️.json')))expect(applyRemodelingDiff(decodeRemodelingDiff(read('🔺️diff/🔣️.json')),before)).toEqual(after)}catch(error){errors.push(slug+'/'+name+': '+String(error))}}
 expect(count).toBe(136);expect(errors.slice(0,8)).toEqual([]);
});
test("SQLite remodeling declared JSON word objects agree with the independent pretty writer",()=>{const actual=owner.remodelingSnapshotToJsonText(owner.decodeRemodelingSnapshot(laws.snapshotJson));expect(actual).toBe(JSON.stringify(JSON.parse(actual),null,2));});
test("SQLite remodeling semantic relationship construction reports its known cancellable frontier",async()=>{
 const wire=JSON.parse(owner.remodelingSnapshotToJsonText(owner.defaultRemodelingSnapshot()));wire.streams=[{id:'retained',frames:Array.from({length:1024},(_,index)=>({index,timestampMs:0,assetId:'unresolved'}))}];const database=await owner.remodelingSnapshotToSqliteDatabase(owner.decodeRemodelingSnapshot(wire)),oracle=Database.deserialize(await exportSqliteDatabase(database));try{expect(oracle.query('SELECT count(*) AS n FROM remodel_frame').get()).toEqual({n:1024});}finally{oracle.close()}
 const controller=new AbortController();let canceled=false;await expect(owner.remodelingSnapshotFromSqliteDatabase(database,{signal:controller.signal,onProgress:event=>{if(event.phase==='reconstructSnapshot'&&event.total===1024&&event.completed===256){canceled=true;controller.abort()}}})).rejects.toThrow();expect(canceled).toBe(true);
},15000);
test("SQLite remodeling committed JSON snapshot spelling matches its declared exact word writer",()=>{
 const root=fileURLToPath(new URL('../../../../🧫️fixtures/🧬️mutations/',import.meta.url)),errors:string[]=[];let count=0;
 for(const slug of readdirSync(root))for(const name of readdirSync(join(root,slug)))for(const direction of['⬅️before','➡️after']){const file=join(root,slug,name,'📸️snapshot',direction,'🔣️.json'),text=readFileSync(file,'utf8').trimEnd();count++;if(owner.remodelingSnapshotToJsonText(owner.decodeRemodelingSnapshot(JSON.parse(text)))!==text)errors.push(slug+'/'+name+'/'+direction)}expect(count).toBe(272);expect(errors.slice(0,8)).toEqual([]);
});

test("SQLite remodeling committed native Text assets preserve their independently authored frame identities",()=>{
 const base=new URL('../../../../',import.meta.url),errors:string[]=[];
 for(const path of['🖼️assets/🎬️demo/🗣️.dsl.semio','📚️examples/🛰️synthetic-orbit/🖼️assets/🗣️.dsl.semio'])try{
  const restored=remodelingSnapshotFromDslText(readFileSync(new URL(path,base),'utf8'));
  expect(restored.results.mesh.mesh.target).toEqual({artifactId:'remodeling-mesh-constant:box',dialect:{artifactKind:'s.stdio.semio',standard:'v1',subset:'mesh'}});
  if(path.startsWith('📚️examples')){const truth=JSON.parse(readFileSync(new URL('📚️examples/🛰️synthetic-orbit/🖼️assets/🔮️ground-truth.json',base),'utf8'));expect(restored.streams[0]?.frames.map(row=>row.assetId)).toEqual(truth.frames.map((row:{assetId:string})=>row.assetId));}
 }catch(error){errors.push(path+': '+String(error))}
 expect(errors).toEqual([]);
});
