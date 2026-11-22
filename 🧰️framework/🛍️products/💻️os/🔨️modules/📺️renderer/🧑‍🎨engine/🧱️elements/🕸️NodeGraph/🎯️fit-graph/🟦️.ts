/** 🔲️ The React half of the framework `fitGraph` action: the camera law over a scene's nodes and the registry a mounted flow canvas joins so the shell can fit the graph a window presents.
 *
 * 🧱️ The law is {@link dagFitCamera}, the twin of `canvas::camera::fit_camera`; its fixture is `🧰️framework/🔨️modules/🖼️canvas/🧪️tests/🧫️fixtures/🔲️node-graph-fit-action/🔣️.json`. */
import { dagContentBounds, dagFitCamera, type DagCameraState } from "../🖼️dag-overlays/🟦️.tsx";
import type { NodeGraphNodeRecord } from "@semio-tech/framework";

/** 🔲️ The camera that frames every node of a scene in a `width` × `height` viewport, or `null` for a graph with nothing to frame. */
export function fitGraphCameraV1(nodes: readonly NodeGraphNodeRecord[] | undefined, width: number, height: number): DagCameraState | null {
  const content = dagContentBounds(nodes);
  return content ? dagFitCamera(content, width, height) : null;
}

/** 🔲️ Fits one mounted graph surface and persists the camera; answers whether it did. */
export type NodeGraphFitRunnerV1 = () => boolean;

const runners = new Map<string, NodeGraphFitRunnerV1>();

/** 🔌️ Joins `run` as the fitter of surface `surfaceId` until the returned cleanup runs. */
export function registerNodeGraphFitV1(surfaceId: string, run: NodeGraphFitRunnerV1): () => void {
  runners.set(surfaceId, run);
  return () => {
    if (runners.get(surfaceId) === run) runners.delete(surfaceId);
  };
}

/** 🚀️ Fits surface `surfaceId`; `false` when no mounted flow canvas owns it or it has nothing to frame. */
export function runNodeGraphFitV1(surfaceId: string): boolean {
  return runners.get(surfaceId)?.() ?? false;
}
