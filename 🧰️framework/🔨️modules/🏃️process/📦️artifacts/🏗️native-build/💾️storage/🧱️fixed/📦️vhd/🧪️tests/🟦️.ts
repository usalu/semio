import{expect,test}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import{readFileSync}from"node:fs";
import ts from"typescript";
import schema from"../🧬️schema.json"with{type:"json"};
import corpus from"./🔣️.json"with{type:"json"};
import{encodeNativeFixedVhdFooterV1}from"../🟦️.ts";

const footerSql=`WITH RECURSIVE
input(bytes,stamp,uuid) AS (VALUES(?1,?2,upper(?3))),
bounded AS (SELECT *,min(bytes/512,65535*16*255) AS sectors FROM input),
initial AS (SELECT *,sectors/17 AS tracks,max(4,(sectors/17+1023)/1024) AS head17 FROM bounded),
selected AS (SELECT *,CASE WHEN sectors>=65535*16*63 THEN 255 WHEN tracks>=head17*1024 OR head17>16 THEN CASE WHEN sectors/31>=16*1024 THEN 63 ELSE 31 END ELSE 17 END AS spt FROM initial),
geometry AS (SELECT *,CASE WHEN spt=17 THEN head17 ELSE 16 END AS heads FROM selected),
encoded AS (SELECT '636F6E6563746978'||printf('%08X',2)||printf('%08X',65536)||'FFFFFFFFFFFFFFFF'||printf('%08X',stamp)||'736D696F'||printf('%08X',65536)||'5769326B'||printf('%016X',bytes)||printf('%016X',bytes)||printf('%04X%02X%02X',sectors/(spt*heads),heads,spt)||printf('%08X',2)||'00000000'||uuid||'00'||hex(zeroblob(427)) AS value FROM geometry),
positions(n) AS (VALUES(1) UNION ALL SELECT n+2 FROM positions WHERE n<1023),
summed AS (SELECT value,sum((instr('0123456789ABCDEF',substr(value,n,1))-1)*16+instr('0123456789ABCDEF',substr(value,n+1,1))-1) AS total FROM encoded,positions)
SELECT substr(value,1,128)||printf('%08X',(~total)&4294967295)||substr(value,137) AS footerHex FROM summed`;

test("fixed VHD footer matches independent SQLite byte encoding for the closed corpus",()=>{
 const ajv=new Ajv({strict:true}),validate=ajv.compile({...schema,$ref:"#/definitions/Request"}),validateCorpus=ajv.compile({...schema,$id:"semio:native-fixed-vhd-corpus:v1",$ref:"#/definitions/Corpus"}),db=new Database(":memory:");
 try{expect(validateCorpus(corpus)).toBe(true);const oracle=db.query<{footerHex:string},[number,number,string]>(footerSql),envelope=db.query<{accepted:number},[string]>("SELECT coalesce(json_extract(?1,'$.filesystemBytes')+1048576<=json_extract(?1,'$.maximumPhysicalBytes'),0) AS accepted"),source=readFileSync(new URL("../../🟦️.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("fixed.ts",source,ts.ScriptTarget.Latest,true),parser=tree.statements.find(row=>ts.isFunctionDeclaration(row)&&row.name?.text==="nativeFixedVhdFooterV1");if(!parser)throw Error("Actual first-party footer parser required");const parse=new Function("Buffer",ts.transpileModule(parser.getText(tree).replace("export ",""),{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.None}}).outputText+";return nativeFixedVhdFooterV1;")(Buffer)as(footer:Uint8Array,file:number,maximum:number,filesystem:number)=>number;
 for(const row of corpus.cases){const accepted=validate(row.request)&&Boolean(envelope.get(JSON.stringify(row.request))?.accepted);expect(accepted).toBe(row.accepted);if(!accepted){expect(()=>encodeNativeFixedVhdFooterV1(row.request)).toThrow();continue;}const request=row.request as{filesystemBytes:number;maximumPhysicalBytes:number;timestampSeconds:number;uuid:string},before=JSON.stringify(request),footer=encodeNativeFixedVhdFooterV1(request),expected=oracle.get(request.filesystemBytes,request.timestampSeconds,request.uuid);if(!expected)throw Error("Independent SQLite footer output required");expect(footer.byteLength).toBe(512);expect(Buffer.from(footer).toString("hex").toUpperCase()).toBe(expected.footerHex);expect(JSON.stringify(request)).toBe(before);expect(parse(footer,request.filesystemBytes+512,request.maximumPhysicalBytes,request.filesystemBytes)).toBe(request.filesystemBytes);const repeat=encodeNativeFixedVhdFooterV1(request);expect(repeat).toEqual(footer);footer[0]=footer[0]!^1;expect(repeat[0]).toBe(99);expect(Buffer.from(footer).toString("hex").toUpperCase()).not.toBe(expected.footerHex);expect(()=>parse(footer,request.filesystemBytes+512,request.maximumPhysicalBytes,request.filesystemBytes)).toThrow();}
 console.log("[DEBUG] Fixed VHD footer: 40 schema rows, exact 512-byte independent SQLite output, actual first-party parser; no image allocation or kernel interoperability claim");
 }finally{db.close();}
});

test("fixed VHD encoding retains private primitive inputs and schema facts",()=>{
 const request={version:1,filesystemBytes:33554432,maximumPhysicalBytes:34603008,timestampSeconds:0,uuid:"00112233445546778899aabbccddeeff"},baseline=encodeNativeFixedVhdFooterV1(request);
 for(const law of corpus.stability){if(law==="accessor-refused"){let reads=0;const changing={...request,get filesystemBytes(){reads++;return reads===1?33554432:1099511627776;}};expect(()=>encodeNativeFixedVhdFooterV1(changing)).toThrow();expect(reads).toBe(0);}else{const features=schema.definitions.Protocol.const.features,minimum=schema.definitions.Request.properties.filesystemBytes.minimum;try{schema.definitions.Protocol.const.features=0;schema.definitions.Request.properties.filesystemBytes.minimum=0;expect(encodeNativeFixedVhdFooterV1(request)).toEqual(baseline);expect(()=>encodeNativeFixedVhdFooterV1({...request,filesystemBytes:1048576})).toThrow();}finally{schema.definitions.Protocol.const.features=features;schema.definitions.Request.properties.filesystemBytes.minimum=minimum;}}}console.log("[DEBUG] Closed VHD stability laws refuse changing accessors before reads and retain private schema snapshots");
});
