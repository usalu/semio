/** @module Interface semio:jcoprobe/probe@0.1.0 **/
export function poll(n: number): Promise<number>;
export function awaitEcho(ms: number, v: number): Promise<number>;
export function spawnDetached(ms: number): Promise<number>;
export function readBody(): Promise<number>;
