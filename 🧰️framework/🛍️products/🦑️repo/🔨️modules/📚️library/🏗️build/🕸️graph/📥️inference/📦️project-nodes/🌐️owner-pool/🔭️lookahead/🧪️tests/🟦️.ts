import{expect,test}from"bun:test";
import{ownerPoolLookaheadV1}from"../🟦️.ts";

const jobs=(...names:string[])=>names.map(name=>({name}));
const view=(rows:ReturnType<typeof ownerPoolLookaheadV1>)=>rows.map(row=>`${row.job.name}:${row.tier}`);

test("the wave owners are tier 0 in their order and the next owners follow as tier 1",()=>{
 expect(view(ownerPoolLookaheadV1({wave:jobs("a","b","c"),ahead:jobs("d","e"),taken:new Set()}))).toEqual(["a:0","b:0","c:0","d:1","e:1"]);
});

test("an owner the read took is in neither tier and an owner named in both stays in the wave",()=>{
 expect(view(ownerPoolLookaheadV1({wave:jobs("a","b","c"),ahead:jobs("c","d","a","e"),taken:new Set(["b","e"])}))).toEqual(["a:0","c:0","d:1"]);
});

test("a read without lookahead publishes only its wave and a wave without open owners publishes only the lookahead",()=>{
 expect(view(ownerPoolLookaheadV1({wave:jobs("a"),ahead:[],taken:new Set()}))).toEqual(["a:0"]);
 expect(view(ownerPoolLookaheadV1({wave:jobs("a"),ahead:jobs("x","y"),taken:new Set(["a"])}))).toEqual(["x:1","y:1"]);
});
