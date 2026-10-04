import {readFileSync} from "node:fs";
import {join} from "node:path";

/** 🧮️ Checks closed wire grammar and copied UTF-8 independently of native allocation widths. */
export async function proveWireValueMaterializationFixture(repoRoot:string):Promise<void>{
    const assert=(await import("node:assert/strict")).default,Ajv=(await import("ajv")).default,equal=(await import("fast-deep-equal")).default;
    const {Database}=await import("bun:sqlite"),root=join(repoRoot,"🧰️framework/🔨️modules/🎒️pack/🌱️value");
    const fixture=JSON.parse(readFileSync(join(root,"🧫️fixtures/🧮️wire-materialization/🔣️.json"),"utf8")),contract=JSON.parse(readFileSync(join(root,"🧬️schema/🔣️.json"),"utf8"));
    const ajv=new Ajv({strict:true,allErrors:true});ajv.addSchema(contract);const validate=ajv.getSchema(`${contract.$id}#/$defs/WireValueMaterialization`)!;
    assert(validate(fixture),JSON.stringify(validate.errors));assert.deepEqual(fixture.cases.map((row:{id:string})=>row.id),contract.$defs.WireValueMaterializationCase.properties.id.enum);assert.deepEqual(fixture.bridge,{fieldId:1,rootTag:"11",exact:true});
    const {decodePackValue,packValueToExactJson}=await import(join(repoRoot,"🧰️framework/🛍️products/💻️os/🟦️.ts")),database=new Database(":memory:");
    try{
        const utf8=database.query<{bytes:number},[string]>("SELECT length(CAST(? AS BLOB)) AS bytes");
        const grammar=(hex:string,limits:{maxFileLen:number;maxSegmentLen:number;maxSymbols:number;maxDepth:number;maxItems:number})=>{
            const bytes=Buffer.from(hex,"hex"),symbols:string[]=[];let offset=0,owned=0n,sqlOwned=0n;
            const refuse=(outcome:"limit"|"malformed"|"truncated"):never=>{throw outcome==="truncated"?{outcome,offset}:{outcome};};
            const copied=(text:string)=>{owned+=BigInt(Buffer.byteLength(text,"utf8"));sqlOwned+=BigInt(utf8.get(text)!.bytes);return text;};
            const byte=()=>{if(offset>=bytes.length)return refuse("truncated");return bytes[offset++];};
            const integer=()=>{let value=0n;for(let index=0;index<10;index++){const next=byte();if(index===9&&(next&0xfe)!==0)return refuse("malformed");value|=BigInt(next&0x7f)<<BigInt(index*7);if((next&0x80)===0)return value;}return refuse("malformed");};
            const text=()=>{const length=integer();if(length>BigInt(limits.maxSegmentLen))return refuse("limit");if(length>BigInt(bytes.length-offset))return refuse("truncated");const start=offset;offset+=Number(length);let value:string;try{value=new TextDecoder("utf-8",{fatal:true}).decode(bytes.subarray(start,offset));}catch{return refuse("malformed");}return copied(value);};
            const string=(tag:number):string=>{if(tag===7)return text();if(tag!==6)return refuse("malformed");const index=integer();if(index>=BigInt(symbols.length))return refuse("malformed");return copied(symbols[Number(index)]);};
            const value=(depth:number):unknown=>{
                if(depth>limits.maxDepth)return refuse("limit");const tag=byte();if(tag===0x12)return null;if(tag===1||tag===2)return tag===2;if(tag===3){const raw=integer();return Number((raw>>1n)^-(raw&1n));}if(tag===4)return Number(integer());
                if(tag===5){if(bytes.length-offset<8)return refuse("truncated");const number=new DataView(bytes.buffer,bytes.byteOffset+offset,8).getFloat64(0,true);offset+=8;return number;}
                if(tag===6||tag===7)return string(tag);if(tag!==0x0c&&tag!==0x10)return refuse("malformed");const count=integer();if(count>BigInt(limits.maxItems))return refuse("limit");
                const values:unknown[]=[],entries:[string,unknown][]=[];for(let index=0n;index<count;index++){if(tag===0x0c)values.push(value(depth+1));else entries.push([string(byte()),value(depth+1)]);}return tag===0x0c?values:Object.fromEntries(entries);
            };
            let result:{outcome:string;value?:unknown};try{
                if(bytes.length>limits.maxFileLen)refuse("limit");const count=integer();if(count>BigInt(limits.maxSymbols))refuse("limit");for(let index=0n;index<count;index++)symbols.push(text());
                if(integer()!==1n||integer()!==BigInt(fixture.bridge.fieldId)||byte()!==0x11)refuse("malformed");const decoded=value(0);if(offset!==bytes.length)refuse("malformed");result={outcome:"accepted",value:decoded};
            }catch(error){if(!error||typeof error!=="object"||!("outcome"in error))throw error;result=error as{outcome:string};}
            assert.equal(sqlOwned,owned,"independent SQLite UTF-8 census");return{result,owned};
        };
        for(const row of fixture.cases){
            const actual=grammar(row.rawHex,row.limits);assert.deepEqual(actual.result,row.grammar,row.id);assert.equal(actual.owned,BigInt(row.ownedUtf8Bytes),row.id+" copied UTF-8");
            const mode=row.physicalAllowance.mode;if(mode==="oneByteShort")assert.equal(row.expect.outcome,"limit",row.id+" physical refusal");else assert.deepEqual(row.expect,row.grammar,row.id+" grammar outcome");
            if(actual.result.outcome==="accepted"){assert(equal(actual.result.value,row.grammar.value),row.id+" independent output");assert(equal(packValueToExactJson(decodePackValue(Buffer.from(row.rawHex,"hex"))),row.grammar.value),row.id+" first-party semantic output");assert.deepEqual(JSON.parse(JSON.stringify(actual.result.value)),row.grammar.value,row.id+" JSON oracle");}
            if(row.allowanceProbeHex){const probe=grammar(row.allowanceProbeHex,{...row.limits,maxFileLen:Buffer.from(row.allowanceProbeHex,"hex").length});assert.deepEqual(probe.result,{outcome:"accepted",value:null});assert.equal(probe.owned,actual.owned);assert.equal(packValueToExactJson(decodePackValue(Buffer.from(row.allowanceProbeHex,"hex"))),null);}
        }
        console.log(`[DEBUG] wire materialization: AJV=1 independent-grammar=${fixture.cases.length} SQLite-UTF8=${fixture.cases.length} first-party-semantic=1; native backing requires owning allocator execution`);
    }finally{database.close();}
}
