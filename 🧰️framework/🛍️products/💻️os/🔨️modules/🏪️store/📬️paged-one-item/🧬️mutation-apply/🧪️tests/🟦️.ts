import {test,expect} from "bun:test";
import Ajv from "ajv";
import fixture from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
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
 console.log("[DEBUG] independent JS array oracle reproduces every post document and restores the base with the ordered inverse rows");
});
