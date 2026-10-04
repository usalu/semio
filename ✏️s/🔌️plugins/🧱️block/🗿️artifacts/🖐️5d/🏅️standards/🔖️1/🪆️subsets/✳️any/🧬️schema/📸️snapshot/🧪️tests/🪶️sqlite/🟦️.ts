/** 🧱️ Exact Block5d canonical parent laws share independent neutral primitive evidence. */
import {expect,test} from 'bun:test';
import Ajv from 'ajv';
import laws from '../../🧫️fixtures/🪶️sqlite/🔣️.json';
import * as artifact from '../../../🟦️.ts';
import * as snapshot from '../../🟦️.ts';
function specimen(bits:bigint):snapshot.Block5dSnapshot{const word={bits};return{schema:'literal\0世界',partKind:{id:'',name:'',label:'',variant:null,description:'',icon:null,unit:null},part2d:{shape:null,radius:word,width:null,height:word,color:null,iconKind:null},part3d:{orientation:[word,word,word,word],scale:[word,word,word]},representations:[{id:'',name:'',meshUrl:null,tags:['',''],lod:null,description:'',attributes:[{key:'',value:'',definition:null}]}],gripKinds:[{id:'',name:'',label:'',color:'',defaultRopeKind:'unresolved'}],grips:[{id:'',gripKind:'unresolved',angle:word,radius2d:word,position:[word,word,word],direction:[word,word,word],radius3d:word}],compatibility:[{id:'',source:'unresolved',target:'',bidirectional:false}],attributes:[{key:'',value:'',definition:null}],authors:[{id:'',name:'',email:null}],camera2d:{x:word,y:word,zoom:word},camera3d:{position:[word,word,word],target:[word,word,word],zoom:word},meta:{description:'literal\0😀'}}}
test('SQLite Block5d neutral full IEEE words agree with independent Ajv and DataView',()=>{
 const validate=new Ajv({strict:true}).compile(laws.wordSchema);for(const raw of laws.binary64Bits){expect(validate({bits:raw})).toBe(true);const bytes=Buffer.from(raw,'hex');expect(new DataView(bytes.buffer,bytes.byteOffset,8).getBigUint64(0)).toBe(BigInt('0x'+raw))}
});
test('SQLite Block5d canonical geometry rejects raw numeric alternatives',()=>{
 expect(()=>artifact.parseBlock5dPart2d({shape:null,radius:0,width:null,height:null,color:null,iconKind:null})).toThrow();
 expect(()=>artifact.parseBlockCamera2d({x:0,y:0,zoom:1})).toThrow();
});
test('SQLite Block5d domain records refuse incomplete native required fields',()=>{
 expect(()=>artifact.parseBlock5dGripKind({id:''})).toThrow();expect(()=>artifact.parseBlockRepresentation({id:''})).toThrow();expect(()=>artifact.parseBlockCamera3d({})).toThrow();
});
test('SQLite Block5d owns one complete canonical persisted parent parser',()=>{
 const parse=Reflect.get(snapshot,'parseBlock5dSnapshot') as ((v:unknown)=>unknown)|undefined;expect(typeof parse).toBe('function');
 for(const raw of laws.binary64Bits){const value=specimen(BigInt('0x'+raw));expect(parse!(value)).toEqual(value);expect(artifact.parseBlock5dArtifact(value)).toEqual(snapshot.parseBlock5dSnapshot(value))}
});

