/** 🔺️ LasDiff schema. */
import type { LasVlr, LasPoint } from '../📸️snapshot/🟦️.ts';

export interface LasVlrDiff {
  userId?: string;
  recordId?: number;
  description?: string;
  data?: number[];
}

export interface LasVlrModified {
  index: number;
  diff: LasVlrDiff;
}

export interface LasVlrAdded {
  index: number;
  vlr: LasVlr;
}

export interface LasVlrsDiff {
  removed: number[];
  modified: LasVlrModified[];
  added: LasVlrAdded[];
}

export interface LasPointDiff {
  x?: number;
  y?: number;
  z?: number;
  intensity?: number;
  returnNumber?: number;
  numberOfReturns?: number;
  scanDirectionFlag?: boolean;
  edgeOfFlightLine?: boolean;
  classification?: number;
  scanAngleRank?: number;
  userData?: number;
  pointSourceId?: number;
  /** tri-state: absent = unchanged, null = cleared, number = set */
  gpsTime?: number | null;
  /** tri-state: absent = unchanged, null = cleared, tuple = set */
  rgb?: [number, number, number] | null;
}

export interface LasPointModified {
  index: number;
  diff: LasPointDiff;
}

export interface LasPointAdded {
  index: number;
  point: LasPoint;
}

export interface LasPointsDiff {
  removed: number[];
  modified: LasPointModified[];
  added: LasPointAdded[];
}

/** 🔺️ Every real header field is a top-level scalar; `schema` is an identity field and never
 * appears here. */
export interface LasDiff {
  versionMajor?: number;
  versionMinor?: number;
  systemIdentifier?: string;
  generatingSoftware?: string;
  creationDayOfYear?: number;
  creationYear?: number;
  headerSize?: number;
  offsetToPointData?: number;
  numberOfVlrs?: number;
  pointDataFormatId?: number;
  pointDataRecordLength?: number;
  numberOfPointRecords?: number;
  pointsByReturn?: [number, number, number, number, number];
  xScale?: number;
  yScale?: number;
  zScale?: number;
  xOffset?: number;
  yOffset?: number;
  zOffset?: number;
  maxX?: number;
  minX?: number;
  maxY?: number;
  minY?: number;
  maxZ?: number;
  minZ?: number;
  vlrs?: LasVlrsDiff;
  points?: LasPointsDiff;
}
