/** 🛡️ Original resident control authority and ephemeral drafts for one action/window/source owner. */
import {applyStagedFormBinding,applyStagedFormPageBinding,sameStagedFormToken,stagedFormValue,STAGED_FORM_EVENT_ACTION,STAGED_FORM_PAGE_ACTION,type StagedArgs,type StagedFormBinding,type StagedFormPage,type StagedFormPageBinding,type StagedFormRequest,type StagedFormStatus,type StagedFormToken} from '@semio-tech/framework/manifest/staging';

export interface StagedControlAction {readonly scope:string;readonly name:string;readonly version:number}
export interface StagedFormIntent<Node,Revision> {
 readonly surface:string;readonly revision:Revision;readonly node:Node;readonly nodeKey:string;readonly trigger:string;
 readonly action:StagedControlAction;readonly args:unknown;readonly input:unknown;readonly seq:bigint;
}
interface StagedResidentControlBase<Node> {
 readonly node:Node;readonly key:string;readonly trigger:string;readonly disabled:boolean;readonly editable:boolean;
 readonly action:StagedControlAction;
}
export interface StagedResidentEditControl<Node> extends StagedResidentControlBase<Node> {readonly input:'text'|'choice'|'none';readonly binding:StagedFormBinding}
export interface StagedResidentPageControl<Node> extends StagedResidentControlBase<Node> {readonly input:'none';readonly binding:StagedFormPageBinding}
export type StagedResidentControl<Node>=StagedResidentEditControl<Node>|StagedResidentPageControl<Node>;
/** 🪟️ Host adapters borrow the exact admitted resident publication and its declared binding. */
export interface StagedControlSource<Node,Revision> {
 view():{readonly surface:string;readonly revision:Revision};
 status():StagedFormStatus|undefined;
 read(node:Node,trigger:string):StagedResidentControl<Node>|undefined;
}
export interface StagedFormOwner {readonly token:StagedFormToken;readonly locale:'en'|'de';readonly controllerId:string}
export type StagedIntentResult={readonly kind:'refused';readonly reason:string}|{readonly kind:'staged';readonly args:StagedArgs;readonly token:StagedFormToken;readonly layout:'preserve'|'replace';readonly page?:StagedFormPage};
interface Proof<Node,Revision> {readonly source:StagedControlSource<Node,Revision>;readonly surface:string;readonly revision:Revision;readonly token:StagedFormToken}
const maximum=(1n<<64n)-1n;
function counter(value:string):bigint|undefined{return /^(0|[1-9][0-9]{0,19})$/u.test(value)&&BigInt(value)<=maximum?BigInt(value):undefined;}
function sameAction(a:StagedControlAction,b:StagedControlAction):boolean{return a.scope===b.scope&&a.name===b.name&&a.version===b.version;}
function preserves(binding:StagedFormBinding):boolean{return binding.layout==='preserve'&&binding.edits.length===1&&binding.edits[0]?.source==='text';}
function pageControl<Node>(control:StagedResidentControl<Node>):control is StagedResidentPageControl<Node>{return control.action.name===STAGED_FORM_PAGE_ACTION;}
function ownPage(page:StagedFormPage):StagedFormPage{return Object.freeze({...page,...(page.choice?{choice:Object.freeze({...page.choice,path:Object.freeze(page.choice.path)})}:{})});}

