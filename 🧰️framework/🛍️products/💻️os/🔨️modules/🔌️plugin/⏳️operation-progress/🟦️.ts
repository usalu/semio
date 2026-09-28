/** 🪪️ Decode an exact operation identity; authority remains with the addressed app instance. */
export function decodeOperationCancellation(value:unknown):{operationId:bigint;generation:bigint}|null {
  if(!value||typeof value!=="object"||Array.isArray(value))return null;
  const row=value as Record<string,unknown>,keys=Object.keys(row);
  if(keys.length!==2||!keys.includes("operationId")||!keys.includes("generation"))return null;
  const valid=(part:unknown):part is string=>typeof part==="string"&&/^[0-9a-f]{16}$/.test(part);
  return valid(row.operationId)&&valid(row.generation)?{operationId:BigInt("0x"+row.operationId),generation:BigInt("0x"+row.generation)}:null;
}
/** 📶️ Describe measured work without inventing a percentage or selecting a default language. */
export function operationProgressText(locale:"en"|"de",completed:bigint,cancelling:boolean):string {
  if(completed<0n||completed>0xffffffffffffffffn)throw new RangeError("Operation progress exceeds its counter width");
  const state=locale==="en"?(cancelling?"Cancelling":"Working"):(cancelling?"Wird abgebrochen":"Wird ausgeführt");
  return `${state} · ${locale==="en"?"Completed units":"Abgeschlossene Schritte"}: ${completed}`;
}
/** 🏁️ A requested stop ends normally while an actual worker failure remains visible. */
export function cancellationResultLane(userRequested:boolean,workerFault:boolean):"terminal"|"fault" {
  return userRequested&&!workerFault?"terminal":"fault";
}
