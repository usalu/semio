/** 🧰️ Accessible pixel tools over the authoritative paint scene and command channel. */
import { useEffect, useMemo, useRef, useState, type PointerEvent, type WheelEvent, type RefObject } from "react";
import { GestureRecognizer } from "@semio-tech/framework";
import { canvasPinchCamera, type CanvasCamera } from "../../📐️Canvas2dHost/🟦️.tsx";
import { useTranslation } from "@semio-tech/ui-react";
import { combineSelections, floodSelection, selectPixels, validateExtent, type PixelImage, type PixelPoint, type SelectionMerge } from "../../../../../../../../🔨️modules/🔲️pixels/✍️editing/🟦️.ts";
import { editSelection, layerPoint, pixelLayers, maskLayers, pixelGestureRevision, selectionBounds, selectionSpans, restoreSelection, type PixelLayer } from "./🟦️.ts";

const labels = {
  en: {
    locked:"Locked",lockedHint:"Unlock this layer or its parent in Inspection to edit pixels or masks.",target:"Edit target",pixelTarget:"Layer pixels",maskTarget:"Mask",maskValue:"Mask value",fillMask:"Fill mask",tools:"Image tools", hand:"Pan canvas", navigation:"Pan with middle mouse or Space; pinch or scroll to zoom", layer:"Layer", layers:"Select layers", brush:"Brush", eraser:"Eraser", rectangle:"Rectangle selection", ellipse:"Ellipse selection", lasso:"Lasso selection", wand:"Magic wand", bucket:"Fill", eyedropper:"Color picker",
    color:"Foreground", size:"Size", opacity:"Opacity", hardness:"Hardness", tolerance:"Tolerance", selection:"Selection", replace:"Replace", add:"Add", subtract:"Subtract", intersect:"Intersect", all:"Select all pixels", none:"Deselect", invertSelection:"Invert selection",
    adjustment:"Adjustment", invert:"Invert colors", grayscale:"Grayscale", brightness:"Brightness", contrast:"Contrast", saturation:"Saturation", gamma:"Gamma", threshold:"Threshold", posterize:"Posterize", blur:"Box blur", sharpen:"Sharpen", value:"Amount", apply:"Apply",
    flipHorizontal:"Flip horizontally", flipVertical:"Flip vertically", rotateClockwise:"Rotate right", rotateCounterclockwise:"Rotate left", resize:"Resize layer", width:"Width", height:"Height", maskFromSelection:"Mask from selection", crop:"Crop to selection", clear:"Clear selected pixels", cancel:"Cancel", selectLayer:"Choose a visible pixel layer", selectMask:"Choose a layer with a mask", busy:"Preparing edit", submitted:"Edit submitted",applied:"Edit applied",pending:"Editing image; progress and cancellation are available in Tasks.",refused:"The edit could not be applied. Try again on the current image.",superseded:"A newer action replaced this edit.", error:"Image edit failed", hidden:"Hidden", pixels:"pixels", more:"Adjustments and transforms"
  },
  de: {
    locked:"Gesperrt",lockedHint:"Entsperre diese Ebene oder ihre Elternebene in der Inspektion, um Pixel oder Masken zu bearbeiten.",target:"Bearbeitungsziel",pixelTarget:"Ebenenpixel",maskTarget:"Maske",maskValue:"Maskenwert",fillMask:"Maske füllen",tools:"Bildwerkzeuge", hand:"Ansicht verschieben", navigation:"Mit mittlerer Maustaste oder Leertaste verschieben; mit zwei Fingern oder Mausrad zoomen", layer:"Ebene", layers:"Ebenen auswählen", brush:"Pinsel", eraser:"Radiergummi", rectangle:"Rechteckauswahl", ellipse:"Ellipsenauswahl", lasso:"Lassoauswahl", wand:"Zauberstab", bucket:"Füllen", eyedropper:"Farbpipette",
    color:"Vordergrund", size:"Größe", opacity:"Deckkraft", hardness:"Härte", tolerance:"Toleranz", selection:"Auswahl", replace:"Ersetzen", add:"Hinzufügen", subtract:"Abziehen", intersect:"Schnittmenge", all:"Alle Pixel auswählen", none:"Auswahl aufheben", invertSelection:"Auswahl umkehren",
    adjustment:"Anpassung", invert:"Farben umkehren", grayscale:"Graustufen", brightness:"Helligkeit", contrast:"Kontrast", saturation:"Sättigung", gamma:"Gamma", threshold:"Schwellenwert", posterize:"Tontrennung", blur:"Box-Weichzeichnung", sharpen:"Schärfen", value:"Stärke", apply:"Anwenden",
    flipHorizontal:"Horizontal spiegeln", flipVertical:"Vertikal spiegeln", rotateClockwise:"Rechts drehen", rotateCounterclockwise:"Links drehen", resize:"Ebene skalieren", width:"Breite", height:"Höhe", maskFromSelection:"Maske aus Auswahl", crop:"Auf Auswahl zuschneiden", clear:"Ausgewählte Pixel löschen", cancel:"Abbrechen", selectLayer:"Eine sichtbare Pixelebene wählen", selectMask:"Eine Ebene mit Maske wählen", busy:"Bearbeitung vorbereiten", submitted:"Bearbeitung übermittelt",applied:"Bearbeitung angewendet",pending:"Bild wird bearbeitet; Fortschritt und Abbruch sind unter Aufgaben verfügbar.",refused:"Die Bearbeitung konnte nicht angewendet werden. Auf dem aktuellen Bild erneut versuchen.",superseded:"Eine neuere Aktion hat diese Bearbeitung ersetzt.", error:"Bildbearbeitung fehlgeschlagen", hidden:"Ausgeblendet", pixels:"Pixel", more:"Anpassungen und Transformationen"
  }
} as const;
type Tool = "layers"|"hand"|"brush"|"eraser"|"rectangle"|"ellipse"|"lasso"|"wand"|"bucket"|"eyedropper";
type Filter = "invert"|"grayscale"|"brightness"|"contrast"|"saturation"|"gamma"|"threshold"|"posterize"|"blur"|"sharpen";
const ranges:Record<Exclude<Filter,"invert"|"grayscale">,readonly [number,number,number,number]> = {
  brightness:[-1,1,0.01,0],contrast:[-1,1,0.01,0],saturation:[-1,1,0.01,0],gamma:[0.01,10,0.01,1],threshold:[0,255,1,128],posterize:[2,256,1,4],blur:[0,16,1,2],sharpen:[0,5,0.1,1]
};
const control = "rounded border border-border bg-background px-2 py-1 text-xs focus-visible:outline focus-visible:outline-2 focus-visible:outline-accent disabled:opacity-40";
const toolList:Tool[] = ["layers","hand","brush","eraser","rectangle","ellipse","lasso","wand","bucket","eyedropper"];
/** 🌊️ How many new samples a stroke gathers before it streams them as one `paintStroke{phase:"stream"}` tick — the wgpu
 * host's `RASTER_STROKE_STREAM_BATCH` (`🗺️surface/🎨️paint/✍️gesture`). */
