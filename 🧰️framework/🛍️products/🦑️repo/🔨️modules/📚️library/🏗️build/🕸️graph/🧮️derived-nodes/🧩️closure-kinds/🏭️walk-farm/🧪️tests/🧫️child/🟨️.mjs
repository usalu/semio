import{createHash}from"node:crypto";

/** 🧫️ The deriver a law puts in a walk child instead of the inference: a pure function of the question, with a question that kills a child (`crash`, a plain failure when asked in the test process), one that fails (`fail`) and one that never answers (`slow`); its `recipe` walks a shared closure entry, burning `burn` milliseconds of processor like a TypeScript parse does. */
export async function openDeriverV1(root){
 const derive=async row=>{
  if(row.path==="crash"){if(root===undefined)throw new Error("deliberate crash outside a child");process.exit(3);}
  if(row.path==="fail")throw Object.assign(new Error("deliberate failure"),{code:"E_FAIL"});
  if(row.path==="slow")await new Promise(resolve=>setTimeout(resolve,600000));
  const digest=createHash("sha256").update(row.path).digest("hex"),shared=Array.from({length:2000},(_,at)=>({kind:"file",path:"/farm/shared/"+at,fact:"s"}));
  return{fact:digest.slice(0,12),parts:[[{kind:"file",path:"/farm/"+row.path,fact:digest.slice(12,20)}],shared]};
 };
 derive.recipe=async recipe=>{
  for(const until=Date.now()+(recipe.burn??0);Date.now()<until;);
  return{value:{recipe,walked:true},rows:[{kind:"file",path:"/recipe/"+recipe.name,fact:"r"}],layers:[[{kind:"file",path:"/recipe/layer",fact:"l"}]],sidecar:{facts:[{row:{kind:"derived",path:"q",fact:"f"}}],members:[["question"]]}};
 };
 return derive;
}
