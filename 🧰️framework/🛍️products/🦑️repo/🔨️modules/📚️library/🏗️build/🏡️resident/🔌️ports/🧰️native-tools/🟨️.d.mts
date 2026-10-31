/** 🧪 Declared fixture tools are data; their paths do not issue execution or storage custody. */
export type NativeFixtureV1=Readonly<{profile:"lifecycle";watcher:string;executionTool:null}|{profile:"execution";watcher:string;executionTool:string}>;
export function readNativeFixtureV1(value:unknown,platform?:"linux"|"darwin"|"win32"):NativeFixtureV1;