const STROKE_STREAM_BATCH=16;
let strokePress=0;

async function decodeLayer(layer:PixelLayer,assetsJson:string,signal:AbortSignal):Promise<PixelImage> {
  validateExtent(layer.width,layer.height);
  if(signal.aborted) throw new DOMException("Cancelled","AbortError");
  if(!layer.imageKey) return {width:layer.width,height:layer.height,pixels:new Uint8Array(layer.width*layer.height*4).fill(layer.target==="mask"?255:0)};
  const assets=JSON.parse(assetsJson) as Record<string,{mime:string;data:string}>;
  const asset=assets[layer.imageKey];
  if(!asset) throw new Error("Layer image is unavailable");
  const binary=atob(asset.data),bytes=Uint8Array.from(binary,c=>c.charCodeAt(0));
  const bitmap=await createImageBitmap(new Blob([bytes],{type:asset.mime}));
  try {
    if(signal.aborted) throw new DOMException("Cancelled","AbortError");
    validateExtent(bitmap.width,bitmap.height);
    const canvas=document.createElement("canvas");canvas.width=bitmap.width;canvas.height=bitmap.height;
    const context=canvas.getContext("2d",{willReadFrequently:true});
    if(!context) throw new Error("Pixel canvas is unavailable");
    context.drawImage(bitmap,0,0);
    const pixels=new Uint8Array(context.getImageData(0,0,bitmap.width,bitmap.height).data);
    return {width:bitmap.width,height:bitmap.height,pixels};
  } finally { bitmap.close(); }
}

