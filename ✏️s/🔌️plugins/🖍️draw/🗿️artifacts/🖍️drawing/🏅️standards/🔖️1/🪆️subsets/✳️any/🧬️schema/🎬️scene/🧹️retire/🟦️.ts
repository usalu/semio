/** 🧹️ Only a genuinely issued plan owner can transfer its funded cell stack into retirement. */
import {ScenePlanOwner} from "../🗺️plan-model/🎟️owned/🟦️.ts";
import type {RetainedWorkLedger,RetainedWorkRetirementCursor,RetainedWorkDemand} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/🟦️.ts";
export type ScenePlanCloseProgress={phase:"closing"|"complete";owners:number;work:number;done:boolean};
/** 🗝️ Source borrows never confer mutation rights over caller arrays, images or glyph paths. */
export class ScenePlanCloseJob implements RetainedWorkRetirementCursor{
 #driver:RetainedWorkRetirementCursor&{owners():number};
 constructor(owner:ScenePlanOwner){this.#driver=ScenePlanOwner.consume(owner);}
 advance(items:number,work:RetainedWorkLedger):ScenePlanCloseProgress{const progress=this.#driver.advance(items,work);return{phase:progress.done?"complete":"closing",owners:this.#driver.owners(),work:progress.work,done:progress.done};}
 lastTurnWork():number{return this.#driver.lastTurnWork();}
 nextDemand():RetainedWorkDemand{return this.#driver.nextDemand();}
 terminalIsEmpty():boolean{return this.#driver.terminalIsEmpty();}
}
