/** 🪟️ Mounted panes consume shell occupancy without reserving the canvas body. */
import * as React from "react";
import {act,render} from "@testing-library/react";
import {expect,it} from "vitest";
import {createMemoryStoragePort} from "@semio-tech/framework";
import {Pane,PaneHost,Window,publishShellChromePanelBox,uiSpacingPx,type Anchor} from "../../🎯️targets/⚛️react/🟦️.tsx";
import {ShellScopeProvider,createShellScope} from "../../🧱️elements/🐚️ShellScope/🟦️.tsx";
import fixture from "../../🧫️fixtures/🛟️chrome-panel-safe-area/🔣️.json";

const rect=(box:readonly number[],inline=0,block=0):DOMRect=>({x:box[0]!+inline,y:box[1]!+block,left:box[0]!+inline,top:box[1]!+block,right:box[0]!+inline+box[2]!,bottom:box[1]!+block+box[3]!,width:box[2]!,height:box[3]!,toJSON:()=>({})}) as DOMRect;

it("moves each mounted pane clear of its shell panel and restores its authored anchor on close",()=>{
 const original=Element.prototype.getBoundingClientRect;
 const rows=fixture.filter(row=>row.id.startsWith("folded-"));
 for(const row of rows){
  const root=document.createElement("div"),app=document.createElement("div");root.append(app);document.body.append(root);
  const scope=createShellScope({shellId:"pane-safe-"+row.id,storage:createMemoryStoragePort(),initialLocale:"en"});scope.rootRef.current=root;
  Element.prototype.getBoundingClientRect=function(){
   if(this.getAttribute("data-slot")==="pane-host")return rect(row.host);
   if(this.getAttribute("data-slot")==="test-safe-pane")return rect(row.affordance,Number(this.getAttribute("data-safe-area-inline")??0)*(row.anchor.includes("right")?-1:1),Number(this.getAttribute("data-safe-area-block")??0)*(row.anchor.includes("bottom")?-1:1));
   return original.call(this);
  };
  const view=render(<ShellScopeProvider scope={scope}><PaneHost><Pane id={row.id} anchor={row.anchor as Anchor} icon="layers" label="Layers" overlaySlot="test-safe-pane" chromePanelGapPx={row.gap}>Body</Pane></PaneHost></ShellScopeProvider>,{container:app});
  try{
   const pane=app.querySelector('[data-slot="test-safe-pane"]')!;
   act(()=>{for(const [i,p] of row.panels.entries())publishShellChromePanelBox(root,String(i),row.anchor as Anchor,{left:p[0]!,top:p[1]!,right:p[0]!+p[2]!,bottom:p[1]!+p[3]!});});
   expect(Number(pane.getAttribute("data-safe-area-inline")??0),row.id).toBe(row.expected.inline);
   expect(Number(pane.getAttribute("data-safe-area-block")??0),row.id).toBe(row.expected.block);
   act(()=>{for(const [i] of row.panels.entries())publishShellChromePanelBox(root,String(i),row.anchor as Anchor,null);});
   expect(pane.getAttribute("data-safe-area-inline")).toBeNull();
   expect(pane.getAttribute("data-safe-area-block")).toBeNull();
  }finally{view.unmount();root.remove();Element.prototype.getBoundingClientRect=original;}
 }
 expect(rows).toHaveLength(8);
 process.stderr.write("[DEBUG] Mounted panes cleared shell occupancy and restored placement at all eight anchors\n");
});

it("connects the Window Actions pane outside PaneHost to the live window bounds",()=>{
 const original=Element.prototype.getBoundingClientRect;
 const root=document.createElement("div"),app=document.createElement("div");root.append(app);document.body.append(root);
 const scope=createShellScope({shellId:"window-pane-safe",storage:createMemoryStoragePort(),initialLocale:"en"});scope.rootRef.current=root;
 Element.prototype.getBoundingClientRect=function(){
  if(this.getAttribute("data-slot")==="window-body")return rect([0,40,960,600]);
  if(this.getAttribute("data-slot")==="window-engagement-overlay")return rect([8,48,90,22],Number(this.getAttribute("data-safe-area-inline")??0),Number(this.getAttribute("data-safe-area-block")??0));
  return original.call(this);
 };
 const view=render(<ShellScopeProvider scope={scope}><Window id="safe-actions" active fill actionPane={<button>Undo</button>}><div data-testid="canvas">Canvas</div></Window></ShellScopeProvider>,{container:app});
 try{
  const pane=app.querySelector('[data-slot="window-engagement-overlay"]')!;
  act(()=>publishShellChromePanelBox(root,"artifact","top-left",{left:4,top:44,right:304,bottom:636}));
  expect(Number(pane.getAttribute("data-safe-area-inline")??0)).toBe(Math.ceil(304+uiSpacingPx(1)-8));
  expect(Number(pane.getAttribute("data-safe-area-block")??0)).toBe(0);
  expect(app.querySelector('[data-testid="canvas"]')?.closest('[data-slot="pane-host-root"]')).not.toBeNull();
 }finally{view.unmount();root.remove();Element.prototype.getBoundingClientRect=original;}
 process.stderr.write("[DEBUG] Window Actions consumed its explicit host bounds outside the canvas PaneHost\n");
});
