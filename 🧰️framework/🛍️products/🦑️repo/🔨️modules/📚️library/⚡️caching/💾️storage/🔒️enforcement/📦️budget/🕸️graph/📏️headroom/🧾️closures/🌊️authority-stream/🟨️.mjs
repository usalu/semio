/** 🌊️ Reads one graph authority receipt (`{"version":..,"observations":[..],..}`) as text chunks without ever holding more than one record: every header value and every element of `observations` is handed to the sink as soon as its text is complete, so memory is bounded by `maximumUnitUnits`, never by the receipt. Self-contained on purpose: the headroom receipt program embeds `createAuthorityStreamV1.toString()` into a plain Node child. */
export function createAuthorityStreamV1(maximumUnitUnits,sink){
 const refuse=()=>{throw Error("Closed streamed authority DATA required");},special=/["\\]/gu;
 let depth=0,inString=false,escaped=false,keyMode=false,pendingKey="",key=null,want="open",unit="",collecting=false,unitDepth=0,inObservations=false,done=false,sawObservations=false;
 const finish=()=>{collecting=false;const text=unit;unit="";if(text.length>maximumUnitUnits)refuse();let value;try{value=JSON.parse(text);}catch{refuse();}if(inObservations)sink.observation(value);else sink.header(key,value);};
 const feed=text=>{
  let seg=collecting?0:-1;
  for(let at=0;at<text.length;){
   if(inString){if(escaped){escaped=false;at++;continue;}special.lastIndex=at;const hit=special.exec(text),end=hit?hit.index:text.length;if(keyMode)pendingKey+=text.slice(at,end);if(!hit){at=text.length;break;}at=end+1;if(hit[0]==="\\"){if(keyMode)refuse();escaped=true;}else{inString=false;if(keyMode){keyMode=false;key=pendingKey;}}continue;}
   const c=text[at];
   if(c===" "||c==="\n"||c==="\r"||c==="\t"){at++;continue;}
   if(done)refuse();
   if(collecting){
    if(c==="\""){inString=true;at++;}
    else if(c==="{"||c==="["){depth++;at++;}
    else if(c==="}"||c==="]"){if(depth===unitDepth){unit+=text.slice(seg,at);finish();want=inObservations?"elementEnd":"headerEnd";seg=-1;}else{depth--;at++;}}
    else if(c===","&&depth===unitDepth){unit+=text.slice(seg,at);finish();want=inObservations?"elementEnd":"headerEnd";seg=-1;}
    else at++;
    continue;
   }
   if(depth===0){if(c!=="{")refuse();depth=1;want="key";at++;continue;}
   if(depth===1){
    if(want==="key"){if(c!=="\"")refuse();inString=true;keyMode=true;pendingKey="";want="colon";at++;}
    else if(want==="colon"){if(c!==":")refuse();want="value";at++;}
    else if(want==="value"){if(key==="observations"){if(c!=="["||sawObservations)refuse();inObservations=true;sawObservations=true;depth=2;want="first";at++;}else{collecting=true;unit="";seg=at;unitDepth=1;}}
    else if(want==="headerEnd"){if(c===","){want="key";at++;}else if(c==="}"){depth=0;done=true;at++;}else refuse();}
    else refuse();
    continue;
   }
   if(depth===2&&inObservations){
    if(want==="first"||want==="element"){if(c==="]"&&want==="first"){depth=1;inObservations=false;want="headerEnd";at++;}else if(c===","||c==="]"){refuse();}else{collecting=true;unit="";seg=at;unitDepth=2;}}
    else if(want==="elementEnd"){if(c===","){want="element";at++;}else if(c==="]"){depth=1;inObservations=false;want="headerEnd";at++;}else refuse();}
    else refuse();
    continue;
   }
   refuse();
  }
  if(collecting){unit+=text.slice(seg<0?0:seg);if(unit.length>maximumUnitUnits)refuse();}
 };
 return{feed,end(){if(!done||depth!==0||inString||collecting||!sawObservations)refuse();}};
}
