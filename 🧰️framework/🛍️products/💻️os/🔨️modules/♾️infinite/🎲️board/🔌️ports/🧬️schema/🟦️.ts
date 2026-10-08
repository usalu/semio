import type {BoardValue,BoardRecord,CameraDescriptor,NodeDescriptor} from "../../🧬️schema/🟦️.ts";
export interface HandleDescriptor {
  id: string;
  nodeId: string;
  angle: number;
  radius?: number | null;
  selected?: boolean | null;
  style?: string | null;
  handleKind?: string | null;
  color?: string | null;
  iconKind?: string | null;
  userData?: BoardValue | null;
  visible?: boolean | null;
  locked?: boolean | null;
  scale?: number | null;
}