export function PixelEditingOverlay({documentJson,assetsJson,assetExtentsJson,selectionJson,pixelSelectionJson,activeUtility,brushSize,brushOpacity,brushColor,brushHardness,paintTarget:target,maskValue,fillTolerance,camera,container,dispatch,onWheel,onCameraChange}:{
  documentJson:string;assetsJson:string;assetExtentsJson:string;selectionJson:string;pixelSelectionJson?:string;activeUtility:string;brushSize:number;brushOpacity:number;brushColor:string;brushHardness:number;paintTarget:"pixels"|"mask";maskValue:number;fillTolerance:number;
  camera:RefObject<{x:number;y:number;zoom:number}>;container:RefObject<HTMLDivElement|null>;
  onCameraChange:(camera:CanvasCamera)=>void;
  dispatch:(action:string,args?:Record<string,unknown>)=>void|Promise<unknown>;onWheel:(event:WheelEvent<HTMLDivElement>)=>void;
}) {
  const {i18n}=useTranslation();
  const locale=i18n.resolvedLanguage?.split("-")[0] as keyof typeof labels|undefined;
  const text=locale && labels[locale];
  const setTarget=(value:"pixels"|"mask")=>dispatch("setPaintTarget",{value});
  const setMaskValue=(value:number)=>dispatch("setMaskValue",{value});
  const layers=useMemo(()=>{try{return (target==="mask"?maskLayers:pixelLayers)(documentJson,assetExtentsJson);}catch{return [];}},[documentJson,assetExtentsJson,target]);
  const [tool,setTool]=useState<Tool>("layers");
  const color=brushColor,hardness=brushHardness;
  const setColor=(value:string)=>dispatch("setBrushColor",{value});
  const setHardness=(value:number)=>dispatch("setBrushHardness",{value});
  const setTolerance=(value:number)=>dispatch("setFillTolerance",{value});
  const [merge,setMerge]=useState<SelectionMerge>("replace");
  const [coverage,setCoverage]=useState<{key:string;mask:Uint8Array}>();
  const [restoreProgress,setRestoreProgress]=useState<number|null>(null);
  const restoration=useRef<AbortController|null>(null);
  const [maskCanvas,setMaskCanvas]=useState<HTMLCanvasElement|null>(null);
  const [filter,setFilter]=useState<Filter>("brightness");
  const [amount,setAmount]=useState(0);
  const [width,setWidth]=useState(512),[height,setHeight]=useState(512);
  const [message,setMessage]=useState("");
  const [error,setError]=useState("");
  const [progress,setProgress]=useState<number|null>(null);
  const [gesture,setGesture]=useState<readonly PixelPoint[]>([]);
  const [collapsed,setCollapsed]=useState(false);
  const abort=useRef<AbortController|null>(null);
  const working=useRef(false),submitted=useRef(false),mounted=useRef(true);
  const [pending,setPending]=useState(false);
  const points=useRef<PixelPoint[]>([]);
  const streamed=useRef<{gesture:string;sent:number;layerId:string;tool:Tool}|null>(null);
  const pointer=useRef<number|null>(null);
  const overlay=useRef<HTMLCanvasElement>(null);
  const epoch=useRef(0);
  const [navigation]=useState(()=>new GestureRecognizer());
  const pan=useRef<PixelPoint|null>(null);
  const space=useRef(false);
  const [viewportVersion,setViewportVersion]=useState(0);
  useEffect(()=>{
    const host=container.current;if(!host) return;
    const observer=new ResizeObserver(()=>setViewportVersion(v=>v+1));observer.observe(host);
    return ()=>observer.disconnect();
  },[container]);
  const selected=useMemo(()=>{try{return JSON.parse(selectionJson) as string[];}catch{return [];}},[selectionJson]);
  const active=layers.find(layer=>layer.id===selected[0]);
  const gestureRevision=pixelGestureRevision(active);
  const selectionKey=JSON.stringify([pixelSelectionJson,active?.id,active?.width,active?.height,target]);
  const source=useMemo(()=>{
    if(!pixelSelectionJson||!active)return null;
    try{
      if(pixelSelectionJson.length>42000)throw new Error("Selection exceeds the transport budget");
      const value=JSON.parse(pixelSelectionJson);
      if(!value||typeof value!=="object"||typeof value.spans!=="string")throw new Error("Invalid selection state");
      return value.layerId===active.id&&value.target===target&&value.width===active.width&&value.height===active.height?{spans:value.spans as string}:null;
    }catch(cause){return {error:cause instanceof Error?cause.message:String(cause)};}
  },[selectionKey]);
  const mask=source&&"spans" in source&&coverage?.key===selectionKey?coverage.mask:undefined;
  const selectionReady=!source||("spans" in source&&coverage?.key===selectionKey);

  const ready=!!active?.visible && progress===null && !pending && selectionReady;
  const editable=ready&&!active?.locked;

  useEffect(()=>{
    if(activeUtility==="paintBrush") setTool("brush");
    else if(activeUtility==="paintEraser") setTool("eraser");
    else setTool("layers");
  },[activeUtility]);
  const abandonStream=()=>{
    const open=streamed.current;streamed.current=null;
    if(open&&open.sent>0)void dispatch("paintStroke",{layerId:open.layerId,tool:open.tool,xs:[],ys:[],phase:"abort",reason:"captureLost",gesture:open.gesture});
  };
  useEffect(()=>{
    epoch.current++;
    abort.current?.abort();working.current=false;abandonStream();
    setProgress(null);setGesture([]);points.current=[];pointer.current=null;
    if(active) {setWidth(active.width);setHeight(active.height);}
  },[gestureRevision,active?.id,active?.width,active?.height,target,activeUtility,pixelSelectionJson]);
  useEffect(()=>{
    setCoverage(undefined);setRestoreProgress(null);
    if(!source||!active)return;
    if("error" in source){setError(source.error!);return;}
    const controller=new AbortController();restoration.current=controller;setRestoreProgress(0);
    void restoreSelection(source.spans!,active.width*active.height,{signal:controller.signal,onProgress:p=>setRestoreProgress(p.completed/p.total)}).then(mask=>{
      if(!controller.signal.aborted){setCoverage({key:selectionKey,mask});setRestoreProgress(null);}
    }).catch(cause=>{if(!controller.signal.aborted){setError(cause instanceof Error?cause.message:String(cause));setRestoreProgress(null);}});
    return ()=>controller.abort();
  },[selectionKey]);
  useEffect(()=>{mounted.current=true;return ()=>{mounted.current=false;abort.current?.abort();};},[]);
  useEffect(()=>{
    setMaskCanvas(null);
    if(!mask||!active) return;
    const controller=new AbortController(),w=active.width,h=active.height;
    void (async()=>{
      const canvas=document.createElement("canvas");canvas.width=w;canvas.height=h;
      const ctx=canvas.getContext("2d");if(!ctx) return;
      const image=ctx.createImageData(w,h);
      for(let from=0;from<mask.length;from+=32768) {
        if(controller.signal.aborted) return;
        for(let i=from;i<Math.min(from+32768,mask.length);i++) {
          if(!mask[i]) continue;
          const edge=i%w===0||i%w===w-1||i<w||i>=mask.length-w||!mask[i-1]||!mask[i+1]||!mask[i-w]||!mask[i+w];
          image.data.set(edge?[255,255,255,220]:[40,120,220,Math.round(mask[i]!*0.2)],i*4);
        }
        await new Promise<void>(resolve=>setTimeout(resolve,0));
      }
      if(controller.signal.aborted) return;
      ctx.putImageData(image,0,0);setMaskCanvas(canvas);
    })();
    return ()=>controller.abort();
  },[mask,active?.width,active?.height]);
  useEffect(()=>{
    const canvas=overlay.current,host=container.current,view=camera.current;
    if(!canvas || !host || !view) return;
    const box=host.getBoundingClientRect();
    const dpr=window.devicePixelRatio||1;
    canvas.width=Math.max(1,Math.round(box.width*dpr));canvas.height=Math.max(1,Math.round(box.height*dpr));canvas.style.width=box.width+"px";canvas.style.height=box.height+"px";
    const ctx=canvas.getContext("2d");
    if(!ctx || !active) return;
    ctx.scale(dpr,dpr);
    ctx.translate(box.width/2-view.x*view.zoom,box.height/2-view.y*view.zoom);
    ctx.scale(view.zoom,view.zoom);
    const m=active.matrix as [number,number,number,number,number,number];
    ctx.transform(...m);
    ctx.lineWidth=1/view.zoom;
    ctx.strokeStyle="#ffffff";ctx.setLineDash([4/view.zoom,4/view.zoom]);
    if(maskCanvas) {ctx.imageSmoothingEnabled=false;ctx.drawImage(maskCanvas,0,0);}
    if(gesture.length) {
      ctx.beginPath();
      const start=gesture[0]!,end=gesture[gesture.length-1]!;
      if(tool==="rectangle") ctx.rect(start[0],start[1],end[0]-start[0],end[1]-start[1]);
      else if(tool==="ellipse") ctx.ellipse((start[0]+end[0])/2,(start[1]+end[1])/2,Math.abs(end[0]-start[0])/2,Math.abs(end[1]-start[1])/2,0,0,Math.PI*2);
      else {
        ctx.moveTo(start[0],start[1]);
        for(const point of gesture) ctx.lineTo(point[0],point[1]);
        if(tool==="brush" || tool==="eraser") {ctx.setLineDash([]);ctx.lineWidth=brushSize;ctx.strokeStyle=target==="mask"?`rgb(${tool==="eraser"?0:maskValue} ${tool==="eraser"?0:maskValue} ${tool==="eraser"?0:maskValue})`:tool==="eraser"?"#ffffff":color;ctx.globalAlpha=brushOpacity;ctx.lineCap="round";ctx.lineJoin="round";}
      }
      ctx.stroke();
    }
  },[active,maskCanvas,gesture,tool,color,target,maskValue,brushSize,brushOpacity,viewportVersion,camera.current?.x,camera.current?.y,camera.current?.zoom]);

  const cancel=()=>{epoch.current++;abort.current?.abort();working.current=false;abandonStream();points.current=[];pointer.current=null;setGesture([]);setProgress(null);};
  const run=async(work:(signal:AbortSignal)=>Promise<void>)=>{
    if(!active?.visible || !selectionReady || working.current || submitted.current) return;
    working.current=true;
    const controller=new AbortController(),version=epoch.current;
    abort.current=controller;setProgress(0);setError("");setMessage("");
    try {await work(controller.signal);}
    catch(cause) {if(epoch.current===version && !(cause instanceof DOMException && cause.name==="AbortError")) setError(cause instanceof Error?cause.message:String(cause));}
    finally {if(epoch.current===version){working.current=false;setProgress(null);}}
  };
  const dispatchEdit=async(action:string,payload:Record<string,unknown>)=>{
    submitted.current=true;setPending(true);setProgress(null);
    try {
      const result=await dispatch(action,payload);
      if(!mounted.current)return;
      const kind=result&&typeof result==="object"&&"kind" in result?result.kind:undefined;
      if(kind==="refused")setError(text?.refused??"");
      else if(kind==="superseded")setError(text?.superseded??"");
      else setMessage((kind==="applied"?text?.applied:text?.submitted)??"");
    } catch(cause) {if(mounted.current)setError(cause instanceof Error?cause.message:String(cause));}
    finally {submitted.current=false;if(mounted.current)setPending(false);}
  };
  const publishSelection=async(mask:Uint8Array,signal:AbortSignal)=>{
    if(!active)return;
    const spans=await selectionSpans(mask,{signal,onProgress:p=>setProgress(p.completed/p.total)});
    if(signal.aborted)return;
    await dispatchEdit("setPixelSelection",{selection:{layerId:active.id,target,width:active.width,height:active.height,spans},expectedImageKey:active.imageKey});
  };
  const changeSelection=(mode:"all"|"invert")=>{void run(async signal=>{
    if(!active)return;
    const next=await editSelection(active.width*active.height,mask,mode,{signal,onProgress:p=>setProgress(p.completed/p.total)});
    if(!signal.aborted)await publishSelection(next,signal);
  });};
  const clearSelection=()=>{if(!submitted.current){cancel();restoration.current?.abort();setRestoreProgress(null);if(pixelSelectionJson)void dispatchEdit("setPixelSelection",{selection:null,expectedImageKey:null});}};
  const createMask=()=>{void run(async signal=>{
    if(!active||active.locked||!mask)return;
    const selection=await selectionSpans(mask,{signal,onProgress:p=>setProgress(p.completed/p.total)});
    const payload={layerId:active.id,expectedImageKey:active.imageKey,selection};
    if(JSON.stringify(payload).length>60000)throw new Error("The selection exceeds the command budget");
    if(signal.aborted)return;
    await dispatchEdit("maskFromSelection",payload);
  });};
  const transformImage=(operation:"rotateClockwise"|"rotateCounterclockwise"|"resize",size:{width:number;height:number}|null=null)=>{if(!active||active.locked)return;const layerId=active.id;void run(()=>dispatchEdit("transformImage",{layerId,operation,x:0,y:0,width:size?.width??0,height:size?.height??0,bilinear:size!==null}));};
  const cropImage=()=>{void run(async signal=>{if(!active||active.locked||!mask)return;const layerId=active.id;const bounds=await selectionBounds(mask,active.width,{signal,onProgress:p=>setProgress(p.completed/p.total)});if(bounds&&!signal.aborted)await dispatchEdit("transformImage",{layerId,operation:"crop",...bounds,bilinear:false});});};
  const fillSelection=()=>{if(!active||active.locked)return;const layerId=active.id;void run(()=>dispatchEdit("fillSelection",{layerId}));};
  const filterImage=(filter:string,amount=0)=>{if(!active||active.locked)return;const layerId=active.id;void run(()=>dispatchEdit("applyFilter",{layerId,filter,amount}));};
  const localPoint=(event:PointerEvent<HTMLDivElement>):PixelPoint=>{
    const host=container.current,view=camera.current;
    if(!host || !view || !active) throw new Error("No active image");
    const box=host.getBoundingClientRect();
    return layerPoint(active,view.x+(event.clientX-box.left-box.width/2)/view.zoom,view.y+(event.clientY-box.top-box.height/2)/view.zoom);
  };
  const select=async(next:Uint8Array,signal:AbortSignal,mode=merge)=>{
    const combined=await combineSelections(mask,next,mode,{signal,onProgress:p=>setProgress(0.5+0.5*p.completed/p.total)});
    if(!signal.aborted)await publishSelection(combined,signal);
  };
  const pick=async(point:PixelPoint,signal:AbortSignal)=>{
    if(!active) return;
    const x=Math.floor(point[0]),y=Math.floor(point[1]);
    if(tool==="bucket") {if(!signal.aborted&&!active.locked&&x>=0&&y>=0&&x<active.width&&y<active.height)await dispatchEdit("fillRegion",{layerId:active.id,x:point[0],y:point[1]});return;}
    const image=await decodeLayer(active,assetsJson,signal);
    if(x<0 || y<0 || x>=image.width || y>=image.height) return;
    if(tool==="eyedropper") {setColor("#"+[...image.pixels.slice((y*image.width+x)*4,(y*image.width+x)*4+3)].map(v=>v.toString(16).padStart(2,"0")).join(""));return;}
    const selected=await floodSelection(image,x,y,fillTolerance,{signal,onProgress:p=>setProgress(0.5*p.completed/p.total)});
    if(!signal.aborted) await select(selected,signal);
  };
  const down=(event:PointerEvent<HTMLDivElement>)=>{
    if(event.button!==0 && event.button!==1) return;
    event.stopPropagation();event.preventDefault();event.currentTarget.focus();event.currentTarget.setPointerCapture(event.pointerId);
    const verdict=navigation.down({pointerId:event.pointerId,x:event.clientX,y:event.clientY});
    if(verdict.kind==="pinchBegin") {cancel();pan.current=null;return;}
    if(verdict.kind!=="single") return;
    if(event.button===1 || tool==="hand" || space.current) {cancel();pan.current=[event.clientX,event.clientY];return;}
    if(!ready || !active || (active.locked&&["brush","eraser","bucket"].includes(tool))) return;
    if(pointer.current!==null) {cancel();return;}
    pointer.current=event.pointerId;
    try {
      const point=localPoint(event);points.current=[point];setGesture([point]);
      if(tool==="brush"||tool==="eraser")streamed.current={gesture:`paint:${Date.now()}:${++strokePress}`,sent:0,layerId:active.id,tool};
      if(["wand","bucket","eyedropper"].includes(tool)) {pointer.current=null;setGesture([]);void run(signal=>pick(point,signal));}
    } catch(cause) {setError(String(cause));cancel();}
  };
  const move=(event:PointerEvent<HTMLDivElement>)=>{
    const verdict=navigation.move({pointerId:event.pointerId,x:event.clientX,y:event.clientY});
    if(verdict.kind==="pinch") {
      const box=container.current?.getBoundingClientRect();
      if(box && camera.current) {
        const step={...verdict.step,centroidX:verdict.step.centroidX-box.left,centroidY:verdict.step.centroidY-box.top};
        onCameraChange(canvasPinchCamera(camera.current,step,box.width,box.height));setViewportVersion(v=>v+1);
      }
      return;
    }
    if(verdict.kind!=="single") return;
    if(pan.current && camera.current) {
      const view=camera.current;
      onCameraChange({...view,x:view.x-(event.clientX-pan.current[0])/view.zoom,y:view.y-(event.clientY-pan.current[1])/view.zoom});
      pan.current=[event.clientX,event.clientY];setViewportVersion(v=>v+1);return;
    }
    if(pointer.current!==event.pointerId) return;
    try {
      const point=localPoint(event),last=points.current[points.current.length-1]!;
      if(Math.hypot(point[0]-last[0],point[1]-last[1])<0.5) return;
      if(points.current.length>=2048) {setError("Stroke limit reached; release to apply");return;}
      points.current.push(point);setGesture([...points.current]);
      const open=streamed.current;
      if(open&&points.current.length-open.sent>=STROKE_STREAM_BATCH){
        const batch=points.current.slice(open.sent);open.sent=points.current.length;
        void dispatch("paintStroke",{layerId:open.layerId,tool:open.tool,xs:batch.map(point=>point[0]),ys:batch.map(point=>point[1]),phase:"stream",gesture:open.gesture});
      }
    } catch(cause) {setError(String(cause));cancel();}
  };
  const up=(event:PointerEvent<HTMLDivElement>)=>{
    const verdict=navigation.up(event.pointerId);
    if(verdict.kind!=="single" || pan.current) {pan.current=null;cancel();return;}
    if(pointer.current!==event.pointerId || !active) return;
    const path=points.current;
    try {if(path.length<2048) path.push(localPoint(event));} catch(cause) {setError(String(cause));cancel();return;}
    const open=streamed.current;streamed.current=null;
    points.current=[];pointer.current=null;setGesture([]);
    if(event.currentTarget.hasPointerCapture(event.pointerId)) event.currentTarget.releasePointerCapture(event.pointerId);
    if(!path.length) return;
    const start=path[0]!,end=path[path.length-1]!;
    if(tool==="brush"||tool==="eraser") {
      const rest=open&&open.sent>0?path.slice(open.sent):path;
      const payload={layerId:open?.layerId??active.id,tool,xs:rest.map(point=>point[0]),ys:rest.map(point=>point[1]),...(open&&open.sent>0?{phase:"commit",gesture:open.gesture}:{})};
      let ran=false;
      void run(async signal=>{ran=true;if(!signal.aborted&&!active.locked)await dispatchEdit("paintStroke",payload);});
      if(!ran&&open&&open.sent>0)void dispatch("paintStroke",{layerId:open.layerId,tool:open.tool,xs:[],ys:[],phase:"abort",reason:"captureLost",gesture:open.gesture});
      return;
    }
    void run(async signal=>{
      if(tool==="rectangle" || tool==="ellipse" || tool==="lasso") {
        if(signal.aborted) return;
        const shape=tool==="lasso"?{kind:"polygon" as const,points:path}:{kind:tool,x:start[0],y:start[1],width:end[0]-start[0],height:end[1]-start[1]};
        if(tool==="lasso" && path.length<3) return;
        const next=await selectPixels(active.width,active.height,shape,{signal,onProgress:p=>setProgress(0.5*p.completed/p.total)});
        await select(next,signal,event.shiftKey?"add":event.altKey?"subtract":merge);
      }
    });
  };
  if(!text) return null;
  return <>
    <canvas ref={overlay} aria-hidden="true" className="pointer-events-none absolute inset-0 z-30" />
    {tool!=="layers" && <div tabIndex={0} role="application" aria-label={text.tools} className="absolute inset-0 z-30 touch-none focus-visible:outline focus-visible:outline-accent"
      onPointerDown={down} onPointerMove={move} onPointerUp={up} onPointerCancel={event=>{navigation.up(event.pointerId);pan.current=null;cancel();}} onLostPointerCapture={()=>{if(pointer.current!==null)cancel();}} onWheel={onWheel}
      onBlur={()=>{space.current=false;pan.current=null;for(const tracked of [...navigation.pointers])navigation.up(tracked.pointerId);if(pointer.current!==null)cancel();}}
      onKeyUp={event=>{if(event.code==="Space"){event.preventDefault();space.current=false;}}}
      onKeyDown={event=>{
        if(event.code==="Space"){event.preventDefault();space.current=true;}
        if(!event.ctrlKey&&!event.metaKey&&!event.altKey) {
          const shortcut=({b:"brush",e:"eraser",m:"rectangle",l:"lasso",w:"wand",g:"bucket",i:"eyedropper",h:"hand",v:"layers"} as Record<string,Tool>)[event.key.toLowerCase()];
          if(shortcut&&!(target==="mask"&&["bucket","wand","eyedropper"].includes(shortcut))){event.preventDefault();cancel();setTool(shortcut);}
          if(event.key==="["||event.key==="]"){event.preventDefault();dispatch("setBrushSize",{value:Math.max(1,Math.min(2048,brushSize+(event.key==="["?-1:1)))});}
        }
        if(event.key==="Escape"){event.preventDefault();if(working.current||pointer.current!==null)cancel();else clearSelection();}
        if((event.ctrlKey||event.metaKey)&&event.key.toLowerCase()==="a"&&active){event.preventDefault();event.stopPropagation();changeSelection("all");}
        if((event.ctrlKey||event.metaKey)&&event.key.toLowerCase()==="d"){event.preventDefault();event.stopPropagation();clearSelection();}
        if(target==="pixels"&&(event.key==="Delete"||event.key==="Backspace")){event.preventDefault();event.stopPropagation();filterImage("clear");}
      }}/>}
    <section aria-label={text.tools} className="absolute left-2 top-2 z-40 max-h-[calc(100%-1rem)] max-w-[calc(100%-1rem)] overflow-auto rounded border border-border bg-background/95 p-2 text-foreground shadow-lg" style={{width:collapsed?"auto":"18rem"}}>
      <button type="button" className={control+" w-full text-left"} aria-expanded={!collapsed} onClick={()=>setCollapsed(!collapsed)}>{text.tools} {collapsed?"+":"−"}</button>
      {!collapsed && <div className="mt-2 flex flex-col gap-2">
        <label className="text-xs">{text.target}<select className={control+" w-full"} value={target} onChange={event=>{cancel();setTarget(event.target.value as "pixels"|"mask");if(["bucket","wand","eyedropper"].includes(tool))setTool("brush");}}><option value="pixels">{text.pixelTarget}</option><option value="mask">{text.maskTarget}</option></select></label>
        <label className="text-xs">{text.layer}<select className={control+" w-full"} value={active?.id??""} onChange={event=>{dispatch("interactionSelect",{domainId:"layers",targets:JSON.stringify([{granularity:"layer",id:event.target.value}]),merge:"replace",method:"pick"});}}>
          <option value="" disabled>{target==="mask"?text.selectMask:text.selectLayer}</option>{layers.map(layer=><option key={layer.id} value={layer.id} disabled={!layer.visible}>{layer.name}{layer.visible?"":" · "+text.hidden}{layer.locked?" · "+text.locked:""}</option>)}
        </select></label>
        {active?.locked&&<p role="status" className="text-xs">{text.lockedHint}</p>}
        <div role="toolbar" aria-label={text.tools} className="grid grid-cols-3 gap-1">{toolList.filter(value=>target==="pixels"||!["bucket","wand","eyedropper"].includes(value)).map(value=><button type="button" key={value} className={control} aria-pressed={tool===value} onClick={()=>{cancel();setTool(value);}}>{text[value]}</button>)}</div>
        <div className="flex gap-2">{target==="pixels"?<label className="text-xs">{text.color}<input aria-label={text.color} type="color" value={color} onChange={e=>setColor(e.target.value)} /></label>:<label className="text-xs">{text.maskValue}<input className={control+" w-20"} type="number" min={0} max={255} step={1} value={maskValue} onChange={e=>{if(e.target.validity.valid&&e.target.value)setMaskValue(Number(e.target.value));}}/></label>}
          <label className="text-xs">{text.size}<input className={control+" w-20"} type="number" min={1} max={2048} step={1} value={brushSize} onChange={e=>{if(e.target.validity.valid && e.target.value)dispatch("setBrushSize",{value:Number(e.target.value)});}} /></label>
        </div>
        <label className="text-xs">{text.opacity} {Math.round(brushOpacity*100)}%<input className="w-full" type="range" min={0} max={1} step={0.01} value={brushOpacity} onChange={e=>dispatch("setBrushOpacity",{value:Number(e.target.value)})}/></label>
        <label className="text-xs">{text.hardness}<input className="w-full" type="range" min={0} max={1} step={0.01} value={hardness} onChange={e=>setHardness(Number(e.target.value))}/></label>
        {["wand","bucket"].includes(tool)&&<label className="text-xs">{text.tolerance}<input className={control+" w-full"} type="number" min={0} max={255} value={fillTolerance} onChange={e=>{if(e.target.validity.valid&&e.target.value)setTolerance(Number(e.target.value));}}/></label>}
        <label className="text-xs">{text.selection}<select className={control+" w-full"} value={merge} onChange={e=>setMerge(e.target.value as SelectionMerge)}>{(["replace","add","subtract","intersect"] as const).map(value=><option key={value} value={value}>{text[value]}</option>)}</select></label>
        <div className="flex flex-wrap gap-1">
          <button className={control} disabled={!ready} onClick={()=>changeSelection("all")}>{text.all}</button>
          <button className={control} disabled={(!mask&&!source)||pending} onClick={clearSelection}>{text.none}</button>
          {target==="pixels"&&<button className={control} disabled={!editable||!mask} onClick={createMask}>{text.maskFromSelection}</button>}
          <button className={control} disabled={!ready} onClick={()=>changeSelection("invert")}>{text.invertSelection}</button>
        </div>
        {target==="mask"&&<button className={control} disabled={!editable} onClick={fillSelection}>{text.fillMask}</button>}
        {target==="pixels"&&<details><summary className="cursor-pointer text-xs">{text.more}</summary><div className="mt-2 flex flex-col gap-2">
          <label className="text-xs">{text.adjustment}<select className={control+" w-full"} value={filter} onChange={e=>{const next=e.target.value as Filter;setFilter(next);setAmount(next in ranges?ranges[next as keyof typeof ranges][3]:0);}}>
            {(["invert","grayscale",...Object.keys(ranges)] as Filter[]).map(value=><option key={value} value={value}>{text[value]}</option>)}
          </select></label>
          {filter in ranges&&<label className="text-xs">{text.value}<input className={control+" w-full"} type="number" min={ranges[filter as keyof typeof ranges][0]} max={ranges[filter as keyof typeof ranges][1]} step={ranges[filter as keyof typeof ranges][2]} value={amount} onChange={e=>setAmount(Number(e.target.value))}/></label>}
          <button className={control} disabled={!editable} onClick={()=>filterImage(filter,filter in ranges?amount:0)}>{text.apply}</button>
          <div className="grid grid-cols-2 gap-1">{(["flipHorizontal","flipVertical","rotateClockwise","rotateCounterclockwise"] as const).map(kind=><button key={kind} className={control} disabled={!editable} onClick={()=>kind==="flipHorizontal"||kind==="flipVertical"?filterImage(kind):transformImage(kind)}>{text[kind]}</button>)}</div>
          <div className="grid grid-cols-2 gap-1"><label className="text-xs">{text.width}<input className={control+" w-full"} type="number" min={1} max={16384} value={width} onChange={e=>setWidth(Number(e.target.value))}/></label>
            <label className="text-xs">{text.height}<input className={control+" w-full"} type="number" min={1} max={16384} value={height} onChange={e=>setHeight(Number(e.target.value))}/></label></div>
          <button className={control} disabled={!editable} onClick={()=>transformImage("resize",{width,height})}>{text.resize}</button>
          <button className={control} disabled={!editable||!mask} onClick={cropImage}>{text.crop}</button>
          <button className={control} disabled={!editable} onClick={()=>filterImage("clear")}>{text.clear}</button>
        </div></details>}
        <p className="text-xs text-muted-foreground">{text.navigation}</p>
        {active&&<output className="text-xs">{active.width} × {active.height} {text.pixels}</output>}
        {pending&&<p role="status" aria-live="polite">{text.pending}</p>}
        {!pending&&(progress!==null||restoreProgress!==null)&&<div role="status" className="text-xs">{text.busy}<progress className="w-full" max={1} value={progress??restoreProgress!}/><button className={control} onClick={restoreProgress!==null?clearSelection:cancel}>{text.cancel}</button></div>}
        {message&&<p role="status" className="text-xs">{message}</p>}
        {error&&<p role="alert" className="text-xs text-destructive">{text.error}: {error}</p>}
      </div>}
    </section>
  </>;
}
