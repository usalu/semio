/** 👁️ Typed authored visibility independent of serialized object maps. */
export interface BoardVisibility { hidden?:boolean;visible?:boolean;locked?:boolean }
export function boardVisibleOption(flags:BoardVisibility):boolean|undefined{return flags.hidden===undefined?flags.visible:!flags.hidden;}
export function boardVisibleOrTrue(flags:BoardVisibility):boolean{return boardVisibleOption(flags)??true;}
export function boardLockedOption(flags:BoardVisibility):boolean|undefined{return flags.locked;}
