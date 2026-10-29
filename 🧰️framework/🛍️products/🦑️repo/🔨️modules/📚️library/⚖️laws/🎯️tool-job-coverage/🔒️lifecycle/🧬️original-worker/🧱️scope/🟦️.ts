/** 🧱️ The sole worker pump is a direct expression in the original once-invoked callback path. */
import {toolJobRustBlock} from "../../../📋️inventory/🟦️.ts";
/** 🪜️ Nested delimiters fence iterator and custom callbacks without naming their dispatchers. */
export function toolJobOriginalWorkerPumpScopeExact(body:string):boolean{
 const code=body.replace(/"(?:[^"\\]|\\.)*"/g,text=>" ".repeat(text.length)).replace(/\s+/g,"");
 const pump="session.pump_one_for_interactive_turn(same)";
 const head="matchcx.with_original_work_loan(step_fuel,step_deadline_us,|same|"+pump+"){";
 const at=code.indexOf(head);
 if(at<0||code.indexOf(head,at+1)>=0||code[at-1]!==";"||/\bdrive_worker_step\b/.test(code))return false;
 const stack:{token:string;at:number}[]=[],pairs:Record<string,string>={")":"(","]":"[","}":"{"};
 for(let index=0;index<at;index++){
  const token=code[index]!;
  if("([{".includes(token))stack.push({token,at:index});
  else if(pairs[token]&&stack.pop()?.token!==pairs[token])return false;
 }
 if(stack.map(row=>row.token).join("")!=="({({")return false;
 const [call,callback,closure,step]=stack;
 if(!code.slice(0,call!.at).endsWith("semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step"))return false;
 if(code.slice(call!.at+1,callback!.at)!=="cx,grant,demand,|cx,grant|")return false;
 if(code.slice(callback!.at+1,closure!.at)!=="letstep=")return false;
 if(code.slice(closure!.at,step!.at)!=="(||->Result<PluginLifecycleStep,semio_framework_job::OriginalTurnFailure>")return false;
 const stepBlock=toolJobRustBlock(code,step!.at),callbackBlock=toolJobRustBlock(code,callback!.at);
 return !!stepBlock&&!!callbackBlock&&code.slice(stepBlock.end,callbackBlock.end-1)===")()?;Ok((step,self.terminal_is_empty()))"&&code.slice(callbackBlock.end)===").map_err(semio_framework_job::OriginalTurnFailure::from)";
}
