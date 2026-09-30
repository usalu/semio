import { describe, expect, it, vi } from "vitest";
//#region 🥽️Puzzle3dBrushMeshUpload
import {
  PUZZLE3D_MESH_COMMAND_RAW_BYTES,
  PUZZLE3D_MESH_PAGE_VALUES,
  PUZZLE3D_MESH_REUPLOAD_CLAIMS,
  PUZZLE3D_MESH_UPLOAD_MAX_PAGES,
  PUZZLE3D_MESH_UPLOAD_SLOTS,
  puzzle3dAnnounceableBrushMeshUrls,
  puzzle3dBrushMeshDigest,
  Puzzle3dBrushMeshRegistry,
  puzzle3dBrushMeshPages,
  puzzle3dBrushMeshQueueStep,
  drainPuzzle3dBrushMeshQueue,
} from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx";
import brushMeshUploadFixture from "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🥽️brush-mesh-upload/🔣️.json";

/** 🥽️ The values one page carries, read back out of its two base64 payloads exactly as the plugin's
 * `decode_brush_mesh_page_values` reads them (`✏️editor/⏳️precompute/🦀️.rs`). */
function decodeBrushMeshPage(page: { readonly positionsB64?: string; readonly indicesB64?: string }): { positions: number[]; indices: number[] } {
  const positionBytes = Buffer.from(page.positionsB64 ?? "", "base64");
  const indexBytes = Buffer.from(page.indicesB64 ?? "", "base64");
  return {
    positions: Array.from(new Float32Array(positionBytes.buffer, positionBytes.byteOffset, positionBytes.byteLength / 4)),
    indices: Array.from(new Uint32Array(indexBytes.buffer, indexBytes.byteOffset, indexBytes.byteLength / 4)),
  };
}

