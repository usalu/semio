/** 🛟️ Shared shell placement vectors agree with independent Three box intersections. */
import {expect,it} from "vitest";
import Ajv from "ajv";
import {Box2,Vector2} from "three";
import polygonClipping from "polygon-clipping";
import { chromeHostedPanelTrailingClearStyle, chromePanelOpenSafeArea, chromePanelSafeArea, chromePanelHeadBox, type SafeAreaYield } from "../../🔨️modules/🛟️chrome-safe-area/🟦️.tsx";
import { type Anchor } from "../../🔨️modules/⚓️panel-anchor/🟦️.ts";
import fixture from "../🧫️fixtures/🛟️chrome-panel-safe-area/🔣️.json";

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
});

it("F4: an open top-left pane yields to the dock that covers its header, not to the chip rows and full-width band its tall body overlaps",()=>{
 const host={left:0,top:29,right:800,bottom:1000},pane={left:0,top:29,right:300,bottom:989};
 const dock={left:0,top:0,right:300,bottom:942},band={left:0,top:971,right:1600,bottom:1000},chips={left:0,top:947,right:285,bottom:971};
 expect(chromePanelSafeArea(pane,host,"top-left",[chips,band,dock],"either",-0.5),"the whole body pins the pane under the dock").toEqual({inlinePx:0,blockPx:0});
 const head=chromePanelHeadBox(pane,"top-left",32);
 expect(head).toEqual({left:0,top:29,right:300,bottom:61});
 expect(chromePanelSafeArea(head,host,"top-left",[chips,band,dock],"either",-0.5)).toEqual({inlinePx:300,blockPx:0});
 expect(chromePanelSafeArea(head,host,"top-left",[chips,band],"either",-0.5),"no dock, no yield to the chip rows").toEqual({inlinePx:0,blockPx:0});
 expect(chromePanelHeadBox(pane,"bottom-left",32)).toEqual({left:0,top:957,right:300,bottom:989});
});

it("F5: an open bottom-left pane whose body sits under a side dock moves clear of it, in step with an independent Three box test",()=>{
 const host={left:0,top:29,right:800,bottom:1000},pane={left:0,top:837,right:300,bottom:1000};
 const dock={left:0,top:0,right:300,bottom:942},band={left:0,top:971,right:1600,bottom:1000},chips={left:0,top:947,right:285,bottom:971};
 expect(chromePanelSafeArea(chromePanelHeadBox(pane,"bottom-left",32),host,"bottom-left",[chips,band,dock],"either",-0.5),"fails-before: the header alone clears the dock, the options above it stay hidden").toEqual({inlinePx:0,blockPx:0});
 const shift=chromePanelOpenSafeArea(pane,host,"bottom-left",[chips,band,dock],"either",-0.5,32,0.5);
 expect(shift).toEqual({inlinePx:300,blockPx:0});
 const moved=new Box2(new Vector2(pane.left+shift.inlinePx,pane.top),new Vector2(pane.right+shift.inlinePx,pane.bottom));
 const covered=new Box2(new Vector2(dock.left,dock.top),new Vector2(dock.right,dock.bottom)).intersect(moved).getSize(new Vector2());
 expect(covered.x>0&&covered.y>0,"the moved pane no longer meets the dock").toBe(false);
 expect(chromePanelOpenSafeArea(pane,host,"bottom-left",[chips,band],"either",-0.5,32,0.5),"short chip rows and bands never push an open pane aside, only its header clears the band").toEqual({inlinePx:0,blockPx:53});
 expect(chromePanelOpenSafeArea({left:0,top:29,right:300,bottom:989},{left:0,top:29,right:800,bottom:1000},"top-left",[chips,band,dock],"either",-0.5,32,0.5),"F4 stays: a top-left pane yields to the dock over its header").toEqual({inlinePx:300,blockPx:0});
});

it("F6: an open top-right dock cuts the navbar's reserved trailing strip out of its own paint and hit area, checked with polygon-clipping",()=>{
 expect(chromeHostedPanelTrailingClearStyle("top-left",88)).toBeUndefined();
 expect(chromeHostedPanelTrailingClearStyle("bottom-right",88)).toBeUndefined();
 expect(chromeHostedPanelTrailingClearStyle("top-right",0)).toBeUndefined();
 const style=chromeHostedPanelTrailingClearStyle("top-right",88)!;
 const clip=String(style.clipPath);
 const numeric=clip.replace(/^polygon\(|\)$/g,"").split(/,(?![^(]*\))/).map(pair=>pair.trim().split(/\s+(?![^(]*\))/).map((term,axis)=>{
  const size=axis===0?300:365;
  if(term.startsWith("calc(100% - "))return size-parseFloat(term.slice(12));
  if(term==="var(--panel-header-height)")return 29;
  return parseFloat(term)*(term.endsWith("vmax")?1600:1);
 }));
 const panel:[number,number][]=[[0,0],[300,0],[300,365],[0,365],[0,0]];
 const kept=polygonClipping.intersection([panel],[[numeric as [number,number][]]]);
 const area=(rings:number[][][][])=>rings.reduce((sum,poly)=>sum+poly.reduce((s,ring,index)=>{let a=0;for(let i=0;i<ring.length-1;i++)a+=ring[i]![0]!*ring[i+1]![1]!-ring[i+1]![0]!*ring[i]![1]!;return s+(index===0?1:-1)*Math.abs(a/2);},0),0);
 expect(area(kept as number[][][][])).toBe(300*365-88*29);
 expect(polygonClipping.intersection(kept,[[[ [212,0],[300,0],[300,29],[212,29],[212,0] ]]]).length,"the trailing strip is outside the dock").toBe(0);
});
