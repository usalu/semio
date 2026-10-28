export type RingPoint=[number,number,number];
export type RingTriangle=[number,number,number];
import {triangulateRingsSteps} from './work.ts';
export {triangulateRingsSteps};
export type {RingSteps} from './work.ts';

/** 🕳️ Synchronous callers drain the shared evaluator with their original work callback. */
export function triangulateRings(vertices:readonly RingPoint[],input:readonly(readonly number[])[],tick:()=>void):RingTriangle[]{
 const steps=triangulateRingsSteps(vertices,input);
 try{for(;;){const step=steps.next();if(step.done)return step.value;tick();}}finally{steps.return([]);}
}