describe("puzzle3d brush mesh paged upload", () => {
  // 🚪️ `registerBrushMesh` is an APP-declared action, so a world-3d guest that never declared it drops
  // every announcement as `undeclared-action` and the upload run can never settle. `🎥️shooting` paints
  // GLB-backed meshes and declares no such action: its boot journalled 12 dropped dispatches and 12
  // console errors before this gate (ticket 26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END slice B3d,
  // `🗑️generated/b3d-shooting-console.txt`). A guest that OWNS the lane publishes its install counter
  // as `interactionJson.meshResidency`, so the counter's presence is the opt-in.
  it("announces brush meshes only to a guest that published a mesh residency", () => {
    const meshes = [{ url: "/test/a.glb" }, { url: "/test/b.glb" }, { url: "/test/a.glb" }, {}];
    expect(puzzle3dAnnounceableBrushMeshUrls(undefined, meshes)).toEqual([]);
    expect(puzzle3dAnnounceableBrushMeshUrls(0, meshes)).toEqual(["/test/a.glb", "/test/b.glb"]);
    expect(puzzle3dAnnounceableBrushMeshUrls(7, meshes)).toEqual(["/test/a.glb", "/test/b.glb"]);
    expect(puzzle3dAnnounceableBrushMeshUrls(7, [])).toEqual([]);
  });

  it("encodes the language-neutral page run the plugin decodes, with the Node base64 oracle", () => {
    expect(PUZZLE3D_MESH_COMMAND_RAW_BYTES).toBe(brushMeshUploadFixture.commandRawBytes);
    expect(PUZZLE3D_MESH_PAGE_VALUES).toBe(brushMeshUploadFixture.pageValues);
    expect(PUZZLE3D_MESH_UPLOAD_SLOTS).toBe(brushMeshUploadFixture.uploadSlots);
    expect(PUZZLE3D_MESH_UPLOAD_MAX_PAGES).toBe(brushMeshUploadFixture.maxPages);
    const example = brushMeshUploadFixture.example;
    expect(puzzle3dBrushMeshDigest(example.positions, example.indices)).toBe(example.digest);
    const pages = puzzle3dBrushMeshPages(example.url, example.surfaceId, example.positions, example.indices);
    expect(pages).toEqual(example.pages.map((page) => ({ url: example.url, digest: example.digest, ...page })));
    expect(pages[0]!.positionsB64).toBe(Buffer.from(new Uint8Array(Float32Array.from(example.positions).buffer)).toString("base64"));
    expect(pages[0]!.indicesB64).toBe(Buffer.from(new Uint8Array(Uint32Array.from(example.indices).buffer)).toString("base64"));
    expect(decodeBrushMeshPage(pages[0]!)).toEqual({ positions: example.positions, indices: example.indices });
  });

  it("pages a document-scale GLB into a run that never exceeds one retained command's raw wire", () => {
    for (const scale of brushMeshUploadFixture.documentScale) {
      const vertices = scale.positions / 3;
      const positions = Array.from({ length: scale.positions }, (_, value) => value * 0.5);
      const indices = Array.from({ length: scale.indices }, (_, value) => value % vertices);
      const pages = puzzle3dBrushMeshPages(scale.url, "world-3d", positions, indices);
      expect(pages.length).toBe(scale.pages);
      const reassembled: { positions: number[]; indices: number[] } = { positions: [], indices: [] };
      for (const [index, page] of pages.entries()) {
        expect(page).toMatchObject({ url: scale.url, page: index, pageCount: scale.pages });
        expect(Buffer.byteLength(JSON.stringify(["registerBrushMesh", { surfaceId: "world-3d", ...page }]), "utf8")).toBeLessThanOrEqual(PUZZLE3D_MESH_COMMAND_RAW_BYTES);
        const decoded = decodeBrushMeshPage(page);
        expect(decoded.positions.length + decoded.indices.length).toBeLessThanOrEqual(PUZZLE3D_MESH_PAGE_VALUES);
        reassembled.positions.push(...decoded.positions);
        reassembled.indices.push(...decoded.indices);
      }
      expect(reassembled.positions).toEqual(positions);
      expect(reassembled.indices).toEqual(indices);
    }
  });

  it("re-announces a mesh this guest already holds by id and digest alone", () => {
    const example = brushMeshUploadFixture.example;
    const url = "/test/already-paged.glb";
    const registry = new Puzzle3dBrushMeshRegistry();
    const digest = puzzle3dBrushMeshDigest(example.positions, example.indices);
    expect(registry.holds(url, digest)).toBe(false);
    registry.confirm(url, digest);
    expect(registry.holds(url, digest)).toBe(true);
    expect(registry.holds(url, puzzle3dBrushMeshDigest(example.positions, example.indices.slice(0, 3)))).toBe(false);
    registry.forget(url);
    expect(registry.holds(url, digest)).toBe(false);
  });

  // 🚚️ Wave W-H: the host's claim about what the guest holds is scoped to the guest instantiation that
  // justified it. A restored actor's mesh store is empty (it is deliberately not part of any checkpoint),
  // and the only evidence the host ever gets is `interactionJson.meshResidency` falling. Before this the
  // claim lived in a page-lifetime `Map`, so every window activation after a restart re-announced seven
  // identities by id alone, every one was refused into a notice nothing read, and the brush utility kept
  // no collision geometry until a full browser reload.
  it("re-pages every mesh instead of re-announcing when the guest restarted", () => {
    const example = brushMeshUploadFixture.example;
    const digest = puzzle3dBrushMeshDigest(example.positions, example.indices);
    const urls = ["/test/restart-a.glb", "/test/restart-b.glb"];
    const registry = new Puzzle3dBrushMeshRegistry();
    expect(registry.observeResidency(0)).toBe(false);
    for (const [index, url] of urls.entries()) {
      registry.confirm(url, digest);
      expect(registry.observeResidency(index + 1)).toBe(false);
    }
    expect(urls.every((url) => registry.holds(url, digest))).toBe(true);
    expect(registry.residency).toBe(urls.length);

    expect(registry.observeResidency(0)).toBe(true);
    expect(urls.some((url) => registry.holds(url, digest))).toBe(false);
    expect(registry.size).toBe(0);

    registry.confirm(urls[0]!, digest);
    expect(registry.observeResidency(1)).toBe(false);
    expect(registry.holds(urls[0]!, digest)).toBe(true);
    expect(registry.holds(urls[1]!, digest)).toBe(false);
  });

  // 🚚️ Wave W-H, rewritten by wave B48: a guest that refuses an id-only announcement publishes the
  // identity on the world body (`meshReuploadUrls`) and republishes it until the bytes land, so the
  // claim has to be BOUNDED — re-driving on every republish is an upload storm, not a recovery.
  //
  // 🐛️ The bound used to be "once per publishing residency value", and `meshResidency` is the guest's
  // own install counter, which every accepted announcement in the tab increments
  // (`derive_brush_mesh`/`adopt_brush_mesh_by_digest`, `✏️editor/⏳️precompute/🦀️.rs`). So the brake was
  // moved by the very traffic it suppressed: one standing request re-opened the gate on each unit of
  // progress, each claim deleted the paged entry and forced the next announcement onto the full
  // 72-command page path. Browser-measured at wasm #58 on the 180-object Nakagin document: 123 of 285
  // console lines were `registerBrushMesh`, seq 22 → 124 over 306 s, still arriving 8 minutes after the
  // example switch (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B46 §5, wave B48 §1.2). The bound is now
  // per guest INSTANTIATION, which is the only fact the guest's accepted work cannot move.
  it("claims a guest re-upload request a bounded number of times per guest instantiation", () => {
    const example = brushMeshUploadFixture.example;
    const digest = puzzle3dBrushMeshDigest(example.positions, example.indices);
    const url = "/test/reupload-claim.glb";
    const registry = new Puzzle3dBrushMeshRegistry();
    registry.observeResidency(4);
    registry.confirm(url, digest);
    expect(registry.claimReupload(url)).toBe(true);
    expect(registry.holds(url, digest)).toBe(false);
    for (let installs = 5; installs < 24; installs += 1) expect(registry.observeResidency(installs)).toBe(false);
    let claims = 1;
    while (registry.claimReupload(url)) claims += 1;
    expect(claims, "a climbing residency is the guest making progress, never a new claim").toBe(PUZZLE3D_MESH_REUPLOAD_CLAIMS);
    registry.confirm(url, digest);
    expect(registry.claimReupload(url), "a completed run does not buy a further claim either").toBe(false);
    expect(registry.observeResidency(0)).toBe(true);
    expect(registry.claimReupload(url), "a restarted guest holds nothing, so every claim is released").toBe(true);
    registry.clear();
    expect(registry.residency).toBe(-1);
    expect(registry.size).toBe(0);
  });

  // 🪢️ Wave B22: the transfer is content-addressed. Every `dist/mesh/*.glb` in this repo is the same
  // 771 728-byte capsule, so a scene placing several object kinds paged byte-identical geometry once per
  // mesh id — browser-measured 2026-09-12 on wasm #47 as 202 `registerBrushMesh` commands for one example
  // switch. A digest already resident under ANY id is announced instead, and the guest aliases its own
  // derived page onto the new id (`adopt_brush_mesh_by_digest`, `✏️editor/⏳️precompute/🦀️.rs`).
  it("knows a digest this guest holds under another id, so the bytes cross once per geometry", () => {
    const example = brushMeshUploadFixture.example;
    const digest = puzzle3dBrushMeshDigest(example.positions, example.indices);
    const registry = new Puzzle3dBrushMeshRegistry();
    expect(registry.holdsDigest(digest)).toBe(false);
    expect(registry.holdsDigest("")).toBe(false);
    registry.confirm("/test/capsule-a.glb", digest);
    expect(registry.holdsDigest(digest)).toBe(true);
    expect(registry.holds("/test/capsule-b.glb", digest)).toBe(false);
    registry.alias("/test/capsule-b.glb", digest);
    expect(registry.holds("/test/capsule-b.glb", digest)).toBe(true);
    registry.forget("/test/capsule-a.glb");
    expect(registry.holdsDigest(digest)).toBe(false);
  });

  // 🪢️ An alias is a claim about a SIBLING's bytes, never proof of its own, and a guest that refuses one
  // must be handed the bytes next time. Without both halves an id whose alias was refused would be
  // re-announced by digest forever — the guest asks for the bytes, the host answers with the same alias —
  // and the brush utility would keep no collision body for it at all.
  it("never re-aliases an identity the guest refused, and never chains one alias off another", () => {
    const example = brushMeshUploadFixture.example;
    const digest = puzzle3dBrushMeshDigest(example.positions, example.indices);
    const registry = new Puzzle3dBrushMeshRegistry();
    registry.observeResidency(7);
    registry.alias("/test/alias-only.glb", digest);
    expect(registry.holdsDigest(digest)).toBe(false);
    registry.confirm("/test/paged.glb", digest);
    expect(registry.holdsDigest(digest)).toBe(true);
    expect(registry.mayAlias("/test/alias-only.glb")).toBe(true);
    expect(registry.claimReupload("/test/alias-only.glb")).toBe(true);
    expect(registry.mayAlias("/test/alias-only.glb")).toBe(false);
    expect(registry.holds("/test/alias-only.glb", digest)).toBe(false);
    registry.confirm("/test/alias-only.glb", digest);
    expect(registry.mayAlias("/test/alias-only.glb")).toBe(true);
    expect(registry.observeResidency(0)).toBe(true);
    expect(registry.mayAlias("/test/alias-only.glb")).toBe(true);
  });

  it("collapses a queued run whose geometry a sibling id already put into the guest", () => {
    const example = brushMeshUploadFixture.example;
    const digest = puzzle3dBrushMeshDigest(example.positions, example.indices);
    const queue = [...puzzle3dBrushMeshPages("/test/queue-a.glb", "world-3d", example.positions, example.indices), ...puzzle3dBrushMeshPages("/test/queue-b.glb", "world-3d", example.positions, example.indices)];
    const first = puzzle3dBrushMeshQueueStep(queue, () => false);
    expect(first.kind).toBe("page");
    const resident = puzzle3dBrushMeshQueueStep(queue, (page) => page.digest === digest);
    expect(resident).toEqual({ kind: "adopt", url: "/test/queue-b.glb", digest });
    expect(queue).toHaveLength(0);
    expect(puzzle3dBrushMeshQueueStep(queue, () => false)).toEqual({ kind: "idle" });
  });

  // ⏳️ Wave B22: the run is back-pressured, so exactly ONE command is outstanding at a time. The macrotask
  // drain it replaces queued every page of every mesh into the actor's one command queue inside a couple
  // of hundred milliseconds — browser-measured 2026-09-12, 202 pages enqueued in 21 s, 40 settled over the
  // next 420 s, and one user click that landed mid-run waited 44.3 s behind 15 pages.
  it("keeps exactly one mesh command outstanding and confirms only on a run's last page", async () => {
    const example = brushMeshUploadFixture.example;
    const digest = puzzle3dBrushMeshDigest(example.positions, example.indices);
    const positions = Array.from({ length: 4_096 }, (_, index) => example.positions[index % example.positions.length]!);
    const indices = Array.from({ length: 4_096 }, (_, index) => example.indices[index % example.indices.length]!);
    const queue = [...puzzle3dBrushMeshPages("/test/backpressure.glb", "world-3d", positions, indices)];
    expect(queue.length).toBeGreaterThan(4);
    const confirmed: string[] = [];
    const registry = { holdsDigest: () => false, mayAlias: () => true, alias: () => {}, confirm: (url: string, held: string) => confirmed.push(`${url}@${held}`) };
    let inFlight = 0;
    let peak = 0;
    let dispatched = 0;
    const pending = queue.length;
    await drainPuzzle3dBrushMeshQueue(
      queue,
      registry,
      async () => {
        inFlight += 1;
        peak = Math.max(peak, inFlight);
        dispatched += 1;
        await Promise.resolve();
        inFlight -= 1;
      },
      () => true,
    );
    expect(peak).toBe(1);
    expect(dispatched).toBe(pending);
    expect(confirmed).toEqual([`/test/backpressure.glb@${puzzle3dBrushMeshDigest(positions, indices)}`]);
    expect(digest).not.toBe(puzzle3dBrushMeshDigest(positions, indices));
  });

  it("retires the drain without dispatching once the surface it belongs to is gone", async () => {
    const example = brushMeshUploadFixture.example;
    const queue = [...puzzle3dBrushMeshPages("/test/unmounted.glb", "world-3d", example.positions, example.indices)];
    let dispatched = 0;
    await drainPuzzle3dBrushMeshQueue(
      queue,
      { holdsDigest: () => false, mayAlias: () => true, alias: () => {}, confirm: () => {} },
      async () => {
        dispatched += 1;
      },
      () => false,
    );
    expect(dispatched).toBe(0);
    expect(queue.length).toBeGreaterThan(0);
  });
});
//#endregion 🥽️Puzzle3dBrushMeshUpload
