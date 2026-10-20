import {compileJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {rustTokens,rustTokenPairs,rustTokenText,type RustToken} from "../../../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/📦️packages/🟦️typescript/🟦️.ts";
import declaration from "./🔣️.json" with {type:"json"};

export type MemberProducerSourceRole=keyof typeof declaration.roles;
export type MemberProducerSourceBody=Readonly<{path:string;source:string}>;
export type MemberProducerSourceExtension=Readonly<{declaration:Readonly<{version:1;owner:string;producerType:string;producerMacro:string;membersMacro:string;roles:Readonly<Record<"producer"|"members",Readonly<{language:"rust";path:string;symbols:readonly string[]}>>>}>;producer:MemberProducerSourceBody;members:MemberProducerSourceBody}>;
export type ToolJobMemberStoreProducerSources=Readonly<Record<MemberProducerSourceRole,MemberProducerSourceBody>&{extensions:readonly MemberProducerSourceExtension[]}>;
export const memberStoreProducerSourceDeclarationV1=declaration;
const inputContract=compileJsonSchemaSubset(schema.$defs.ToolJobMemberStoreProducerSources,schema);

type RustBody=Readonly<{header:string;body:string}>;
const compact=(tokens:readonly RustToken[])=>rustTokenText(tokens.map(token=>token.kind==="string"?{...token,text:'""'}:token),0,tokens.length).replace(/\s/gu,"");
const spelling=(source:string)=>compact(rustTokens(source));
const contains=(body:string,...required:readonly string[])=>required.every(value=>body.includes(spelling(value)));
function memberProducerSourceInspector(){
 const documents=new Map<string,{tokens:readonly RustToken[];pairs:ReadonlyMap<number,number>}>(),itemCache=new Map<string,Map<string,RustBody[]>>();
function items(source:string,kind:string,name?:string,rootOnly=false):RustBody[]{
 const cache=itemCache.get(source)??new Map<string,RustBody[]>();itemCache.set(source,cache);
 const key=kind+"\0"+(name??"")+"\0"+rootOnly;const prior=cache.get(key);if(prior)return prior;
 let document=documents.get(source);if(!document){const tokens=rustTokens(source);document={tokens,pairs:rustTokenPairs(tokens)};documents.set(source,document);}
 const {tokens,pairs}=document,result:RustBody[]=[];
 for(let index=0;index<tokens.length;index++){
  if(tokens[index]?.text==="#"&&tokens[index+1]?.text==="["){
   const end=pairs.get(index+1);if(end===undefined)return [];
   const attribute=tokens.slice(index+2,end);
   if(attribute.some(token=>token.text==="cfg")&&attribute.some(token=>token.text==="test")){
    let at=end+1;
    for(;at<tokens.length;at++){const text=tokens[at]?.text;if(text===";")break;if(text==="{"){at=pairs.get(at)??tokens.length;break;}if(text==="("||text==="[")at=pairs.get(at)??tokens.length;}
    index=at;continue;
   }
  }
  if(tokens[index]?.text==="mod"){
   for(let at=index+1;at<tokens.length;at++){if(tokens[at]?.text===";"){index=at;break;}if(tokens[at]?.text==="{"){index=pairs.get(at)??tokens.length;break;}}
   continue;
  }
  if(rootOnly&&["{","(","["].includes(tokens[index]?.text??"")){index=pairs.get(index)??tokens.length;continue;}
  if(tokens[index]?.kind!=="identifier"||tokens[index]?.text!==kind)continue;
  let head=index+1;if(tokens[head]?.text==="!")head++;
  if(name!==undefined&&tokens[head]?.text!==name)continue;
  for(let end=head;end<tokens.length;end++){
   const text=tokens[end]?.text;
   if(text===";"){result.push({header:compact(tokens.slice(index,end+1)),body:""});break;}
   if(text==="{"){const close=pairs.get(end);if(close===undefined)return [];result.push({header:compact(tokens.slice(index,end)),body:rustTokenText(tokens,end+1,close)});break;}
   if(text==="("||text==="["){const close=pairs.get(end);if(close===undefined)return [];end=close;}
  }
 }
 cache.set(key,result);return result;
}
function selected(source:string,kind:string,name:string):RustBody|undefined{const found=items(source,kind,name,kind==="macro_rules"||kind==="struct"||kind==="trait");return found.length===1?found[0]:undefined;}
function implementation(source:string,trait:string,owner:string):RustBody|undefined{const found=items(source,"impl").filter(row=>contains(row.header,trait,"for "+owner));return found.length===1?found[0]:undefined;}
function method(source:string,name:string,...required:readonly string[]):boolean{const item=selected(source,"fn",name);return item!==undefined&&item.body.length>0&&contains(spelling(item.body),...required);}
function boundBody(source:string,kind:string,name:string,...required:readonly string[]):boolean{const item=selected(source,kind,name);return item!==undefined&&item.body.length>0&&contains(spelling(item.body),...required);}

 return {items,selected,implementation,method,boundBody};
}

/** 🏭️ Checks exact domain-declared producer bodies and the explicitly bound instance extension. */
export function toolJobMemberStoreProducerExact(sources:ToolJobMemberStoreProducerSources):boolean{
 if(inputContract.validate(sources).length)return false;
 const {items,selected,implementation,method,boundBody}=memberProducerSourceInspector();
 if(!sources||typeof sources!=="object"||Object.keys(sources).length!==Object.keys(declaration.roles).length+1)return false;
 for(const role of Object.keys(declaration.roles) as MemberProducerSourceRole[]){const row=sources[role];if(!row||row.path!==declaration.roles[role].path||typeof row.source!=="string"||!row.source)return false;}
 const {producer,catalog,opening,store,memberStore,storeOwnerInstall,storeComponents,storeLive,storeDisposer,storeCloseCursor,storeBackbone,valueFactory,readRetirement,editRetirement,messageLedger,storeLaws,catalogLaws}=Object.fromEntries(Object.entries(sources).filter(([role])=>role!=="extensions").map(([role,row])=>[role,(row as MemberProducerSourceBody).source])) as Record<MemberProducerSourceRole,string>;
 const trait=selected(producer,"trait","MemberStoreProducer"),owners=selected(catalog,"struct","DocumentStoreOwners"),open=selected(opening,"struct","InitialMemberStoreOpen");
 if(!trait||!owners||!open||!contains(trait.header,"MemberStoreProducer<P,M>: Copy + Send + Sync + 'static")||!contains(spelling(trait.body),"type SnapshotOpen:MemberSnapshotOpenOperation<Snapshot=P>;"))return false;
 for(const name of ["member_store_owners_birth_demand","member_store_owners"]){const fn=selected(trait.body,"fn",name);if(!fn||fn.body||!fn.header.endsWith(";"))return false;}
 if(!contains(spelling(trait.body),"fn member_store_owners(&self,grant:RetainedCloneGrant)","DocumentStoreOwners<P,M>","DocumentStoreOwnersAdmissionError<P,M>"))return false;
 if(!contains(spelling(owners.body),"snapshot_retirement:DocumentCatalogOwner<Arc<dyn SnapshotRetirementFactory<P>>>","initial_snapshot_retirement:DocumentCatalogOwner<Arc<dyn ArtifactOwnedValueRetirementFactory<P>>>","mutation_retirement:DocumentCatalogOwner<Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>>","store_disposer:DocumentCatalogOwner<Box<dyn ArtifactStoreOwnedDisposer<P,Mutation>>>","factory_retirement_tickets:semio_framework_value::FactoryChildTickets<6>"))return false;
 if(!method(catalog,"admit_constructor","if self.constructor_is_complete()","grant.maximum_items==0","grant.maximum_copy_bytes<demand.copy_bytes","grant.maximum_capacity_bytes<demand.capacity_bytes","grant.maximum_release_bytes<demand.release_bytes","grant.maximum_depth<demand.depth","self.factory_retirement_tickets.slots","grant.maximum_depth-1"))return false;
 if(!method(catalog,"constructor_is_complete","factory.is_none()||self.factory_retirement_tickets.slots[index].preparation_is_complete()")||!contains(spelling(catalog),"Err(DocumentStoreOwnersAdmissionError { error, owners: Some(owners), progress })"))return false;
 if(!method(catalog,"funded_member_store_owners","producer.member_store_owners_birth_demand()?","producer.member_store_owners(RetainedCloneGrant","birth.capacity_bytes","birth.depth.max(1)"))return false;
 if(!method(catalog,"complete_unscheduled_catalog","owners.constructor_is_complete()","owners.admit_constructor(","close_unscheduled_catalog(owners)"))return false;
 if(!method(catalog,"close_uninstalled_owners_step","self.uninstalled_owners_terminal_is_empty()","self.store_disposer.close_uninstalled_step(child)","admit_retained_clone_close(child, step, self.store_disposer.uninstalled_terminal_is_empty()","self.factory_retirement_tickets.terminal_is_empty()"))return false;
 if(!contains(spelling(open.body),"producer:C","source_loan:ManuallyDrop<Option<OriginalCallerSourceLoan>>","snapshot_open:ManuallyDrop<Option<C::SnapshotOpen>>","owners:ManuallyDrop<Option<semio_framework_os_store_engine_owners::DocumentStoreOwners<P,M>>>"))return false;
 const initial=items(opening,"impl").filter(row=>contains(row.header,"InitialMemberStoreOpen<F,P,M,C>")).map(row=>row.body).join("\n");
 if(!method(initial,"begin_birth_demand","producer.member_store_owners_birth_demand()","C::SnapshotOpen::begin_birth_bytes(request)?","bytes.checked_add(source.capacity_bytes)"))return false;
 if(!method(initial,"begin","request.check_step_authority(cx)","validate_context(cx)","producer.member_store_owners(source_grant)","progress.fits(source_grant)&&progress.retained_capacity_bytes==source_capacity","admission.request.take()","admission.source_loan.take()","C::SnapshotOpen::begin(request)"))return false;
 if(!method(initial,"step_store_inner","semio_framework_os_store_member_open::member_step_grant(cx,grant)","if !owners.constructor_is_complete()","owners.admit_constructor(grant)","self.owners.take().expect("))return false;
 if(!method(initial,"take_source_before_shell_return","if !self.payload_terminal_is_empty()","validate_context(cx)","self.source_loan.take().unwrap()"))return false;
 const exported=(owner:string,...names:readonly string[])=>items(store,"use").some(row=>contains(row.header,owner)&&contains(spelling(row.body),...names));
 if(!exported("semio_framework_os_store_space::lifted_store::","create_member_store","install_funded_member_store_owners","install_unscheduled_catalog","open_member_store")||!exported("semio_framework_os_store_engine_owners::lifted_store::","ArtifactStoreCursorDisposer","ArtifactStoreOwnedDisposer")||!exported("semio_framework_os_store_engine::lifted_store::","ArtifactStore,ConfigStore"))return false;
 const initialDrop=implementation(opening,"Drop","InitialMemberStoreOpen"),disposer=implementation(storeDisposer,"ArtifactStoreOwnedDisposer","ArtifactStoreCursorDisposer");
 if(!initialDrop||!method(initialDrop.body,"drop","self.ownership_is_empty()&&self.source_loan.is_none()&&!self.history_page_due()")||!disposer)return false;
 if(!method(storeOwnerInstall,"install_document_store_owners_exact","if !owners.constructor_is_complete()","self.snapshot_retirement_factory.is_some()","self.initial_snapshot_retirement_factory.is_some()","self.mutation_retirement_factory.is_some()","self.owned_disposer.is_some()","self.owned_disposer_terminal","*self.factory_retirement_tickets=owners.take_factory_retirement_tickets()","*self.snapshot_retirement_factory=owners.snapshot_retirement.take()","*self.initial_snapshot_retirement_factory=owners.initial_snapshot_retirement.take()","*self.mutation_retirement_factory=owners.mutation_retirement.take()","*self.owned_disposer=owners.store_disposer.take()"))return false;
 for(const name of ["create_member_store","open_member_store"]){const fn=selected(memberStore,"fn",name);if(!fn||!contains(fn.header,"C:MemberStoreProducer<P,Mutation>")||!contains(spelling(fn.body),"install_funded_member_store_owners(&mut store,catalog)?"))return false;}
 if(!method(memberStore,"install_funded_member_store_owners","funded_member_store_owners::<P,Mutation,C>(producer)")||!method(memberStore,"install_unscheduled_catalog","store.install_document_store_owners_exact(owners)","close_unscheduled_catalog(owners)"))return false;
 if(!boundBody(valueFactory,"trait","ArtifactOwnedValueRetirementFactory","fn retirement_birth_bytes(&self,value:&T)->usize;","fn retire_owned(&self,value:T,grant:RetainedCloneGrant)")||!contains(selected(valueFactory,"trait","ArtifactOwnedValueRetirementFactory")!.header,"crate::FactoryRetirement + Send + Sync"))return false;
 if(!boundBody(storeComponents,"struct","StoreLog","envelope:std::mem::ManuallyDrop<ArtifactEnvelope<P,Mutation>>")||!boundBody(storeLive,"struct","StoreLive","current:std::mem::ManuallyDrop<Arc<P>>")||!boundBody(storeComponents,"struct","StoreOwners","factory_retirement_tickets:std::mem::ManuallyDrop<semio_framework_value::FactoryChildTickets<6>>"))return false;
 if(!method(disposer.body,"close_step","grant.maximum_copy_bytes<demand.copy_bytes","grant.maximum_capacity_bytes<demand.capacity_bytes","grant.maximum_release_bytes<demand.release_bytes","store.close_cursor_admit(phase,RetainedCloneGrant","if !receipt.fits(grant)"))return false;
 if(!method(disposer.body,"terminal_is_empty","self.phase==ArtifactStoreCursorDisposerPhase::Complete&&self.active.is_none()&&store.owned_roots_terminal_is_empty()"))return false;
 if(!method(storeCloseCursor,"close_cursor_admit","ArtifactStoreCursorDisposerPhase::Backbone=>self.close_take_backbone_retirement(grant)","ArtifactStoreCursorDisposerPhase::Envelope","retire_document_envelope(original,snapshots,mutations,grant)","self.log.envelope=std::mem::ManuallyDrop::new(original)","self.live.current=std::mem::ManuallyDrop::new(original);Err(error)"))return false;
 if(!method(storeCloseCursor,"owned_roots_terminal_is_empty","self.live.current_detached","self.log.envelope_detached","self.wire.durable_group_root.is_none()","self.live.tail_undo_cache.is_none()","self.live.prefix_ring.is_empty()","self.snapshot_read_leases_terminal_is_empty()","self.close_structural_owners_terminal_is_empty()"))return false;
 if(!method(readRetirement,"retire_document_envelope","admit_artifact_retirement((envelope,initial_snapshot_factory,mutation_factory),grant","ArtifactStoreEnvelopeRetirement::new(envelope,snapshots,mutations)"))return false;
 if(!boundBody(readRetirement,"struct","ArtifactStoreEnvelopeRetirement","vcs:std::mem::ManuallyDrop<Option<ArtifactStoreVcsRetirement<P,Mutation>>>","messages:std::mem::ManuallyDrop<Option<ArtifactEditMessageLedgerRetirement>>","metadata:std::mem::ManuallyDrop<Option<ArtifactEnvelopeMetadata>>"))return false;
 const envelopeImpl=items(readRetirement,"impl").filter(row=>contains(row.header,"ArtifactStoreEnvelopeRetirement<P,Mutation>")).map(row=>row.body).join("\n");
 if(!method(envelopeImpl,"ownership_is_empty","self.vcs.is_none()&&self.messages.is_none()&&self.metadata.is_none()&&self.active.is_none()")||!method(envelopeImpl,"close_step","retirement_owner_close(&mut self.vcs,grant)","retirement_owner_close(&mut self.messages,grant)","artifact_retirement_admit_owned(&mut self.metadata,&mut self.active,grant)"))return false;
 for(const [source,owner] of [[readRetirement,"ArtifactStoreVcsRetirement"],[readRetirement,"ArtifactStoreEnvelopeRetirement"],[editRetirement,"ArtifactStoreOperationRowsRetirement"],[editRetirement,"ArtifactStoreDecodedEditRetirement"],[messageLedger,"ArtifactEditMessageLedgerRetirement"]] as const){const drop=implementation(source,"Drop",owner);if(!drop||!method(drop.body,"drop","assert!(std::thread::panicking()||"))return false;}
 const vcsDrop=implementation(readRetirement,"Drop","ArtifactStoreVcsRetirement"),envelopeDrop=implementation(readRetirement,"Drop","ArtifactStoreEnvelopeRetirement"),editDrop=implementation(editRetirement,"Drop","ArtifactStoreDecodedEditRetirement"),ledgerDrop=implementation(messageLedger,"Drop","ArtifactEditMessageLedgerRetirement"),catalogDrop=implementation(catalog,"Drop","DocumentStoreOwners");
 if(!vcsDrop||!method(vcsDrop.body,"drop","let empty=self.vcs.is_none()&&self.active.is_none()&&self.initial_snapshot_factory.is_none()&&self.mutation_factory.is_none()&&self.closing_factories.iter().all(Option::is_none);","assert!(std::thread::panicking()||empty,"))return false;
 if(!envelopeDrop||!method(envelopeDrop.body,"drop","assert!(std::thread::panicking()||self.ownership_is_empty(),")||!editDrop||!method(editDrop.body,"drop","assert!(std::thread::panicking()||(self.edit.is_none()&&self.active.is_none()&&self.pending_mutation.is_none()&&self.mutation_factory.is_none()&&self.closing_factory.is_none()),")||!ledgerDrop||!method(ledgerDrop.body,"drop","assert!(std::thread::panicking()||self.terminal_is_empty(),")||!catalogDrop||!method(catalogDrop.body,"drop","assert!(std::thread::panicking()||self.uninstalled_owners_terminal_is_empty(),"))return false;
 if(!method(storeBackbone,"close_take_backbone_retirement","ArtifactStoreBackboneRetirement::admit_attached(&mut self.member_inbox,&mut self.backbone,grant)"))return false;
 const edit=implementation(editRetirement,"ErasedSnapshotRetirement","ArtifactStoreDecodedEditRetirement"),ledger=implementation(messageLedger,"ErasedSnapshotRetirement","ArtifactEditMessageLedgerRetirement");
 if(!edit||!method(edit.body,"terminal_is_empty","self.edit.is_none()&&self.active.is_none()&&self.pending_mutation.is_none()&&self.mutation_factory.is_none()&&self.closing_factory.is_none()")||!ledger||!method(ledger.body,"terminal_is_empty","self.active.is_none()&&self.original.len==0&&self.original.backing_is_empty()"))return false;
 if(!contains(spelling(messageLedger),"pub const ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY:usize=8_192;")||!boundBody(messageLedger,"struct","ArtifactEditMessageLedger","slots:")||contains(spelling(storeComponents),"edit_messages:BTreeMap<String,Vec<crate::os_spr::MutationMessage>>")||contains(spelling(storeComponents),"edit_messages:HashMap<String,Vec<crate::os_spr::MutationMessage>>"))return false;
 for(const name of declaration.roles.storeLaws.symbols){if(!selected(storeLaws,"fn",name)?.body)return false;}
 for(const name of declaration.roles.catalogLaws.symbols){if(!selected(catalogLaws,"fn",name)?.body)return false;}
 if(!Array.isArray(sources.extensions))return false;
 for(const extension of sources.extensions){
 if(!extension||!extension.declaration)return false;
 const symbols=extension.declaration.roles.producer.symbols;if(symbols.length!==2||symbols[0]!==extension.declaration.producerType||symbols[1]!==extension.declaration.producerMacro)return false;
 for(const role of ["producer","members"] as const){if(extension[role]?.path!==extension.declaration.roles[role]?.path||!extension[role]?.source)return false;}
 const macro=selected(extension.producer.source,"macro_rules",extension.declaration.producerMacro);if(!macro)return false;
 const implementationBody=implementation(macro.body,"::semio_framework_os_kernel::os_store::document_store_owners::MemberStoreProducer","$crate::"+extension.declaration.producerType);
 if(!implementationBody||!contains(spelling(implementationBody.body),"type SnapshotOpen=$open;")||!method(implementationBody.body,"member_store_owners_birth_demand","bounded_artifact_store_owners_birth_demand::<$snapshot,$mutation>()")||!method(implementationBody.body,"member_store_owners","bounded_artifact_store_owners::<$snapshot,$mutation>(grant)"))return false;
 const tables=items(extension.members.source,extension.declaration.membersMacro,undefined,true);if(tables.length!==1||!tables[0])return false;
 const memberTokens=rustTokens(tables[0].body),memberPairs=rustTokenPairs(memberTokens),names=extension.declaration.roles.members.symbols,enumAt=memberTokens.findIndex(token=>token.text==="enum");
 if(names.length!==2||enumAt<0||memberTokens[enumAt+1]?.text!==names[0]||memberTokens[enumAt+2]?.text!==","||memberTokens[enumAt+3]?.text!==names[1])return false;
 const start=enumAt+4,end=memberPairs.get(start);if(memberTokens[start]?.text!=="{"||end===undefined)return false;let arms=0;
 for(let cursor=start+1;cursor<end;cursor++){
   if(memberTokens[cursor]?.text!=="=>")continue;
   const open=cursor+1,close=memberPairs.get(open);if(memberTokens[open]?.text!=="("||close===undefined)return false;
   const parts:string[]=[];let part=open+1,depth=0;
   for(let at=part;at<close;at++){
    const text=memberTokens[at]?.text;
    if(text==="<")depth++;else if(text===">")depth--;else if(text===">>")depth-=2;
    const pair=memberPairs.get(at);if(pair!==undefined&&pair>at){at=pair;continue;}
    if(text===","&&depth===0){parts.push(compact(memberTokens.slice(part,at)));part=at+1;}
    if(depth<0)return false;
   }
   parts.push(compact(memberTokens.slice(part,close)));
   if(depth!==0||parts.length!==4)return false;
   const [snapshot,mutation,type,value]=parts,args="<"+snapshot+","+mutation+">",name=extension.declaration.producerType;
   if(!type||!value||!type.endsWith("::"+name+args)||value!==type.slice(0,-args.length)+"::"+args+"::new()")return false;
   arms++;cursor=close;
  }
 if(arms===0)return false;
 }
 return true;
}
