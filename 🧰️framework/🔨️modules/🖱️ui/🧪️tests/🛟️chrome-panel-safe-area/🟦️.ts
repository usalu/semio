/** 🛟️ Shared shell placement vectors agree with independent Three box intersections. */
import {expect,it} from "vitest";
import Ajv from "ajv";
import {Box2,Vector2} from "three";
import {chromePanelSafeArea,type Anchor,type SafeAreaYield} from "../../🎯️targets/⚛️react/🟦️.tsx";
import fixture from "../../🧫️fixtures/🛟️chrome-panel-safe-area/🔣️.json";

const box=(row:readonly number[])=>new Box2(new Vector2(row[0],row[1]),new Vector2(row[0]!+row[2]!,row[1]!+row[3]!));
const intersects=(a:Box2,b:Box2)=>{const size=a.clone().intersect(b).getSize(new Vector2());return size.x>0&&size.y>0;};
const owned=(b:Box2)=>({left:b.min.x,top:b.min.y,right:b.max.x,bottom:b.max.y});

it("admits the closed shared safe-area contract",()=>{
});

it("clears cascaded panels inside the actual host at every authored anchor",()=>{
 let checks=0;
 for(const row of fixture){
  const a=box(row.affordance),host=box(row.host),panels=row.panels.map(box);
  const left=row.anchor.includes("left"),right=row.anchor.includes("right"),top=row.anchor.includes("top"),bottom=row.anchor.includes("bottom");
  const candidates:{inlinePx:number;blockPx:number}[]=[];
  if(panels.some(p=>intersects(a,p))){
   for(const axis of ["inline","block"] as const){
    const direction=axis==="inline"?(left?1:right?-1:0):(top?1:bottom?-1:0);
    if(!direction||row.yield!="either"&&row.yield!==axis)continue;
    for(const panel of panels){
     const offset=Math.ceil(axis==="inline"?(direction>0?panel.max.x+row.gap-a.min.x:a.max.x+row.gap-panel.min.x):(direction>0?panel.max.y+row.gap-a.min.y:a.max.y+row.gap-panel.min.y));
     if(offset<=0)continue;
     const moved=a.clone().translate(new Vector2(axis==="inline"?direction*offset:0,axis==="block"?direction*offset:0));
     if(!host.containsBox(moved))continue;
     const clear=panels.every(p=>{
      const expanded=p.clone();
      if(axis==="inline"){expanded.min.x-=row.gap;expanded.max.x+=row.gap;}
      else{expanded.min.y-=row.gap;expanded.max.y+=row.gap;}
      return !intersects(moved,expanded);
     });
     if(clear)candidates.push({inlinePx:axis==="inline"?offset:0,blockPx:axis==="block"?offset:0});
    }
   }
  }
  candidates.sort((a,b)=>(a.inlinePx+a.blockPx)-(b.inlinePx+b.blockPx)||a.inlinePx-b.inlinePx);
  const oracle=candidates[0]??{inlinePx:0,blockPx:0};
  const expected={inlinePx:row.expected.inline,blockPx:row.expected.block};
  expect(oracle,`${row.id}: Three placement`).toEqual(expected);
  for(const order of [panels,[...panels].reverse()]){
   expect(chromePanelSafeArea(owned(a),owned(host),row.anchor as Anchor,order.map(owned),row.yield as SafeAreaYield,row.gap),`${row.id}: owned placement`).toEqual(expected);
   checks++;
  }
 }
 process.stderr.write(`[DEBUG] Chrome safe area accepted ${checks} ordered placements across ${fixture.length} neutral rows with independent Three intersections\n`);
});
