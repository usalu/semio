/** 🔁️ The volatile owners and paths an inventory sealed without: they changed in every follow-up round, so their paths left the membership hash and the next read observes them again. */
export type InventoryVolatileV1=Readonly<{paths:readonly string[];owners:readonly string[]}>;

/** 🔁️ What one inventory follow-up needs from its caller: one attempt (the owners it forgot are observed, every other owner answers from memory), the classification of a failure, the forgetting of the owners that read moved paths and the bounds. */
export type InventoryRoundsPortsV1<T>=Readonly<{
  attempt:(round:number,volatile:ReadonlySet<string>)=>Promise<T>;
  follow:(error:unknown)=>{readonly membership:boolean;readonly paths:readonly string[]};
  forget:(paths:readonly string[])=>readonly string[];
  limits:Readonly<{repairRounds:number;volatileRounds:number;inventoryRounds:number}>;
  orNothing:boolean;
  pause?:(round:number,membership:boolean)=>Promise<void>;
  check?:()=>void;
  dirty?:(event:{round:number;paths:readonly string[];owners:readonly string[];streak:number})=>void;
}>;

const meets=(left:ReadonlySet<string>,right:ReadonlySet<string>)=>[...left].some(name=>right.has(name));

/** 🔁️ Follows a live tree until the inventory settles: after a failed attempt only the owners that read a moved path are forgotten (the dirty set), so the next attempt pays for them alone. When the same dirty owners (owners that nobody read are named by their paths) stay dirty for `volatileRounds` rounds in a row, or `inventoryRounds` rounds passed, every path that moved becomes volatile: the following attempts tolerate it and the inventory seals with the volatile owners and paths named instead of repeating forever. Failures that name no path, and rounds beyond twice `inventoryRounds`, end the read as before.
 * @see ./🧪️tests/🟦️.ts */
export async function settleInventoryRoundsV1<T>(ports:InventoryRoundsPortsV1<T>):Promise<{value:T|undefined;volatile:InventoryVolatileV1}>{
  const tolerated=new Set<string>(),owners=new Set<string>(),{limits}=ports;
  let previous=new Set<string>(),streak=0,sealing=false;
  for(let round=1;;round++){
    try{return {value:await ports.attempt(round,tolerated),volatile:{paths:[...tolerated],owners:[...owners]}};}
    catch(error){
      const {membership,paths}=ports.follow(error);
      if(!membership&&paths.length===0)throw error;
      ports.check?.();
      if(paths.length===0&&round>=limits.repairRounds||round>=2*limits.inventoryRounds){if(ports.orNothing&&membership)return {value:undefined,volatile:{paths:[...tolerated],owners:[...owners]}};throw error;}
      const dirty=ports.forget(paths),key=new Set(dirty.length>0?dirty:paths);
      streak=meets(key,previous)?streak+1:1;previous=key;
      if(sealing||streak>=limits.volatileRounds||round>=limits.inventoryRounds){
        sealing=true;
        for(const path of paths)tolerated.add(path);
        for(const name of dirty)owners.add(name);
      }
      ports.dirty?.({round,paths,owners:dirty,streak});
      await ports.pause?.(round,membership);
    }
  }
}
