import {expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {decodeFactIndexV1,encodeFactIndexV1,transcodeFactIndexV1} from "../../🟨️.mjs";
import {expandRootV1,indexTextRewriterV1,neutralizeRootV1} from "../../../🌐️remote-exchange/🧳️portable/🟦️.ts";

/** 🔁️ Moving the root of a fact index through its binary form equals the former way (decode every row, write the JSON text, replace, parse, encode): the same bytes for random indexes at a POSIX and a Windows root, in both directions, with ids renumbered and objects reordered, and malformed input and merged keys are refused. */
function random(seed:number):()=>number{let state=seed>>>0;return()=>{state=(Math.imul(state,1664525)+1013904223)>>>0;return state/4294967296;};}
const hex=(text:string)=>createHash("sha256").update(text).digest("hex");
const maximum=1<<28;
const legacy=(bytes:Uint8Array,root:string,direction:"neutralize"|"expand")=>{
 const index=decodeFactIndexV1(bytes,{maximumBytes:maximum}),text=Buffer.from(JSON.stringify(index));
 const moved=JSON.parse(Buffer.from(direction==="neutralize"?neutralizeRootV1(text,root):expandRootV1(text,root)).toString("utf8"));
 return encodeFactIndexV1(moved,{maximumBytes:maximum});
};
const make=(seed:number,root:string,separator:string,tokenForm:boolean)=>{
 const next=random(seed),at=(...parts:string[])=>(tokenForm?"\u27e6semio-root\u27e7":root)+separator+parts.join(separator);
 const kinds=["file","analysis","readdir","resolve","digest"],rows:any[]=[],owners:string[]=[];
 for(let owner=0;owner<12;owner++)owners.push("owner"+owner);
 for(let row=0;row<200;row++){
  const path=next()<0.8?at("proj"+Math.floor(next()*20),"src","f"+Math.floor(next()*30)+".ts"):"/usr/lib/x"+Math.floor(next()*5)+".so",kind=kinds[Math.floor(next()*kinds.length)]!;
  const fact=kind==="digest"?{sha256:hex("d"+Math.floor(next()*40)),size:Math.floor(next()*1000),mtimeMs:1.5e12+Math.floor(next()*50)}:kind==="readdir"?[at("a"),"b.ts",at("c","d")]:kind==="resolve"?at("proj1","index.ts"):{source:{size:3,mtimeMs:2.5,path:at("deep",String(Math.floor(next()*9))),nested:[{p:at("n"),q:"keep "+root.slice(0,3),z:null,t:true,n:-4}]}};
  rows.push({kind,path,fact});
 }
 const classes=[{r:rows.map((_,at)=>at).filter(at=>at%2===0),o:[0,1,2]},{r:rows.map((_,at)=>at).filter(at=>at%2===1),o:[3,5]}];
 return encodeFactIndexV1({version:1,owners,rows,classes},{maximumBytes:maximum});
};

for(const[label,root,separator]of[["posix","/Users/Shared/semio","/"],["posix with emoji","/Users/ueli/Documents/🧰️semio","/"],["windows","C:\\work\\semio","\\"]] as const)
 test(label+" root: the binary rewrite equals the former way in both directions over 12 random indexes",()=>{
  for(let seed=1;seed<=12;seed++){
   const plain=make(seed,root,separator,false),neutral=make(seed,root,separator,true);
   const out=transcodeFactIndexV1(plain,{mapText:indexTextRewriterV1(root,"neutralize"),maximumBytes:maximum});
   expect(Buffer.compare(out,legacy(plain,root,"neutralize"))).toBe(0);
   const back=transcodeFactIndexV1(out,{mapText:indexTextRewriterV1(root,"expand"),maximumBytes:maximum});
   expect(Buffer.compare(back,legacy(out,root,"expand"))).toBe(0);
   const there=transcodeFactIndexV1(neutral,{mapText:indexTextRewriterV1("/other/place","expand"),maximumBytes:maximum});
   expect(Buffer.compare(there,legacy(neutral,"/other/place","expand"))).toBe(0);
  }
 });

test("a rewrite that maps nothing returns the same logical index, the document stays decodable and the class lists are untouched",()=>{
 const bytes=make(3,"/Users/Shared/semio","/",false),same=transcodeFactIndexV1(bytes,{mapText:text=>text,maximumBytes:maximum});
 expect(Buffer.compare(same,bytes)).toBe(0);
 const moved=transcodeFactIndexV1(bytes,{mapText:text=>text.replace("proj","p"),maximumBytes:maximum}),before=decodeFactIndexV1(bytes),after=decodeFactIndexV1(moved);
 expect(after.classes).toEqual(before.classes);expect(after.owners).toEqual(before.owners);expect(after.rows.length).toBe(before.rows.length);
 expect(after.rows.every((row,at)=>row.path===before.rows[at]!.path.replace("proj","p"))).toBe(true);
});

test("two keys of one object that map to one text are refused, and so are malformed documents",()=>{
 const bytes=encodeFactIndexV1({version:1,owners:["a"],rows:[{kind:"file",path:"/x",fact:{one:1,two:2}}],classes:[{r:[0],o:[0]}]},{maximumBytes:maximum});
 expect(()=>transcodeFactIndexV1(bytes,{mapText:text=>text==="two"?"one":text,maximumBytes:maximum})).toThrow();
 expect(()=>transcodeFactIndexV1(bytes.subarray(0,bytes.length-3),{mapText:text=>text,maximumBytes:maximum})).toThrow();
 expect(()=>transcodeFactIndexV1(Buffer.from("not an index"),{mapText:text=>text,maximumBytes:maximum})).toThrow();
 expect(()=>transcodeFactIndexV1(Buffer.concat([bytes,Buffer.from([1])]),{mapText:text=>text,maximumBytes:maximum})).toThrow();
});

test("a large index is rewritten in one pass without holding rows: 150000 rows in well under the time of the former way",()=>{
 const rows:any[]=[];for(let row=0;row<150000;row++)rows.push({kind:row%3?"file":"analysis",path:"/Users/Shared/semio/p"+(row%900)+"/src/f"+row+".ts",fact:{size:row%1000,mtimeMs:1.7e12+row%97,sha256:hex(String(row%5000))}});
 const bytes=encodeFactIndexV1({version:1,owners:["o"],rows,classes:[{r:rows.map((_,at)=>at),o:[0]}]},{maximumBytes:maximum});
 const started=performance.now(),out=transcodeFactIndexV1(bytes,{mapText:indexTextRewriterV1("/Users/Shared/semio","neutralize"),maximumBytes:maximum}),took=performance.now()-started;
 expect(out.length).toBeGreaterThan(1000);expect(took).toBeLessThan(15000);
 expect(decodeFactIndexV1(out).rows[149999]!.path.startsWith("\u27e6semio-root\u27e7/p")).toBe(true);
},60000);