import {Database} from 'bun:sqlite';
test('SQLite Block5d handwritten tables satisfy the independent native schema oracle',async()=>{
 const sql=await Bun.file(new URL('../../🪶️sqlite/🗄️.sql',import.meta.url)).text(),db=new Database(':memory:');try{db.exec(sql);expect(db.query('PRAGMA integrity_check').values()).toEqual([['ok']]);expect(db.query('PRAGMA foreign_key_check').values()).toEqual([]);expect(db.query("SELECT name FROM sqlite_schema WHERE type='table' ORDER BY name").values().map(r=>r[0])).toEqual(Object.keys(laws.tableRowCounts).sort());expect(db.query('PRAGMA table_info(block5_grip)').all().length).toBe(32)}finally{db.close()}
});
test('SQLite Block5d public provider must retain the complete every-entity parent',async()=>{
 const project=Reflect.get(artifact,'block5dSnapshotToSqliteDatabase') as ((v:snapshot.Block5dSnapshot)=>Promise<unknown>)|undefined;
 const restore=Reflect.get(artifact,'block5dSnapshotFromSqliteDatabase') as ((v:unknown)=>Promise<unknown>)|undefined;
 expect(typeof project).toBe('function');expect(typeof restore).toBe('function');
 const value=snapshot.parseBlock5dSnapshot(specimen(BigInt('0x'+laws.binary64Bits[0]!)));expect(await restore!(await project!(value))).toEqual(value);
});

import {exportSqliteDatabase,importSqliteDatabase,type SqliteDatabase} from '../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts';

