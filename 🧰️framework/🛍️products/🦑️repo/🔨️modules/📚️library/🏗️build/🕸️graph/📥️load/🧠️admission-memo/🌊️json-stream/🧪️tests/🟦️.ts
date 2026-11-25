/** 🌊️ Contract: a JSON object read in chunks yields exactly what `JSON.parse` of its whole text holds, whatever the chunk size cuts through.
 * @see ../🟦️.ts */
import{expect,test}from"bun:test";
import{mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{open}from"node:fs/promises";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{streamJsonObjectV1,type JsonStreamEventV1}from"../🟦️.ts";

const sample={version:2,text:"a\"b\\c,]}\u00e9\u{1F600}\n",admission:{owners:["o"],nested:{deep:[1,{"k]":"}"}],flag:true,none:null}},observations:[{kind:"file",path:"/w/ü/😀.ts",fact:{n:-1.5e3}},"x",[1,2,[3]],7,true,null,{}],owners:[{suffix:"o\0[]",facts:{a:"]"},rows:[0,1]}],empty:[],last:-0.25};
const read=async(text:string,chunk:number)=>{
 const root=mkdtempSync(join(tmpdir(),"json-stream-"));
 try{
  const path=join(root,"x.json");writeFileSync(path,text);
  const file=await open(path,"r"),events:JsonStreamEventV1[]=[];
  try{for await(const event of streamJsonObjectV1(file,new Set(["observations","owners","empty"]),chunk))events.push(event);}finally{await file.close();}
  return events;
 }finally{rmSync(root,{recursive:true,force:true});}
};
const fold=(events:JsonStreamEventV1[])=>{
 const value:Record<string,unknown>={};
 for(const event of events){
  if("value" in event)value[event.key]=event.value;
  else if("element" in event)((value[event.key]??=[]) as unknown[]).push(event.element);
  else value[event.key]??=[];
 }
 return value;
};

test("every chunk size, down to one byte, folds back to the value JSON.parse reads from the same text",async()=>{
 for(const text of [JSON.stringify(sample),JSON.stringify(sample,null,2)])for(const chunk of [1,2,3,5,7,64,1<<20])expect(fold(await read(text,chunk))).toEqual(JSON.parse(text));
});

test("the elements of a streamed array come one by one in order and the array ends with an end event",async()=>{
 const events=await read(JSON.stringify(sample),11),keys=events.map(event=>("element" in event?"element:":"end" in event?"end:":"value:")+event.key);
 expect(keys.filter(key=>key==="element:observations").length).toBe(sample.observations.length);
 expect(keys.indexOf("end:observations")).toBeGreaterThan(keys.lastIndexOf("element:observations"));
 expect(keys).toContain("end:empty");
});

test("a text that is not one whole JSON object throws",async()=>{
 for(const text of ["[1]","{\"a\":1","{\"a\" 1}","{\"a\":1} x","","{\"observations\":[1,}"])await expect(read(text,3)).rejects.toThrow();
});
