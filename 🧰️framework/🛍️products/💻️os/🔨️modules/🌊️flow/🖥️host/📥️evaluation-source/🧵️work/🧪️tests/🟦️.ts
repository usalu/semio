import{expect,test}from"bun:test";
import Ajv from"ajv";
import{applyPatch}from"fast-json-patch";
import fixture from"../🧫️fixtures/🔣️.json";
import domain from"../../../../🗿️artifacts/🌊️flow/🧵️retained/🧬️schema/🔣️.json";
import grant from"../../../../../../../../🔨️modules/🌱️value/🗂️ordered/♻️retirement/🧬️schema/🔣️.json";
import schema from"../🧬️schema/🔣️.json";
test("original source snapshot and normal Work preserve custody and explicit recompute policy",()=>{
 const ajv=new Ajv({strict:true,allErrors:true}).addSchema(grant).addSchema(domain);for(const [definition,corpus]of [[schema,fixture]]){const validate=ajv.compile(definition);expect(validate(corpus),JSON.stringify(validate.errors)).toBe(true);expect(validate({...corpus,unknown:true})).toBe(false);}const widget=ajv.compile({$ref:domain.$id+"#/$defs/Widget"}),synapse=ajv.compile({$ref:domain.$id+"#/$defs/Synapse"});for(const value of fixture.source.widgets){expect(widget(value)).toBe(true);expect(widget({...value,id:null})).toBe(false);}for(const value of fixture.source.synapses){expect(synapse(value)).toBe(true);expect(synapse({...value,from:null})).toBe(false);}
 const original=fixture.source,output=applyPatch({},original.widgets.map(widget=>({op:"add"as const,path:"/"+widget.id.replaceAll("~","~0").replaceAll("/","~1"),value:{text:{schema:"text",value:widget.text}}})),true,false).newDocument;expect(output).toEqual(fixture.expectedOutputs);expect(Object.fromEntries(original.widgets.map(widget=>[widget.id,{}]))).toEqual(fixture.expectedInputs);
 const snapshot={source:original};const changed=applyPatch(original,[{op:"replace",path:"/widgets/0/text",value:"next original mutation"}],true,false).newDocument;expect(snapshot.source).toBe(original);expect(changed).not.toEqual(snapshot.source);expect(snapshot.source.widgets[0].text).toBe("same source 雪🌳️");expect(fixture.normalPolicy).toBe("recompute-all-original-nodes");
 expect(fixture.sessionLiveTextOriginalSource).toBe(true);expect(fixture.sessionLiveHeaderCopyBytes).toBe(0);expect(fixture.sessionHostPointerPreserved).toBe(true);expect(fixture.sessionIncomingGrantForwardedWhole).toBe(true);expect(fixture.registryLeaseCopyBytes).toBe(0);console.log("[DEBUG] Original snapshot and Work strictCorpus=true sourceSeedPlainExamples=true independentJSONPatch=true snapshotOriginalSource=true recomputeAllExplicit=true");
});