async function oracle(value:snapshot.Block5dSnapshot):Promise<Database>{return Database.deserialize(await exportSqliteDatabase(await artifact.block5dSnapshotToSqliteDatabase(value)))}
for(const raw of laws.binary64Bits)test('SQLite Block5d independent SQLite reserialization exact word '+raw,async()=>{
 const value=specimen(BigInt('0x'+raw)),db=await oracle(value);try{
  expect(db.query('PRAGMA integrity_check').values()).toEqual([['ok']]);expect(db.query('PRAGMA foreign_key_check').values()).toEqual([]);
  for(const [name,count] of Object.entries(laws.tableRowCounts))expect(db.query('SELECT count(*) FROM '+name).values()[0]![0]).toBe(count);
  const query=db.query('SELECT angle_ieee754_bits FROM block5_grip') as unknown as {safeIntegers(v:boolean):void;values():unknown[][]};query.safeIntegers(true);expect(query.values()).toEqual([[BigInt.asIntN(64,BigInt('0x'+raw))]]);
  expect(db.query('SELECT r.native_id,g.grip_kind FROM block5_representation r JOIN block5_grip g ON r.document_id=g.document_id').values()).toEqual([['','unresolved']]);
  expect(await artifact.block5dSnapshotFromSqliteDatabase(await importSqliteDatabase(db.serialize()))).toEqual(value);
 }finally{db.close()}
});
test('SQLite Block5d SQL edits change literal semantic entities without resolving native references',async()=>{
 const value=specimen(0n),db=await oracle(value);try{db.exec("UPDATE block5_grip SET grip_kind='other unresolved'; UPDATE block5_representation_tag SET value='edited' WHERE ordinal=1");const query=db.query('UPDATE block5_meta SET description=?');query.run('edited\0😀');value.grips[0]!.gripKind='other unresolved';value.representations[0]!.tags[1]='edited';value.meta.description='edited\0😀';expect(await artifact.block5dSnapshotFromSqliteDatabase(await importSqliteDatabase(db.serialize()))).toEqual(value)}finally{db.close()}
});
test('SQLite Block5d structural identities may be independently consistently renumbered',async()=>{
 const value=specimen(0x8000000000000000n),db=await oracle(value);try{
  db.exec('PRAGMA foreign_keys=OFF');for(const name of Object.keys(laws.tableRowCounts))db.exec('UPDATE '+name+' SET id=id+1000');
  db.exec('UPDATE block5_kind SET document_id=document_id+1000; UPDATE block5_part2d SET document_id=document_id+1000; UPDATE block5_part3d SET document_id=document_id+1000; UPDATE block5_representation SET document_id=document_id+1000; UPDATE block5_grip_kind SET document_id=document_id+1000; UPDATE block5_grip SET document_id=document_id+1000; UPDATE block5_compatibility SET document_id=document_id+1000; UPDATE block5_attribute SET document_id=document_id+1000; UPDATE block5_author SET document_id=document_id+1000; UPDATE block5_camera2d SET document_id=document_id+1000; UPDATE block5_camera3d SET document_id=document_id+1000; UPDATE block5_meta SET document_id=document_id+1000; UPDATE block5_representation_tag SET representation_id=representation_id+1000; UPDATE block5_representation_attribute SET representation_id=representation_id+1000');
  expect(db.query('PRAGMA foreign_key_check').values()).toEqual([]);expect(await artifact.block5dSnapshotFromSqliteDatabase(await importSqliteDatabase(db.serialize()))).toEqual(value);
 }finally{db.close()}
});
for(const sql of laws.malformedSql)test('SQLite Block5d malformed independent edit '+sql,async()=>{
 const db=await oracle(specimen(0x7ff8000000000001n));try{db.exec(sql);expect(db.query('PRAGMA integrity_check').values()).toEqual([['ok']]);const file=await importSqliteDatabase(db.serialize());await expect(artifact.block5dSnapshotFromSqliteDatabase(file)).rejects.toThrow()}finally{db.close()}
});
test('SQLite Block5d native absence differs from present NaN and empty arrays remain exact',async()=>{
 const value=specimen(0x7ff0000000000001n);value.part2d.radius=null;value.part3d.orientation=null;value.part3d.scale=null;value.representations=[];value.gripKinds=[];value.grips=[];value.compatibility=[];value.attributes=[];value.authors=[];
 const db=await artifact.block5dSnapshotToSqliteDatabase(value);expect(await artifact.block5dSnapshotFromSqliteDatabase(db)).toEqual(value);
});
test('SQLite Block5d schema and complete row limits admit before semantic ownership',async()=>{
 const value=specimen(0n),rows=Object.values(laws.tableRowCounts).reduce((a,n)=>a+n,0);await expect(artifact.block5dSnapshotToSqliteDatabase(value,{maxRows:rows-1})).rejects.toThrow('row limit');expect(await artifact.block5dSnapshotFromSqliteDatabase(await artifact.block5dSnapshotToSqliteDatabase(value,{maxRows:rows}),{maxRows:rows})).toEqual(value);
 for(const restore of [false,true]){const options={maxSchemaBytes:1};await expect(restore?artifact.block5dSnapshotFromSqliteDatabase(await artifact.block5dSnapshotToSqliteDatabase(value),options):artifact.block5dSnapshotToSqliteDatabase(value,options)).rejects.toThrow('schema');}
});
test('SQLite Block5d projection and reconstruction provide interior known work cancellation',async()=>{
 const value=specimen(0n);value.grips=Array.from({length:1024},(_,i)=>({...value.grips[0]!,id:String(i)}));
 const db=await artifact.block5dSnapshotToSqliteDatabase(value);for(const restore of [false,true]){const controller=new AbortController();let known=false;const options={signal:controller.signal,onProgress:(p:{completed:number;total:number})=>{if(p.total>512&&p.completed>=256&&p.completed<p.total){known=true;controller.abort()}}};await expect(restore?artifact.block5dSnapshotFromSqliteDatabase(db,options):artifact.block5dSnapshotToSqliteDatabase(value,options)).rejects.toThrow();expect(known).toBe(true)}
});

import {block5dToJsonText} from '../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🟦️.ts';

import {block5dFromJsonText} from '../../../../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🟦️.ts';

test('SQLite Block5d actual declared JSON boundary preserves every native IEEE word',()=>{
 for(const raw of laws.binary64Bits){const value=specimen(BigInt('0x'+raw)),text=block5dToJsonText(value),wire=JSON.parse(text);expect(wire.grips[0].angle).toEqual({bits:raw});expect(block5dFromJsonText(text)).toEqual(value);}
});

