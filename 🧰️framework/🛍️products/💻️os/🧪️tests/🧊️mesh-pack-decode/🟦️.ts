import { readFileSync } from "node:fs";

type TestSource = { readonly directory: string; readonly url: string };

/**
 * 🧊️ The TypeScript half of the cross-language preview-mesh wire contract. Rust encodes the golden
 * vector in `semio-framework-os-flow`'s `flow_mesh_pack_wire` test; this decodes the SAME base64
 * bytes out of the SAME fixture, so a drift on either side fails on both.
 */
export async function registerTests5(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { decodeMeshPackBody, decodeMeshPackChunks } = dependencies;
  const { describe, expect, it } = vitest;

  const fixture = JSON.parse(readFileSync(new URL("🧫️fixtures/🧊️mesh/mesh-pack-body-v1.json", source.url), "utf8"));
  const bytes = Uint8Array.from(Buffer.from(fixture.packBodyBase64, "base64"));

  describe("mesh pack decode", () => {
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
