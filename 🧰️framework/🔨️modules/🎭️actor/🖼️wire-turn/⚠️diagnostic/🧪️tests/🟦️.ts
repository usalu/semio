import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import corpus from "./🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
test("original delivery descriptor agrees with Buffer wire ABI and preserves its body view",()=>{expect(corpus.delivery.actorMaximumCapacityBytes).toBe(corpus.originalDriver.maximumCapacityBytes);expect(corpus.delivery.physicalBodyCopy).toBe(false);const token={receiver:1,operation:7n,generation:2n,sequence:3,attempt:1},body=Buffer.alloc(4096,0x5a),header=Buffer.alloc(TYPED_OPERATION_PAGE_MAGIC.length+30);body.writeUInt32LE(0);body[4]=1;Buffer.from(TYPED_OPERATION_PAGE_MAGIC).copy(header);let offset=TYPED_OPERATION_PAGE_MAGIC.length;header.writeUInt32LE(token.receiver,offset);header.writeBigUInt64LE(token.operation,offset+4);header.writeBigUInt64LE(token.generation,offset+12);header.writeUInt32LE(token.sequence,offset+20);header[offset+24]=token.attempt;header[offset+25]=corpus.delivery.laneTag;header.writeUInt32LE(body.length,offset+26);const descriptor={header,body:body.subarray()};expect(descriptor.body.buffer).toBe(body.buffer);expect(descriptor.header.length+descriptor.body.length).toBeGreaterThan(corpus.originalDriver.maximumCapacityBytes);const scan=scanTypedOperationPages([{tag:"send-message",val:{target:{tag:"shell",val:token.receiver},payload:Buffer.concat([descriptor.header,descriptor.body])}}]);expect(scan.pages).toHaveLength(1);expect(scan.pages[0]!.token).toEqual(token);expect(scan.pages[0]!.lane).toBe(corpus.delivery.laneTag);expect(Buffer.compare(Buffer.from(scan.pages[0]!.payload),body)).toBe(0);console.log("[DEBUG] original delivery descriptor Buffer wire oracle: exact magic+30 header, full4096 body, original alias identity and existing production decoder agreement");});
test("retained original page framing preserves fixed capacity and exact alias retirement order",()=>{const law=corpus.retainedPage;expect(law.capacity).toBe(corpus.maximumPageBytes);expect(law.initializeBytes).toBe(2048);expect(law.ordinaryClone).toBe(false);const backing=Buffer.allocUnsafe(law.capacity);for(let offset=0;offset<backing.length;offset+=law.initializeBytes){backing.fill(0,offset,Math.min(backing.length,offset+law.initializeBytes));expect(Math.min(law.initializeBytes,backing.length-offset)).toBeLessThan(corpus.originalDriver.maximumCopyBytes);}const original=backing.subarray(0,257),receiver=original.subarray();expect(receiver.buffer).toBe(original.buffer);const db=new Database(":memory:");try{db.exec("CREATE TABLE custody(original INTEGER,receiver INTEGER,ack INTEGER);INSERT INTO custody VALUES(1,1,0)");expect(db.query("UPDATE custody SET ack=1 WHERE receiver=0").run().changes).toBe(0);db.exec("UPDATE custody SET receiver=0");expect(db.query("UPDATE custody SET ack=1 WHERE receiver=0 AND ack=0").run().changes).toBe(1);db.exec("UPDATE custody SET original=0 WHERE ack=1");expect(db.query("SELECT original,receiver,ack FROM custody").get()).toEqual({original:0,receiver:0,ack:1});console.log("[DEBUG] retained page Buffer/SQLite oracle: fixed4096 backing, staged2048 initialization, original alias identity and receiver-close/exact-ACK/original-close order");}finally{db.close();}});
import {semioSchemaAjvV1} from "@semio-tech/schema-ts/oracles/validator";

