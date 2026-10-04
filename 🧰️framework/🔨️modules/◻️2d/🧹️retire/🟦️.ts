/** 🧹️ Shared work-granted retirement protocol for private geometry owners. */
export type WorkRetirementProgress={phase:"closing"|"complete";work:number;done:boolean};
export interface WorkRetirement{advance(grant:number):WorkRetirementProgress;terminalIsEmpty():boolean;}
/** 🧹️ Releases its kernel closure only when that owner reports an empty terminal destructor. */
export class UnitRetirement implements WorkRetirement{
 private work=0;constructor(private step:(()=>boolean)|null){}
 terminalIsEmpty():boolean{return this.step===null;}
 advance(grant:number):WorkRetirementProgress{if(!Number.isSafeInteger(grant)||grant<1)throw new RangeError("Invalid retirement work grant");for(let at=0;at<grant&&this.step;at++){if(this.step())this.step=null;this.work++;}const done=this.terminalIsEmpty();return{phase:done?"complete":"closing",work:this.work,done};}
}
