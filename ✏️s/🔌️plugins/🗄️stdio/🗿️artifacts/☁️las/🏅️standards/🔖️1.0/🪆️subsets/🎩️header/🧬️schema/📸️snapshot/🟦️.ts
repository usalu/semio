/** 🧬️ LasSnapshot schema with exact owned native binary64 scalars. */
import type {Binary64} from "../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
export type { Binary64 };
export interface LasHeader {
  versionMajor: number;
  versionMinor: number;
  systemIdentifier: string;
  generatingSoftware: string;
  creationDayOfYear: number;
  creationYear: number;
  headerSize: number;
  offsetToPointData: number;
  numberOfVlrs: number;
  pointDataFormatId: number;
  pointDataRecordLength: number;
  numberOfPointRecords: number;
  pointsByReturn: [number, number, number, number, number];
  xScale: Binary64;
  yScale: Binary64;
  zScale: Binary64;
  xOffset: Binary64;
  yOffset: Binary64;
  zOffset: Binary64;
  maxX: Binary64;
  minX: Binary64;
  maxY: Binary64;
  minY: Binary64;
  maxZ: Binary64;
  minZ: Binary64;
}

/** 📦️ One Variable Length Record — `data` is retained byte-verbatim. */
export interface LasVlr {
  userId: string;
  recordId: number;
  description: string;
  data: number[];
}

/** 📍 One LAS point record (formats 0-3; `gpsTime`/`rgb` absent unless the format carries them). */
export interface LasPoint {
  x: Binary64;
  y: Binary64;
  z: Binary64;
  intensity: number;
  returnNumber: number;
  numberOfReturns: number;
  scanDirectionFlag: boolean;
  edgeOfFlightLine: boolean;
  classification: number;
  scanAngleRank: number;
  userData: number;
  pointSourceId: number;
  gpsTime?: Binary64;
  rgb?: [number, number, number];
}

export interface LasSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ header: LasHeader;
  /** @state artifact */ vlrs: LasVlr[];
  /** @state artifact */ points: LasPoint[];
}