test("original operation diagnostics have exact ACK custody and lossless bounded fragments",()=>{
 const validate=semioSchemaAjvV1({strict:true,allErrors:true}).compile(schema);expect(validate(corpus)).toBe(true);
 const db=new Database(":memory:");
 try{
  db.exec("CREATE TABLE original(row INTEGER PRIMARY KEY,body TEXT NOT NULL);CREATE TABLE acknowledgement(receiver INTEGER,operation INTEGER,generation INTEGER,sequence INTEGER,attempt INTEGER,accepted INTEGER)");
  for(const [index,row]of corpus.cases.entries()){
   const diagnostic={code:row.code,...(row.expected===null?{}:{expected:row.expected}),message:row.pattern.repeat(row.repeat),scope:row.scope,severity:row.severity,span:row.span};
   const encoded=Buffer.from(JSON.stringify(diagnostic));db.query("INSERT INTO original VALUES(?,?)").run(index,encoded.toString());
   const fragments:Buffer[]=[];for(let offset=0;offset<encoded.length;offset+=corpus.maximumPageBytes-corpus.fragmentHeaderBytes){const bytes=encoded.subarray(offset,offset+corpus.maximumPageBytes-corpus.fragmentHeaderBytes);const frame=Buffer.alloc(bytes.length+corpus.fragmentHeaderBytes);frame.writeUInt32LE(index);frame[4]=Number(offset+bytes.length===encoded.length);bytes.copy(frame,5);expect(frame.length).toBeLessThanOrEqual(4096);expect(frame.readUInt32LE(0)).toBe(index);fragments.push(frame.subarray(5));}
   expect(JSON.parse(Buffer.concat(fragments).toString())).toEqual(diagnostic);
   expect(db.query("SELECT json_extract(body,'$.message') AS message FROM original WHERE row=?").get(index)).toEqual({message:diagnostic.message});
   const original=db.query("SELECT body FROM original WHERE row=?").get(index);db.exec("DELETE FROM acknowledgement");db.query("INSERT INTO acknowledgement VALUES(1,7,2,3,0,0)").run();
   const keys=corpus.tokenFields;for(let field=0;field<keys.length;field++){const token=[1,7,2,3,0];token[field]!+=1;db.query("UPDATE acknowledgement SET accepted=1 WHERE receiver=? AND operation=? AND generation=? AND sequence=? AND attempt=? AND accepted=0").run(...token);expect(db.query("SELECT accepted FROM acknowledgement").get()).toEqual({accepted:0});expect(db.query("SELECT body FROM original WHERE row=?").get(index)).toEqual(original);}
   const exact=db.query("UPDATE acknowledgement SET accepted=1 WHERE receiver=1 AND operation=7 AND generation=2 AND sequence=3 AND attempt=0 AND accepted=0");expect(exact.run().changes).toBe(1);expect(exact.run().changes).toBe(0);expect(db.query("SELECT body FROM original WHERE row=?").get(index)).toEqual(original);
  }
  console.log(`[DEBUG] operation diagnostic schema/Buffer/SQLite oracle: ${corpus.cases.length} lossless fragment cases; all five exact ACK fields, duplicate rejection, original custody and nonterminal ordering`);
 }finally{db.close();}
});

import {typedOperationDiagnosticFragment,typedOperationPageAnswerV1,scanTypedOperationPages,TYPED_OPERATION_PAGE_MAGIC} from "../../🔖️turn-result/🟦️.ts";

test("shared host preserves original diagnostic fragments and acknowledges without terminal refusal",()=>{
 const token={receiver:1,operation:7n,generation:2n,sequence:3,attempt:1};const payload=new Uint8Array([0,0,0,0,1,123,125]);const acknowledgement={kind:"message" as const,payload:{source:{tag:"shell" as const,val:"1"},payload:[]}};const page={token,lane:15,payload,acknowledgement};const decoded=typedOperationDiagnosticFragment(page)!;expect(decoded.token).toBe(token);expect(decoded.row).toBe(0);expect(decoded.complete).toBe(true);expect(decoded.bytes.buffer).toBe(payload.buffer);expect([...decoded.bytes]).toEqual([123,125]);expect(typedOperationPageAnswerV1(page).refused).toBe(false);expect(typedOperationDiagnosticFragment({...page,lane:10})).toBeNull();for(const malformed of[new Uint8Array(4),new Uint8Array([0,0,0,0,2])])expect(()=>typedOperationDiagnosticFragment({...page,payload:malformed})).toThrow();
 const wire=new Uint8Array(TYPED_OPERATION_PAGE_MAGIC.length+30+payload.length);wire.set(TYPED_OPERATION_PAGE_MAGIC);const header=new DataView(wire.buffer,TYPED_OPERATION_PAGE_MAGIC.length,30);header.setUint32(0,1,true);header.setBigUint64(4,7n,true);header.setBigUint64(12,2n,true);header.setUint32(20,3,true);header.setUint8(24,1);header.setUint8(25,15);header.setUint32(26,payload.length,true);wire.set(payload,TYPED_OPERATION_PAGE_MAGIC.length+30);const scan=scanTypedOperationPages([{tag:"send-message",val:{target:{tag:"shell",val:1},payload:wire}}]);expect(scan.diagnosticFragments).toHaveLength(1);expect(scan.terminal).toBe(false);expect(scan.faults).toEqual([]);expect(scan.acknowledgements).toHaveLength(1);console.log("[DEBUG] shared host diagnostic fragment actual decoder: lane15, original byte view, malformed framing refusal, exact ACK and nonterminal success");
});

