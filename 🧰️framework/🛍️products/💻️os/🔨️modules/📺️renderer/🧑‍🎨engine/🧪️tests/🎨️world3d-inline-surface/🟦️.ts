/** 🎨️ Shared indexed surface witness against the installed Three draw/material owner. */
import { describe,expect,it,vi } from "vitest";
import React from "react";
import {render,fireEvent,cleanup} from "@testing-library/react";
import Ajv2020 from "ajv/dist/2020";
import { BufferGeometry,Float32BufferAttribute,MeshStandardMaterial,SRGBColorSpace,FrontSide,DoubleSide } from "three";
import fixture from "../../🧫️fixtures/🎨️world3d-inline-surface/🔣️.json" with {type:"json"};
import schema from "../../🧬️schema/🎨️world3d-inline-surface/🔣️.json" with {type:"json"};
import {buildMeshVisuals,disposeMeshVisuals,cloneWorldGlbSurface,disposeWorldGlbSurface,advanceWorldMeshResidency,WorldSurfaceStatusPane} from "../../🧱️elements/🌐️World3dHost/🟦️.tsx";
import {Group,Mesh,Texture,TextureLoader} from "three";

describe("🎨️ authored inline surface",()=>{
 it("validates the neutral schema before constructing installed draw buffers",()=>{const validate=new Ajv2020({strict:true,allErrors:true}).compile(schema);expect(validate(fixture),JSON.stringify(validate.errors)).toBe(true);});
 it("binds indexed corner/face channels without changing triangle order",()=>{
  const ours=buildMeshVisuals({id:"surface",data:fixture.mesh});
  const reference=new BufferGeometry();reference.setAttribute("normal",new Float32BufferAttribute(fixture.expected.normal,3));reference.setAttribute("uv",new Float32BufferAttribute(fixture.expected.uv,2));reference.setAttribute("color",new Float32BufferAttribute(fixture.expected.color,4));
  expect(ours.geometry!.getAttribute("position").count).toBe(fixture.expected.drawVertexCount);
  for(const key of ["normal","uv","color"]){expect(ours.geometry!.getAttribute(key).itemSize).toBe(reference.getAttribute(key).itemSize);expect([...ours.geometry!.getAttribute(key).array]).toEqual([...reference.getAttribute(key).array]);}
  expect(ours.geometry!.groups).toEqual(fixture.expected.groups);expect(ours.geometry!.getAttribute("uv1").count).toBe(6);
  expect(ours.appearance!.materials.map(m=>m.name)).toEqual(fixture.expected.materialOrder);
  disposeMeshVisuals(ours);reference.dispose();
 });
 it("matches installed material alpha, culling, emissive and five-map role semantics",()=>{
  const ours=buildMeshVisuals({id:"surface",data:fixture.mesh});const material=ours.appearance!.materials[0]!;
  const reference=new MeshStandardMaterial({color:0xffffff,metalness:.7,roughness:.3,emissive:0xffffff,side:FrontSide});reference.color.setRGB(.8,.1,.1);reference.emissive.setRGB(.1,.2,.3);
  expect(material.color.toArray()).toEqual(reference.color.toArray());expect(material.emissive.toArray()).toEqual(reference.emissive.toArray());expect(material.metalness).toBe(reference.metalness);expect(material.roughness).toBe(reference.roughness);
  expect(material.opacity).toBe(1);expect(material.transparent).toBe(false);expect(material.side).toBe(FrontSide);
  for(const role of fixture.expected.textureRoles)expect((material as any)[role]).toBeTruthy();expect(material.map!.colorSpace).toBe(SRGBColorSpace);expect(material.emissiveMap!.colorSpace).toBe(SRGBColorSpace);expect(material.normalMap!.colorSpace).toBe("");expect(material.map!.flipY).toBe(false);
  expect(material.metalnessMap).toBe(material.roughnessMap);expect(material.map).toBe(material.emissiveMap);expect(material.map).not.toBe(material.normalMap);
  const blend=ours.appearance!.materials[1]!;expect(blend.side).toBe(DoubleSide);expect(blend.transparent).toBe(true);expect(blend.depthWrite).toBe(false);expect(blend.opacity).toBe(.6);
  disposeMeshVisuals(ours);reference.dispose();
 });
 it("retires shared assets once and refuses stale texture publication",()=>{
  const ours=buildMeshVisuals({id:"surface",data:fixture.mesh});const appearance=ours.appearance!;const resources=[...appearance.materials,...appearance.textures.values()];const spies=resources.map(resource=>vi.spyOn(resource,"dispose"));disposeMeshVisuals(ours);disposeMeshVisuals(ours);expect(appearance.closed).toBe(true);for(const spy of spies)expect(spy).toHaveBeenCalledTimes(1);
 });
 it("changes material-only identity and keeps an unaffected sibling",()=>{
  const initial=advanceWorldMeshResidency(null,JSON.stringify([{id:"a",data:fixture.mesh},{id:"b",data:fixture.mesh}]));const next=structuredClone(fixture.mesh);next.materials.opaque.roughness=.5;const changed=advanceWorldMeshResidency(initial,JSON.stringify([{id:"a",data:next},{id:"b",data:fixture.mesh}]));expect(changed.records[0]).not.toBe(initial.records[0]);expect(changed.records[1]).toBe(initial.records[1]);
 });
 it("clones authored GLB materials and restores them after selected override",()=>{
  const source=new Group();const geometry=new BufferGeometry();geometry.setAttribute("uv",new Float32BufferAttribute([0,0,1,0,.5,1],2));const material=new MeshStandardMaterial({metalness:.8,roughness:.2,opacity:.4,transparent:true,side:DoubleSide});const texture=new Texture();material.map=texture;material.metalnessMap=texture;material.roughnessMap=texture;material.emissiveMap=texture;const textureDispose=vi.spyOn(texture,"dispose");source.add(new Mesh(geometry,[material,material]));const originalDispose=vi.spyOn(material,"dispose");
  const style={meshColor:"#ff0000",emissiveIntensity:.3,opacity:1};const neutral=cloneWorldGlbSurface(source,"neutral",style);const selected=cloneWorldGlbSurface(source,"selected",style);const restored=cloneWorldGlbSurface(source,"neutral",style);
  const authored=(neutral.children[0] as Mesh).material as MeshStandardMaterial[];expect(authored[0]).not.toBe(material);expect(authored[0]!.metalness).toBe(.8);expect(authored[0]!.opacity).toBe(.4);expect(authored[0]).toBe(authored[1]);expect(authored[0]!.map!.colorSpace).toBe(SRGBColorSpace);expect(authored[0]!.metalnessMap!.colorSpace).toBe("");expect(authored[0]!.map).not.toBe(authored[0]!.metalnessMap);expect(authored[0]!.map).toBe(authored[0]!.emissiveMap);expect((selected.children[0] as Mesh).material).not.toEqual(authored);expect(((restored.children[0] as Mesh).material as MeshStandardMaterial[])[0]!.metalness).toBe(.8);
  disposeWorldGlbSurface(neutral);disposeWorldGlbSurface(selected);disposeWorldGlbSurface(restored);expect(originalDispose).not.toHaveBeenCalled();expect(textureDispose).not.toHaveBeenCalled();geometry.dispose();material.dispose();texture.dispose();
 });
 it("retires asynchronous texture callbacks before a replaced surface can publish",()=>{
  const create=vi.fn(()=>"blob:owned-surface");const revoke=vi.fn();vi.stubGlobal("URL",Object.assign(class {},{createObjectURL:create,revokeObjectURL:revoke}));
  const callbacks:((texture:Texture)=>void)[]=[];const loading=vi.spyOn(TextureLoader.prototype,"load").mockImplementation((_url,onLoad)=>{callbacks.push(onLoad!);return new Texture();});
  try {const visual=buildMeshVisuals({id:"retired",data:fixture.mesh});const appearance=visual.appearance!;const listener=vi.fn();appearance.listeners.add(listener);expect(appearance.pending).toBe(4);disposeMeshVisuals(visual);expect(listener).toHaveBeenCalledTimes(1);listener.mockClear();for(const complete of callbacks){const texture=new Texture();texture.image={width:1,height:1};const dispose=vi.spyOn(texture,"dispose");complete(texture);expect(dispose).toHaveBeenCalledTimes(1);}expect(listener).not.toHaveBeenCalled();for(const texture of appearance.textures.values())expect(texture.image).toBe(null);expect(appearance.urls.size).toBe(0);}
  finally {loading.mockRestore();vi.unstubAllGlobals();}
 });
 it("refuses invalid shader coefficients and shared domain indices before publication",()=>{
  const invalid=structuredClone(fixture.mesh);invalid.attributes.normal.indices[0]=100;expect(()=>buildMeshVisuals({id:"invalid",data:invalid})).toThrow(/sample|cardinality/);
  const material=structuredClone(fixture.mesh);material.materials.opaque.metallic=NaN;expect(()=>buildMeshVisuals({id:"invalid",data:material})).toThrow(/coefficient/);
 });

 it("preserves selected UV sets and authored normal/AO coefficients",()=>{
  const visual=buildMeshVisuals({id:"uv-roles",data:fixture.mesh});const material=visual.appearance!.materials[0]!;
  expect(material.normalScale.toArray()).toEqual([.5,.5]);expect(material.aoMapIntensity).toBe(.25);
  expect(material.normalMap!.channel).toBe(63);expect(material.aoMap!.channel).toBe(1);expect(material.map!.channel).toBe(0);
  for(const role of ["map","metalnessMap","roughnessMap","normalMap","aoMap","emissiveMap"] as const){const texture=material[role]!;expect(texture.wrapS).toBe(1001);expect(texture.wrapT).toBe(1002);expect(texture.magFilter).toBe(1003);expect(texture.minFilter).toBe(role==="metalnessMap"||role==="roughnessMap"?1008:1006);}
  expect(material.map).not.toBe(material.normalMap);expect(material.normalMap).not.toBe(material.aoMap);
  disposeMeshVisuals(visual);
 });

 it("owns selected GLB UV4 aliases and retires them without closing loader resources",()=>{
  const source=new Group();const geometry=new BufferGeometry();geometry.setAttribute("texcoord_4",new Float32BufferAttribute([0,0,1,0,.5,1],2));const material=new MeshStandardMaterial();const texture=new Texture();texture.channel=4;material.map=texture;source.add(new Mesh(geometry,material));
  const original=vi.spyOn(geometry,"dispose");const clone=cloneWorldGlbSurface(source,"neutral",{meshColor:"#fff",emissiveIntensity:0,opacity:1});const owned=(clone.children[0] as Mesh).geometry;expect(owned).not.toBe(geometry);expect(owned.getAttribute("uv4").array).toEqual(geometry.getAttribute("texcoord_4").array);const retired=vi.spyOn(owned,"dispose");disposeWorldGlbSurface(clone);expect(retired).toHaveBeenCalledTimes(1);expect(original).not.toHaveBeenCalled();geometry.dispose();material.dispose();texture.dispose();
 });

});

