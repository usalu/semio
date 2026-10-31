/** ⏱️ A defining owner commits actual bounded work through one expiring observation loan. */
export interface RetainedWorkTurn{commitActualWork(units?:number):void;actualWork():number}
const activeTurns=new WeakSet<RetainedWorkTurn>();
/** 🔐️ Only a defining counter's current issued observation loan may receive actual work. */
export function assertRetainedWorkTurn(turn:RetainedWorkTurn):void{if(!activeTurns.has(turn))throw Error("Retained work turn loan is unissued or expired");turn.actualWork();}
/** 🔒️ Counter metadata stays with its owner; old loans cannot mutate a later turn. */
export class RetainedWorkTurnCounter{
 #generation=0;#active=false;#actual=0;#loan:RetainedWorkTurn|null=null;
 beginTurn():RetainedWorkTurn{if(this.#active)throw Error("Retained work turn already active");const generation=this.#generation+1;if(!Number.isSafeInteger(generation))throw Error("Retained work turn generation overflow");this.#generation=generation;this.#actual=0;this.#active=true;const check=()=>{if(!this.#active||this.#generation!==generation)throw Error("Retained work turn loan expired");};const loan:RetainedWorkTurn=Object.freeze({commitActualWork:(units=1)=>{check();const actual=this.#actual+units;if(!Number.isSafeInteger(units)||units<0||!Number.isSafeInteger(actual))throw Error("Invalid actual retained work");this.#actual=actual;},actualWork:()=>{check();return this.#actual;}});this.#loan=loan;activeTurns.add(loan);return loan;}
 endTurn(loan:RetainedWorkTurn):number{if(!this.#active||loan!==this.#loan)throw Error("Retained work turn loan is not current");activeTurns.delete(loan);this.#active=false;this.#loan=null;return this.#actual;}
}