import * as two from '../../../../../../../../../◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts';

import * as three from '../../../../../../../../../🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts';

test('SQLite shared Block2d snapshot uses the same exact common word and required record authority',()=>{
 const parse=Reflect.get(two,'parseBlock2dSnapshot') as ((v:unknown)=>unknown)|undefined;expect(typeof parse).toBe('function');const base=specimen(0x7ff0000000000001n),value={schema:'literal',nodeKind:base.partKind,presentation:base.part2d,handleKinds:[{id:'',name:'',label:'',color:'',defaultWireKind:''}],handles:[{id:'',handleKind:'unresolved',angle:{bits:0x7ff0000000000001n},radius:{bits:0n}}],compatibility:base.compatibility,attributes:base.attributes,authors:base.authors,camera2d:base.camera2d,meta:base.meta};expect(parse!(value)).toEqual(value);expect(()=>parse!({...value,camera2d:{x:0,y:0,zoom:1}})).toThrow();
});
test('SQLite shared Block3d snapshot retains the literal composed child and exact owned vortex words',()=>{
 const parse=Reflect.get(three,'parseBlock3dSnapshot') as ((v:unknown)=>unknown)|undefined;expect(typeof parse).toBe('function');const base=specimen(0x8000000000000000n),value={schema:'literal',objectKind:base.partKind,representations:base.representations,catalog:{childId:'local',target:{artifactId:'different target',dialect:{artifactKind:'literal',standard:'',subset:''}}},vortexKindExtra:[{id:'',name:'',label:'',color:'',defaultCableKind:''}],vortices:[{id:'',vortexKind:'unresolved',position:base.camera3d.position,direction:base.camera3d.target,radius:{bits:0x8000000000000000n},label:null}],compatibility:base.compatibility,attributes:base.attributes,authors:base.authors,camera3d:base.camera3d,meta:base.meta};expect(parse!(value)).toEqual(value);expect(()=>parse!({...value,vortices:[{...value.vortices[0],radius:1}]})).toThrow();
});

test('SQLite Block5d authored schema and exact row refusal precede untouched domain fields',async()=>{
 const value=specimen(0n);let touched=false;Object.defineProperty(value,'meta',{get(){touched=true;throw new Error('domain touched')}});await expect(artifact.block5dSnapshotToSqliteDatabase(value,{maxRows:15})).rejects.toThrow('row limit');expect(touched).toBe(false);
 const schemaValue=specimen(0n);Object.defineProperty(schemaValue,'representations',{get(){touched=true;throw new Error('domain touched')}});await expect(artifact.block5dSnapshotToSqliteDatabase(schemaValue,{maxSchemaBytes:1})).rejects.toThrow('schema');expect(touched).toBe(false);
});

import jsonSchema from '../../../🔣️.json';
const jsonValidate=new Ajv({strict:false}).compile(jsonSchema);
import {fileURLToPath} from "node:url";
const committedFixtureRoot=fileURLToPath(new URL('../../../../🧫️fixtures/🧬️mutations/',import.meta.url));
const committedSnapshots=[...new Bun.Glob('**/📸️snapshot/**/🔣️.json').scanSync({cwd:committedFixtureRoot})].sort();
test('SQLite Block5d committed neutral snapshot corpus remains complete',()=>{expect(committedSnapshots.length).toBe(82)});
for(const relative of committedSnapshots)test('SQLite Block5d committed semantic snapshot '+relative,async()=>{
 const wire=await Bun.file(committedFixtureRoot+relative).json();expect(jsonValidate(wire),JSON.stringify(jsonValidate.errors)).toBe(true);
 const value=block5dFromJsonText(JSON.stringify(wire));expect(snapshot.parseBlock5dSnapshot(value)).toEqual(value);
 const emitted=block5dToJsonText(value);expect(jsonValidate(JSON.parse(emitted)),JSON.stringify(jsonValidate.errors)).toBe(true);expect(block5dFromJsonText(emitted)).toEqual(value);
 const db=await oracle(value);try{expect(db.query('PRAGMA integrity_check').values()).toEqual([['ok']]);expect(db.query('PRAGMA foreign_key_check').values()).toEqual([]);expect(await artifact.block5dSnapshotFromSqliteDatabase(await importSqliteDatabase(db.serialize()))).toEqual(value)}finally{db.close()}
});
test('SQLite Block5d declared minimal JSON defaults produce the complete canonical parent',()=>{
 const minimal={schema:'literal',partKind:{id:'',name:'',label:''}};expect(jsonValidate(minimal)).toBe(true);const value=block5dFromJsonText(JSON.stringify(minimal));expect(snapshot.parseBlock5dSnapshot(value)).toEqual(value);expect(value.grips).toEqual([]);expect(value.camera2d.zoom.bits).toBe(0x3ff0000000000000n);expect(value.part2d.radius).toBe(null);expect(jsonValidate(JSON.parse(block5dToJsonText(value)))).toBe(true);
});


