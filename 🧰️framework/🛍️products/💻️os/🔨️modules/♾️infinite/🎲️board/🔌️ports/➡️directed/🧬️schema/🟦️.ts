import type {BoardValue,BoardRecord,CameraDescriptor,NodeDescriptor} from "../../../🧬️schema/🟦️.ts";
import type {HandleDescriptor} from "../../🧬️schema/🟦️.ts";
export interface EdgeDescriptor {
  id: string;
  source: string;
  target: string;
  edgeKind?: string | null;
  sourceTip?: string | null;
  targetTip?: string | null;
  selected?: boolean | null;
  style?: string | null;
  userData?: BoardValue | null;
  visible?: boolean | null;
  locked?: boolean | null;
}

export interface WireDescriptor {
  id: string;
  source: string;
  wireKind?: string | null;
  target?: string | null;
  endX?: number | null;
  endY?: number | null;
  selected?: boolean | null;
  style?: string | null;
  userData?: BoardValue | null;
  visible?: boolean | null;
  locked?: boolean | null;
}

export interface RegionDescriptor {
  id: string;
  x: number;
  y: number;
  width: number;
  height: number;
  label?: string | null;
  hidden?: boolean | null;
  locked?: boolean | null;
  selected?: boolean | null;
}

export interface SceneDescriptor {
  nodes: NodeDescriptor[];
  handles: HandleDescriptor[];
  edges: EdgeDescriptor[];
  wires: WireDescriptor[];
  regions: RegionDescriptor[];
  selectionExitHighlightIds: String[];
}

export interface BoardSnapshot {
  schema: string;
  camera?: CameraDescriptor | null;
  nodes: BoardRecord[];
  edges: BoardRecord[];
  targetRegions: BoardRecord[];
  meta?: BoardValue | null;
}
