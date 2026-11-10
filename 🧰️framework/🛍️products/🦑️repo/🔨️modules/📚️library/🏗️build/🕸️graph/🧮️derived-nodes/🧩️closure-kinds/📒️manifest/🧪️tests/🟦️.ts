/** 📒️ Contract: the inverted index of the settle manifest names exactly the parts `rowTouchedV1` calls touched (files, directories, bare ancestors, configuration, volatility, an `excluded` predicate) plus the parts of the outside rows that failed, for random part summaries and random dirty sets; and the persisted form reads back equal. */
import{expect,test}from"bun:test";
import{affectedPartsV1,createManifestV1,indexPartV1,partIdsOfV1,readManifestV1,recordEntryV1,writeManifestV1}from"../🟨️.mjs";
import{dirtySummaryV1,rowTouchedV1}from"../../🧭️row-paths/🟨️.mjs";

const rng=(seed:number)=>{let state=seed;return(limit:number)=>{state=(state*1103515245+12345)%2147483648;return(state>>8)%limit;};};
const names=Array.from({length:40},(_,at)=>(at%5===0?"d"+at:"d"+(at%4)+"/f"+at+".ts"));

test("the inverted index equals rowTouchedV1 part by part, outside rows included, and survives a round trip",async()=>{
 const draw=rng(5),book=createManifestV1(),metas:any[]=[];
 for(let part=0;part<60;part++){
  const pick=(count:number)=>Array.from({length:draw(count)},()=>names[draw(names.length)]!);
  const meta={paths:pick(5),dirs:pick(3),bare:draw(6)===0?pick(2):[],config:draw(8)===0,volatile:draw(15)===0,outside:draw(5)===0?[{kind:"file",path:"/outside/"+part,fact:"x"}]:[],size:3};
  metas.push(meta);indexPartV1(book,"p"+String(part).padStart(2,"0"),meta);
 }
 for(let closure=0;closure<10;closure++)recordEntryV1(book,"q"+closure,{digest:"d"+closure,fact:"f"+closure,length:3,ids:Array.from({length:1+draw(9)},()=>"p"+String(draw(60)).padStart(2,"0"))});
 const failing=new Set(["/outside/7","/outside/20"]);
 for(let trial=0;trial<200;trial++){
  const dirty={paths:Array.from({length:draw(4)},()=>names[draw(names.length)]!),directories:Array.from({length:draw(3)},()=>"d"+draw(5)),...draw(4)===0?{excluded:(path:string)=>path==="d3/f19.ts"}:{}},summary=dirtySummaryV1(dirty);
  if(draw(6)===0)(summary as any).configs=true;
  const bits=await affectedPartsV1(book,summary as any,async rows=>(rows as any[]).filter(row=>failing.has(row.path)));
  metas.forEach((meta,part)=>{
   const want=rowTouchedV1(meta,summary as any)||meta.outside.some((row:any)=>failing.has(row.path)),number=book.partNo.get("p"+String(part).padStart(2,"0"));
   const got=number!==undefined&&(bits[number>>5]!==undefined)&&((bits[number>>5]!>>>(number&31))&1)===1;
   expect(got,"part "+part+" trial "+trial).toBe(want);
  });
 }
 const back=readManifestV1(JSON.parse(JSON.stringify(writeManifestV1(book))));
 expect([...back.entries.keys()]).toEqual([...book.entries.keys()]);
 for(const name of book.entries.keys()){expect(partIdsOfV1(back,back.entries.get(name)!)).toEqual(partIdsOfV1(book,book.entries.get(name)!));expect(back.entries.get(name)!.fact).toBe(book.entries.get(name)!.fact);}
 expect([...back.paths]).toEqual([...book.paths]);expect(back.outside.size).toBe(book.outside.size);
});
