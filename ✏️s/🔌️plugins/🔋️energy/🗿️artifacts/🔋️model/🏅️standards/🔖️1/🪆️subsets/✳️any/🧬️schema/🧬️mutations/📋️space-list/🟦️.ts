/** 🧾️ Energy-model mutation payloads of the `space-list` group: the TypeScript twin of the Rust leaves below `./`, one interface per leaf, wire-tagged by the camelCase variant name. */
/** 📋️ `create-space-list` payload. */
export interface CreateSpaceList {
  readonly mutation: "createSpaceList";
  readonly index: number;
  readonly id: number;
  readonly name: string;
  readonly spaceIds: readonly number[];
}

/** 🗒️ `delete-space-list` payload. */
export interface DeleteSpaceList {
  readonly mutation: "deleteSpaceList";
  readonly id: number;
}

/** 🪶️ `rename-space-list` payload. */
export interface RenameSpaceList {
  readonly mutation: "renameSpaceList";
  readonly id: number;
  readonly newName: string;
}

/** ➡️ `add-space-list-member` payload. */
export interface AddSpaceListMember {
  readonly mutation: "addSpaceListMember";
  readonly id: number;
  readonly index: number;
  readonly spaceId: number;
}

/** 🪤️ `remove-space-list-member` payload. */
export interface RemoveSpaceListMember {
  readonly mutation: "removeSpaceListMember";
  readonly id: number;
  readonly spaceId: number;
}
