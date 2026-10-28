/** 📖️ Portable original media read binding retains object and live issuer identity independently of snapshot value. */
export interface OriginalMediaRead<T>{get():T;commitAuthorityMatches(generation:bigint,revision:Uint8Array):boolean}
export function mediaOriginalReadMatches<T>(read:OriginalMediaRead<T>,snapshot:T,generation:bigint,revision:Uint8Array):boolean{return read.get()===snapshot&&read.commitAuthorityMatches(generation,revision)}
export function mediaOriginalReadCaptureAdmits(grant:{items:number;copyBytes:number;depth:number},copyBytes:number):boolean{return grant.items>0&&grant.copyBytes>=copyBytes&&grant.depth>=2}
