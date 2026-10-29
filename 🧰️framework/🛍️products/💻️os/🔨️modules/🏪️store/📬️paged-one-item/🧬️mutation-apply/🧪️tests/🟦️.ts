import {test,expect} from "bun:test";
import Ajv from "ajv";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import stages from "./🧫️fixtures/📋️stages/🔣️.json";
import stagesSchema from "./🧫️fixtures/📋️stages/🧬️schema/🔣️.json";
type Node={id:string;text:string};
type Row=Record<string,any>;
const apply=(nodes:Node[],row:Row):Node[]=>{const next=nodes.map(node=>({...node}));switch(row.kind){case"insert":next.splice(row.index,0,{id:row.id,text:row.text});break;case"remove":next.splice(row.index,1);break;case"set-text":next[row.index]!.text=row.text;break;case"remove-range":next.splice(row.start,row.count);break;}return next;};
test("structural mutation cases agree with an independent array oracle",()=>{
 expect(new Ajv({strict:true,allowUnionTypes:true}).compile(schema)(fixture)).toBe(true);
 for(const row of fixture.cases){
  const post=apply(fixture.base.nodes,row.mutation);
  expect(post.map(node=>[node.id,node.text])).toEqual(row.post);
  const restored=row.inverse.reduce((nodes:Node[],inverse:Row)=>apply(nodes,inverse),post);
  expect(restored).toEqual(fixture.base.nodes);
 }
});
test("stage law agrees with its schema and the retained stage frontier order",()=>{
 const law=stages;
 expect(new Ajv({strict:true}).compile(stagesSchema)(stages)).toBe(true);
 for(const flow of [law.admitted,law.decisionRefused,law.preflightRefused]){
  expect(flow[0]).toBe("Birth");expect(flow.at(-1)).toBe("Done");
  expect(flow.every((stage,index)=>law.stages.includes(stage)&&(index===0||law.stages.indexOf(flow[index-1]!)<law.stages.indexOf(stage)))).toBe(true);
 }
 for(const flow of [law.decisionRefused,law.preflightRefused])expect(flow).not.toContain("Commit");
});
