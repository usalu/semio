/** 🔮️ Independent test-only schema, collection and affine geometry checks. */
import Ajv from 'ajv';
import lodash from 'lodash';
import {Matrix4,Vector3,Quaternion} from 'three';
import {readFileSync,existsSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {resolve,dirname} from 'node:path';
let root=dirname(fileURLToPath(import.meta.url));while(!existsSync(resolve(root,'AGENTS.md'))){const parent=dirname(root);if(parent===root)throw Error('repository root absent');root=parent;}
const drawingSchema=JSON.parse(readFileSync(resolve(root,'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/📸️snapshot/🔣️.json'),'utf8'));
const admit=new Ajv({strict:false}).compile(drawingSchema);
function descendants(node){return[node,node.kind==='group'?node.children.map(descendants):[]];}
export function verify(corpus){for(const fixture of corpus.cases){if(!admit(fixture.content))throw Error(fixture.name+': independent Drawing schema '+JSON.stringify(admit.errors));const nodes=lodash.flattenDeep((fixture.content.layers??[]).map(layer=>descendants(layer.root)));if(!lodash.isEqual(nodes.filter(node=>node.kind==='text').map(node=>node.value),fixture.texts))throw Error('independent text traversal');if(!lodash.isEqual(nodes.filter(node=>node.kind==='image').flatMap(node=>node.bytes),fixture.imageBytes))throw Error('independent image bytes');if(!lodash.isEqual((fixture.content.styles??[]).map(style=>style.name),fixture.styles))throw Error('independent styles');if(fixture.transformedPoint){const t=fixture.content.layers[0].root.transform,p=new Vector3(1,2,0).applyMatrix4(new Matrix4().compose(new Vector3(t.translation.x,t.translation.y,t.translation.z),new Quaternion(t.rotation.x,t.rotation.y,t.rotation.z,t.rotation.w),new Vector3(t.scale.x,t.scale.y,t.scale.z)));if(p.x!==fixture.transformedPoint.x||p.y!==fixture.transformedPoint.y)throw Error('independent group geometry');}}for(const fixture of corpus.invalid)if(admit(fixture.content))throw Error(fixture.name+': expected independent schema refusal');return{cases:corpus.cases.length,invalid:corpus.invalid.length};}

const protocolSchema=JSON.parse(readFileSync(resolve(dirname(fileURLToPath(import.meta.url)),"../🔣️.json"),"utf8"));
const protocolAdmit=new Ajv({strict:false}).compile(protocolSchema);
export function verifyStep(grant,event){const projected=event.kind==="image"?{...event,bytes:Array.from(event.bytes)}:event;if(!protocolAdmit({grant,event:projected}))throw Error("independent event protocol "+JSON.stringify(protocolAdmit.errors));}
