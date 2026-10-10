import "../../../🧵️retained-command/🧬️context/🧪️tests/🟦️.ts";
import "../../../../🏪️store/🔗️read/🪞️projection/🧪️tests/🟦️.ts";
import {test,expect} from 'bun:test';
import {readFileSync} from 'node:fs';
import {join} from 'node:path';
import Ajv from 'ajv';
import patch from 'fast-json-patch';
import corpus from '../🧫️fixtures/🔣️.json';
import schema from '../🧬️schema/🔣️.json';
test('eight store stages preserve original body, separate frame and each denied currency',()=>{
 expect(new Ajv({strict:false}).compile(schema)(corpus)).toBe(true);
 for(const row of corpus.cases){
  const initial={stage:0,body:{identity:'original'},frame:{identity:'original'}};
  const permitted=row.grant.items>0&&Object.entries(corpus.demand).every(([axis,value])=>row.grant[axis as keyof typeof row.grant]>=value);expect(permitted).toBe(row.advanced);
  if(!permitted){expect(patch.applyPatch(structuredClone(initial),[]).newDocument).toEqual(initial);continue;}
  const afterBody=patch.applyPatch(structuredClone(initial),[{op:'remove',path:'/body'}]).newDocument;expect(afterBody.frame).toEqual(initial.frame);expect(afterBody.stage).toBe(0);
  const afterFrame=patch.applyPatch(afterBody,[{op:'remove',path:'/frame'}]).newDocument;expect(afterFrame.stage).toBe(0);
  const afterAdvance=patch.applyPatch(afterFrame,[{op:'replace',path:'/stage',value:1}]).newDocument;expect(afterAdvance).toEqual({stage:1});
 }
});
test('global close advances authentic store authorities and never reports an unproven terminal',()=>{
 const host=readFileSync(join(import.meta.dir,'../../../🦀️.rs'),'utf8');
 const frontier=readFileSync(join(import.meta.dir,'../🦀️.rs'),'utf8');
 const ladder=readFileSync(join(import.meta.dir,'../../../🪜️close-ladder/🦀️.rs'),'utf8');
 expect(ladder).toContain('self.close_owned_stores_step(grant)');expect(ladder).toContain('self.close_owned_stores_demands(body)');expect(host).toContain('self.close_ladder_step(grant)');
 expect(frontier).toContain('self.close_owned_stage += 1');expect(frontier).toContain('terminal_frame_release_bytes');
 for(const stage of corpus.stages)expect(frontier).toContain('"'+stage+'"');
 const start=host.indexOf('fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault>',host.indexOf('impl<A: ArtifactApp, M: SpaceMember + MemberFactory + Send'));
 const close=host.slice(start,host.indexOf('fn close_terminal_is_empty',start));expect(close).not.toContain('else {return Ok(PluginLifecycleStep::Complete(Default::default()));}');
 const presence=readFileSync(join(import.meta.dir,'../../../👥️presence/♻️retirement/🦀️.rs'),'utf8');
 expect(presence).toContain('owner.detached_terminal_is_empty()');expect(presence).not.toContain('terminal_root.upgrade()');expect(presence).not.toContain('terminal_is_empty)(owner.local())');
});

test('original transient detach transfers custody without a replacement root or registry',()=>{
 const moved=patch.applyPatch(structuredClone({live:corpus.detach.live,retained:{}}),Object.keys(corpus.detach.live).map(field=>({op:'move' as const,from:'/live/'+field,path:'/retained/'+field}))).newDocument;expect(moved.retained).toEqual(corpus.detach.retained);expect(moved.live).toEqual({});expect(corpus.detach.allocated).toBe(0);expect(corpus.detach.released).toBe(0);
 const store=readFileSync(join(import.meta.dir,'../../../../🏪️store/🦀️.rs'),'utf8');const start=store.indexOf('pub fn begin_retirement',store.indexOf('pub struct TransientStore<'));const end=store.indexOf('pub struct TransientStoreRetirement',start);const detach=store.slice(start,end);expect(detach).toContain('self.current.take()');expect(detach).toContain('self.reads.take()');expect(detach).not.toContain('Self::new(terminal)');
 const disposer=readFileSync(join(import.meta.dir,'../../../🫧️transient/🧵️publication/🦀️.rs'),'utf8').split('pub struct TransientStoreDisposer')[1]!;expect(disposer).toContain('owner.detached_terminal_is_empty()');expect(disposer).not.toContain('terminal_root: Option<Weak<P>>');expect(disposer).toContain('self.factory.take()');
 const factory=readFileSync(join(import.meta.dir,'../../../🫧️transient/♻️retirement/🦀️.rs'),'utf8');expect(factory).toContain('admit_shared_retirement');expect(factory).not.toContain('struct NoTransientRetirement(Option<Arc<NoTransient>>)');
});