it("preserves indexed corner tangent handedness on the authored GPU draw",()=>{const visual=buildMeshVisuals({id:"tangent",data:fixture.mesh});expect(Array.from(visual.geometry!.getAttribute("tangent").array)).toEqual(Array(6).fill([0,1,0,-1]).flat());disposeMeshVisuals(visual);});

it("publishes localized loading, cancellation and recoverable surface failure through the existing visual owner",()=>{
 const loading=vi.spyOn(TextureLoader.prototype,"load").mockImplementation(()=>new Texture());const revoke=vi.fn();vi.stubGlobal("URL",Object.assign(class {},{createObjectURL:()=>"blob:status",revokeObjectURL:revoke}));
 try {for(const row of fixture.statusCases){const visual=buildMeshVisuals({id:row.locale,data:fixture.mesh});const pending=visual.appearance!.pending;expect(pending).toBe(4);const views=new Map([[row.locale,visual]]);const result=render(React.createElement(WorldSurfaceStatusPane,{visuals:views,locale:row.locale,glassClass:""}));expect(result.getByRole("status").getAttribute("aria-busy")).toBe("true");expect(result.getByRole("progressbar").getAttribute("aria-label")).toBe(row.loading);fireEvent.click(result.getByRole("button",{name:row.cancel}));expect(result.getByRole("status").textContent).toBe(row.cancelled);expect(visual.appearance!.closed).toBe(true);disposeMeshVisuals(visual);cleanup();const invalid={record:visual.record,geometry:null,border:null,vertexPick:null,edge:null,appearance:null,fault:"invalid indexed normal",closed:false};const fault=render(React.createElement(WorldSurfaceStatusPane,{visuals:new Map([["invalid",invalid]]),locale:row.locale,glassClass:""}));expect(fault.getByRole("status").textContent).toBe(row.failed);cleanup();}}
 finally{loading.mockRestore();vi.unstubAllGlobals();cleanup();}
});

it("allows MASK cutoffs above one with the same fully hidden semantics as installed Three",()=>{
 const validate=new Ajv2020({strict:true,allErrors:true}).compile(schema);expect(validate(fixture),JSON.stringify(validate.errors)).toBe(true);
 const prepared=structuredClone(fixture.mesh);Object.assign(prepared.materials.opaque,{alphaMode:"MASK",alphaCutoff:fixture.alphaCutoffCase.value});
 const visual=buildMeshVisuals({id:"hidden-mask",data:prepared}), material=visual.appearance!.materials[0]!;
 const reference=new MeshStandardMaterial({alphaTest:fixture.alphaCutoffCase.value});expect(material.alphaTest).toBe(reference.alphaTest);expect(material.alphaTest).toBe(1.5);
 disposeMeshVisuals(visual);reference.dispose();
});
