/** 🧹️ Settles every finite owned cleanup and retains the original body failure as the first outcome. */
export async function withTrustedCatalogCleanupV1<T>(body:()=>Promise<T>,cleanups:readonly(()=>void|Promise<void>)[]):Promise<T>{
 if(typeof body!=="function"||!Array.isArray(cleanups)||cleanups.length>4||cleanups.some(step=>typeof step!=="function"))throw Error("Finite actual trusted catalog settlement required");const steps=Object.freeze([...cleanups]),failures:unknown[]=[];let result:{value:T}|undefined;
 try{result={value:await body()};}catch(error){failures.push(error);}
 for(const cleanup of steps)try{await cleanup();}catch(error){if(!failures.some(primary=>Object.is(primary,error)))failures.push(error);}
 if(failures.length>1)throw new AggregateError(failures,"Trusted catalog body and owned cleanup failed");if(failures.length)throw failures[0];if(!result)throw Error("Trusted catalog body did not settle");return result.value;
}
