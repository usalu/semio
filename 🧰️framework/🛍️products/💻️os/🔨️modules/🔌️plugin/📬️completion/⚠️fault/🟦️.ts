/** ⚠️ Completion reports preserve the original diagnostic owner and limit only their inline publication projection. */
export interface CompletionFaultSource {origin:"edge"|"renderer"|"os"|"module"|"plugin"|"app"|"extension"|"framework";code:string;severity:"info"|"warning"|"error"|"fatal";message:string;retryable:boolean}
export interface CompletionFault<T extends CompletionFaultSource> {original:T;report:string}
const utf8=new TextEncoder();
function prefix(text:string,maximum:number):{text:string;complete:boolean}{let output="",bytes=0;for(const scalar of text){const escaped=JSON.stringify(scalar).slice(1,-1);const length=utf8.encode(escaped).length;if(bytes+length>maximum)return {text:output,complete:false};output+=scalar;bytes+=length;}return {text:output,complete:true};}
/** 📥️ Retains the exact original reference and constructs the same finite report as the native carrier. */
export function createCompletionFault<T extends CompletionFaultSource>(original:T):CompletionFault<T>{
 const code=prefix(original.code,128),message=prefix(original.message,224);
 const report=JSON.stringify({origin:original.origin,code:code.complete?code.text:"interactive-job.fault-capacity",severity:original.severity,message:message.text,scope:{},retryable:original.retryable});
 if(utf8.encode(report).length>480)throw new RangeError("Completion fault report exceeds its declared frame");
 return {original,report};
}
