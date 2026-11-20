/** 🌊️ Contract: a graph authority receipt is read record by record. A receipt far beyond the node bound of one record (`maximumNodes`) streams through with bounded memory, every record equals what `JSON.parse` yields, and a malformed or truncated receipt is refused. */
import{expect,test}from"bun:test";
import{createAuthorityStreamV1}from"../../🌊️authority-stream/🟨️.mjs";
import schema from"../../🧬️schema/🔣️.json";

const P=schema.definitions.Protocol.const,hex="ab".repeat(32);
const collect=(text:string,sizes:readonly number[],limit=P.chunkBytes*16)=>{
 const header:Record<string,unknown>={},observations:unknown[]=[],stream=createAuthorityStreamV1(limit,{header(key,value){header[String(key)]=value;},observation(value){observations.push(value);}});
 for(let at=0,step=0;at<text.length;step++){const size=sizes[step%sizes.length]!;stream.feed(text.slice(at,at+size));at+=size;}
 stream.end();return{header,observations};
};
const receipt=(observations:unknown[],order:"last"|"first"|"middle"="last")=>{
 const head={version:4,module:{implementation:hex,publication:hex},membership:hex,graph:hex},seal={version:1,mode:"quiet",rounds:0,sealedWithLaterChanges:0,laterChangedOwners:[],membershipMoved:false,admissionMoved:false,owners:{workspace:hex}};
 return JSON.stringify(order==="last"?{...head,seal,observations}:order==="first"?{observations,...head,seal}:{version:4,observations,module:head.module,membership:hex,graph:hex,seal});
};
const rows=Array.from({length:50},(_,i)=>({kind:i%2?"file":"lstat",path:"a/\"q\"\\\\é🧪/"+i+"],{[",fact:i%3?{size:i,text:"x,}]\\\"y"}:null}));

test("every record equals JSON.parse for any chunking and any key order, including quotes, brackets and escapes inside strings",()=>{
 for(const order of["last","first","middle"]as const){
  const text=receipt(rows,order),oracle=JSON.parse(text);
  for(const sizes of[[1],[2,3],[7],[64],[text.length]]){
   const read=collect(text,sizes);
   expect(read.observations,order+sizes).toEqual(oracle.observations);
   const{observations:_,...expected}=oracle;expect(read.header,order+sizes).toEqual(expected);
  }
 }
});

test("a receipt over the node bound of one record streams with bounded memory",()=>{
 const count=Math.ceil(P.maximumNodes/5)+1000;let seen=0,peak=0;
 const stream=createAuthorityStreamV1(P.chunkBytes*16,{header(){},observation(value:any){if(value.kind!=="lstat"||value.fact!==true)throw Error("record");seen++;if(!(seen%100000))peak=Math.max(peak,process.memoryUsage().heapUsed);}});
 Bun.gc(true);const before=process.memoryUsage().heapUsed;
 stream.feed("{\"version\":3,\"observations\":[");
 for(let at=0;at<count;at+=5000){let block="";for(let i=at;i<Math.min(count,at+5000);i++)block+=(i?",":"")+"{\"fact\":true,\"kind\":\"lstat\",\"path\":\"p/"+i+"\"}";stream.feed(block);}
 stream.feed("],\"graph\":\""+hex+"\"}");stream.end();
 expect(seen).toBe(count);expect(count*5).toBeGreaterThan(P.maximumNodes);
 expect(Math.max(0,peak-before)).toBeLessThan(256*1048576);
},120000);

test("a malformed, truncated, oversized-record or trailing receipt is refused",()=>{
 const good=receipt(rows),message="Closed streamed authority DATA required";
 for(const bad of[good.slice(0,-1),good+"x","[]","{\"observations\":[{\"a\":1},]}","{\"version\":3}","{\"observations\":{}}","{\"observations\":[1 2]}",good.replace("\"module\"","\"mod\\u0075le\"").replace("\"version\":3","\"version\":3,\"observations\":[]")])expect(()=>collect(bad,[5]),bad.slice(0,40)).toThrow();
 expect(()=>collect(receipt([{kind:"file",path:"x".repeat(100),fact:1}]),[9],50)).toThrow(message);
});


test("authority streams ordered root indexes and refuses an oversized record within one chunk",()=>{
 const indexed=[{kind:"file",path:"/w/source",fact:"digest"},0,0],image=receipt(indexed);
 expect(collect(image,[1,7,19],P.chunkBytes).observations).toEqual(JSON.parse(image).observations);
 const oversized=receipt([{kind:"file",path:"x".repeat(10000),fact:0},0]);
 expect(()=>collect(oversized,[oversized.length],2048)).toThrow("Closed streamed authority DATA required");
});
