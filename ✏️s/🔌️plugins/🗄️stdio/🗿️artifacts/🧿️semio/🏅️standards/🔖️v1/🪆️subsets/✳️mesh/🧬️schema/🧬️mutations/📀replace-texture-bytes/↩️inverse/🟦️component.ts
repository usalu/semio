/** ↩️ inverse for `ReplaceTextureBytes` — undoes to another `ReplaceTextureBytes` restoring the prior bytes. */
export interface ReplaceTextureBytesInverseReplaceTextureBytes {
  id: string;
  newBytes: number[];
}
