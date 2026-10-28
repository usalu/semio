/** 🔤️ The text of one fact inside the identity of an observation row; flat file facts avoid JSON serialisation. */
export function factKeyV1(fact){
 if(fact===null||typeof fact!=="object")return typeof fact+String(fact);
 if(typeof fact.sha256==="string"&&fact.provider===undefined&&typeof fact.size==="number")return "f"+fact.size+","+fact.mtimeMs+","+fact.ctimeMs+","+fact.ino+","+fact.sha256;
 return JSON.stringify(fact);
}

/** #️⃣ A 32 bit FNV-1a hash of a text, continued from `seed`. */
export function hashTextV1(text,seed){let hash=seed>>>0;for(let at=0;at<text.length;at++){hash^=text.charCodeAt(at);hash=Math.imul(hash,16777619);}return hash>>>0;}

/** 🔑️ The 53 bit hash of one row identity: 32 bits of kind and path, 21 bits of the fact continued from them. */
export function rowHashV1(kind,path,factKey){const place=hashTextV1(path,hashTextV1(kind,2166136261));return place*2097152+(hashTextV1(factKey,place)>>>11);}

/**
 * 🔢️ Creates the table of canonical observation rows: equal (kind, path, fact) is one row object and one dense number, found through a table of hashes whose buckets are verified by comparison, so no permanent key string is kept per row (a 150 character path costs its own copy in a key, a table entry and a cache entry; the number costs one map entry). Numbers are valid until `clear()`.
 * @param {{factKey?:(fact:unknown)=>string;hash?:(kind:string,path:string,factKey:string)=>number}} [options]
 */
export function createRowTableV1({factKey=factKeyV1,hash=rowHashV1}={}){
 let slots=new WeakMap(),next=1,count=0;const table=new Map();
 const same=(row,kind,path,key)=>row.kind===kind&&row.path===path&&factKey(row.fact)===key;
 /** 🗺️ The canonical row of one fact: the row made before for an equal fact, else the row `make()` returns (registered with the next number). */
 const canonical=(kind,path,fact,make)=>{
  const key=factKey(fact),at=hash(kind,path,key),bucket=table.get(at);
  if(bucket!==undefined){if(Array.isArray(bucket)){for(const row of bucket)if(same(row,kind,path,key))return row;}else if(same(bucket,kind,path,key))return bucket;}
  const row=make();slots.set(row,next++);count++;
  if(bucket===undefined)table.set(at,row);else if(Array.isArray(bucket))bucket.push(row);else table.set(at,[bucket,row]);
  return row;
 };
 /** 📊️ The number of a row: that of its canonical row, so two row objects of one fact have one number. */
 const id=row=>{let known=slots.get(row);if(known===undefined){known=slots.get(canonical(row.kind,row.path,row.fact,()=>row));slots.set(row,known);}return known;};
 return Object.freeze({canonical,id,size:()=>count,clear(){table.clear();slots=new WeakMap();next=1;count=0;}});
}
