import {expect,test} from "bun:test";
import {readyStalledV1,readyWardenHoldsV1} from "../../🟦️.ts";

/** 🧊️ Contract: while the host warden holds a process of the stage tree suspended, the stage is not stalled; a hold of a process outside the tree, an absent warden record or an empty hold changes nothing. */
test("the warden holds the stage when one of its suspended processes is in the tree of the stage and not otherwise",()=>{
 const tree=new Set([100,101,102]);
 expect(readyWardenHoldsV1({suspended:[{pid:101}]},tree)).toBe(true);
 expect(readyWardenHoldsV1({suspended:[{pid:7},{pid:102}]},tree)).toBe(true);
 expect(readyWardenHoldsV1({suspended:[{pid:7}]},tree)).toBe(false);
 expect(readyWardenHoldsV1({suspended:[]},tree)).toBe(false);
 expect(readyWardenHoldsV1({absent:true},tree)).toBe(false);
 expect(readyWardenHoldsV1({suspended:[{pid:101}]},new Set())).toBe(false);
});

test("a stage that the warden holds is never stopped for silence, a stage that it does not hold is stopped at the stall stop",()=>{
 const stop=600_000;
 const decide=(held:boolean,quiet:number)=>readyStalledV1(held?0:quiet,undefined,false);
 expect(decide(false,stop-1)).toBe(false);
 expect(decide(false,stop)).toBe(true);
 expect(decide(true,10*stop)).toBe(false);
});
