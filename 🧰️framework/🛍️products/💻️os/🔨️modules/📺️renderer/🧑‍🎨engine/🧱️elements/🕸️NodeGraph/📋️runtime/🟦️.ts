import corpus from"./🔣️.json" with{type:"json"};
const methods=Object.freeze([...corpus.methods]);
/** 🕸️ Declares the complete current first-party graph session, independently of generated binding declarations. */
export interface NodeGraphSessionV1{
 syncFromSceneJson(json:string):void;
 syncFromScenePack(bytes:Uint8Array):void;
 attachCanvas(canvas:HTMLCanvasElement,width:number,height:number,dpr:number):Promise<unknown>;
 gpuReady():boolean;
 setSize(width:number,height:number,dpr:number):void;
 setCanvasThemeJson(json:string):void;
 renderFrame():void;
 pointerDownScreen(x:number,y:number,button:number,shift:boolean,ctrlOrMeta:boolean,alt:boolean):void;
 pointerMoveScreen(x:number,y:number,shift:boolean,ctrlOrMeta:boolean,alt:boolean):void;
 pointerUpScreen(x:number,y:number,shift:boolean,ctrlOrMeta:boolean,alt:boolean):void;
 pointerCancelScreen():void;
 wheelScreen(x:number,y:number,deltaX:number,deltaY:number,zoomGesture:boolean):void;
 labelOverlayPaintStateJson():string;
 sliderOverlayStateJson():string;
 selectionUnionBoundsScreenJson():string;
 selectionPreviewPointsJson():string;
 selectionPreviewCrossing():boolean;
 selectionPreviewMethod():string;
 selectedNodeIdsJson():string;
 hoveredNodeId():string|null|undefined;
 hoveredChannelJson():string;
 viewport():Readonly<{x:number;y:number;zoom:number}>;
 lodScaleJson():string;
 drawLodLabel():string;
 setAutomaticLod(enabled:boolean):void;
 setForcedDrawLodLabel(label:string):void;
 setGhostNodeJson(json:string):void;
 clearGhostNode():void;
 pickTargetsAtScreenJson(x:number,y:number):string;
 entityScreenJson(domain:string,id:string):string;
 syncInteraction(selectedIdsJson:string,hoveredId?:string|null):void;
 takeSelectionGatherJson():string|null|undefined;
 alignSelection(mode:string):void;
 takeGraphEditsJson():string;
 hostSnapshotJson():string;
 takePendingOpenInstanceId():string|null|undefined;
 screenToWorld(x:number,y:number):unknown[];
 worldFromScreen(x:number,y:number):unknown[];
 selectAll():void;
 deleteSelection():void;
 cancelAreaSelect():void;
 reorganize(optionsJson:string,maximumBytes:number,maximumWork:number,progress:(phase:string,completed:number)=>boolean):void;
 free():void;
}
/** 🧬️ Refuses incomplete generated implementations before exposing the current domain session. */
export function requireNodeGraphSessionV1(value:unknown):NodeGraphSessionV1{if(!isNodeGraphSessionV1(value))throw Error("node-graph.current-browser-binding-required: regenerate and qualify the complete graph session");return value;}
function isNodeGraphSessionV1(value:unknown):value is NodeGraphSessionV1{return value!==null&&typeof value==="object"&&methods.every(name=>typeof Reflect.get(value,name)==="function");}

/** 📦️ Retains one checked namespace and constructor before initialization or session allocation. */
export interface NodeGraphRuntimeV1{readonly assets:object;initialize():Promise<unknown>;create():NodeGraphSessionV1}
/** 🧬️ Checks the complete prototype before permitting either initialization or construction. */
export function requireNodeGraphRuntimeV1(value:unknown):NodeGraphRuntimeV1{
 if(!value||typeof value!=="object"||Array.isArray(value))throw Error("node-graph.current-browser-binding-required: regenerate the managed graph module");const initialize:unknown=Reflect.get(value,"default"),constructor:unknown=Reflect.get(value,"GraphSession");if(typeof initialize!=="function"||typeof constructor!=="function")throw Error("node-graph.current-browser-binding-required: regenerate the graph constructor");requireNodeGraphSessionV1(Reflect.get(constructor,"prototype"));return Object.freeze({assets:value,initialize:async()=>Reflect.apply(initialize,value,[]),create:()=>requireNodeGraphSessionV1(Reflect.construct(constructor,[]))});
}