import {OperationDiagnosticReceiver} from "../🟦️.ts";
import {NativeDecodeControl} from "@semio-tech/value-ts/decode";

function hostControl(cancelled=()=>false){let yields=0;return{syntax:{maximumBytes:corpus.host.maximumBytes,maximumNodes:corpus.host.maximumNodes,maximumDepth:corpus.host.maximumDepth,chunk:corpus.host.chunk,cancelled,progress:()=>{},yield:async()=>{yields++;}},ownership:new NativeDecodeControl(corpus.host.maximumBytes*32,()=>!cancelled()),yields:()=>yields};}
function fragment(bytes:Uint8Array,sequence:number,row=0,complete=false){return{token:{receiver:1,operation:7n,generation:2n,sequence,attempt:0},row,complete,bytes};}
test("command-owned diagnostic reception matches SQLite across UTF8 cuts and exact replay",async()=>{
 const db=new Database(":memory:");try{db.exec("CREATE TABLE diagnostics(row INTEGER PRIMARY KEY,body TEXT NOT NULL)");for(const [ordinal,row]of corpus.cases.entries()){
  const expected={code:row.code,...(row.expected===null?{}:{expected:row.expected}),message:row.pattern.repeat(row.repeat),scope:row.scope,severity:row.severity,span:row.span};const bytes=Buffer.from(JSON.stringify(expected));db.query("INSERT INTO diagnostics VALUES(?,?)").run(ordinal,bytes.toString());
  for(const cut of corpus.host.cuts){const control=hostControl(),receiver=new OperationDiagnosticReceiver(control);let actual:unknown=null;let sequence=0;for(let offset=0;offset<bytes.length;offset+=cut){const page=fragment(bytes.subarray(offset,offset+cut),sequence++,0,offset+cut>=bytes.length);actual=await receiver.receive(page);expect(await receiver.receive(page)).toBeNull();}expect(actual).toEqual(expected);expect(db.query("SELECT json_extract(body,'$.message') AS message FROM diagnostics WHERE row=?").get(ordinal)).toEqual({message:(actual as {message:string}).message});if(bytes.length>corpus.maximumPageBytes)expect(control.yields()).toBeGreaterThan(0);receiver.finish();}
 }console.log("[DEBUG] actual shared diagnostic receiver: SQLite original text; 12 UTF8 fragment traversals, exact replay and complete terminal custody");}finally{db.close();}
});
test("diagnostic receiver refuses foreign identity, gaps, truncation, malformed text, limits and cancellation",async()=>{
 const valid=Buffer.from(JSON.stringify({code:"test",severity:"warning",span:{line:1,column:2,length:3},message:"🧩",scope:{}}));
 for(const field of corpus.host.rejectedBindings){const receiver=new OperationDiagnosticReceiver(hostControl());await receiver.receive(fragment(valid.subarray(0,3),4));const next=fragment(valid.subarray(3),5,0,true);const bad=field==="row"?{...next,row:1}:field==="sequence"?{...next,token:{...next.token,sequence:7}}:{...next,token:{...next.token,[field]:field==="receiver"?2:9n}};await expect(receiver.receive(bad)).rejects.toThrow();expect(()=>receiver.finish()).toThrow();receiver.cancel();}
 for(const source of ['{"code":"a","code":"b"}', '{"unknown":true}', '"not-a-diagnostic"']){const receiver=new OperationDiagnosticReceiver(hostControl());await expect(receiver.receive(fragment(Buffer.from(source),0,0,true))).rejects.toThrow();receiver.cancel();}
 const limited=hostControl();const small=new OperationDiagnosticReceiver({...limited,syntax:{...limited.syntax,maximumBytes:2}});await expect(small.receive(fragment(valid,0,0,true))).rejects.toThrow();small.cancel();
 const utf8=new OperationDiagnosticReceiver(hostControl());await expect(utf8.receive(fragment(new Uint8Array([0xff]),0,0,true))).rejects.toThrow();utf8.cancel();
 let stopped=false;const cancelled=new OperationDiagnosticReceiver(hostControl(()=>stopped));await cancelled.receive(fragment(valid.subarray(0,1),0));stopped=true;await expect(cancelled.receive(fragment(valid.subarray(1),1,0,true))).rejects.toThrow();cancelled.cancel();expect(()=>cancelled.finish()).toThrow();
 console.log("[DEBUG] actual diagnostic receiver: original operation/generation/receiver, ordinal/sequence refusal; bounded decode, invalid UTF8/closed schema and cancellation");
});

