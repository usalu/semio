import { checkedU64AsNumber } from "./common.js";
import { inputStreamCreate, outputStreamCreate, pollableCreate } from "./io.js";
const symbolDispose = Symbol.dispose || Symbol.for("dispose");
const unsupported = () => {
    throw "not-supported";
};
function addressKey(address) {
    return `${address.tag}:${address.val.address.join(".")}:${address.val.port}`;
}
function takeBytes(chunks, length) {
    const chunk = chunks.shift();
    if (!chunk) {
        throw { tag: "closed" };
    }
    const requested = checkedU64AsNumber(length, "length");
    if (chunk.byteLength <= requested) {
        return chunk;
    }
    chunks.unshift(chunk.subarray(requested));
    return chunk.subarray(0, requested);
}
/** The browser-side endpoint of an in-memory TCP connection. */
export class InMemoryTcpClient {
    #toServer = [];
    #fromServer = [];
    write(bytes) {
        this.#toServer.push(new Uint8Array(bytes));
    }
    read(length = 4096n) {
        return takeBytes(this.#fromServer, length);
    }
    _serverStreams() {
        return [
            inputStreamCreate({ blockingRead: (length) => takeBytes(this.#toServer, length) }),
            outputStreamCreate({
                write: (bytes) => this.#fromServer.push(new Uint8Array(bytes)),
            }),
        ];
    }
}
/** Deterministic, process-local TCP namespaces with a browser-side client API. */
export class InMemoryTcpSockets {
    #pending = new Map();
    tcp;
    tcpCreateSocket;
    constructor() {
        const pending = this.#pending;
        class TcpSocket {
            #family;
            #localAddress;
            #remoteAddress;
            #listening = false;
            constructor(family) {
                this.#family = family;
            }
            startBind(_network, localAddress) {
                this.#localAddress = localAddress;
            }
            finishBind() {
                if (!this.#localAddress) {
                    throw "not-in-progress";
                }
            }
            startConnect = unsupported;
            finishConnect = unsupported;
            startListen() {
                if (!this.#localAddress) {
                    throw "invalid-state";
                }
            }
            finishListen() {
                this.#listening = true;
            }
            accept() {
                if (!this.#listening || !this.#localAddress) {
                    throw "invalid-state";
                }
                const client = pending.get(addressKey(this.#localAddress))?.shift();
                if (!client) {
                    throw "would-block";
                }
                const connected = new TcpSocket(this.#family);
                connected.#localAddress = this.#localAddress;
                const [input, output] = client._serverStreams();
                return [connected, input, output];
            }
            localAddress() {
                if (!this.#localAddress) {
                    throw "invalid-state";
                }
                return this.#localAddress;
            }
            remoteAddress() {
                if (!this.#remoteAddress) {
                    throw "invalid-state";
                }
                return this.#remoteAddress;
            }
            isListening() {
                return this.#listening;
            }
            addressFamily() {
                return this.#family;
            }
            setListenBacklogSize = unsupported;
            keepAliveEnabled = unsupported;
            setKeepAliveEnabled = unsupported;
            keepAliveIdleTime = unsupported;
            setKeepAliveIdleTime = unsupported;
            keepAliveInterval = unsupported;
            setKeepAliveInterval = unsupported;
            keepAliveCount = unsupported;
            setKeepAliveCount = unsupported;
            hopLimit = unsupported;
            setHopLimit = unsupported;
            receiveBufferSize = unsupported;
            setReceiveBufferSize = unsupported;
            sendBufferSize = unsupported;
            setSendBufferSize = unsupported;
            subscribe() {
                return pollableCreate();
            }
            shutdown() { }
            [symbolDispose]() { }
        }
        this.tcp = { TcpSocket };
        this.tcpCreateSocket = {
            createTcpSocket: (family) => new TcpSocket(family),
        };
    }
    connect(serverAddress) {
        const client = new InMemoryTcpClient();
        const key = addressKey(serverAddress);
        const clients = this.#pending.get(key);
        if (clients) {
            clients.push(client);
        }
        else {
            this.#pending.set(key, [client]);
        }
        return client;
    }
}
/** The browser-side endpoint of an in-memory UDP socket. */
export class InMemoryUdpClient {
    #send;
    #received = [];
    constructor(send) {
        this.#send = send;
    }
    send(bytes, serverAddress) {
        this.#send(bytes, serverAddress);
    }
    read() {
        return takeBytes(this.#received, 65535n);
    }
    _receive(bytes) {
        this.#received.push(new Uint8Array(bytes));
    }
}
/** Deterministic, process-local UDP namespaces with a browser-side client API. */
export class InMemoryUdpSockets {
    #datagrams = new Map();
    #clients = new Map();
    udp;
    udpCreateSocket;
    constructor() {
        const datagrams = this.#datagrams;
        const clients = this.#clients;
        class IncomingDatagramStream {
            queue;
            constructor(queue) {
                this.queue = queue;
            }
            receive(maxResults) {
                return this.queue.splice(0, checkedU64AsNumber(maxResults, "max results"));
            }
            subscribe() {
                return pollableCreate();
            }
            [symbolDispose]() { }
        }
        class OutgoingDatagramStream {
            remoteAddress;
            #permit = 0;
            constructor(remoteAddress) {
                this.remoteAddress = remoteAddress;
            }
            checkSend() {
                this.#permit = 1_024;
                return 1024n;
            }
            send(outgoing) {
                if (outgoing.length > this.#permit) {
                    throw new TypeError("datagram count exceeds the permit returned by checkSend");
                }
                this.#permit -= outgoing.length;
                for (const datagram of outgoing) {
                    const destination = datagram.remoteAddress ?? this.remoteAddress;
                    if (!destination) {
                        throw "invalid-argument";
                    }
                    const client = clients.get(addressKey(destination));
                    if (!client) {
                        throw "remote-unreachable";
                    }
                    client._receive(datagram.data);
                }
                return BigInt(outgoing.length);
            }
            subscribe() {
                return pollableCreate();
            }
            [symbolDispose]() { }
        }
        class UdpSocket {
            #family;
            #localAddress;
            #remoteAddress;
            constructor(family) {
                this.#family = family;
            }
            startBind(_network, localAddress) {
                this.#localAddress = localAddress;
            }
            finishBind() {
                if (!this.#localAddress) {
                    throw "not-in-progress";
                }
            }
            stream(remoteAddress) {
                if (!this.#localAddress) {
                    throw "invalid-state";
                }
                this.#remoteAddress = remoteAddress;
                const key = addressKey(this.#localAddress);
                let queue = datagrams.get(key);
                if (!queue) {
                    queue = [];
                    datagrams.set(key, queue);
                }
                return [
                    new IncomingDatagramStream(queue),
                    new OutgoingDatagramStream(remoteAddress),
                ];
            }
            localAddress() {
                if (!this.#localAddress) {
                    throw "invalid-state";
                }
                return this.#localAddress;
            }
            remoteAddress() {
                return this.#remoteAddress;
            }
            addressFamily() {
                return this.#family;
            }
            unicastHopLimit = unsupported;
            setUnicastHopLimit = unsupported;
            receiveBufferSize = unsupported;
            setReceiveBufferSize = unsupported;
            sendBufferSize = unsupported;
            setSendBufferSize = unsupported;
            subscribe() {
                return pollableCreate();
            }
            [symbolDispose]() { }
        }
        this.udp = {
            IncomingDatagramStream,
            OutgoingDatagramStream,
            UdpSocket,
        };
        this.udpCreateSocket = {
            createUdpSocket: (family) => new UdpSocket(family),
        };
    }
    createClient(localAddress) {
        const client = new InMemoryUdpClient((bytes, serverAddress) => {
            const key = addressKey(serverAddress);
            const queue = this.#datagrams.get(key);
            const incoming = { data: new Uint8Array(bytes), remoteAddress: localAddress };
            if (queue) {
                queue.push(incoming);
            }
            else {
                this.#datagrams.set(key, [incoming]);
            }
        });
        this.#clients.set(addressKey(localAddress), client);
        return client;
    }
}
