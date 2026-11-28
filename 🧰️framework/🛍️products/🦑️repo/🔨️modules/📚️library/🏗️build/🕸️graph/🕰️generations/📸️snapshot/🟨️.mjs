import{fail}from"../🧱️guards/🟨️.mjs";

/**
 * 📸️ Creates the reader of one complete graph without any lease: a fragment retired while the snapshot is read restarts it from the newest pointer, so the result is always the graph of one sealed generation.
 * @see ../🟨️.mjs
 */
export function createSnapshotV1({read,closureOf,assemble,sleep}){
 return async function snapshot({roots,neighbors,compare,signal,progress}={}){
  for(let attempt=0;attempt<8;attempt++){
   const current=await read(signal);if(!current?.manifest)throw fail("A sealed generation is required","GENERATION_EMPTY");
   try{const names=roots?[...await closureOf(current,roots,{neighbors,signal})]:undefined;return{current,graph:await assemble(current,{names,compare,signal,progress})};}
   catch(error){if(error?.code!=="GENERATION_INCOMPLETE")throw error;await sleep(5*(attempt+1),signal);}
  }
  throw fail("Generation fragments keep disappearing","GENERATION_UNREADABLE");
 };
}
