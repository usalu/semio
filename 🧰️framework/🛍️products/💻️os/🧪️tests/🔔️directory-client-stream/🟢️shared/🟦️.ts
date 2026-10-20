/** 🟢️ The shared declarations of the DirectoryClient.stream suite tests. */
import { DirectoryClient } from "@semio-tech/framework/directory-client";
import { type DirectoryEvent } from "../../🧮️directory-client-helpers/🟦️.ts";

export type BackboneWorkerResponse = import("@semio-tech/framework/backbone").BackboneWorkerResponse;

export type BrowserActorUiMountedV1 = import("@semio-tech/framework/backbone").BrowserActorUiMountedV1;

export type DirectoryCommand = import("@semio-tech/framework/directory/schema").DirectoryCommand;

export type DirectoryStreamMessage = import("@semio-tech/framework/directory/schema").DirectoryStreamMessage;

export type SocketGrantIssuerV1 = import("@semio-tech/framework/directory-client").SocketGrantIssuerV1;

export type SocketGrantReceiptV1 = import("@semio-tech/framework/directory-client").SocketGrantReceiptV1;

export class FakeDirectoryWebSocket {
  static instances: FakeDirectoryWebSocket[] = [];
  readonly url: string;
  readonly protocol = "semio.socket.v1";
  readyState = 0;
  onopen: (() => void) | null = null;
  onmessage: ((event: { data: string }) => void) | null = null;
  onclose: ((event: CloseEvent) => void) | null = null;
  onerror: (() => void) | null = null;
  constructor(url: string, readonly protocols?: string | string[]) {
    this.url = url;
    FakeDirectoryWebSocket.instances.push(this);
  }
  send(): void {}
  close(): void {
    this.readyState = 3;
  }
  triggerOpen(): void {
    this.readyState = 1;
    this.onopen?.();
  }
  triggerMessage(message: DirectoryStreamMessage): void {
    this.onmessage?.({ data: JSON.stringify(message) });
  }
  triggerClose(code = 1006): void {
    this.readyState = 3;
    this.onclose?.({ code } as CloseEvent);
  }
}

export const testSocketGrantReceipt: SocketGrantReceiptV1 = {
  schema: "semio.hub.socket-grant/v1",
  protocol: "semio.socket.v1",
  grant: `socket.v1.${"1".repeat(32)}.${"2".repeat(64)}`,
  actorId: `hub.v1.${"3".repeat(64)}`,
  expiresAtMs: Number.MAX_SAFE_INTEGER,
};

export const testSocketGrantIssuer: SocketGrantIssuerV1 = {
  issueDirectory: async () => testSocketGrantReceipt,
  issueDirectoryScoped: async () => testSocketGrantReceipt,
};

export const testDirectoryClient = (): InstanceType<typeof DirectoryClient> => new DirectoryClient("http://hub.test", { socketGrantIssuer: testSocketGrantIssuer });

export function sampleDirectoryEvent(seq: number): DirectoryEvent {
  return {
    seq,
    id: `evt-${seq}`,
    hlc: { physicalMs: seq, logical: 0 },
    actor: { kind: "user", id: "u-1" },
    body: { kind: "space.renamed", spaceId: "sp-1", name: `space ${seq}` },
    recordedAtMs: seq,
  };
}