import {block5dFromDslText} from '../../../../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🟦️.ts';
for(const item of laws.dslWordCases)test('SQLite Block5d published Text consumer returns canonical exact word '+item.bits,()=>{
 const source='semio block.block5d.dsl v1\nschema="literal"\npart-kind {\nid="" name="" label=""\n}\ncamera2d {\nx='+item.literal+' y=-0 zoom=1\n}\ncamera3d {\nposition=@0,0,0 target=@0,0,0 zoom=1\n}\n';
 const parsed=block5dFromDslText(source);expect(snapshot.parseBlock5dSnapshot(parsed)).toEqual(parsed);
 expect(parsed.camera2d.x.bits).toBe(BigInt('0x'+item.bits));
 expect(parsed.camera2d.y.bits).toBe(0x8000000000000000n);
});


import {block2dFromJsonText} from '../../../../../../../../../◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🟦️.ts';
import {block2dToJsonText} from '../../../../../../../../../◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🟦️.ts';
import {block2dFromDslText} from '../../../../../../../../../◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🟦️.ts';
for(const item of laws.dslWordCases)test('SQLite shared Block2d published JSON/Text consumers own canonical word '+item.bits,()=>{
 const base=specimen(BigInt('0x'+item.bits)),value:two.Block2dSnapshot={schema:'literal',nodeKind:base.partKind,presentation:base.part2d,handleKinds:[{id:'',name:'',label:'',color:'',defaultWireKind:''}],handles:[{id:'',handleKind:'unresolved',angle:base.camera2d.x,radius:base.camera2d.x}],compatibility:base.compatibility,attributes:base.attributes,authors:base.authors,camera2d:base.camera2d,meta:base.meta};
 const wire=block2dToJsonText(value);expect(JSON.parse(wire).handles[0].angle).toEqual({bits:item.bits});expect(block2dFromJsonText(wire)).toEqual(value);
 const text='semio block.block2d.dsl v1\nschema="literal"\nnode-kind {\nid="" name="" label=""\n}\ncamera2d {\nx='+item.literal+' y=-0 zoom=1\n}\n';
 const parsed=block2dFromDslText(text);expect(two.parseBlock2dSnapshot(parsed)).toEqual(parsed);expect(parsed.camera2d.x.bits).toBe(BigInt('0x'+item.bits));expect(parsed.camera2d.y.bits).toBe(0x8000000000000000n);
});


import twoJsonSchema from '../../../../../../../../../◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json';
const twoJsonValidate=new Ajv({strict:false}).compile(twoJsonSchema);
for(const item of laws.dslWordCases)test('SQLite shared Block2d actual declared JSON schema admits exact word '+item.bits,()=>{
 const base=specimen(BigInt('0x'+item.bits)),value:two.Block2dSnapshot={schema:'literal',nodeKind:base.partKind,presentation:base.part2d,handleKinds:[],handles:[],compatibility:base.compatibility,attributes:base.attributes,authors:base.authors,camera2d:base.camera2d,meta:base.meta};
 expect(twoJsonValidate(JSON.parse(block2dToJsonText(value))),JSON.stringify(twoJsonValidate.errors)).toBe(true);
});


