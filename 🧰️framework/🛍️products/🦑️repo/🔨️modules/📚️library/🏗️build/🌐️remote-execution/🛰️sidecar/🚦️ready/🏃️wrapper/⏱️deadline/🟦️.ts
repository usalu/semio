/** ⏱️ The absolute deadline of a stage, enforced by the wrapper itself: at `deadlineMs` the command is asked to end (`terminate`), and once `graceMs` passed without an end it is killed (`kill`); the timers are injected so the rule runs under a fake clock. Returns the function that disarms it. A deadline in the past fires at once.
 * @see ./🧪️tests/🟦️.ts */
export function armWrapperDeadlineV1(input:Readonly<{deadlineMs:number;graceMs:number;now:()=>number;setTimer:(run:()=>void,milliseconds:number)=>unknown;clearTimer:(timer:unknown)=>void;terminate:()=>void;kill:()=>void}>):()=>void{
 let killTimer:unknown;
 const first=input.setTimer(()=>{input.terminate();killTimer=input.setTimer(input.kill,input.graceMs);},Math.max(0,input.deadlineMs-input.now()));
 return()=>{input.clearTimer(first);if(killTimer!==undefined)input.clearTimer(killTimer);};
}

/** ⏱️ The deadline the launcher wrote as `--deadline=<epoch milliseconds>`, `undefined` when absent or not a positive safe integer. */
export function wrapperDeadlineOfV1(argument:string|undefined):number|undefined{
 if(argument===undefined||!argument.startsWith("--deadline="))return undefined;
 const value=Number(argument.slice("--deadline=".length));
 return Number.isSafeInteger(value)&&value>0?value:undefined;
}
