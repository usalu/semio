import{createHash}from"node:crypto";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
import{canonicalJsonV1}from"../🧮️fact-codec/🟨️.mjs";

/** ✂️ The declared constants of the derived fact cutoff; the schema file is the single source and a law keeps both equal. */
export const CUTOFF_POLICY_V1=Object.freeze(structuredClone(schema.definitions.Policy.const));
const P=CUTOFF_POLICY_V1;

/** 🏷️ Whether a row is a derived fact: the answer of a question, never a raw fact that can be checked against the disk. */
export const isDerivedRowV1=row=>row.kind===P.kind;

/** 👻️ Whether an owner name is the pseudo owner that holds a derived fact together with its support. */
export const isDerivedOwnerV1=name=>name.startsWith(P.ownerPrefix);

/** 🪪️ The pseudo owner of one derived fact: named by the whole row, so a changed answer is another owner and the projects still holding the old answer keep the old support. */
export function derivedOwnerNameV1(row){
 return P.ownerPrefix+createHash("sha256").update(canonicalJsonV1([row.kind,row.path,row.fact])).digest("hex").slice(0,P.nameDigestCharacters);
}

const views=new WeakMap();

/** 🔭️ The derived rows of one index, the row of each pseudo owner and the projects that hold each derived row, read once per index. */
function viewOf(index){
 let view=views.get(index);if(view)return view;
 const derivedIds=new Set(),pseudoIds=new Set(),pseudoRow=new Map(),holders=new Map();
 if(index.rowKindOf){for(let id=0;id<index.rows.length;id++)if(index.rowKindOf(id)===P.kind)derivedIds.add(id);}else index.rows.forEach((row,id)=>{if(isDerivedRowV1(row))derivedIds.add(id);});
 index.owners.forEach((name,id)=>{if(isDerivedOwnerV1(name))pseudoIds.add(id);});
 if(derivedIds.size)for(const cls of index.classes){
  const here=cls.r.filter(id=>derivedIds.has(id));if(!here.length)continue;
  for(const owner of cls.o)for(const id of here){
   if(pseudoIds.has(owner))pseudoRow.set(index.owners[owner],id);
   else{let set=holders.get(id);if(!set)holders.set(id,set=new Set());set.add(index.owners[owner]);}
  }
 }
 views.set(index,view={derivedIds,pseudoIds,pseudoRow,holders});return view;
}

/** 🧾️ The ids of the derived rows of an index. */
export const derivedRowIdsV1=index=>viewOf(index).derivedIds;

/** 🚫️ Drops the derived rows from the row ids that are checked against the disk. */
export function verifiableRowIdsV1(index,ids){
 const{derivedIds}=viewOf(index);return derivedIds.size?ids.filter(id=>!derivedIds.has(id)):[...ids];
}

/** 🎯️ Adds to a set of project names the pseudo owners that support the derived facts those projects hold, so a verification scoped to some projects checks their supports too. */
export function scopeWithDerivedV1(index,names){
 const view=viewOf(index),out=new Set(names);if(!view.derivedIds.size)return out;
 const ids=new Set();index.owners.forEach((name,id)=>{if(names.has(name))ids.add(id);});
 for(const cls of index.classes){
  if(!cls.r.some(id=>view.derivedIds.has(id))||!cls.o.some(id=>ids.has(id)))continue;
  for(const owner of cls.o)if(view.pseudoIds.has(owner))out.add(index.owners[owner]);
 }
 return out;
}

/** 🔀️ Splits the consumers of the stale raw rows: projects that read a stale row themselves, and the pseudo owners whose derived fact has to be asked again. */
export function splitStaleConsumersV1(index,classes,stale){
 const direct=new Set(),suspects=new Set(),{pseudoIds}=viewOf(index);
 for(const at of classes){
  const cls=index.classes[at];if(!cls.r.some(id=>stale.has(id)))continue;
  for(const owner of cls.o)(pseudoIds.has(owner)?suspects:direct).add(index.owners[owner]);
 }
 return{direct,suspects};
}

/** 👥️ The projects that hold the derived fact of each pseudo owner. */
export function holdersOfDerivedV1(index,pseudoOwners){
 const view=viewOf(index),out=new Map();
 for(const name of pseudoOwners){const id=view.pseudoRow.get(name);out.set(name,id===undefined?new Set():new Set(view.holders.get(id)??[]));}
 return out;
}

/** ✅️ The suspects a verification scoped to the project names `names` asks again: the pseudo owners whose derived fact one of those projects holds. A question that only projects outside the scope hold cannot stale any of them; its stale support stays recorded, so the verification that selects one of its holders asks it then. */
export function suspectsWithinV1(index,suspects,names){
 const view=viewOf(index),out=new Set();
 for(const name of suspects){const id=view.pseudoRow.get(name);if(id!==undefined&&[...(view.holders.get(id)??[])].some(holder=>names.has(holder)))out.add(name);}
 return out;
}

/** 🧾️ The derived row of each suspect pseudo owner: the questions the node graph (`../../🧮️derived-nodes/🧩️closure-kinds`) asks again. */
export function suspectRowsV1(index,suspects){
 const view=viewOf(index),out=new Map();
 for(const name of suspects){const id=view.pseudoRow.get(name);if(id!==undefined)out.set(name,index.rows[id]);}
 return out;
}

/** 🧱️ The rows of the pseudo owners a build writes: the facts the compute produced with their fresh support, and the supports of the questions that still held, as they are (a held support is a lazy list of its parts: the patch flattens one question at a time when it reaches it, so the rows of all questions never exist as lists together). */
export function derivedUpsertsV1(derived,held=new Map()){
 const out=new Map();
 for(const fact of derived)out.set(derivedOwnerNameV1(fact.row),!Array.isArray(fact.support)&&typeof fact.support?.concat==="function"?fact.support.concat([fact.row]):[fact.row,...fact.support]);
 for(const[name,rows]of held)if(!out.has(name))out.set(name,rows);
 return out;
}

/** 🧹️ The pseudo owners whose derived fact no project holds any more. */
export function orphanedDerivedOwnersV1(index){
 const view=viewOf(index),out=[];
 for(const name of index.owners)if(isDerivedOwnerV1(name)){const id=view.pseudoRow.get(name);if(id===undefined||!(view.holders.get(id)?.size))out.push(name);}
 return out;
}

/** 🚨️ The derived rows that projects hold and no pseudo owner supports: such a fact could never be asked again, so a generation that carries one is refused. */
export function unsupportedDerivedV1(index){
 const view=viewOf(index),supported=new Set(view.pseudoRow.values()),out=[];
 for(const[id,holders]of view.holders)if(holders.size&&!supported.has(id))out.push(index.rows[id]);
 return out;
}
