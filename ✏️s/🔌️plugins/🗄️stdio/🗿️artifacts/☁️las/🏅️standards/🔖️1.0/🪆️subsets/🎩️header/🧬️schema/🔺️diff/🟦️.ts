/** 🔺️ LasDiff schema. */
import type { LasVlr, LasPoint, Binary64 } from '../📸️snapshot/🟦️.ts';

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
  x?: Binary64;
  y?: Binary64;
  z?: Binary64;
  intensity?: number;
  returnNumber?: number;
  numberOfReturns?: number;
  scanDirectionFlag?: boolean;
  edgeOfFlightLine?: boolean;
  classification?: number;
  scanAngleRank?: number;
  userData?: number;
  pointSourceId?: number;
  /** 🫥️ Absent leaves GPS time unchanged, null clears it, and Binary64 sets exact native bits. */
  gpsTime?: Binary64 | null;
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
  xScale?: Binary64;
  yScale?: Binary64;
  zScale?: Binary64;
  xOffset?: Binary64;
  yOffset?: Binary64;
  zOffset?: Binary64;
  maxX?: Binary64;
  minX?: Binary64;
  maxY?: Binary64;
  minY?: Binary64;
  maxZ?: Binary64;
  minZ?: Binary64;
  vlrs?: LasVlrsDiff;
  points?: LasPointsDiff;
}
