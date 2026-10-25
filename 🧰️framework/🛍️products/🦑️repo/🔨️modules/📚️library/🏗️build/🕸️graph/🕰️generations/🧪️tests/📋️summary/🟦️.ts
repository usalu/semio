/** 📋️ Contract: a build says in one line which kinds of stale facts made how many projects stale, before the first few named reasons, so a wide refresh is never explained by sixteen unrelated files. */
import{expect,test}from"bun:test";
import{announceGenerationBuildV1,renderGenerationMessageV1}from"../../🔌️load/🟨️.mjs";

const host=(said:string[],language:"en"|"de")=>({language,workspaceRoot:"/w",separator:"/",say:(key:Parameters<typeof renderGenerationMessageV1>[0],fields:Record<string,string|number>)=>{said.push(renderGenerationMessageV1(key,fields,language));}});

test("the summary names the fact kinds by descending count and the number of stale projects, in both languages",()=>{
 for(const[language,expected]of[["en","graph: 7 stale facts (analysis 4, file 2, readdir 1) leave 12 projects stale"],["de","Graph: 7 veraltete Fakten (analysis 4, file 2, readdir 1) lassen 12 Projekte veralten"]] as const){
  const said:string[]=[];
  announceGenerationBuildV1(host(said,language),{cold:false,owners:Array.from({length:12},(_,at)=>"p"+at),staleRows:7,staleKinds:{readdir:1,file:2,analysis:4},generation:3,reasons:[]} as never,[]);
  expect(said[1]).toBe(expected);
 }
});

test("a build without stale facts of its own (membership changes only) prints no summary",()=>{
 const said:string[]=[];
 announceGenerationBuildV1(host(said,"en"),{cold:false,owners:["p"],staleRows:0,staleKinds:{},generation:3,reasons:[]} as never,[]);
 expect(said.length).toBe(1);
});
