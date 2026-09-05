/** 🧬️ LasSnapshot schema. */
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
  xScale: number;
  yScale: number;
  zScale: number;
  xOffset: number;
  yOffset: number;
  zOffset: number;
  maxX: number;
  minX: number;
  maxY: number;
  minY: number;
  maxZ: number;
  minZ: number;
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
  x: number;
  y: number;
  z: number;
  intensity: number;
  returnNumber: number;
  numberOfReturns: number;
  scanDirectionFlag: boolean;
  edgeOfFlightLine: boolean;
  classification: number;
  scanAngleRank: number;
  userData: number;
  pointSourceId: number;
  gpsTime?: number;
  rgb?: [number, number, number];
}

export interface LasSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ header: LasHeader;
  /** @state artifact */ vlrs: LasVlr[];
  /** @state artifact */ points: LasPoint[];
}
