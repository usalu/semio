/** ⏱️ A defining owner commits actual bounded work through one expiring observation loan. */
export interface RetainedWorkTurn{commitActualWork(units?:number):void;actualWork():number}
/** 🔒️ One loan per turn: liveness and generation live in private fields so issuing and expiring allocate no closures. */
class TurnLoan implements RetainedWorkTurn{
 #counter:RetainedWorkTurnCounter;#generation:number;#live=true;
 constructor(counter:RetainedWorkTurnCounter,generation:number){this.#counter=counter;this.#generation=generation;}
 static live(value:unknown):boolean{return value instanceof TurnLoan&&value.#live;}
 static expire(loan:TurnLoan):void{loan.#live=false;}
 static owner(loan:TurnLoan):RetainedWorkTurnCounter{return loan.#counter;}
 commitActualWork(units=1):void{const actual=this.#counter.commit(this,this.#generation,units);void actual;}
 actualWork():number{return this.#counter.read(this,this.#generation);}
}
/** 🔐️ Only a defining counter's current issued observation loan may receive actual work. */
export function assertRetainedWorkTurn(turn:RetainedWorkTurn):number{if(!TurnLoan.live(turn))throw Error("Retained work turn loan is unissued or expired");return turn.actualWork();}
/** 🔒️ Counter metadata stays with its owner; old loans cannot mutate a later turn. */
export class RetainedWorkTurnCounter{
 #generation=0;#active=false;#actual=0;#loan:TurnLoan|null=null;
 beginTurn():RetainedWorkTurn{if(this.#active)throw Error("Retained work turn already active");const generation=this.#generation+1;if(!Number.isSafeInteger(generation))throw Error("Retained work turn generation overflow");this.#generation=generation;this.#actual=0;this.#active=true;const loan=new TurnLoan(this,generation);this.#loan=loan;return loan;}
 endTurn(loan:RetainedWorkTurn):number{if(!this.#active||loan!==this.#loan)throw Error("Retained work turn loan is not current");TurnLoan.expire(loan as TurnLoan);this.#active=false;this.#loan=null;return this.#actual;}
 /** 🧾️ Loan-facing accessors validate the issuing generation before touching the counter. */
 commit(loan:TurnLoan,generation:number,units:number):number{if(!this.#active||this.#generation!==generation||this.#loan!==loan)throw Error("Retained work turn loan expired");const actual=this.#actual+units;if(!Number.isSafeInteger(units)||units<0||!Number.isSafeInteger(actual))throw Error("Invalid actual retained work");this.#actual=actual;return actual;}
 read(loan:TurnLoan,generation:number):number{if(!this.#active||this.#generation!==generation||this.#loan!==loan)throw Error("Retained work turn loan expired");return this.#actual;}
}
