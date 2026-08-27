/** ♻️ Direct replace-model payload. */
export interface ReplaceModel {
  readonly mutation: "replace-model";
  readonly newModelJson: string;
}