import {OperationDiagnosticInbox} from "../🟦️.ts";
test("original diagnostic inbox survives polling slices and isolates equal operation ids in different generations",async()=>{
 const row=corpus.cases[0]!,expected={code:row.code,severity:row.severity,span:row.span,message:row.pattern,scope:row.scope};const bytes=Buffer.from(JSON.stringify(expected));const issued:unknown[]=[];const inbox=new OperationDiagnosticInbox(2,token=>{issued.push(token);return hostControl();});
 const first=fragment(bytes.subarray(0,7),4),second={...fragment(bytes.subarray(0,7),4),token:{...first.token,generation:3n}};
 expect(await inbox.receive(first)).toBeNull();expect(await inbox.receive(second)).toBeNull();expect(issued).toHaveLength(2);
 await expect(inbox.receive({...first,token:{...first.token,generation:4n}})).rejects.toThrow("capacity");
 const a=await inbox.receive(fragment(bytes.subarray(7),5,0,true));expect(a?.token.operation).toBe(7n);expect(a?.token.generation).toBe(2n);expect<unknown>(a?.diagnostic).toEqual(expected);inbox.finish(a!.token);
 const b=await inbox.receive({...fragment(bytes.subarray(7),5,0,true),token:{...second.token,sequence:5}});expect(b?.token.generation).toBe(3n);expect<unknown>(b?.diagnostic).toEqual(expected);inbox.finish(b!.token);
 expect(issued).toHaveLength(2);inbox.cancelAll();console.log("[DEBUG] actual original diagnostic inbox: two retained polling streams, same operation/different generations isolated, finite admission, exact terminal frees original slots");
});

for(const frontier of corpus.host.admissionFrontiers)test(`diagnostic inbox original admission frontier: ${frontier}`,async()=>{
 const db=new Database(":memory:");let release!:()=>void;let entered!:()=>void;const gate=new Promise<void>(resolve=>release=resolve),birth=new Promise<void>(resolve=>entered=resolve);
 class OriginalControl extends NativeDecodeControl{override async charge(bytes:number):Promise<void>{if(bytes===96){entered();await gate;}await super.charge(bytes);}}
 let issued=0;const inbox=new OperationDiagnosticInbox(2,()=>{const c=hostControl();return ++issued===1?{...c,ownership:new OriginalControl(corpus.host.maximumBytes*32,()=>true)}:c;});const first=fragment(new Uint8Array([123]),0),other={...first,token:{...first.token,generation:3n}};const pending=inbox.receive(first);void pending.catch(()=>{});
 try{
  db.exec("CREATE TABLE admission(slot INTEGER PRIMARY KEY CHECK(slot=1),cancelled INTEGER NOT NULL);INSERT INTO admission VALUES(1,0);CREATE TABLE receiver(done INTEGER NOT NULL);CREATE TABLE lifetime(closed INTEGER NOT NULL);INSERT INTO lifetime VALUES(0)");await birth;
  if(frontier==="concurrent-birth"){expect(()=>db.query("INSERT INTO admission VALUES(1,0)").run()).toThrow();await expect(inbox.receive(other)).rejects.toThrow("admission");expect(issued).toBe(1);release();expect(await pending).toBeNull();}
  else if(frontier==="terminal-during-birth"){expect(db.query("SELECT COUNT(*) AS complete FROM receiver WHERE done=1").get()).toEqual({complete:0});expect(()=>inbox.finish(first.token)).toThrow("admission");release();expect(await pending).toBeNull();}
  else{db.exec("UPDATE admission SET cancelled=1");if(frontier==="close-during-birth"){db.exec("UPDATE lifetime SET closed=1");inbox.cancelAll();}else inbox.cancel(first.token);release();await expect(pending).rejects.toThrow("cancel");expect(db.query("SELECT COUNT(*) AS complete FROM receiver WHERE done=1").get()).toEqual({complete:0});if(frontier==="close-during-birth"){await expect(inbox.receive(other)).rejects.toThrow("cancel");expect(issued).toBe(1);}else{expect(await inbox.receive(other)).toBeNull();expect(issued).toBe(2);}}
  console.log(`[DEBUG] actual diagnostic inbox/SQLite original admission frontier: ${frontier}, original issuer count ${issued}, no early terminal or cancelled birth installation`);
 }finally{release();await pending.catch(()=>{});inbox.cancelAll();db.close();}
});

