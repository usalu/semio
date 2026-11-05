/** 🎯️ Applies the shared world selection merge algebra to one semantic entity id. */
export function mergeStoryWorldSelectionV1(current:readonly string[],id:string,merge:string):string[]{
 if(merge==="replace"||merge==="range")return [id];
 const selected=new Set(current);
 if(merge==="additive")selected.add(id);else if(merge==="subtractive")selected.delete(id);else if(selected.has(id))selected.delete(id);else selected.add(id);
 return [...selected];
}

/** 🖱️ Resolves the renderer numeric-index/null pick contract against the currently displayed instance order. */
export function pickStoryWorldSelectionV1(current:readonly string[],targets:readonly {readonly id:string}[],id:unknown,merge:string):string[]{
 if(id===null)return merge==="replace"?[]:[...current];
 if(typeof id!=="number"||!Number.isSafeInteger(id)||id<0)return [...current];
 const target=targets[id];return target?mergeStoryWorldSelectionV1(current,target.id,merge):[...current];
}
