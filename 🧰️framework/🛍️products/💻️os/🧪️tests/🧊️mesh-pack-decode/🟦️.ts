import { readFileSync } from "node:fs";

type TestSource = { readonly directory: string; readonly url: string };

/**
 * 🧊️ The TypeScript half of the cross-language preview-mesh wire contract. Rust encodes the golden
 * vector in `semio-framework-os-flow`'s `flow_mesh_pack_wire` test; this decodes the SAME base64
 * bytes out of the SAME fixture, so a drift on either side fails on both.
 */
export async function registerTests5(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: Pick<typeof import("../../🟦️.ts"), "decodeMeshPackBody" | "decodeMeshPackChunks">, source: TestSource): Promise<void> {
  const { decodeMeshPackBody, decodeMeshPackChunks } = dependencies;
  const { describe, expect, it } = vitest;

  const fixture = JSON.parse(readFileSync(new URL("🧫️fixtures/🧊️mesh/mesh-pack-body-v1.json", source.url), "utf8"));
  const bytes = Uint8Array.from(Buffer.from(fixture.packBodyBase64, "base64"));

  describe("mesh pack decode", () => {
    it("refuses malformed analytic references and missing component pick buffers", () => {
      const fixture=JSON.parse(readFileSync(new URL("../../🔨️modules/🧊️3d/📐️brep/⚙️engine/🧫️fixtures/🎯️component-picking/🔣️.json",source.url),"utf8"));
      for(const row of fixture.referenceRefusals) {
        const payload=new TextEncoder().encode(JSON.stringify({componentReferences:row.references}));
        const header=[0,1,13,8];let size=payload.length;
        while(size>=128){header.push((size&127)|128);size=Math.floor(size/128);}header.push(size);
        expect(()=>decodeMeshPackBody(Uint8Array.from([...header,...payload]))).toThrow(/component references/);
      }
    });
    it("preserves lossless analytic component references in the shared preview record", () => {
      const fixture = JSON.parse(readFileSync(new URL("../../🔨️modules/🧊️3d/📐️brep/⚙️engine/🧫️fixtures/🎯️component-picking/🔣️.json", source.url), "utf8"));
      const mesh = decodeMeshPackBody(Uint8Array.from(Buffer.from(fixture.meshPack.bodyBase64, "base64")));
      expect(Array.from(mesh.faceIds)).toEqual(fixture.expected.faceIds);
      expect(Array.from(mesh.edgeIds)).toEqual(fixture.expected.edgeIds);
      expect((mesh as unknown as { componentReferences: unknown }).componentReferences).toEqual(fixture.expected.componentReferences);
      console.log("[DEBUG] Shared preview record retains four full analytic labels and segment pick ranges");
    });
    it("decodes owned indexed channels, seam values and texture bytes from the portable record", () => {
      const fixture = JSON.parse(readFileSync(new URL("🧫️fixtures/🧊️mesh/mesh-pack-attributes.json", source.url), "utf8"));
      const mesh = decodeMeshPackBody(Uint8Array.from(Buffer.from(fixture.packBodyBase64, "base64")));
      expect(mesh.attributes).toEqual(fixture.mesh.attributes);
      expect(mesh.materials).toEqual(fixture.mesh.materials);
      expect(mesh.textures).toEqual(fixture.mesh.textures);
      expect(Array.from(mesh.uvs)).toEqual(fixture.mesh.uvs);
      expect(mesh.attributes!.normal.values.length).toBe(1);
      expect(mesh.attributes!.labels.values.length).toBe(1);
      expect(mesh.attributes!.normal.indices!.length).toBe(mesh.positions.length / 3);
    });
    it("decodes the shared golden vector into typed arrays that match the fixture's mesh", () => {
      expect(bytes.length).toBe(fixture.packBodyBytes);
      const mesh = decodeMeshPackBody(bytes);
      expect(mesh.positions).toBeInstanceOf(Float32Array);
      expect(mesh.indices).toBeInstanceOf(Uint32Array);
      expect(Array.from(mesh.positions)).toEqual(fixture.mesh.positions);
      expect(Array.from(mesh.normals)).toEqual(fixture.mesh.normals);
      expect(Array.from(mesh.indices)).toEqual(fixture.mesh.indices);
      expect(Array.from(mesh.faceIds)).toEqual(fixture.mesh.faceIds);
      expect(Array.from(mesh.edgePositions)).toEqual(fixture.mesh.edgePositions);
      expect(Array.from(mesh.edgeIds)).toEqual(fixture.mesh.edgeIds);
      expect(Array.from(mesh.colors)).toEqual([]);
      expect(Array.from(mesh.uvs)).toEqual([]);
      expect(mesh.paintTextureBase64).toBeUndefined();
    });

    it("reassembles the chunked transfer the tessellate round trip streams", () => {
      const split = Math.floor(fixture.packBodyBase64.length / 4) * 2;
      const chunks = [fixture.packBodyBase64.slice(0, split), fixture.packBodyBase64.slice(split)];
      const mesh = decodeMeshPackChunks(chunks);
      expect(Array.from(mesh.positions)).toEqual(fixture.mesh.positions);
      expect(Array.from(mesh.indices)).toEqual(fixture.mesh.indices);
    });

    it("rejects a truncated body rather than returning a half-decoded mesh", () => {
      expect(() => decodeMeshPackBody(bytes.subarray(0, bytes.length - 4))).toThrow();
    });

    it("rejects trailing bytes rather than silently ignoring them", () => {
      const padded = new Uint8Array(bytes.length + 1);
      padded.set(bytes);
      expect(() => decodeMeshPackBody(padded)).toThrow(/trailing bytes/);
    });

    it("decodes an empty record body as an all-empty mesh", () => {
      const mesh = decodeMeshPackBody(Uint8Array.from([0x00, 0x00]));
      expect(mesh.positions.length).toBe(0);
      expect(mesh.indices.length).toBe(0);
      expect(mesh.edgeIsSeam.length).toBe(0);
    });
  });
}
