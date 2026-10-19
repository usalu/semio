/** 🎛️ Declares exact executable ownership and required semantic preparation. */
export type CommandRequestV1=Readonly<{version:1;cwd:string;command:string;args:readonly string[];manifests:readonly string[];preparation:"none"|"required"}>;
