/** 🧹️ Transfers a scene plan through shallow owner and container-entry close units. */
import type {DocumentScenePlan,DocumentSceneNode,DocumentSceneContent} from "../📋️prepare/🟦️.ts";
import type {RasterSceneAsset,RasterSceneGroup} from "../📷️raster/🟦️.ts";
import type {Fill} from "../../🎨️fill/🟦️.ts";
import type {PathRasterStroke} from "../../🧮️geometry/📷️raster/🟦️.ts";
type Owner={kind:"plan";value:DocumentScenePlan}|{kind:"assets";value:RasterSceneAsset[]}|{kind:"asset";value:RasterSceneAsset}|{kind:"nodes";value:DocumentSceneNode[]}|{kind:"node";value:DocumentSceneNode}|{kind:"groups";value:RasterSceneGroup[]}|{kind:"group";value:RasterSceneGroup}|{kind:"strings";value:string[]}|{kind:"content";value:DocumentSceneContent}|{kind:"fill";value:Fill}|{kind:"stroke";value:PathRasterStroke}|{kind:"leaf";value:unknown};
export type ScenePlanCloseProgress={phase:"closing"|"complete";owners:number;work:number;done:boolean};
/** 🧹️ Consumes an owned plan; completed owner counts never pretend to report allocator bytes. */
export class ScenePlanCloseJob{
 private stack:Owner[];private owners=0;private work=0;
 constructor(plan:DocumentScenePlan){this.stack=[{kind:"plan",value:plan}];}
 private leaf(value:unknown):void{this.stack.push({kind:"leaf",value});}
 private paint(fill:Fill|null,stroke:PathRasterStroke|null):void{if(fill)this.stack.push({kind:"fill",value:fill});if(stroke)this.stack.push({kind:"stroke",value:stroke});}
 private step():boolean{
  const owner=this.stack.pop()!;
  switch(owner.kind){
   case "plan":{const p=owner.value;this.stack.push({kind:"assets",value:p.assets},{kind:"nodes",value:p.nodes});p.assets=[];p.nodes=[];break;}
   case "assets":{const value=owner.value.pop();if(value){this.stack.push(owner,{kind:"asset",value});return false;}break;}
   case "nodes":{const value=owner.value.pop();if(value){this.stack.push(owner,{kind:"node",value});return false;}break;}
   case "groups":{const value=owner.value.pop();if(value){this.stack.push(owner,{kind:"group",value});return false;}break;}
   case "strings":{const value=owner.value.pop();if(value!==undefined){this.stack.push(owner,{kind:"leaf",value});return false;}break;}
   case "asset":{const a=owner.value;this.leaf(a.id);this.leaf(a.image);break;}
   case "node":{const n=owner.value;this.leaf(n.sourcePath);this.leaf(n.id);this.leaf(n.blendMode);this.stack.push({kind:"groups",value:n.groups},{kind:"content",value:n.content});break;}
   case "group":this.leaf(owner.value.id);this.leaf(owner.value.blendMode);break;
   case "content":{const c=owner.value;switch(c.kind){
    case "path":this.leaf(c.segments);this.paint(c.fill,c.stroke);break;
    case "image":this.leaf(c.asset);break;
    case "group":this.stack.push({kind:"strings",value:c.children});break;
    case "text":this.leaf(c.content);this.paint(c.fill,c.stroke);break;
    case "boolean":this.leaf(c.operation);this.stack.push({kind:"strings",value:c.children});this.paint(c.fill,c.stroke);break;
    case "trace":this.leaf(c.source);this.paint(c.fill,c.stroke);break;
   }break;}
   case "fill":if(owner.value.kind!=="solid")this.leaf(owner.value.stops);break;
   case "stroke":if(owner.value.dash)this.leaf(owner.value.dash);break;
   case "leaf":break;
  }return true;
 }
 advance(grant:number):ScenePlanCloseProgress{if(!Number.isSafeInteger(grant)||grant<1)throw Error("Invalid scene retirement work grant");for(let at=0;at<grant&&!this.terminalIsEmpty();at++){if(this.step())this.owners++;this.work++;}const done=this.terminalIsEmpty();return{phase:done?"complete":"closing",owners:this.owners,work:this.work,done};}
 terminalIsEmpty():boolean{return this.stack.length===0;}
}
