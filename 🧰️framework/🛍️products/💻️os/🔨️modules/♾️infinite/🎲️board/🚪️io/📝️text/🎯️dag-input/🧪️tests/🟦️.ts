/** 🧪️ Shared DAG facts and hostile inputs agree with an independent schema compiler. */
import {expect,test} from "bun:test";
import fixture from "../🧫️fixtures/🔣️.json";
import schema from "../../../../🧬️schema/🎯️dag-input/🔣️.json";
import {decodeDagSelectionJson,decodeDagChannelsJson,decodeDagNodeStatusesJson,decodeDagHoverJson,type DagTextDecodeControl} from "../🟦️.ts";
import {NativeDecodeControl} from "../../../../../../../../../🔨️modules/🌱️value/🛬️decode/🟦️.ts";
import {semioSchemaAjvV1} from "../../../../../../../../../🔨️modules/🧬️schema/🔮️oracles/✅️validator/🟦️.ts";

const readers={selection:decodeDagSelectionJson,channels:decodeDagChannelsJson,statuses:decodeDagNodeStatusesJson,hover:decodeDagHoverJson};
function control(maximum=65536,observe:()=>boolean=()=>true):DagTextDecodeControl{return {syntax:{maximumBytes:65536,maximumNodes:4096,maximumDepth:16,chunk:7,cancelled:()=>false,progress:()=>{},yield:async()=>{}},ownership:new NativeDecodeControl(maximum,observe)}}

test("DAG physical admission agrees with all schema-first selection channel and status vectors",async()=>{
 const oracles=Object.fromEntries(Object.keys(readers).map(kind=>[kind,semioSchemaAjvV1().compile({...schema,oneOf:[{$ref:"#/definitions/"+kind}]})]));let accepted=0,refused=0;
 for(const [rows,valid]of [[fixture.accepted,true],[fixture.refused,false]]as const)for(const row of rows){const oracle=oracles[row.kind]!;expect(Boolean(oracle(row.value))).toBe(valid);const source=JSON.stringify(row.value),reader=readers[row.kind as keyof typeof readers];if(valid){expect(await reader(source,control())).toEqual(JSON.parse(source));accepted++;}else{await expect(reader(source,control())).rejects.toThrow();refused++;}expect(source).toBe(JSON.stringify(row.value));}
 for(const row of fixture.rawRefused){await expect(readers[row.kind as keyof typeof readers](row.text,control())).rejects.toThrow();expect(()=>JSON.parse(row.text)).not.toThrow();}
 console.log(`[DEBUG] DAG TypeScript actual admission: accepted=${accepted} refused=${refused} duplicate=${fixture.rawRefused.length}; independent Ajv and JSON.parse`);
});

test("DAG authority refuses every observed cancellation boundary and zero ownership before publication",async()=>{
 const source=JSON.stringify(fixture.accepted[1]!.value);let observed=0;
 await decodeDagSelectionJson(source,control(65536,()=>{observed++;return true}));
 for(let boundary=1;boundary<=observed;boundary++){let current=0;await expect(decodeDagSelectionJson(source,control(65536,()=>++current<boundary))).rejects.toThrow();expect(source).toBe(JSON.stringify(fixture.accepted[1]!.value));}
 await expect(decodeDagSelectionJson(source,control(0))).rejects.toThrow();
 const base=control();let interrupted=false;const raw={...base,syntax:{...base.syntax,progress:()=>{interrupted=true},cancelled:()=>interrupted}};
 await expect(decodeDagSelectionJson(source,raw)).rejects.toThrow();
 console.log(`[DEBUG] DAG TypeScript authority: cancellationBoundaries=${observed} zeroOwnership=true syntaxCancellation=true acceptedSourcePreserved=true`);
});

test("hover admission preserves accepted facts and refuses every cancellation boundary",async()=>{
 const row=fixture.accepted.filter(row=>row.kind==="hover").at(-1)!;const source=JSON.stringify(row.value);let observations=0;const admitted=await decodeDagHoverJson(source,control(65536,()=>{observations++;return true}));
 for(let stop=1;stop<=observations;stop++){let seen=0;await expect(decodeDagHoverJson(source,control(65536,()=>++seen<stop))).rejects.toThrow();expect(admitted).toEqual(JSON.parse(source));}
 await expect(decodeDagHoverJson(source,control(0))).rejects.toThrow();expect(admitted).toEqual(JSON.parse(source));console.log(`[DEBUG] Actual hover TS authority: cancellationBoundaries=${observations} zeroOwnershipRefused=true acceptedFactsPreserved=true independentJSON=true`);
});

test("DAG physical strings preserve literal map identities and refuse non-Unicode escaped strings",async()=>{
 const source='{"__proto__":{"status":"ok"},"constructor":{"status":"error","message":"雪 ☃"}}';
 const value=await decodeDagNodeStatusesJson(source,control());expect(Object.getPrototypeOf(value)).toBeNull();expect(value).toEqual(JSON.parse(source));
 for(const text of ['{"nodes":["\\ud800"],"edges":[],"handles":[]}','{"n":{"status":"error","message":"\\udfff"}}'])await expect((text.includes('"nodes"')?decodeDagSelectionJson:decodeDagNodeStatusesJson)(text,control())).rejects.toThrow();
 console.log("[DEBUG] DAG native-compatible Unicode refusal and prototype-neutral identities executed");
});

import selectionFixture from "../../../../🔌️ports/➡️directed/🕸️dag/🧫️fixtures/🎯️selected-json.json";
import {DagSelectionJsonCursor,DagSelectionTextError,type DagSelectionTextGrant} from "../📤️selection/🟦️.ts";

test("retained selection physical cursor preserves original neutral bytes and every guard frontier",()=>{
 const grant:DagSelectionTextGrant={fuel:1,nowMilliseconds:1,deadlineMilliseconds:8,cancelled:false,interrupted:false};let grants=0,refusals=0;
 for(const row of selectionFixture.cases)for(const domain of ["nodes","edges"]as const){const source={selectionCandidateCount:()=>row.ids.length,selectionCandidateId:(_:unknown,index:number)=>row.ids[index]},cursor=new DagSelectionJsonCursor(domain),output:number[]=[];let census:number|undefined;
 while(true){for(const [bad,fault]of [[{...grant,fuel:0},"NoFuel"],[{...grant,cancelled:true},"Cancelled"],[{...grant,interrupted:true},"Interrupted"],[{...grant,nowMilliseconds:8},"Deadline"]]as const){try{cursor.step(source,bad);throw Error("Expected authority refusal")}catch(error){expect(error).toBeInstanceOf(DagSelectionTextError);expect((error as DagSelectionTextError).fault).toBe(fault);refusals++;}}grants++;const next=cursor.step(source,grant);if(next.kind==="byte")output.push(next.byte);else if(next.kind==="census")census=next.bytes;else if(next.kind==="complete")break;}
 const expected=new TextEncoder().encode(JSON.stringify(row.ids));expect(new Uint8Array(output)).toEqual(expected);expect(census).toBe(expected.length);expect(cursor.step(source,grant)).toEqual({kind:"complete"});}
 const text="x".repeat(65536),source={selectionCandidateCount:()=>1,selectionCandidateId:()=>text},cursor=new DagSelectionJsonCursor("nodes");expect(()=>{while(true){const next=cursor.step(source,grant);expect(next.kind).toBe("progress");}}).toThrow();
 console.log(`[DEBUG] Actual TS retained selection IO: originalCases=3 domains=2 singleUnitGrants=${grants} guardRefusals=${refusals} exactJSONBytes=true ceilingBeforeOutput=true`);
});
