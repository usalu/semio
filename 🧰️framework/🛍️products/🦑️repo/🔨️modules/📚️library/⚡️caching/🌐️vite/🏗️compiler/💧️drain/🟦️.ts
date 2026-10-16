/** 💧️ Joins the optimizer snapshot before its actual final fence and preserves the first actual failure. */
export async function withViteOptimizerDrainV1<T>(operation:()=>Promise<T>,processing:()=>readonly (Promise<unknown>|undefined)[],finish:()=>void):Promise<T>{
 let outcome:{status:"fulfilled";value:T}|{status:"rejected";reason:unknown};
 try{outcome={status:"fulfilled",value:await operation()};}catch(reason){outcome={status:"rejected",reason};}
 try{for(const row of await Promise.allSettled(processing()))if(row.status==="rejected"&&outcome.status==="fulfilled")outcome={status:"rejected",reason:row.reason};}catch(reason){if(outcome.status==="fulfilled")outcome={status:"rejected",reason};}
 try{finish();}catch(reason){if(outcome.status==="fulfilled")outcome={status:"rejected",reason};}
 if(outcome.status==="rejected")throw outcome.reason;return outcome.value;
}
