import {test,expect} from "bun:test";
import vectors from "../../🧫️fixtures/🔣️rfc4648-base64-vectors.json";
import urlVectors from "../../🧫️fixtures/🔣️rfc4648-base64url-vectors.json";
import {base64StandardDecode,base64StandardEncode,base64UrlDecode,base64UrlEncode,decodeBase64Quad,Base64DecodeError} from "../../🟦️.ts";
test("standard base64 corpus and shared quartets agree with the system codec",()=>{
 for(const row of vectors.cases){expect(base64StandardEncode(Buffer.from(row.input_utf8))).toBe(row.encoded);expect(Buffer.from(base64StandardDecode(row.encoded)).toString()).toBe(row.input_utf8);}
 for(let length=0;length<=128;length++){const bytes=Uint8Array.from({length},(_,i)=>(i*73+length*19)&255),text=Buffer.from(bytes).toString("base64"),out=new Uint8Array(length);let at=0;
  for(let i=0;i<text.length;i+=4)at+=decodeBase64Quad(Array.from(text.slice(i,i+4),c=>c.charCodeAt(0)),i,i+4===text.length,out,at);
  expect(at).toBe(length);expect(out).toEqual(bytes);expect(base64StandardDecode(text)).toEqual(bytes);expect(base64StandardEncode(bytes)).toBe(text);
 }
});
test("malformed quartets preserve caller storage",()=>{
 for(const text of ["=m9v","Zm=v","Zh==","Zm9=","Z g=","Zm_=","Zm9é"]){const out=new Uint8Array([42,43,44]);expect(()=>decodeBase64Quad(Array.from(text,c=>c.charCodeAt(0)),0,true,out)).toThrow(Base64DecodeError);expect(Array.from(out)).toEqual([42,43,44]);}
 for(const text of ["Zg==","Zm8="]){const out=new Uint8Array([42,43,44]);expect(()=>decodeBase64Quad(Array.from(text,c=>c.charCodeAt(0)),0,false,out)).toThrow();expect(Array.from(out)).toEqual([42,43,44]);}
 const out=new Uint8Array([42,43]);expect(()=>decodeBase64Quad([90,109,57,118],0,true,out)).toThrow();expect(Array.from(out)).toEqual([42,43]);
});
test("base64url corpus remains distinct from standard transport",()=>{
 for(const row of urlVectors.cases){const bytes=Buffer.from(row.input_hex,"hex");expect(base64UrlEncode(bytes)).toBe(row.encoded);expect(Buffer.from(base64UrlDecode(row.encoded))).toEqual(bytes);expect(row.encoded).toBe(bytes.toString("base64url"));}
 for(const row of urlVectors.rejected)expect(()=>base64UrlDecode(row.encoded)).toThrow();
});