import threeJsonSchema from '../../../../../../../../../🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json';
test('SQLite shared Block2d schema refuses malformed words and incomplete owned kinds',()=>{
 const base=specimen(0n),value:two.Block2dSnapshot={schema:'literal',nodeKind:base.partKind,presentation:base.part2d,handleKinds:[],handles:[],compatibility:[],attributes:[],authors:[],camera2d:base.camera2d,meta:base.meta};const wire=JSON.parse(block2dToJsonText(value));wire.camera2d.x={bits:'wrong'};expect(twoJsonValidate(wire)).toBe(false);wire.camera2d.x={bits:'0000000000000000'};wire.handleKinds=[{id:'only'}];expect(twoJsonValidate(wire)).toBe(false);
});
test('SQLite shared Block3d schema refuses malformed coordinates and incomplete owned vortices',()=>{
 const schema={$defs:threeJsonSchema.$defs,$ref:'#/$defs/Block3dVortexTemplate'},validate=new Ajv({strict:false}).compile(schema);expect(validate({id:'only'})).toBe(false);expect(validate({id:'',vortexKind:'',position:[0,0],direction:[0,0,0],radius:0,label:null})).toBe(false);
});


test('SQLite shared Block2d JSON cannot invent required native handle scalars',()=>{
 const input={schema:'literal',nodeKind:{id:'',name:'',label:''},handles:[{id:'',handleKind:''}]};expect(()=>block2dFromJsonText(JSON.stringify(input))).toThrow();
});

test('SQLite shared Block canonical text refuses lossy UTF-16 states excluded by native UTF-8 strings',()=>{
 for(const units of laws.invalidUtf16CodeUnits){const value=specimen(0n),invalid=String.fromCharCode(...units);expect(Buffer.from(invalid,'utf8').toString('utf8')).not.toBe(invalid);value.partKind.name=invalid;expect(()=>snapshot.parseBlock5dSnapshot(value)).toThrow();expect(()=>block5dFromJsonText(JSON.stringify({schema:'literal',partKind:{id:'',name:invalid,label:''}}))).toThrow()}
});

test('SQLite shared Block3d canonical child identities preserve native UTF-8 domain without URI restrictions',()=>{
 const base=specimen(0n),value:three.Block3dSnapshot={schema:'literal',objectKind:base.partKind,representations:[],catalog:{childId:'local',target:{artifactId:'different target',dialect:{artifactKind:'literal',standard:'',subset:''}}},vortexKindExtra:[],vortices:[],compatibility:[],attributes:[],authors:[],camera3d:base.camera3d,meta:base.meta};expect(three.parseBlock3dSnapshot(value)).toEqual(value);
 for(const units of laws.invalidUtf16CodeUnits){value.catalog.childId=String.fromCharCode(...units);expect(()=>three.parseBlock3dSnapshot(value)).toThrow()}
});