test('rejected original read return retains every original field before custody transfer',()=>{
 for(const refusal of corpus.readReturn.refusals){expect(['busy','stale','already-returned','changed-owner']).toContain(refusal);expect(patch.applyPatch(structuredClone(corpus.readReturn.original),[]).newDocument).toEqual(corpus.readReturn.original);}
 const moved=patch.applyPatch(structuredClone(corpus.readReturn.original),[{op:'move',from:'/registry',path:'/witness'},{op:'replace',path:'/owner',value:null},{op:'replace',path:'/lease',value:null}]).newDocument;expect(moved).toEqual(corpus.readReturn.success);
 const store=readFileSync(join(import.meta.dir,'../../../../🏪️store/🦀️.rs'),'utf8');const start=store.indexOf('fn try_return_original_snapshot_read');const end=store.indexOf('pub struct SnapshotRead<T',start);const boundary=store.slice(start,end);expect(start).toBeGreaterThan(0);expect(boundary).toContain('state.try_lock()');expect(boundary.indexOf('compare_exchange')).toBeLessThan(boundary.indexOf('drop(owner.take())'));expect(boundary).toContain('let original = lease.take()');expect(store.split('try_return_original_snapshot_read(&mut self.owner, &mut self.lease)').length-1).toBe(2);
});

test('export source handback retains the original read and its physical return witness',()=>{
 const consumer=corpus.readReturn.exportConsumer;expect(consumer.preserved).toBe(true);expect(consumer.phases).toEqual(['producer','pending-read','active-read','terminal']);for(const axis of consumer.denied){expect(['items','copy','capacity','release','depth']).toContain(axis);const original={producer:{identity:consumer.identity},pending:null,active:null};expect(patch.applyPatch(structuredClone(original),[]).newDocument).toEqual(original);}
 const base=join(import.meta.dir,'../../../../../../../../✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any');const work=readFileSync(join(base,'✏️editor/🎮️commands/📤️export-document/🧵️export/🦀️.rs'),'utf8');const producer=readFileSync(join(base,'🚪️io/📤️export/📦️owned/🦀️.rs'),'utf8');for(const source of [work,producer]){expect(source).toContain('read_close:Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>');expect(source).toContain('store::artifact_retirement_admit_owned(&mut self.read,&mut self.read_close,grant)');expect(source).toContain('store::artifact_retirement_box_close_step(&mut self.read_close,grant)');expect(source).toContain('self.read=Some(read)');expect(source).not.toContain('read.return_to_registry()');expect(source).toContain('self.read_close.is_none()');}expect(work).toContain('io_begin_owned_serialization(self.request.take().unwrap(),self.birth_grant)');expect(work).toContain('request.retained_birth_grant');expect(work).not.toContain('ExportProducer::new');expect(producer).toContain('producer:ExportProducer::new(read,options)');
});

test('registered owned serializers preserve original sources on every rejected boundary',()=>{
 const root=join(import.meta.dir,'../../../../🚪️io/📤️serialization/📦️owned');const owned=JSON.parse(readFileSync(join(root,'🧫️fixtures/🔣️.json'),'utf8'));const declared=JSON.parse(readFileSync(join(root,'🧬️schema/🔣️.json'),'utf8'));expect(new Ajv({strict:true}).compile(declared)(owned)).toBe(true);
 for(const row of owned.cases){const reason=!row.capability||row.direction!=='export'?'unsupported':!row.current?'stale':Object.keys(row.demand).some(axis=>row.grant[axis]<row.demand[axis])?'grant':null;expect(reason).toBe(row.reason);const original={read:{identity:'original-read'},options:{identity:'original-options'},children:{identity:'original-children'}};const actual=patch.applyPatch(structuredClone({caller:original,job:null}),reason===null?[{op:'move',from:'/caller',path:'/job'}]:[]).newDocument;expect(reason===null?actual.job:actual.caller).toEqual(original);expect(row.factoryCalls).toBe(reason===null?1:0);expect(row.releases).toBe(0);}
 for(const row of owned.terminalHandoffs){const accepted=row.available&&row.finished&&!row.cancelled&&(row.kind==='fault'||row.current)&&row.grant.items>=row.demand.items&&row.grant.depth>=row.demand.depth;expect(accepted).toBe(row.accepted);const original={identity:row.kind,scope:{identity:'scope'},causes:{identity:'causes'},params:{identity:'params'}};const moved=patch.applyPatch(structuredClone({owner:original,recipient:null}),accepted?[{op:'move',from:'/owner',path:'/recipient'}]:[]).newDocument;expect(accepted?moved.recipient:moved.owner).toEqual(original);expect(row.transferredItems).toBe(Number(accepted));expect([row.allocations,row.releases]).toEqual([0,0]);}const io=readFileSync(join(root,'../../🦀️.rs'),'utf8');expect(io.includes('pub owned_serializer: Option<OwnedSerializerFactory>')).toBe(true);expect(io.includes('same_owned_serializer(left.owned_serializer,right.owned_serializer)')).toBe(true);const host=readFileSync(join(import.meta.dir,'../../../🦀️.rs'),'utf8');expect(host.includes('pub retained_birth_grant: RetainedCloneGrant')).toBe(true);expect(host.includes('pub retained_handoff_grant: RetainedCloneGrant')).toBe(true);expect(host.match(/retained_handoff_grant: artifact_owned_tool_handoff_grant\(\)/g)?.length).toBe(2);expect(host.match(/retained_birth_grant: artifact_owned_tool_birth_grant\(\)/g)?.length).toBe(2);const boundary=readFileSync(join(root,'🦀️.rs'),'utf8');expect(boundary).toContain('request.source.commit_authority_matches');expect(boundary).toContain('return Err(OwnedSerializerRefusal');expect(boundary).not.toContain('decode_pack');expect(boundary).not.toContain('native_carrier');expect(boundary).not.toContain('resolve_ready');
});
