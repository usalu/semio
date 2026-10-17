/** 🧾️ Copies a bounded ordinary Source observation without a descriptor, owner or effect port. */
export interface BoundedSourceRevisionV1{readonly sha256:string;readonly byteLength:number;readonly text:string|null}
/** 🔏️ Allows only reducing byte extents; retained UTF8 text also requires the private text ceiling. */
export function captureBoundedSourceRevisionV1(path:string,maximumBytes:number,retainText?:boolean):BoundedSourceRevisionV1;