import callerLedger from '../../🧫️fixtures/🪶️sqlite/💰️caller/🔣️.json';
import callerLedgerSchema from '../../🧫️fixtures/🪶️sqlite/💰️caller/🧬️schema/🔣️.json';
import {sqliteOperation} from '../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts';
import {NativeDecodeControl} from '../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🛬️decode/🟦️.ts';
test('Block5d closed supplied caller ledger contract has independent exact octet backing',()=>{
 expect(new Ajv({strict:true}).validate(callerLedgerSchema,callerLedger)).toBe(true);
 expect(Buffer.from(callerLedger.byteBacking).length).toBe(callerLedger.exactBytes);
 expect(new Uint8Array(callerLedger.byteBacking).byteLength).toBe(callerLedger.exactBytes);
 expect(callerLedger.oneByteShort+1).toBe(callerLedger.exactBytes);
 expect(callerLedger.twoCallBytes).toBe(callerLedger.exactBytes*2);
});
test('Block5d supplied operation exposes paid child stage with genuine byte backing',async()=>{
 const bytes=new Uint8Array(callerLedger.byteBacking),operation=sqliteOperation({maxAllocationBytes:callerLedger.twoCallBytes});
 const stage=Reflect.get(operation,'allocationStage');expect(typeof stage).toBe('function');
 const run=(owner:ReturnType<typeof sqliteOperation>)=>Reflect.get(owner,'allocationStage').call(owner,(maximum:number)=>new NativeDecodeControl(maximum,()=>true),async(control:NativeDecodeControl)=>control.copyBytes(bytes));
 expect(await run(operation)).toEqual(bytes);expect(operation.ownedBytes).toBe(callerLedger.exactBytes);
 expect(await run(operation)).toEqual(bytes);expect(operation.ownedBytes).toBe(callerLedger.twoCallBytes);
 await expect(run(operation)).rejects.toMatchObject({kind:'ownershipLimit'});expect(operation.ownedBytes).toBe(callerLedger.twoCallBytes);
 for(const maximum of[0,callerLedger.oneByteShort]){const denied=sqliteOperation({maxAllocationBytes:maximum});await expect(run(denied)).rejects.toMatchObject({kind:'ownershipLimit'});expect(denied.ownedBytes).toBe(0);}
});
function ledgerAt(value:any,path:readonly(string|number)[]):any{return path.reduce((row,key)=>row[key],value)}
function ledgerSet(value:any,path:readonly(string|number)[],next:unknown):void{ledgerAt(value,path.slice(0,-1))[path[path.length-1]!]=next}
for(const raw of laws.binary64Bits)for(const path of callerLedger.optionalPaths)for(const present of[false,true])test('Block5d full word optional ordered owner '+raw+' '+JSON.stringify(path)+' '+present,async()=>{
 const owned=specimen(BigInt('0x'+raw));
 if(!present)ledgerSet(owned,path,null);
 else if(ledgerAt(owned,path)===null)ledgerSet(owned,path,path[0]==='part2d'&&['radius','width','height'].includes(String(path[1]))?{bits:BigInt('0x'+raw)}:callerLedger.literal);
 expect(snapshot.parseBlock5dSnapshot(owned)).toEqual(owned);
 const operation=sqliteOperation(),database=await artifact.block5dSnapshotToSqliteDatabase(owned,operation);
 expect(await artifact.block5dSnapshotFromSqliteDatabase(database,operation)).toEqual(owned);
 const independent=Database.deserialize(await exportSqliteDatabase(database),{safeIntegers:true});
 try{expect(independent.query('PRAGMA integrity_check').values()).toEqual([['ok']]);expect(independent.query('PRAGMA foreign_key_check').values()).toEqual([]);expect(await artifact.block5dSnapshotFromSqliteDatabase(await importSqliteDatabase(independent.serialize()))).toEqual(owned);}finally{independent.close();}
});
for(const direction of['projection','reconstruction']as const)test('Block5d supplied caller exact short cumulative and interior '+direction,async()=>{
 const value=specimen(BigInt('0x'+laws.binary64Bits[0]!));value.authors=Array.from({length:callerLedger.repeat},(_,index)=>({id:String(index),name:callerLedger.literal,email:index%2===0?null:''}));
 const unchanged=structuredClone(value),database=await artifact.block5dSnapshotToSqliteDatabase(value),expected=direction==='projection'?database:value;
 const run=(operation:ReturnType<typeof sqliteOperation>)=>direction==='projection'?artifact.block5dSnapshotToSqliteDatabase(value,operation):artifact.block5dSnapshotFromSqliteDatabase(database,operation);
 const probe=sqliteOperation();expect(await run(probe)).toEqual(expected);expect(probe.ownedBytes).toBeGreaterThan(0);
 const exact=sqliteOperation({maxAllocationBytes:probe.ownedBytes});expect(await run(exact)).toEqual(expected);expect(exact.remainingBytes()).toBe(0);await expect(run(exact)).rejects.toMatchObject({kind:'ownershipLimit'});
 await expect(run(sqliteOperation({maxAllocationBytes:probe.ownedBytes-1}))).rejects.toMatchObject({kind:'ownershipLimit'});
 const twice=sqliteOperation({maxAllocationBytes:probe.ownedBytes*2});expect(await run(twice)).toEqual(expected);expect(await run(twice)).toEqual(expected);expect(twice.remainingBytes()).toBe(0);await expect(run(twice)).rejects.toMatchObject({kind:'ownershipLimit'});
 const controller=new AbortController();let interior=false;
 const canceled=sqliteOperation({signal:controller.signal,onProgress:event=>{if(event.completed>=callerLedger.minimumInterior&&event.completed<event.total){interior=true;controller.abort()}}});
 await expect(run(canceled)).rejects.toMatchObject({kind:callerLedger.requiredKind});expect(interior).toBe(true);
 expect(value).toEqual(unchanged);
});

