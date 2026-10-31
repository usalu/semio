import {expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {createHash} from "node:crypto";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🔣️.json" with {type:"json"};
import {inspectNativeDirectoryEnvelopeRecoveryV1,readNativeDirectoryEnvelopePreparationV1,inspectNativeDirectoryUnpreparedMetadataV1,encodeNativeDirectoryEnvelopeJournalV1,decodeNativeDirectoryEnvelopeJournalV1,inspectNativeDirectoryInitialMetadataV1,encodeNativeDirectoryEnvelopeInitialV1} from "../🟦️.ts";

test("closed canonical directory transition binds changed anchor length and exact preserved payload identities",()=>{
 const oracle=new Ajv({strict:true}),validate=oracle.compile({...schema,$ref:"#/definitions/Transition"}),encoded=encodeNativeDirectoryEnvelopeJournalV1(fixture.request,fixture.preparation,fixture.transition);
 expect(validate(fixture.transition)).toBe(true);expect(encoded.length).toBeLessThanOrEqual(4096);
 const read=decodeNativeDirectoryEnvelopeJournalV1(encoded);expect(read.authority).toBe(false);expect<unknown>(read.request).toEqual(fixture.request);expect<unknown>(read.transition).toEqual(fixture.transition);
 expect(fixture.transition.before.identity.size).not.toBe(fixture.transition.after.identity.size);
 for(const kind of ["before","after"]as const){const anchor=fixture.transition[kind],bytes=Buffer.from(JSON.stringify(anchor.value)+"\n");expect(createHash("sha256").update(bytes).digest("hex")).toBe(anchor.sha256);expect(BigInt(bytes.length)).toBe(BigInt(anchor.identity.size));}
 for(const change of ["hash","underlying","caseFold","hardlink","old-receipt","new-inode","root","volume-alias","extra","over-cap"]){
  const value:any=structuredClone(fixture.transition),request:any=structuredClone(fixture.request);
  if(change==="hash")value.after.sha256="f".repeat(64);
  if(change==="underlying")value.after.value.underlying.ino="99";
  if(change==="caseFold")value.after.value.caseFold="insensitive";
  if(change==="hardlink")value.after.identity.nlink="2";
  if(change==="old-receipt")value.nextReceiptSha256=request.receiptSha256;
  if(change==="new-inode")value.after.identity.ino=value.before.identity.ino;
  if(change==="root")request.root.ino="99";
  if(change==="volume-alias"){value.volume.ino=value.before.value.underlying.ino;value.volume.mode=16877;request.root.ino=value.volume.ino;}
  if(change==="extra")value.authorize=true;
  if(change==="over-cap")value.after.value.filesystemBytes=value.after.value.maximumPhysicalBytes+1;
  expect(()=>encodeNativeDirectoryEnvelopeJournalV1(request,fixture.preparation,value),change).toThrow();
 }
 expect(()=>decodeNativeDirectoryEnvelopeJournalV1(Buffer.concat([encoded,Buffer.from("\n")]))).toThrow();
 expect(()=>decodeNativeDirectoryEnvelopeJournalV1(Buffer.alloc(4097))).toThrow();
 let bufferReads=0;const native=Buffer.from(encoded);Object.defineProperty(native,"buffer",{get(){bufferReads++;throw Error("accessed");}});expect<unknown>(decodeNativeDirectoryEnvelopeJournalV1(native).transition).toEqual(fixture.transition);expect(bufferReads).toBe(0);
 const accessor=Object.defineProperty({},"version",{enumerable:true,get(){throw Error("accessed");}});expect(()=>encodeNativeDirectoryEnvelopeJournalV1(fixture.request,fixture.preparation,accessor)).toThrow("DATA");
});

test("neutral interruption cuts match an independent SQLite state relation and closed Ajv observations",()=>{
 const db=new Database(":memory:"),validate=new Ajv({strict:true}).compile({...schema,$ref:"#/definitions/Observation"});
 try{for(const row of fixture.cases){
  const transition:any=structuredClone(fixture.transition),windows="platform"in row&&row.platform==="windows";if(windows){transition.prepared.platform="windows";transition.prepared.retainedName=".b5678abc";const facts=(id:string)=>({volumeSerial:"1",fileId:id.repeat(32),attributes:32,security:"a".repeat(64),streams:"b".repeat(64)});transition.prepared.windows={parent:facts("9"),stage:facts("3"),prior:facts("2")};}
  const fact=(kind:string|null)=>kind===null?null:{identity:transition[kind==="old"?"before":"after"].identity,sha256:transition[kind==="old"?"before":"after"].sha256};
  const attached=row.mapping==="attached",underlying=transition.before.value.underlying,volume=transition.volume;
  const observed={anchor:fact(row.anchor),stage:fact(row.stage),retained:fact(row.retained),receiptSha256:row.receipt==="old"?fixture.request.receiptSha256:row.receipt==="new"?transition.nextReceiptSha256:"f".repeat(64),root:attached?volume:underlying,volume:attached?underlying:row.mapping==="foreign"?{...volume,ino:"99"}:volume,pendingSwap:"pendingSwap"in row&&row.pendingSwap};
  expect(validate(observed),row.id).toBe(true);
  const expected=db.query("select case when ?1 or ?2='foreign' or ?3='foreign' then null when ?4='old' and ?5='new' and ?6 is null and ?3='old' and ?2='inactive' then 'publish-anchor' when ?7 and ?4 is null and ?5='new' and ?6='old' and ?3='old' and ?2='inactive' then 'recover-anchor' when ?4='new' and ((not ?7 and ?5='old' and ?6 is null) or (?7 and ?5 is null and ?6='old')) then case when ?3='old' and ?2='inactive' then 'publish-receipt' when ?3='new' then case when ?2='inactive' then 'attach' when ?2='attached' then 'qualify' end end end result").get(observed.pendingSwap?1:0,row.mapping,row.receipt,row.anchor,row.stage,row.retained,windows?1:0)as{result:string|null};
  expect(expected.result,row.id).toBe(row.expected);
  let result:string|null=null;try{result=inspectNativeDirectoryEnvelopeRecoveryV1(fixture.request,transition,observed).next;}catch{}
  expect(result,row.id).toBe(row.expected);
 }}finally{db.close();}
});


test("pre-stage baseline binds the current target without granting staged inode recovery",()=>{
 const validate=new Ajv({strict:true}).compile({...schema,$ref:"#/definitions/Preparation"}),data=readNativeDirectoryEnvelopePreparationV1(fixture.request,fixture.preparation);
 expect(validate(data)).toBe(true);expect(data.authority).toBe(false);expect(encodeNativeDirectoryEnvelopeJournalV1(fixture.request,data).length).toBeLessThanOrEqual(4096);
 expect("after"in data).toBe(false);expect("identity"in data.planned).toBe(false);
 for(const fault of["underlying","fold","root","stage","retained","roster","hash","extra"]){
  const request:any=structuredClone(fixture.request),value:any=structuredClone(fixture.preparation);
  if(fault==="underlying")value.target.underlying.ino="99";
  if(fault==="fold")value.target.caseFold="insensitive";
  if(fault==="root")request.root.ino="99";
  if(fault==="stage")value.planned.stage="fixed.dir";
  if(fault==="retained")value.planned.retainedName=".bforeign";
  if(fault==="roster")value.planned.names.reverse();
  if(fault==="hash")value.before.sha256="f".repeat(64);
  if(fault==="extra")value.adoptStage=true;
  expect(()=>readNativeDirectoryEnvelopePreparationV1(request,value),fault).toThrow();
 }
});


test("fresh unprepared metadata facts distinguish byte role from historical ownership and grant no effects",()=>{
 const validate=new Ajv({strict:true}).compile({...schema,$ref:"#/definitions/UnpreparedMetadata"}),db=new Database(":memory:");
 try{for(const row of fixture.unpreparedCases){
  const value=fixture.preparation,target=JSON.stringify(value.target)+"\n",text=row.mode==="empty"?"":row.mode==="partial"?target.slice(0,17):target;
  const actual:any={declaration:{filesystemBytes:value.target.filesystemBytes,maximumPhysicalBytes:value.target.maximumPhysicalBytes},parent:{...value.planned.parent,nlink:"6"},anchor:{identity:value.before.identity,sha256:value.before.sha256},receiptSha256:fixture.request.receiptSha256,root:value.target.underlying,volume:value.volume,names:[...value.planned.names,value.planned.stage].sort(),stage:{name:value.planned.stage,kind:"file",identity:{...fixture.transition.after.identity,size:String(Buffer.byteLength(text))},text},pendingSwap:false};
  if(row.mode==="foreign-bytes")actual.stage.text="foreign";
  if(row.mode==="suffix")actual.stage.text=target+"foreign";
  if(row.mode==="role")actual.stage.name="payload";
  if(row.mode==="hardlink")actual.stage.identity.nlink="2";
  if(row.mode==="directory")actual.stage.kind="directory";
  if(row.mode==="mode")actual.stage.identity.mode="33188";
  if(row.mode==="device")actual.stage.identity.dev="2";
  if(row.mode==="parent")actual.parent.ino="99";
  if(row.mode==="alias")actual.stage.identity.ino=value.before.identity.ino;
  if(row.mode==="declaration")actual.declaration.filesystemBytes++;
  if(row.mode==="receipt")actual.receiptSha256="f".repeat(64);
  if(row.mode==="anchor")actual.anchor.sha256="f".repeat(64);
  if(row.mode==="root")actual.root={...actual.root,ino:"99"};
  if(row.mode==="roster")actual.names.push("foreign");
  if(row.mode==="pending")actual.pendingSwap=true;
  if(row.mode==="extra")actual.adoptPayload=true;
  actual.stage.identity.size=String(Buffer.byteLength(actual.stage.text));
  let accepted=false;try{const result=inspectNativeDirectoryUnpreparedMetadataV1(fixture.request,value,actual);expect(result).toEqual({role:"current-unprepared-anchor-stage",stagePresent:true,historicalInodeOwned:false,adoptPayload:false,authority:false});accepted=true;}catch{}
  const relation=db.query("select json_extract(?1,'$.pendingSwap')=0\n and json_extract(?1,'$.declaration.filesystemBytes')=json_extract(?2,'$.target.filesystemBytes')\n and json_extract(?1,'$.declaration.maximumPhysicalBytes')=json_extract(?2,'$.target.maximumPhysicalBytes')\n and json_extract(?1,'$.receiptSha256')=json_extract(?3,'$.receiptSha256')\n and json_extract(?1,'$.anchor.sha256')=json_extract(?2,'$.before.sha256')\n and json_extract(?1,'$.anchor.identity')=json_extract(?2,'$.before.identity')\n and json_extract(?1,'$.parent.dev')=json_extract(?2,'$.planned.parent.dev')\n and json_extract(?1,'$.parent.ino')=json_extract(?2,'$.planned.parent.ino')\n and json_extract(?1,'$.parent.mode')=json_extract(?2,'$.planned.parent.mode')\n and json_extract(?1,'$.stage.name')=json_extract(?2,'$.planned.stage')\n and json_extract(?1,'$.stage.kind')='file'\n and json_extract(?1,'$.stage.identity.dev')=json_extract(?2,'$.planned.parent.dev')\n and json_extract(?1,'$.stage.identity.nlink')='1'\n and (cast(json_extract(?1,'$.stage.identity.mode') as integer)&61440)=32768\n and (cast(json_extract(?1,'$.stage.identity.mode') as integer)&511)=384\n and cast(json_extract(?1,'$.stage.identity.size') as integer)=length(cast(json_extract(?1,'$.stage.text') as blob))\n and length(json_extract(?1,'$.stage.text'))<=length(?4)\n and substr(?4,1,length(json_extract(?1,'$.stage.text')))=json_extract(?1,'$.stage.text')\n and json_extract(?1,'$.stage.identity.ino') not in(json_extract(?2,'$.before.identity.ino'),json_extract(?2,'$.planned.parent.ino'),json_extract(?2,'$.volume.ino'),json_extract(?2,'$.target.underlying.ino'))\n and ((json_extract(?1,'$.root')=json_extract(?2,'$.target.underlying') and json_extract(?1,'$.volume')=json_extract(?2,'$.volume')) or (json_extract(?1,'$.root')=json_extract(?2,'$.volume') and json_extract(?1,'$.volume')=json_extract(?2,'$.target.underlying')))\n and json_extract(?1,'$.names')=(select json_group_array(value) from(select value from json_each(?2,'$.planned.names') union all select json_extract(?2,'$.planned.stage') order by value)) accepted").get(JSON.stringify(actual),JSON.stringify(value),JSON.stringify(fixture.request),target)as{accepted:number};expect(accepted,row.id).toBe(row.accepted);expect(accepted,row.id).toBe(Boolean(relation.accepted&&validate(actual)));
 }}finally{db.close();}
});


test("bounded write-ahead journal separates planned metadata from exact prepared inode recovery",()=>{
 const baseline=encodeNativeDirectoryEnvelopeJournalV1(fixture.request,fixture.preparation),prepared=encodeNativeDirectoryEnvelopeJournalV1(fixture.request,fixture.preparation,fixture.transition);
 expect(baseline.toString().split("\n")).toHaveLength(3);expect(prepared.toString().split("\n")).toHaveLength(4);expect(prepared.length).toBeLessThanOrEqual(4096);
 const first=decodeNativeDirectoryEnvelopeJournalV1(baseline),second=decodeNativeDirectoryEnvelopeJournalV1(prepared);expect(first.transition).toBeNull();expect<unknown>(second.transition).toEqual(fixture.transition);expect(second.authority).toBe(false);expect<unknown>(first.preparation).toEqual(fixture.preparation);
 const changed:any=structuredClone(fixture.preparation);changed.target.filesystemBytes++;expect(()=>encodeNativeDirectoryEnvelopeJournalV1(fixture.request,changed,fixture.transition)).toThrow();
 expect(()=>decodeNativeDirectoryEnvelopeJournalV1(Buffer.from(prepared.toString()+"\n"))).toThrow();
});


test("absent unprepared stage still requires the full current layout and closed reserved roster",()=>{
 const value=fixture.preparation,actual={declaration:{filesystemBytes:value.target.filesystemBytes,maximumPhysicalBytes:value.target.maximumPhysicalBytes},parent:value.planned.parent,anchor:{identity:value.before.identity,sha256:value.before.sha256},receiptSha256:fixture.request.receiptSha256,root:value.target.underlying,volume:value.volume,names:value.planned.names,stage:null,pendingSwap:false},validate=new Ajv({strict:true}).compile({...schema,$ref:"#/definitions/UnpreparedMetadata"}),db=new Database(":memory:");
 try{expect(validate(actual)).toBe(true);expect(inspectNativeDirectoryUnpreparedMetadataV1(fixture.request,value,actual).stagePresent).toBe(false);expect((db.query("select json_array_length(?1,'$.names')=5 and json_extract(?1,'$.stage') is null accepted").get(JSON.stringify(actual))as{accepted:number}).accepted).toBe(1);
  for(const fault of["receipt","root","parent","declaration","roster","anchor"]){const row:any=structuredClone(actual);if(fault==="receipt")row.receiptSha256="f".repeat(64);if(fault==="root")row.root.ino="99";if(fault==="parent")row.parent.ino="99";if(fault==="declaration")row.declaration.filesystemBytes++;if(fault==="roster")row.names.push("foreign");if(fault==="anchor")row.anchor.sha256="f".repeat(64);expect(()=>inspectNativeDirectoryUnpreparedMetadataV1(fixture.request,value,row),fault).toThrow();}
 }finally{db.close();}
});


test("fresh Native metadata role module exposes the actual proof-gated owner without issuing authority from DATA",async()=>{
 const owner=await import("../../../📤️publication/🎛️owner/🧾️metadata/🟦️.ts");expect(typeof owner.retireNativeDirectoryUnpreparedMetadataV1).toBe("function");
 let getterCalls=0;const input=Object.defineProperty({},"directory",{enumerable:true,get(){getterCalls++;throw Error("accessed");}});
 await expect(owner.retireNativeDirectoryUnpreparedMetadataV1(input as never,undefined as never,undefined as never,undefined as never)).rejects.toThrow("DATA options");expect(getterCalls).toBe(0);
});


test("initial no-effect restart has a closed role table and never claims historical recovery",()=>{
 const validate=new Ajv({strict:true}).compile({...schema,$ref:"#/definitions/InitialMetadata"}),db=new Database(":memory:");
 try{for(const row of fixture.initialCases){
  const value:any=structuredClone(fixture.initialMetadata),entry=value.namespace.entries[0];
  if(row.id==="empty")value.namespace.entries=[];
  if(row.id==="two-known")value.namespace.entries.push({...structuredClone(entry),name:"receipt.json",identity:{...entry.identity,ino:"12"}});
  if(row.id==="extra-base")value.names.push("payload");
  if(row.id==="anchor-stage")value.names.push(".12345678");
  if(row.id==="intent-stage")entry.name=".1234567890";
  if(row.id==="unknown-namespace")entry.name="foreign";
  if(row.id==="directory-entry")entry.kind="directory";
  if(row.id==="symlink-entry")entry.kind="link";
  if(row.id==="hardlink")entry.identity.nlink="2";
  if(row.id==="oversize")entry.identity.size="4097";
  if(row.id==="permissions")entry.identity.mode="33188";
  if(row.id==="namespace-kind")value.namespace.identity.mode="33152";
  if(row.id==="namespace-mode")value.namespace.identity.mode="16877";
  if(row.id==="anchor-hash")value.anchor.sha256="f".repeat(64);
  if(row.id==="receipt-anchor")value.receipt.ino++;
  if(row.id==="receipt-envelope")value.receipt.filesystemBytes++;
  if(row.id==="receipt-underlay")value.receipt.mountDirectory.ino="99";
  if(row.id==="foreign-platform")value.receipt.platform="win32";
  if(row.id==="mapping"){value.root.ino="99";value.volume.ino="98";}
  if(row.id==="alias-volume")value.volume.ino=value.root.ino;
  if(row.id==="parent-device")value.parent.dev="2";
  if(row.id==="shrink-declaration")value.declaration.filesystemBytes=value.anchor.value.filesystemBytes-1;
  if(row.id==="pending")value.pendingSwap=true;
  if(row.id==="duplicate-namespace")value.namespace.entries.push(structuredClone(entry));
  let accepted=false;try{expect(inspectNativeDirectoryInitialMetadataV1(value)).toEqual({role:"initial-metadata-restart",historicalPreservationRecovered:false,authority:false});accepted=true;}catch{}
  const relation=db.query("select json_extract(?1,'$.pendingSwap')=0\n and json_extract(?1,'$.names')=?2\n and json_extract(?1,'$.anchor.sha256')=?3\n and json_extract(?1,'$.anchor.identity.nlink')='1'\n and (cast(json_extract(?1,'$.anchor.identity.mode') as integer)&61440)=32768\n and (cast(json_extract(?1,'$.parent.mode') as integer)&61440)=16384\n and (cast(json_extract(?1,'$.namespace.identity.mode') as integer)&61440)=16384\n and (cast(json_extract(?1,'$.namespace.identity.mode') as integer)&511)=448\n and json_extract(?1,'$.anchor.identity.dev')=json_extract(?1,'$.parent.dev')\n and json_extract(?1,'$.namespace.identity.dev')=json_extract(?1,'$.parent.dev')\n and json_extract(?1,'$.root.dev')=json_extract(?1,'$.parent.dev')\n and json_extract(?1,'$.volume.dev')=json_extract(?1,'$.parent.dev')\n and json_extract(?1,'$.root.ino')<>json_extract(?1,'$.volume.ino')\n and (json_extract(?1,'$.root')=json_extract(?1,'$.anchor.value.underlying') or json_extract(?1,'$.volume')=json_extract(?1,'$.anchor.value.underlying'))\n and json_extract(?1,'$.anchor.value.filesystemBytes')<=json_extract(?1,'$.declaration.filesystemBytes')\n and json_extract(?1,'$.anchor.value.maximumPhysicalBytes')<=json_extract(?1,'$.declaration.maximumPhysicalBytes')\n and json_extract(?1,'$.receipt.platform')=json_extract(?1,'$.platform')\n and cast(json_extract(?1,'$.receipt.dev') as text)=json_extract(?1,'$.anchor.identity.dev')\n and cast(json_extract(?1,'$.receipt.ino') as text)=json_extract(?1,'$.anchor.identity.ino')\n and cast(json_extract(?1,'$.receipt.size') as text)=json_extract(?1,'$.anchor.identity.size')\n and json_extract(?1,'$.receipt.filesystemBytes')=json_extract(?1,'$.anchor.value.filesystemBytes')\n and json_extract(?1,'$.receipt.maximumPhysicalBytes')=json_extract(?1,'$.anchor.value.maximumPhysicalBytes')\n and json_extract(?1,'$.receipt.mountDirectory')=json_extract(?1,'$.anchor.value.underlying')\n and not exists(select 1 from json_each(?1,'$.namespace.entries') where json_extract(value,'$.name') not in('intent.json','receipt.json') or json_extract(value,'$.kind')<>'file' or json_extract(value,'$.identity.nlink')<>'1' or cast(json_extract(value,'$.identity.size') as integer)>4096 or (cast(json_extract(value,'$.identity.mode') as integer)&61440)<>32768 or (cast(json_extract(value,'$.identity.mode') as integer)&511)<>384)\n and (select count(*) from json_each(?1,'$.namespace.entries'))=(select count(distinct json_extract(value,'$.name')) from json_each(?1,'$.namespace.entries')) accepted").get(JSON.stringify(value),JSON.stringify(fixture.initialMetadata.names),fixture.initialMetadata.anchor.sha256)as{accepted:number};
  expect(accepted,row.id).toBe(row.accepted);expect(accepted,row.id).toBe(Boolean(validate(value)&&relation.accepted));
 }}finally{db.close();}
});


test("equal-current initial restart needs the exact typed no-effects marker",()=>{
 const value:any=structuredClone(fixture.initialMetadata);value.declaration={filesystemBytes:value.anchor.value.filesystemBytes,maximumPhysicalBytes:value.anchor.value.maximumPhysicalBytes};expect(()=>inspectNativeDirectoryInitialMetadataV1(value)).toThrow("Opaque equal-current");
 const request={...fixture.request,receiptSha256:value.receiptSha256},header=encodeNativeDirectoryEnvelopeInitialV1(request,{before:value.anchor,target:value.anchor.value,volume:value.root});value.marker=JSON.parse(header.toString());expect(inspectNativeDirectoryInitialMetadataV1(value).historicalPreservationRecovered).toBe(false);
 const db=new Database(":memory:");try{expect((db.query("select json_extract(?1,'$.marker.phase')='observed' and json_extract(?1,'$.marker.request.receiptSha256')=json_extract(?1,'$.receiptSha256') accepted").get(JSON.stringify(value))as{accepted:number}).accepted).toBe(1);}finally{db.close();}
 value.marker.request.receiptSha256="f".repeat(64);expect(()=>inspectNativeDirectoryInitialMetadataV1(value)).toThrow();
});


test("canonical Native publisher imports the actual envelope effect owners",async()=>{
 const engine=await import("../../../📤️publication/🎛️owner/⚓️envelope/🟦️.ts"),owner=await import("../../../📤️publication/🎛️owner/🟦️.ts");expect(typeof owner.publishObservedNativeImagePublicationV1).toBe("function");for(const name of["nativeDirectoryEnvelopeInitialMetadataV1","prepareNativeDirectoryEnvelopePublicationV1","captureNativeDirectoryEnvelopeRecoveryV1","commitNativeDirectoryEnvelopePublicationV1","finishNativeDirectoryEnvelopePublicationV1"]as const)expect(typeof engine[name]).toBe("function");
});

test("portable journal retains complete bounded Windows facts through the canonical wire",()=>{
 const value:any=structuredClone(fixture.transition),preparation:any=structuredClone(fixture.preparation),row=fixture.wireWindows,facts=(fileId:string)=>({volumeSerial:row.volumeSerial,fileId,attributes:row.attributes,security:row.security,streams:row.streams});
 value.prepared.platform="windows";value.prepared.retainedName=row.retainedName;value.prepared.windows={parent:facts(row.parentFileId),prior:facts(row.priorFileId),stage:facts(row.stageFileId)};preparation.planned.platform="windows";preparation.planned.retainedName=row.retainedName;preparation.planned.windows={parent:value.prepared.windows.parent,prior:value.prepared.windows.prior};
 const bytes=encodeNativeDirectoryEnvelopeJournalV1(fixture.request,preparation,value),read=decodeNativeDirectoryEnvelopeJournalV1(bytes);expect(bytes.length).toBeLessThanOrEqual(4096);expect<unknown>(read.transition).toEqual(value);expect(new Ajv({strict:true}).compile({...schema,$ref:"#/definitions/Transition"})(read.transition)).toBe(true);
 const db=new Database(":memory:");try{const lines=bytes.toString().trimEnd().split("\n"),actual=db.query("select json_extract(?1,'$.planned.windows.parent.volumeSerial') volumeSerial,json_extract(?2,'$.windowsStage.fileId') fileId,json_extract(?2,'$.windowsStage.security') security,json_extract(?2,'$.windowsStage.streams') streams,json_extract(?2,'$.windowsStage.attributes') attributes").get(lines[1]!,lines[2]!);expect(actual).toEqual({volumeSerial:row.volumeSerial,fileId:row.stageFileId,security:row.security,streams:row.streams,attributes:row.attributes});}finally{db.close();}
});

test("fresh metadata namespace reissue always retains one complete agreeing baseline",async()=>{
 const owner=await import("../🟦️.ts"),inspect=(owner as unknown as{inspectNativeDirectoryBaselineNamespaceV1:(value:unknown)=>Readonly<{keeper:string;discard:readonly string[];bytes:Uint8Array;authority:false}>}).inspectNativeDirectoryBaselineNamespaceV1;expect(typeof inspect).toBe("function");
 const baseline=encodeNativeDirectoryEnvelopeJournalV1(fixture.request,fixture.preparation).toString(),db=new Database(":memory:");
 try{for(const row of fixture.baselineNamespaceCases){
  const entries=[{name:".1234567890",text:baseline}];
  if(["two-agree","partial-other","disagree-witness","three","disagree-platform","foreign-context"].includes(row.id))entries.push({name:".abcdef1234",text:baseline});
  if(row.id==="partial-other")entries[1]!.text="{";
  if(row.id==="unknown-name")entries[0]!.name="foreign";
  if(row.id==="disagree-witness"){const changed:any=structuredClone(fixture.preparation);changed.witness.applicationSha256="f".repeat(64);entries[1]!.text=encodeNativeDirectoryEnvelopeJournalV1(fixture.request,changed).toString();}
  if(row.id==="disagree-platform"){const changed:any=structuredClone(fixture.preparation),row=fixture.wireWindows,facts=(fileId:string)=>({volumeSerial:row.volumeSerial,fileId,attributes:row.attributes,security:row.security,streams:row.streams});changed.planned.platform="windows";changed.planned.retainedName=row.retainedName;changed.planned.windows={parent:facts(row.parentFileId),prior:facts(row.priorFileId)};entries[1]!.text=encodeNativeDirectoryEnvelopeJournalV1(fixture.request,changed).toString();}
  if(row.id==="foreign-context"){const changed:any=structuredClone(fixture.preparation),request:any=structuredClone(fixture.request);request.directory.ino="99";changed.planned.parent.ino="99";entries[1]!.text=encodeNativeDirectoryEnvelopeJournalV1(request,changed).toString();}
  if(row.id==="no-complete")entries[0]!.text="{";
  if(row.id==="prepared-record")entries[0]!.text=encodeNativeDirectoryEnvelopeJournalV1(fixture.request,fixture.preparation,fixture.transition).toString();
  if(row.id==="three")entries.push({name:".fedcba1234",text:"{"});
  let accepted=false,actualKeeper:string|undefined;try{const result=inspect({entries});actualKeeper=result.keeper;expect(entries.some(entry=>entry.name===result.keeper&&entry.text===Buffer.from(result.bytes).toString())).toBe(true);expect(result.discard).not.toContain(result.keeper);expect(result.authority).toBe(false);accepted=true;}catch{}
  expect(accepted,row.id).toBe(row.accepted);const reference=db.query("with entries as(select json_extract(value,'$.name') name,json_extract(value,'$.text') text from json_each(?1,'$.entries')),first as(select *,instr(text,char(10)) a from entries),second as(select *,instr(substr(text,a+1),char(10)) b from first),parts as(select name,text,case when a>0 then substr(text,1,a-1) else 'null' end h,case when b>0 then substr(text,a+1,b-1) else 'null' end w,case when b>0 then substr(text,a+b+1) else text end tail from second),safe as(select *,case when json_valid(h) then h else 'null' end header,case when json_valid(w) then w else 'null' end witnessed,case when json_valid(trim(tail)) then trim(tail) else 'null' end event from parts),complete as(select *,json_array(json_extract(header,'$.request.directory'),json_extract(header,'$.request.backing'),json_extract(header,'$.request.receiptSha256'),json_extract(header,'$.before'),json_extract(header,'$.target'),json_extract(header,'$.volume'),json_extract(witnessed,'$.witness'),json_extract(witnessed,'$.planned.platform'),json_extract(witnessed,'$.planned.parent.dev'),json_extract(witnessed,'$.planned.parent.ino'),json_extract(witnessed,'$.planned.parent.mode'),json_extract(witnessed,'$.planned.windows')) signature from safe where json_extract(header,'$.purpose')='initial-no-effect-directory-envelope' and json_extract(witnessed,'$.purpose')='witnessed-directory-envelope' and tail='') select (select count(*) between 1 and 2 from entries) and not exists(select 1 from entries where length(name)<>11 or substr(name,1,1)<>'.' or substr(name,2) glob '*[^a-f0-9]*' or length(cast(text as blob))>4096) and (select count(*) from entries)=(select count(distinct name) from entries) and (select count(*)>0 from complete) and (select count(distinct signature)=1 from complete) and not exists(select 1 from safe where json_extract(event,'$.purpose')='prepared-current-directory-envelope') accepted,(select min(name) from complete) keeper").get(JSON.stringify({entries}))as{accepted:number;keeper:string|null};if(accepted)expect<unknown>(actualKeeper).toBe(reference.keeper);expect(accepted).toBe(Boolean(new Ajv({strict:true}).compile({...schema,$ref:"#/definitions/BaselineNamespace"})({entries})&&reference.accepted));
 }}finally{db.close();}
});

test("initial marker accepts no foreign or partial second record",async()=>{
 const module=await import("../🟦️.ts"),inspect=(module as unknown as{readNativeDirectoryInitialRecoveryV1:(input:unknown)=>Readonly<{kind:string;authority:false}>}).readNativeDirectoryInitialRecoveryV1;expect(typeof inspect).toBe("function");const header=encodeNativeDirectoryEnvelopeInitialV1(fixture.request,{before:fixture.preparation.before,target:fixture.preparation.target,volume:fixture.preparation.volume}).toString(),db=new Database(":memory:");
 try{for(const row of fixture.initialRecoveryCases){
  let text=header;if(row.id.includes("equal-current"))text=encodeNativeDirectoryEnvelopeInitialV1(fixture.request,{before:fixture.preparation.before,target:fixture.preparation.before.value,volume:fixture.preparation.volume}).toString();if(row.id==="unknown-suffix-equal-current")text+=JSON.stringify({purpose:"foreign",authority:false})+"\n";if(row.id==="opaque-old")text=JSON.stringify(fixture.request)+"\n";if(row.id==="opaque-partial")text="{\"version\":";if(row.id==="marker-missing-lf")text=text.trimEnd();if(row.id==="unknown-suffix")text+=JSON.stringify({purpose:"foreign",authority:false})+"\n";if(row.id==="known-witness-suffix")text+=JSON.stringify({purpose:"witnessed-directory-envelope"})+"\n";if(row.id==="partial-witness-suffix")text+="{\"version\":1";if(row.id==="extra-lf")text+="\n";if(row.id==="extra-field"){const value=JSON.parse(header);value.extra=true;text=JSON.stringify(value)+"\n";}if(row.id==="foreign-prepared-first")text=JSON.stringify({purpose:"prepared-current-directory-envelope"})+"\n";
  let actual="refuse";try{const value=inspect({text});actual=value.kind;expect(value.authority).toBe(false);}catch{}expect(actual,row.id).toBe(row.expected);
  const reference=db.query("with x as(select ?1 text,instr(?1,char(10)) lf),v as(select *,case when lf>0 then substr(text,1,lf-1) else text end body from x),j as(select *,case when json_valid(body) then body else 'null' end row from v)select case when lf>0 and lf<>length(text) then 'refuse' when json_extract(row,'$.purpose') in('witnessed-directory-envelope','prepare-current-directory-envelope','prepared-current-directory-envelope','current-directory-envelope-publication') then 'refuse' when json_extract(row,'$.purpose')='initial-no-effect-directory-envelope' and json_type(row,'$.extra') is not null then 'refuse' when json_extract(row,'$.purpose')='initial-no-effect-directory-envelope' and lf=length(text) then 'observed-marker' when json_extract(row,'$.purpose')='initial-no-effect-directory-envelope' then 'refuse' else 'opaque-predecessor' end kind from j").get(text)as{kind:string};expect(actual).toBe(reference.kind);
 }}finally{db.close();}
});
