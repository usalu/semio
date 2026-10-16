import{requireIssuedResourceControlV1,withRequestedIssuedResourceControlV1}from"../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{checkResourceLeaseControl,type ResourceLeaseControl}from"../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import policyData from"./🧬️schema/🔣️.json"with{type:"json"};
const policy=Object.freeze({...policyData.definitions.Policy.const});
import{PRINT_PDF_OPS_V1 as OPS,printPdfTransformV1,printPdfApplyV1,printPdfInverseV1,printPdfArrayV1,printPdfNumberV1,printPdfVectorV1,printPdfNameV1,printPdfRecordV1,printPdfGlyphsV1,requirePrintPdfOperatorFactsV1,type PrintPdfValueV1,type PrintPdfPageV1}from"../../../📦️artifacts/📖️pdf/🟦️.ts";
/** 🎨️ Captures actual normalized paint and visible stroke/fill facts in physical canvas coordinates. */
export type NativeControlPdfPath={paint:string;closed:boolean;points:number[][];bounds:number[];width:number;opacity:number;fillOpacity:number;stroke:number[];fill:number[];dash:number[];dashPhase:number};
/** 🎨️ Collects owned PDF paint in physical canvas coordinates with exact cubic extrema. */
export type NativePdfPaintV1={paths:NativeControlPdfPath[];glyphs:{text:string;bounds?:number[];origin?:number[];advance?:number[];fill?:number[];fillOpacity?:number;strokeOpacity?:number;textMode?:number}[];images:{operator:string;corners:number[][];opacity:number;resource?:string;sourceWidth?:number;sourceHeight?:number}[]};
/** 🎨️ Interprets immutable operator DATA under one finite issued paint phase. */
export function nativeControlPdfPaint(operators:PrintPdfPageV1["operators"],canvas:number[],marker:string,control:ResourceLeaseControl,excluded:readonly string[]=[]):Promise<NativePdfPaintV1>{
 requireIssuedResourceControlV1(control);operators=requirePrintPdfOperatorFactsV1(operators);canvas=printPdfVectorV1(canvas,2);if(operators.fnArray.length>policy.maximumOperators)throw Error("Print paint operator capacity exceeded");if(canvas.some(value=>value<=0)||typeof marker!=="string"||!marker||marker.length>256||excluded.length>64||excluded.some(name=>typeof name!=="string"||!name||name.length>256))throw Error("Finite PDF paint selection required");excluded=Object.freeze([...excluded]);
 return withRequestedIssuedResourceControlV1(control,0,async control=>{
 let steps=0,pointCount=0,rowCount=0,logicalBytes=0,textBytes=0;const started=performance.now(),checkpoint=async()=>{checkResourceLeaseControl(control);if(steps%policy.progressSteps===0)await control.publish({resource:"print:pdf:paint",mode:"shared",elapsedMs:performance.now()-started});await control.yieldContinuation();checkResourceLeaseControl(control);},tick=():Promise<void>|undefined=>{if(++steps>policy.maximumSteps)throw Error("Print paint work capacity exceeded");if(steps%policy.yieldSteps===0)return checkpoint();},charge=(bytes:number)=>{if(bytes>policy.maximumLogicalOutputBytes-logicalBytes)throw Error("Print paint logical output capacity exceeded");logicalBytes+=bytes;},point=(count=1)=>{if(count>policy.maximumPoints-pointCount)throw Error("Print paint point capacity exceeded");pointCount+=count;charge(count*policy.pointChargeBytes);},row=(extra=0)=>{if(++rowCount>policy.maximumRows)throw Error("Print paint row capacity exceeded");charge(policy.rowChargeBytes+extra);};await checkpoint();

  const mm=72/25.4,paths:NativeControlPdfPath[]=[],glyphs:{text:string;bounds?:number[];origin?:number[];advance?:number[];fill?:number[];fillOpacity?:number;strokeOpacity?:number;textMode?:number}[]=[],images:{operator:string;corners:number[][];opacity:number;resource?:string;sourceWidth?:number;sourceHeight?:number}[]=[],markers:string[]=[],stack:{matrix:number[];width:number;opacity:number;fillOpacity:number;stroke:number[];fill:number[];dash:number[];dashPhase:number}[]=[];
  const excludedNames=new Set(excluded);let selectedDepth=0,excludedDepth=0;
  let matrix=[1,0,0,1,0,0],width=1,opacity=1,fillOpacity=1,stroke=[0,0,0],fill=[0,0,0],dash:number[]=[],dashPhase=0,reference:number[]|undefined,matched=false;
  let textMatrix=[1,0,0,1,0,0],lineMatrix=[...textMatrix],fontSize=0,horizontalScale=1,charSpacing=0,wordSpacing=0,rise=0,leading=0,textMode=0;
  const rgbValue=(args:readonly PrintPdfValueV1[]):number[]=>{const first=args[0];if(typeof first==="string"){if(!/^#[\da-f]{6}$/i.test(first))throw Error("Actual RGB paint string required");return[1,3,5].map(index=>parseInt(first.slice(index,index+2),16)/255);}return printPdfVectorV1(args.length===1&&Array.isArray(first)?first:args,3).map(value=>value>1?value/255:value);};
  for(const[index,id]of operators.fnArray.entries()){
    const waiting=tick();if(waiting)await waiting;
    const args=printPdfArrayV1(operators.argsArray[index]);
    if(id===OPS.save||id===OPS.paintFormXObjectBegin){if(stack.length>=policy.maximumStackDepth)throw Error("Print paint graphics depth exceeded");stack.push({matrix:[...matrix],width,opacity,fillOpacity,stroke:[...stroke],fill:[...fill],dash:[...dash],dashPhase});if(id===OPS.paintFormXObjectBegin&&args[0])matrix=printPdfTransformV1(matrix,printPdfVectorV1(args[0],6));}
    else if(id===OPS.restore||id===OPS.paintFormXObjectEnd){const state=stack.pop();if(!state)throw Error("Control PDF graphics stack underflow");({matrix,width,opacity,fillOpacity,stroke,fill,dash,dashPhase}=state);}
    else if(id===OPS.transform)matrix=printPdfTransformV1(matrix,printPdfVectorV1(args,6));
    else if(id===OPS.setLineWidth)width=printPdfNumberV1(args[0]);
    else if(id===OPS.setDash){const values=printPdfArrayV1(args[0]);if(values.length>policy.maximumDashEntries)throw Error("Print paint dash capacity exceeded");dash=printPdfVectorV1(values);dashPhase=printPdfNumberV1(args[1]);}
    else if(id===OPS.setStrokeRGBColor)stroke=rgbValue(args);
    else if(id===OPS.setFillRGBColor)fill=rgbValue(args);
    else if(id===OPS.setGState){for(const incoming of printPdfArrayV1(args[0])){const waiting=tick();if(waiting)await waiting;const state=printPdfArrayV1(incoming);if(state[0]==="LW")width=printPdfNumberV1(state[1]);if(state[0]==="CA")opacity=printPdfNumberV1(state[1]);if(state[0]==="ca")fillOpacity=printPdfNumberV1(state[1]);}}
    else if(id===OPS.beginMarkedContent||id===OPS.beginMarkedContentProps){const name=printPdfNameV1(args[0]);if(markers.length>=policy.maximumStackDepth)throw Error("Print paint marker depth exceeded");markers.push(name);if(name===marker){matched=true;selectedDepth++;}if(excludedNames.has(name))excludedDepth++;}
    else if(id===OPS.endMarkedContent){if(!markers.length)throw Error("Control PDF marked-content stack underflow");const name=markers.pop();if(name===marker)selectedDepth--;if(name!==undefined&&excludedNames.has(name))excludedDepth--;}
    else if(id===OPS.beginText){textMatrix=[1,0,0,1,0,0];lineMatrix=[...textMatrix];}
    else if(id===OPS.setFont)fontSize=printPdfNumberV1(args[1]);
    else if(id===OPS.setHScale)horizontalScale=printPdfNumberV1(args[0])/100;
    else if(id===OPS.setCharSpacing)charSpacing=printPdfNumberV1(args[0]);
    else if(id===OPS.setWordSpacing)wordSpacing=printPdfNumberV1(args[0]);
    else if(id===OPS.setTextRise)rise=printPdfNumberV1(args[0]);
    else if(id===OPS.setLeading)leading=printPdfNumberV1(args[0]);
    else if(id===OPS.setTextRenderingMode)textMode=printPdfNumberV1(args[0]);
    else if(id===OPS.setTextMatrix){textMatrix=printPdfVectorV1(args,6);lineMatrix=[...textMatrix];}
    else if(id===OPS.moveText||id===OPS.setLeadingMoveText||id===OPS.nextLine){const x=id===OPS.nextLine?0:printPdfNumberV1(args[0]),y=id===OPS.nextLine?-leading:printPdfNumberV1(args[1]);if(id===OPS.setLeadingMoveText)leading=-y;lineMatrix=printPdfTransformV1(lineMatrix,[1,0,0,1,x,y]);textMatrix=[...lineMatrix];}
    else if(id===OPS.showText){
      const parts:string[]=[];let advance=0;for(const incoming of printPdfArrayV1(args[0])){const waiting=tick();if(waiting)await waiting;const value=printPdfGlyphsV1([incoming])[0];if(value===undefined)throw Error("Actual PDF glyph required");if(typeof value==="number")advance+=-value*fontSize/1000*horizontalScale;else{const text=value.unicode??"",bytes=Buffer.byteLength(text);if(bytes>policy.maximumTextBytes-textBytes)throw Error("Print paint text capacity exceeded");textBytes+=bytes;parts.push(text);advance+=((value.width??0)*fontSize/1000+charSpacing+(value.isSpace?wordSpacing:0))*horizontalScale;}}const text=parts.join("");
      if(text&&selectedDepth>0&&excludedDepth===0&&reference&&(([0,2,4,6].includes(textMode)&&fillOpacity>0)||([1,2,5,6].includes(textMode)&&opacity>0))){
        const relative=printPdfTransformV1(printPdfInverseV1(reference),printPdfTransformV1(matrix,textMatrix)),transform=(point:number[])=>{printPdfApplyV1(point,relative);return point.map(value=>value/mm);},origin=transform([0,rise]),end=transform([advance,rise]),corners=[[0,rise],[advance,rise],[0,rise+Math.abs(fontSize)],[advance,rise+Math.abs(fontSize)]].map(transform);
        row(Buffer.byteLength(text));point(10);glyphs.push({text,origin,fill:[...fill],fillOpacity,strokeOpacity:opacity,textMode,advance:end.map((value,axis)=>value-origin[axis]!),bounds:[Math.min(...corners.map(point=>point[0]!)),Math.min(...corners.map(point=>point[1]!)),Math.max(...corners.map(point=>point[0]!)),Math.max(...corners.map(point=>point[1]!))]});
      }
      textMatrix=printPdfTransformV1(textMatrix,[1,0,0,1,advance,0]);
    }
    else if([OPS.paintImageXObject,OPS.paintInlineImageXObject,OPS.paintImageMaskXObject].includes(id)&&selectedDepth>0&&excludedDepth===0){
      if(!reference)throw Error("Control PDF image precedes canvas reference");
      const relative=printPdfTransformV1(printPdfInverseV1(reference),matrix),corners=[[0,0],[1,0],[1,1],[0,1]].map(point=>{printPdfApplyV1(point,relative);return point.map(value=>value/mm);}),source=args[0]!==null&&typeof args[0]==="object"&&!Array.isArray(args[0])?printPdfRecordV1(args[0]):undefined;
      row();point(4);images.push({operator:id,corners,opacity:fillOpacity,...typeof args[0]==="string"?{resource:args[0]}:{},sourceWidth:Number(source?.width??args[1])||undefined,sourceHeight:Number(source?.height??args[2])||undefined});
    }
    else if(id===OPS.constructPath&&args[2]){
      const bounds=printPdfVectorV1(args[2]),packed=printPdfVectorV1(printPdfArrayV1(args[1])[0]),isReference=args[0]===OPS.stroke&&Math.abs(width-.01*mm)<.002&&bounds.every((value,axis)=>Math.abs(value-[0,0,canvas[0]!*mm,canvas[1]!*mm][axis]!)<.03);
      if(isReference&&selectedDepth===0){if(reference)throw Error("Control PDF duplicate canvas reference");reference=[...matrix];continue;}
      if(selectedDepth===0||excludedDepth>0)continue;
      if(!reference)throw Error("Control PDF paint precedes canvas reference");
      const stroked=[OPS.stroke,OPS.closeStroke,OPS.fillStroke,OPS.eoFillStroke,OPS.closeFillStroke,OPS.closeEOFillStroke].includes(printPdfNameV1(args[0])),filled=[OPS.fill,OPS.eoFill,OPS.fillStroke,OPS.eoFillStroke,OPS.closeFillStroke,OPS.closeEOFillStroke].includes(printPdfNameV1(args[0]));
      if((!stroked||opacity<=0)&&(!filled||fillOpacity<=0))continue;
      const relative=printPdfTransformV1(printPdfInverseV1(reference),matrix),transform=(point:number[])=>{const value=[...point];printPdfApplyV1(value,relative);return value.map(coordinate=>coordinate/mm);};
      let cursor=0,current:number[]|undefined,first:number[]|undefined,points:number[][]=[],extrema:number[][]=[],closed=false;
      const publish=()=>{if(points.length){const values=extrema.length?extrema:points,bounds=[Infinity,Infinity,-Infinity,-Infinity];for(const point of values){bounds[0]=Math.min(bounds[0]??Infinity,point[0]??NaN);bounds[1]=Math.min(bounds[1]??Infinity,point[1]??NaN);bounds[2]=Math.max(bounds[2]??-Infinity,point[0]??NaN);bounds[3]=Math.max(bounds[3]??-Infinity,point[1]??NaN);}row(dash.length*8);paths.push({paint:printPdfNameV1(args[0]),closed,points,bounds,width:width/mm*Math.sqrt(Math.abs(relative[0]!*relative[3]!-relative[1]!*relative[2]!)),opacity,fillOpacity,stroke:[...stroke],fill:[...fill],dash:[...dash],dashPhase});}points=[];extrema=[];closed=false;};
      while(cursor<packed.length){
        const waiting=tick();if(waiting)await waiting;
        const command=packed[cursor++]!;
        if(command===0){publish();current=transform([packed[cursor++]!,packed[cursor++]!]);first=[...current];point(2);points.push(current);extrema.push(current);}
        else if(command===1){current=transform([packed[cursor++]!,packed[cursor++]!]);point(2);points.push(current);extrema.push(current);}
        else if(command===2){
          if(!current)throw Error("Control PDF cubic without start");
          const curve=[current,transform([packed[cursor++]!,packed[cursor++]!]),transform([packed[cursor++]!,packed[cursor++]!]),transform([packed[cursor++]!,packed[cursor++]!])],at=(t:number)=>[0,1].map(axis=>(1-t)**3*curve[0]![axis]!+3*(1-t)**2*t*curve[1]![axis]!+3*(1-t)*t*t*curve[2]![axis]!+t**3*curve[3]![axis]!);
          for(let sample=1;sample<=128;sample++){const waiting=tick();if(waiting)await waiting;point();points.push(at(sample/128));}
          point(2);extrema.push(curve[0]!,curve[3]!);
          for(let axis=0;axis<2;axis++){const[p0,p1,p2,p3]=curve.map(point=>point[axis]!),a=-p0!+3*p1!-3*p2!+p3!,b=2*(p0!-2*p1!+p2!),c=p1!-p0!,discriminant=b*b-4*a*c,roots=Math.abs(a)<1e-12?Math.abs(b)<1e-12?[]:[-c/b]:discriminant<0?[]:[(-b+Math.sqrt(discriminant))/(2*a),(-b-Math.sqrt(discriminant))/(2*a)];for(const t of roots)if(t>0&&t<1){point();extrema.push(at(t));}}
          current=curve[3]!;
        }else if(command===4){closed=true;if(first)current=first;}
        else throw Error("Unsupported owned PDF path command "+command);
      }
      publish();
    }
  }
  if(stack.length||markers.length)throw Error("Control PDF unbalanced graphics or ownership stack");
  if(!reference||!matched)throw Error("Control PDF canvas or ownership marker missing: "+marker);
  checkResourceLeaseControl(control);return{paths,glyphs,images};
 });
}