import captureContract from '../../🧫️fixtures/🪶️sqlite/🧱️capture/🔣️.json';
import captureSchema from '../../🧫️fixtures/🪶️sqlite/🧱️capture/🧬️schema/🔣️.json';
test('Block5d exact literal owner capture precedes mutable callbacks',async()=>{
 const validate=new Ajv({strict:true}).compile(captureSchema);expect(validate(captureContract)).toBe(true);expect(validate({...captureContract,foreign:true})).toBe(false);expect(validate({...captureContract,ieeeWord:'0'})).toBe(false);
 expect(Reflect.get(captureContract,'authority')).toBe('literalTopologyBeforeCallback');
 expect(Reflect.get(captureContract,'ieeeWord')).toBe('7ff8000000000042');
 const bytes=Buffer.from(Reflect.get(captureContract,'ieeeWord'),'hex'),bits=new DataView(bytes.buffer,bytes.byteOffset,8).getBigUint64(0);expect(bits).toBe(0x7ff8000000000042n);
 for(const path of Reflect.get(captureContract,'extraPaths') as (string|number)[][]){const value=specimen(bits);ledgerAt(value,path).foreign=true;await expect(artifact.block5dSnapshotToSqliteDatabase(value)).rejects.toMatchObject({kind:'invalidValue'})}
 const value=specimen(bits),expected=structuredClone(value);let projected=false;
 const database=await artifact.block5dSnapshotToSqliteDatabase(value,{onProgress:()=>{if(!projected){projected=true;value.meta.description='mutated after borrow';value.authors.length=0}}});expect(projected).toBe(true);
 const independent=Database.deserialize(await exportSqliteDatabase(database),{safeIntegers:true});let imported:Awaited<ReturnType<typeof importSqliteDatabase>>;try{expect(independent.query('PRAGMA integrity_check').values()).toEqual([['ok']]);expect(independent.query('PRAGMA foreign_key_check').values()).toEqual([]);imported=await importSqliteDatabase(independent.serialize());expect(await artifact.block5dSnapshotFromSqliteDatabase(imported)).toEqual(expected)}finally{independent.close()}
 let restored=false;expect(await artifact.block5dSnapshotFromSqliteDatabase(imported!,{onProgress:()=>{if(!restored){restored=true;imported!.tables[0]!.rows[0]!.values[1]='mutated SQL cell';imported!.tables[0]!.rows.length=0;imported!.tables.length=0}}})).toEqual(expected);expect(restored).toBe(true);
});