import {OperationDiagnosticObservers} from "../📣️delivery/🟦️.ts";
test("original diagnostic observers isolate failure and preserve finite publication membership",()=>{
 const law=corpus.host.observers,db=new Database(":memory:"),delivered:string[]=[],refused:string[]=[],returns=new Map<string,()=>void>();
 const observers=new OperationDiagnosticObservers<{receipt:number}>(law.capacity,error=>{refused.push(String(error));throw new Error("refusal observer failed");});
 try{
  db.exec("CREATE TABLE listener(id TEXT PRIMARY KEY,ordinal INTEGER NOT NULL);CREATE TABLE delivered(receipt INTEGER,id TEXT)");
  let ordinal=0;
  for(const step of law.steps){
   if(step.action==="subscribe"&&typeof step.observer==="string"){const id=step.observer;returns.set(id,observers.subscribe(row=>{if(id==="broken")throw id;delivered.push(id);if(id==="self-removing")returns.get(id)!();}));db.query("INSERT INTO listener VALUES(?,?)").run(id,ordinal++);}
   else if(step.action==="unsubscribe"&&typeof step.observer==="string"){returns.get(step.observer)!();db.query("DELETE FROM listener WHERE id=?").run(step.observer);}
   else if(typeof step.receipt==="number"){delivered.length=0;refused.length=0;const original={receipt:step.receipt};expect(()=>observers.publish(original)).not.toThrow();const expected=db.query("SELECT id FROM listener WHERE id!='broken' ORDER BY ordinal").all() as {id:string}[];expect(delivered).toEqual(expected.map(row=>row.id));expect<unknown>(delivered).toEqual(step.delivered);expect<unknown>(refused).toEqual(step.refused);for(const id of delivered)db.query("INSERT INTO delivered VALUES(?,?)").run(step.receipt,id);if(delivered.includes("self-removing"))db.query("DELETE FROM listener WHERE id='self-removing'").run();}
  }
  expect(observers.size).toBe(2);const release=observers.subscribe(()=>{});expect(()=>observers.subscribe(()=>{})).toThrow("capacity");release();expect(observers.size).toBe(2);for(const returned of returns.values())returned();expect(observers.size).toBe(0);
  console.log("[DEBUG] diagnostic observers actual bounded publisher: failing listener and failing refusal observer isolated; exact original receipt, finite snapshot membership, self removal and idempotent unsubscribe match SQLite");
 }finally{db.close();}
});
test("diagnostic observer admission during delivery begins at the next original receipt",()=>{
 const seen:string[]=[],observers=new OperationDiagnosticObservers<number>(3,()=>{});let late=false;
 const a=observers.subscribe(row=>{seen.push("a"+row);if(!late){late=true;observers.subscribe(next=>seen.push("late"+next));}});const b=observers.subscribe(row=>seen.push("b"+row));
 observers.publish(1);expect(seen).toEqual(["a1","b1"]);observers.publish(2);expect(seen).toEqual(["a1","b1","a2","b2","late2"]);a();b();
});
