/** ⏸️ Finish incidental CSS transitions without changing frozen keyframe animations. */
import assert from "node:assert/strict";
import {readFile} from "node:fs/promises";
import {finishIncidentalTransitions,finishTransitions} from "../🧰️tools.mjs";
const cases=JSON.parse(await readFile(new URL("./paused-transitions.cases.json",import.meta.url),"utf8"));
globalThis.CSSTransition=class {constructor(state){this.playState=state;this.finished=false}finish(){this.finished=true}};
const animations=cases.map(c=>c.kind==="transition"?new CSSTransition(c.state):{playState:c.state,finished:false});
globalThis.document={getAnimations:()=>animations};
const page={evaluate:async fn=>fn(),clock:{runFor:async()=>{}}};
await finishIncidentalTransitions(page);
for(let i=0;i<cases.length;i++){assert.equal(animations[i].finished,cases[i].finish,cases[i].id);console.log("PASS "+cases[i].id)}

const transition=new CSSTransition("running");
transition.effect={getComputedTiming:()=>({endTime:100})};
transition.finished=Promise.resolve();
const keyframe={effect:{getComputedTiming:()=>({endTime:100})},get finished(){throw Error("keyframe clock was awaited")}};
globalThis.document={getAnimations:()=>[transition,keyframe]};
await finishTransitions(page,100);
console.log("PASS finite-keyframe-not-awaited");
