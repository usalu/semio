/** 🖼️ Orders an atomic asset replacement without exceeding capacity during undo. */
export type AssetReplacementInput={count:number;capacity:number;removable:boolean;nextExists:boolean};
export type AssetReplacementStep="detach"|"remove"|"add"|"replace";
export function replacementSteps(input:AssetReplacementInput):AssetReplacementStep[]{
  const {count,capacity,removable,nextExists}=input;
  if(!Number.isInteger(count)||!Number.isInteger(capacity)||capacity<1||count<0||count>capacity||typeof removable!=="boolean"||typeof nextExists!=="boolean"||(count===0&&(removable||nextExists)))throw new RangeError("Invalid asset replacement envelope");
  if(!nextExists&&count===capacity){
    if(!removable)throw new RangeError("Raster asset capacity is exhausted");
    return ["detach","remove","add","replace"];
  }
  return [...(nextExists?[]:["add" as const]),"replace",...(removable?["remove" as const]:[])];
}
