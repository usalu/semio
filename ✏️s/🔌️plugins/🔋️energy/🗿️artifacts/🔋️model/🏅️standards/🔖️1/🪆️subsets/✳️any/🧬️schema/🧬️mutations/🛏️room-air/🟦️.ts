/** 🧾️ Energy-model mutation payloads of the `room-air` group: the TypeScript twin of the Rust leaves below `./`, one interface per leaf, wire-tagged by the camelCase variant name. */
/** 🛏️ `create-room-air-model-assignment` payload. */
export interface CreateRoomAirModelAssignment {
  readonly mutation: "createRoomAirModelAssignment";
  readonly zoneId: number;
  readonly model: "WellMixed" | "OneNodeDisplacement" | "TwoNodeBuoyancy" | "UnderFloorAirDistribution";
  readonly index: number | null;
}

/** 🧺️ `delete-room-air-model-assignment` payload. */
export interface DeleteRoomAirModelAssignment {
  readonly mutation: "deleteRoomAirModelAssignment";
  readonly zoneId: number;
}

/** 🪭️ `change-room-air-model` payload. */
export interface ChangeRoomAirModel {
  readonly mutation: "changeRoomAirModel";
  readonly zoneId: number;
  readonly newModel: "WellMixed" | "OneNodeDisplacement" | "TwoNodeBuoyancy" | "UnderFloorAirDistribution";
}
