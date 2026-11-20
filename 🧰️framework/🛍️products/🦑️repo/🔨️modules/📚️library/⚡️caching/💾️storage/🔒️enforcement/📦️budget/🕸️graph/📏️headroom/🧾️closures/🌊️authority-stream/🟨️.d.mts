/** 🌊️ A bounded streaming reader of a graph authority receipt. */
export type AuthorityStreamSinkV1={header(key:string|null,value:unknown):void;observation(value:unknown):void};
/** 🌊️ Feeds decoded text chunks; `end` refuses an incomplete receipt. */
export function createAuthorityStreamV1(maximumUnitUnits:number,sink:AuthorityStreamSinkV1):{feed(text:string):void;end():void};
