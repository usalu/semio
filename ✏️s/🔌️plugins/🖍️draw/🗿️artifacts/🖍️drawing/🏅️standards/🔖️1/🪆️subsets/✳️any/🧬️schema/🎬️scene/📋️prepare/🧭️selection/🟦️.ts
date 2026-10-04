/** 🎯️ Bound selected-group contribution separately from locked leaf picking. */
import {validateSceneSourceAddress} from "../🟦️.ts";
export interface SceneSelectionRelation{pickable:boolean;boundsSelection:number|null;}
/** 🧭️ Preserve the outermost unlocked selected prefix through ordinary groups. */
export function sceneSelectionRelation(sourcePath:readonly number[],lockedAncestors:number,selectedPaths:readonly(readonly number[])[]):SceneSelectionRelation{
 validateSceneSourceAddress({sourcePath,lockedAncestors});
 if(selectedPaths.length>256)throw Error("Scene selection exceeds ancestry capacity");
 let boundsSelection:number|null=null,depth=33;
 for(let index=0;index<selectedPaths.length;index++){
  const selected=selectedPaths[index]!;validateSceneSourceAddress({sourcePath:selected,lockedAncestors:0});
  if(selected.length<depth&&selected.length<=sourcePath.length&&lockedAncestors%2**selected.length===0&&selected.every((value,at)=>value===sourcePath[at])){boundsSelection=index;depth=selected.length;}
 }
 return{pickable:lockedAncestors===0,boundsSelection};
}
