/** 🪢️ Contract: the facts of every owner of a closure that list the same paths hold those strings once, and the shared copy equals the original.
 * @see ../🟦️.ts */
import{expect,test}from"bun:test";
import{createStringSharingV1}from"../🟦️.ts";

const MiB=1048576;
const scope=(owners:number,files:number)=>{
 const names=Array.from({length:files},(_,index)=>"✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/src/file-"+index+".rs");
 return Array.from({length:owners},(_,owner)=>JSON.parse(JSON.stringify({project:{name:"owner-"+owner,targets:{build:{inputs:names.slice(0,50)}}},files:names,observedFiles:names.slice(0,200),dependencies:["a","b"]})) as {files:string[];project:{name:string}});
};
const heap=()=>{Bun.gc(true);return process.memoryUsage().heapUsed/MiB;};

test("the shared copy equals the original and shares equal strings, keys and arrays' elements",()=>{
 const sharing=createStringSharingV1(),[first,second]=scope(2,5),a=sharing.share(first!),b=sharing.share(second!);
 expect(a).toEqual(first!);expect(b).toEqual(second!);
 expect(a.files.length).toBe(5);expect(sharing.size).toBeLessThan(5+5+10);
 expect(sharing.share({n:1,t:true,z:null,list:[1,"x"]})).toEqual({n:1,t:true,z:null,list:[1,"x"]});
});

test("a closure of 600 owners that list the same 1500 paths keeps the heap of one list, not of 600",()=>{
 const owners=600,files=1500,before=heap();
 let unshared:ReturnType<typeof scope>|undefined=scope(owners,files);
 const held=heap()-before,sharing=createStringSharingV1(),shared=unshared.map(fact=>sharing.share(fact));
 unshared=undefined;
 const retained=heap()-before;
 expect(shared.length).toBe(owners);
 expect(held).toBeGreaterThan(100);
 expect(retained).toBeLessThan(held/4);
 expect(retained).toBeLessThan(120);
});
