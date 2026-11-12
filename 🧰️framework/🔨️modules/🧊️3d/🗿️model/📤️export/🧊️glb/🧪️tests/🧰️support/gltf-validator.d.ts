/** 🧪️ Minimal typing of the Khronos `gltf-validator` package (tests only): the report fields the oracle reads. */
declare module "gltf-validator" {
  export type ValidationMessage = { code: string; message: string; severity: 0 | 1 | 2 | 3; pointer?: string };
  export type ValidationReport = { info: { version: string; totalTriangleCount: number }; issues: { numErrors: number; numWarnings: number; numInfos: number; numHints: number; messages: ValidationMessage[] } };
  const validator: { validateBytes(data: Uint8Array, options?: { uri?: string; maxIssues?: number }): Promise<ValidationReport> };
  export default validator;
}
