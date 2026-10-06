/** @module Interface semio:jcoprobe/probe-host@0.1.0 **/
export function slowEcho(ms: number, v: number): Promise<number>;
export function fetchBody(): Promise<AsyncIterable<number>>;
