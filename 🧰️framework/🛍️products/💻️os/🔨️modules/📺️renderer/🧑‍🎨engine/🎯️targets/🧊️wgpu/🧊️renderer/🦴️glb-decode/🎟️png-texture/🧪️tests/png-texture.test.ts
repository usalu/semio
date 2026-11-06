import {expect,test} from "bun:test";
import Ajv from "ajv";
import {inflateSync} from "node:zlib";
import corpus from "./cases.json";
import schema from "./schema.json";

test("original renderer PNG source has the independently decoded retained pixels",()=>{
 expect(new Ajv({strict:true}).compile(schema)(corpus)).toBe(true);
 const bytes=Buffer.from(corpus.data,"base64"),idat:Buffer[]=[];
 expect(bytes.readUInt32BE(16)).toBe(corpus.width);expect(bytes.readUInt32BE(20)).toBe(corpus.height);
 expect(bytes[24]).toBe(1);expect(bytes[25]).toBe(0);
 for(let at=8;at<bytes.length;){const length=bytes.readUInt32BE(at),kind=bytes.toString("ascii",at+4,at+8);if(kind==="IDAT")idat.push(bytes.subarray(at+8,at+8+length));at+=length+12;}
 const rows=inflateSync(Buffer.concat(idat)),rgba:number[]=[];
 for(let y=0;y<corpus.height;y++){expect(rows[y*2]).toBe(0);for(let x=0;x<corpus.width;x++){const gray=(rows[y*2+1]!>>(7-x))&1;rgba.push(gray*255,gray*255,gray*255,255);}}
 expect(rgba).toEqual(corpus.rgba);
});
