/** 🎨️ Model materials as FBX `Material` objects: Phong for lit surfaces, Lambert for unlit ones, colours sRGB-encoded because FBX colours are display-referred.
 *
 * @see https://help.autodesk.com/view/FBX/2017/ENU/?guid=__cpp_ref_class_fbx_surface_phong_html
 */
import type { Material } from "../../../🧬️schema/🟦️.ts";
import { field, int, property70, properties70, text } from "../🧱️records/🟦️.ts";
import type { FbxNode } from "../🧱️records/🟦️.ts";
import * as portable from "../../../🧮️math/🎯️portable/🟦️.ts";

/** 🌈️ Linear light to sRGB encoding of one channel, clamped at zero. */
export function srgbEncode(linear: number): number {
  const value = Math.max(0, linear);
  return value <= 0.0031308 ? 12.92 * value : 1.055 * portable.pow(value, 1 / 2.4) - 0.055;
}

/** 🖌️ The `Material` children (after the id/name/class properties) of `material`. */
export function materialChildren(material: Material): FbxNode[] {
  const [r, g, b, a] = material.baseColor;
  const alpha = material.alphaMode === "blend" ? a : 1;
  const diffuse = [srgbEncode(r), srgbEncode(g), srgbEncode(b)];
  const emissive = material.emissive.map(srgbEncode);
  const entries = [
    property70("DiffuseColor", "Color", "", "A", ...diffuse),
    property70("DiffuseFactor", "Number", "", "A", 1),
    property70("EmissiveColor", "Color", "", "A", ...emissive),
    property70("EmissiveFactor", "Number", "", "A", 1),
    property70("AmbientColor", "Color", "", "A", 0, 0, 0),
    property70("TransparencyFactor", "Number", "", "A", 1 - alpha),
    property70("Opacity", "Number", "", "A", alpha),
  ];
  if (!material.unlit) {
    const specular = [r, g, b].map((channel) => srgbEncode(0.04 + (channel - 0.04) * material.metallic));
    const shininess = Math.min(1000, Math.max(0, 2 / (material.roughness * material.roughness + 1e-4) - 2));
    entries.push(property70("SpecularColor", "Color", "", "A", ...specular), property70("SpecularFactor", "Number", "", "A", 1), property70("Shininess", "Number", "", "A", shininess), property70("ShininessExponent", "Number", "", "A", shininess), property70("ReflectionFactor", "Number", "", "A", material.metallic));
  }
  return [field("Version", int(102)), field("ShadingModel", text(material.unlit ? "lambert" : "phong")), field("MultiLayer", int(0)), properties70(...entries)];
}

/** 🏷️ The curve node property name a colour channel animates. */
export const COLOUR_PROPERTY = { baseColor: "DiffuseColor", emissive: "EmissiveColor" } as const;
