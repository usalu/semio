/** 🛤️ World3dHost tool-run trace lane: mounts the scene's `toolRunTrace` lane for one window as instanced meshes per mesh and verdict batch. */
import { useCallback, useEffect, useMemo, useState, Suspense } from "react";
import { BufferGeometry } from "@semio-tech/ui-react";
import { ToolRunTraceLayer, type ToolRunTraceRecordStore } from "../⏯️tool-run-trace/🟦️.tsx";
import { type WorldMeshRecord } from "../🔭️scene-schema/🟦️.ts";
import { geometryFromMesh } from "../🎨️mesh-render/🔺️mesh-geometry-and-appearance/🟦️.ts";
import { ToolRunTraceGlbGeometry } from "../🧩️glb-surface/🟦️.tsx";

//#region 🔖️World3dHost
//#region WorldSceneParsing
/** ⏯️ Mounts the scene's `toolRunTrace` lane for this window: one instanced mesh per `(mesh, verdict)` batch,
 * each drawn through `meshesJson[mesh]` — inline mesh data directly, a GLB once it has loaded, a unit box
 * until then. Only meshes a resident `instance3d` record names are loaded.
 * @see ./⏯️tool-run-trace/🟦️.tsx */
export function WorldToolRunTrace({ lane, meshes, trace, focus }: { readonly lane: string | null | undefined; readonly meshes: readonly WorldMeshRecord[]; readonly trace: { readonly store: ToolRunTraceRecordStore; readonly version: number }; readonly focus?: bigint | null }) {
  const inline = useMemo(() => meshes.map((mesh) => (mesh.data ? geometryFromMesh(mesh.data) : null)), [meshes]);
  useEffect(() => () => inline.forEach((geometry) => geometry?.dispose()), [inline]);
  const [loaded, setLoaded] = useState<ReadonlyMap<string, BufferGeometry>>(() => new Map());
  const onGeometry = useCallback((url: string, geometry: BufferGeometry | null) => {
    setLoaded((previous) => {
      const next = new Map(previous);
      if (geometry) next.set(url, geometry);
      else next.delete(url);
      return next;
    });
  }, []);
  const urls = useMemo(() => {
    const named = new Set<string>();
    for (const batch of trace.store.batches()) {
      const mesh = batch.family === "instance3d" && batch.count > 0 ? meshes[batch.index] : undefined;
      if (mesh?.url && !mesh.data) named.add(mesh.url);
    }
    return [...named];
  }, [meshes, trace.store, trace.version]);
  const geometryForMesh = useCallback((index: number) => inline[index] ?? loaded.get(meshes[index]?.url ?? "") ?? null, [inline, loaded, meshes]);
  return (
    <>
      {urls.map((url) => (
        <Suspense key={url} fallback={null}>
          <ToolRunTraceGlbGeometry url={url} onGeometry={onGeometry} />
        </Suspense>
      ))}
      <ToolRunTraceLayer lane={lane} store={trace} geometryForMesh={geometryForMesh} focus={focus} />
    </>
  );
}
//#endregion WorldSceneParsing
//#endregion 🔖️World3dHost
