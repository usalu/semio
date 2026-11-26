import {processPidExistsV1} from "../../../../🔖️identity/🟦️.ts"
/** 🛑️ The parent pid is no longer the parent (the process was handed to another one when it ended), or the parent no longer exists; a parent we may not signal still exists. See https://man7.org/linux/man-pages/man2/kill.2.html */
export function parentGoneV1(parent:number,current:number=process.ppid):boolean{
 if(current!==parent)return true;
 return!processPidExistsV1(parent);
}
