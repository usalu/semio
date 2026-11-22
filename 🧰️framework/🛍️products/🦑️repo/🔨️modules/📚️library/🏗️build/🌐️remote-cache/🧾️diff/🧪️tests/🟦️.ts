import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {brotliCompressSync} from "node:zlib";
import {resolve} from "node:path";
import {fileURLToPath} from "node:url";
import {diffEvidenceV1,evidenceLinesV1} from "../🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};

/** 🔬️ The comparison of the two sides of a divergence equals the fixture, and the fixture equals an independent Python computation. */
const here=fileURLToPath(new URL(".",import.meta.url)),row=(text:string)=>({sha256:"0".repeat(64),bytes:new Uint8Array(Buffer.from(text)),size:Buffer.byteLength(text)});

test("every fixture case gives the recorded first differing byte, count of differing bytes and lines",()=>{
 for(const item of fixture.cases){
  const result=diffEvidenceV1("p",row(item.kept),row(item.refused));
  expect(result.firstDifferenceByte,item.name).toBe(item.firstDifferenceByte);expect(result.differingBytes,item.name).toBe(item.differingBytes);expect(result.lines,item.name).toEqual(item.lines);
 }
});

test("an independent Python computation gives the same answers on every fixture case",()=>{
 const python=spawnSync("python3",["-I",resolve(here,"🐍️.py"),resolve(here,"🧫️fixtures/🔣️.json")],{encoding:"utf8"});
 if(python.error===undefined)expect(python.stdout.trim(),python.stderr).toBe("ok "+fixture.cases.length);
});

test("Brotli is unpacked before the lines are compared, a binary or missing side is named by its digest, and the lines are bounded",()=>{
 const packed=(text:string)=>({sha256:"1".repeat(64),bytes:new Uint8Array(brotliCompressSync(Buffer.from(text))),size:9});
 expect(diffEvidenceV1("p",packed("{\"a\":1}"),packed("{\"a\":2}")).lines).toEqual(["- \"a\": 1","+ \"a\": 2"]);
 const binary={sha256:"2".repeat(64),bytes:new Uint8Array([0,1,2]),size:3},other={sha256:"3".repeat(64),bytes:new Uint8Array([0,1,9]),size:3};
 expect(diffEvidenceV1("p",binary,other)).toMatchObject({firstDifferenceByte:2,differingBytes:1,lines:[]});
 expect(diffEvidenceV1("p",{sha256:"4".repeat(64),bytes:undefined,size:7},row("x"))).toMatchObject({kept:{stored:false,bytes:7},refused:{stored:true},firstDifferenceByte:null,lines:[]});
 expect(diffEvidenceV1("p",undefined,row("x")).kept).toBeNull();
 expect(evidenceLinesV1(Array.from({length:50},(_,index)=>"a"+index).join("\n"),"").length).toBe(20);
 expect(evidenceLinesV1("x".repeat(500),"")[0]!.length).toBe(201);
});
