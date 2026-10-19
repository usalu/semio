/** 🐍️ Names the producer operations: the locked dependency environment and the published wheel. */
export type StylingPythonOperationV1 = "deps" | "build";
/** 👓️ Exposes the published wheel, its interpreter and its funded environment for the duration of one consumer; `Phase` is the custody token the composition root issues. */
export type StylingPythonWheelViewV1<Phase> = Readonly<{ wheel: string; python: string; project: string; environment: Readonly<Record<string, string | undefined>>; control: Phase }>;

/** 🔌️ Produces and reads the styling Python wheel under the storage and build mode an issuing parent selected for the whole script process. The ui owns this port; the repository binds it, and `Invocation` and `Phase` are the binder's script-invocation and custody types. */
export interface StylingPythonProviderV1<Invocation, Phase> {
  run(invocation: Invocation, phase: Phase, operation: StylingPythonOperationV1): Promise<void>;
  withWheel<T>(invocation: Invocation, phase: Phase, body: (view: StylingPythonWheelViewV1<Phase>) => Promise<T>): Promise<T>;
}
