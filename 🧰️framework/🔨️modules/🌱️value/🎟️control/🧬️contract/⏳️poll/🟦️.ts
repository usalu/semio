export type RetainedWorkOutcome<T,E>={readonly result:{readonly ok:T}|{readonly error:E};readonly work:bigint};
export type RetainedWorkPoll<T,E>={readonly type:"pending";readonly work:bigint}|{readonly type:"complete";readonly turn:RetainedWorkOutcome<T,E>};
export const retainedPollWork=<T,E>(poll:RetainedWorkPoll<T,E>):bigint=>poll.type==="pending"?poll.work:poll.turn.work;
export function mapRetainedWorkPoll<T,U,E>(poll:RetainedWorkPoll<T,E>,map:(value:T)=>U):RetainedWorkPoll<U,E>{if(poll.type==="pending")return poll;if("error" in poll.turn.result)return poll as RetainedWorkPoll<U,E>;return{type:"complete",turn:{work:poll.turn.work,result:{ok:map(poll.turn.result.ok)}}};}
export function retainedWorkFromWire(input:string):bigint|null{if(!/^(0|[1-9][0-9]*)$/.test(input))return null;const value=BigInt(input);return value<=340282366920938463463374607431768211455n?value:null;}
export function retainedWorkToWire(work:bigint):string|null{return work>=0n&&work<=340282366920938463463374607431768211455n?work.toString():null;}
