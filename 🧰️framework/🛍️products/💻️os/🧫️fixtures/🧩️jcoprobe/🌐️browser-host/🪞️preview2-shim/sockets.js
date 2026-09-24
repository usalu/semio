export { InMemoryTcpClient, InMemoryTcpSockets, InMemoryUdpClient, InMemoryUdpSockets, } from "./in-memory-sockets.js";
const unsupported = () => {
    throw "not-supported";
};
class Network {
}
const defaultNetwork = new Network();
export const instanceNetwork = {
    instanceNetwork: () => defaultNetwork,
};
export const network = { Network };
class ResolveAddressStream {
    resolveNextAddress = unsupported;
    subscribe = unsupported;
}
export const ipNameLookup = {
    ResolveAddressStream,
    resolveAddresses: unsupported,
};
class TcpSocket {
    startBind = unsupported;
    finishBind = unsupported;
    startConnect = unsupported;
    finishConnect = unsupported;
    startListen = unsupported;
    finishListen = unsupported;
    accept = unsupported;
    localAddress = unsupported;
    remoteAddress = unsupported;
    isListening = unsupported;
    addressFamily = unsupported;
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
    subscribe = unsupported;
    shutdown = unsupported;
}
export const tcpCreateSocket = {
    createTcpSocket: unsupported,
};
export const tcp = { TcpSocket };
class IncomingDatagramStream {
    receive = unsupported;
    subscribe = unsupported;
}
class OutgoingDatagramStream {
    checkSend = unsupported;
    send = unsupported;
    subscribe = unsupported;
}
class UdpSocket {
    startBind = unsupported;
    finishBind = unsupported;
    stream = unsupported;
    localAddress = unsupported;
    remoteAddress = unsupported;
    addressFamily = unsupported;
    unicastHopLimit = unsupported;
    setUnicastHopLimit = unsupported;
    receiveBufferSize = unsupported;
    setReceiveBufferSize = unsupported;
    sendBufferSize = unsupported;
    setSendBufferSize = unsupported;
    subscribe = unsupported;
}
export const udpCreateSocket = {
    createUdpSocket: unsupported,
};
export const udp = {
    IncomingDatagramStream,
    OutgoingDatagramStream,
    UdpSocket,
};
