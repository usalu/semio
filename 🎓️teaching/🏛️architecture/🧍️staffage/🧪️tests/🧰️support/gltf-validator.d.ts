/** 🧪️ Minimal typing of the untyped `gltf-validator` package (tests only). */
declare module "gltf-validator" {
  const validator: { validateBytes(bytes: Uint8Array, options?: { uri?: string; maxIssues?: number }): Promise<{ issues: { numErrors: number; numWarnings: number; numInfos: number; messages: { code: string; message: string; pointer?: string; severity: number }[] } }> };
  export default validator;
}
