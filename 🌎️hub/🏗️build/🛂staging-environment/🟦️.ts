export type CargoStageEnvironments = Readonly<{
  env: NodeJS.ProcessEnv;
  nativeEnv: NodeJS.ProcessEnv;
}>;

/** 🛂 Projects inherited build variables and the separately bounded native stack override. */
export function exactCargoStageEnvironments(source: NodeJS.ProcessEnv = process.env): CargoStageEnvironments {
  return {
    env: { ...source, RUST_MIN_STACK: source.SEMIO_BUILD_RUST_MIN_STACK ?? "33554432" },
    nativeEnv: { RUST_MIN_STACK: "268435456" },
  };
}
