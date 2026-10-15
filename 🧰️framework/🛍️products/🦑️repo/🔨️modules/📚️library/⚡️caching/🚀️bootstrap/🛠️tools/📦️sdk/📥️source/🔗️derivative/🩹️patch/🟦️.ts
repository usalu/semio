import policy from"../../../🎟️capacity/📋️policy.json"with{type:"json"};
function count(images:Iterable<Uint8Array>,maximum:number,label:string):number{let total=0;for(const bytes of images){if(!(bytes instanceof Uint8Array))throw Error("Owned SDK publication bytes required");total+=bytes.byteLength;if(!Number.isSafeInteger(total)||total>maximum)throw Error(label);}return total;}
/** 🩹️ Admits every actual post-image before the derivative replaces its first owned file. */
export function requireNxSdkDerivativePatchImagesV1(batches:Iterable<ReadonlyMap<string,Uint8Array>>):number{return count((function*(){for(const batch of batches)yield*batch.values();})(),policy.maximumDerivativePatchBytes,"SDK derivative patch output ceiling exceeded");}
/** 🧾️ Bounds generated receipt contents independently of installation input bytes. */
export function requireNxSdkDerivativeReceiptImagesV1(images:Iterable<Uint8Array>):number{return count(images,policy.maximumDerivativeReceiptBytes,"SDK derivative receipt output ceiling exceeded");}
