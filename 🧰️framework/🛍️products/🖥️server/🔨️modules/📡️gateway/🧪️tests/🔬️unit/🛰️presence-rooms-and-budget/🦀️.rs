//! 🛰️ Presence room readers, pages, the per-address budget and the state frame of the gateway unit laws. See [`super`].

use super::*;

//#region 🔖️PresenceStream
type ClientFrames = futures::channel::mpsc::UnboundedSender<Result<Message, std::convert::Infallible>>;

pub(super) type ServerFrames = futures::channel::mpsc::UnboundedReceiver<Message>;

pub(super) fn quick_presence(idle: Duration) -> PresenceSettings {
    PresenceSettings { tick: Duration::from_millis(20), idle, keepalive: Duration::from_secs(30), ..PresenceSettings::default() }
}

pub(super) async fn presence_server(settings: PresenceSettings) -> ServerState<TestInstance> {
    let server = Server::<TestInstance>::builder(StorageProfile::Embedded { data_dir: "/tmp/semio-gateway".to_string() }).module(CountingModule).presence(settings).build().await.expect("built");
    let state = server.state().clone();
    grant(&state, PolicyPoint::Subscription, PRESENCE_JOIN);
    grant(&state, PolicyPoint::Subscription, PRESENCE_PUBLISH);
    state
}

pub(super) fn connect(state: &ServerState<TestInstance>, scope: &str, surface: &str) -> (ClientFrames, ServerFrames, tokio::task::JoinHandle<()>) {
    let (client, mut inbound) = unbounded::<Result<Message, std::convert::Infallible>>();
    let (mut outbound, frames) = unbounded::<Message>();
    let (state, scope, surface) = (state.clone(), Scope(scope.to_string()), surface.to_string());
    let task = tokio::spawn(async move { run_presence(&state, &scope, &surface, &PresenceGrant { principal: Principal::Anonymous, may_publish: true, client: caller() }, &mut outbound, &mut inbound).await });
    (client, frames, task)
}

pub(super) async fn next_frame(frames: &mut ServerFrames) -> PresenceFrame {
    loop {
        match tokio::time::timeout(Duration::from_secs(5), frames.next()).await.expect("a frame arrives").expect("the socket is open") {
            Message::Text(text) => return serde_json::from_str(text.as_str()).expect("a presence frame"),
            Message::Ping(_) => continue,
            other => panic!("unexpected message {other:?}"),
        }
    }
}

pub(super) fn say(client: &ClientFrames, text: &str) {
    client.unbounded_send(Ok(Message::Text(text.to_string().into()))).expect("the session is open");
}
//#endregion 🔖️PresenceStream