/** 📝️ Host-owned immutable args remain local; a current ready publication alone permits execution. */
export class StagedFormSession<Node=string,Revision=string> {
 private token:StagedFormToken;
 private args:StagedArgs;
 private page:StagedFormPage|undefined;
 private proof:Proof<Node,Revision>|undefined;
 private closed=false;
 private readonly controllerId:string;
 private readonly locale:'en'|'de';
 private readonly sequences=new WeakMap<StagedControlSource<Node,Revision>,bigint>();
 constructor(owner:StagedFormOwner,args:StagedArgs){
  if(!owner.token.action||!owner.token.window||!owner.token.revision||counter(owner.token.generation)===undefined||!owner.controllerId||!['en','de'].includes(owner.locale))throw new Error('staging.owner');
  this.token=Object.freeze({...owner.token});this.args=args;this.controllerId=owner.controllerId;this.locale=owner.locale;
 }
 request():StagedFormRequest|undefined{return this.closed?undefined:{token:this.token,args:this.args,locale:this.locale,...(this.page?{page:this.page}:{})};}
 private next():StagedFormToken|undefined{const value=counter(this.token.generation);return value===undefined||value===maximum?undefined:Object.freeze({...this.token,generation:String(value+1n)});}
 private current():{proof:Proof<Node,Revision>;status:StagedFormStatus}|undefined{
  const proof=this.proof;if(this.closed||!proof)return undefined;const view=proof.source.view(),status=proof.source.status();
  return view.surface===proof.surface&&view.revision===proof.revision&&status&&sameStagedFormToken(status.token,proof.token)?{proof,status}:undefined;
 }
 publish(source:StagedControlSource<Node,Revision>):boolean{
  if(this.closed)return false;const view=source.view(),status=source.status();if(!status||!sameStagedFormToken(status.token,this.token))return false;
  this.proof={source,surface:view.surface,revision:view.revision,token:Object.freeze({...status.token})};return true;
 }
 private control(node:Node,trigger:string):{proof:Proof<Node,Revision>;status:StagedFormStatus;control:StagedResidentControl<Node>}|undefined{
  const current=this.current();if(!current)return undefined;const control=current.proof.source.read(node,trigger);
  if(!control||control.node!==node||control.trigger!==trigger||control.disabled||!control.editable||control.action.scope!==this.controllerId||![STAGED_FORM_EVENT_ACTION,STAGED_FORM_PAGE_ACTION].includes(control.action.name)||control.action.version!==1||!sameStagedFormToken(control.binding.token,current.status.token))return undefined;
  if(pageControl(control))return control.input==='none'&&trigger==='activate'&&sameStagedFormToken(current.status.token,this.token)?{...current,control}:undefined;
  if(!Array.isArray(control.binding.edits))return undefined;
  const inputs=control.binding.edits.filter(edit=>edit.source!=='fixed');if(inputs.length>1||control.binding.layout==='preserve'&&!preserves(control.binding))return undefined;
  const input=inputs.length?inputs[0]!.source:'none';if(control.input!==input||trigger!==(input==='none'?'activate':'change'))return undefined;
  return {...current,control};
 }
 handle(intent:StagedFormIntent<Node,Revision>):StagedIntentResult{
  const resident=this.control(intent.node,intent.trigger);if(!resident)return {kind:'refused',reason:'staging.resident'};
  const {proof,status,control}=resident;
  if(intent.surface!==proof.surface||intent.revision!==proof.revision||intent.nodeKey!==control.key||!sameAction(intent.action,control.action))return {kind:'refused',reason:'staging.origin'};
  if(typeof intent.seq!=='bigint'||intent.seq<1n||intent.seq>maximum||intent.seq<=(this.sequences.get(proof.source)??0n))return {kind:'refused',reason:'staging.sequence'};
  const next=this.next();if(!next)return {kind:'refused',reason:'staging.generation'};
  if(control.input==='none'?intent.input!==null&&intent.input!==undefined:typeof intent.input!=='string')return {kind:'refused',reason:'staging.input'};
  if(pageControl(control)){
   let page:StagedFormPage;try{page=ownPage(applyStagedFormPageBinding(status,this.token,control.binding));}catch{return {kind:'refused',reason:'staging.binding'};}
   this.page=page;this.token=next;this.sequences.set(proof.source,intent.seq);this.proof=undefined;
   return {kind:'staged',args:this.args,token:next,layout:'replace',page};
  }
  let args:StagedArgs;try{args=applyStagedFormBinding(this.args,status,control.binding,control.input==='none'?undefined:intent.input as string);}catch{return {kind:'refused',reason:'staging.binding'};}
  this.args=args;this.token=next;this.sequences.set(proof.source,intent.seq);if(!preserves(control.binding)){this.proof=undefined;this.page=undefined;}
  return {kind:'staged',args,token:next,layout:control.binding.layout,...(this.page?{page:this.page}:{})};
 }
 display(node:Node,trigger:string):string|undefined{
  const resident=this.control(node,trigger);if(!resident||pageControl(resident.control)||!preserves(resident.control.binding))return undefined;
  const edit=resident.control.binding.edits[0];if(edit?.source!=='text')return undefined;const value=stagedFormValue(this.args,edit.path);return typeof value==='string'?value:undefined;
 }
 /** 🎛️ Lets ordinary controls reflect the same original resident gate used by dispatch. */
 canInteract(node:Node,trigger:string):boolean{return this.control(node,trigger)!==undefined;}
 canExecute():boolean{const current=this.current();return !!current&&current.status.ready&&sameStagedFormToken(current.status.token,this.token);}
 executionArgs():StagedArgs|undefined{return this.canExecute()?this.args:undefined;}
 reset(args:StagedArgs):boolean{if(this.closed)return false;const next=this.next();if(!next)return false;this.args=args;this.token=next;this.proof=undefined;this.page=undefined;return true;}
 revise(revision:string):boolean{if(this.closed||!revision)return false;if(revision===this.token.revision)return true;const next=this.next();if(!next)return false;this.token=Object.freeze({...next,revision});this.proof=undefined;this.page=undefined;return true;}
 close():void{this.closed=true;this.proof=undefined;this.page=undefined;}
}
