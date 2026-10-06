//! 📎 Bounded duplex dashboard transport; terminal I/O stays off the UI thread.

use super::ipc::{self, ClientMsg, ServerMsg};
use std::io::Read;
use std::path::Path;
use std::sync::mpsc::{self, Receiver, SyncSender};
use std::time::Duration;

pub enum Message {
    Control(ServerMsg),
    Output { session_id: String, data: Vec<u8> },
    Disconnected(String),
}

pub struct Connection {
    sender: SyncSender<ClientMsg>,
    receiver: Receiver<Message>,
    daemon_pid: u32,
}

impl Connection {
    pub fn connect(root: &Path) -> std::io::Result<Self> {
        let mut stream = ipc::connect(root)?;
        #[cfg(unix)]
        stream.set_read_timeout(Some(Duration::from_secs(2)))?;
        #[cfg(windows)]
        {
            let deadline = std::time::Instant::now() + Duration::from_secs(2);
            while ipc::peek(&stream).ok_or_else(|| std::io::Error::other("daemon disconnected"))? < 4 {
                if std::time::Instant::now() >= deadline { return Err(std::io::Error::new(std::io::ErrorKind::TimedOut, "daemon greeting timed out")); }
                std::thread::sleep(Duration::from_millis(10));
            }
        }
        let (_, payload) = ipc::read_frame(&mut stream)?;
        let ServerMsg::Attached { daemon_pid } = ipc::decode_control(&payload)? else { return Err(std::io::Error::other("invalid daemon greeting")) };
        #[cfg(unix)]
        stream.set_nonblocking(true)?;
        let (sender, commands) = mpsc::sync_channel::<ClientMsg>(64);
        let (messages, receiver) = mpsc::sync_channel::<Message>(256);
        std::thread::spawn(move || {
            let mut buffer = Vec::new();
            let mut chunk = [0u8; 16 * 1024];
            let result = (|| -> std::io::Result<()> {
                loop {
                    for _ in 0..32 {
                        match commands.try_recv() {
                            Ok(command) => ipc::write_control(&mut stream, &command)?,
                            Err(mpsc::TryRecvError::Empty) => break,
                            Err(mpsc::TryRecvError::Disconnected) => return Ok(()),
                        }
                    }
                    #[cfg(windows)]
                    let available = ipc::peek(&stream).ok_or_else(|| std::io::Error::other("daemon disconnected"))?.min(chunk.len());
                    #[cfg(unix)]
                    let available = chunk.len();
                    if available > 0 {
                        match stream.read(&mut chunk[..available]) {
                            Ok(0) => return Err(std::io::Error::other("daemon disconnected")),
                            Ok(count) => buffer.extend_from_slice(&chunk[..count]),
                            Err(error) if matches!(error.kind(), std::io::ErrorKind::WouldBlock | std::io::ErrorKind::Interrupted) => {}
                            Err(error) => return Err(error),
                        }
                    }
                    for _ in 0..64 {
                        let Some((kind, payload)) = ipc::try_decode_frame(&mut buffer)? else { break };
                        let message = match kind {
                            ipc::KIND_CONTROL => Message::Control(ipc::decode_control(&payload)?),
                            ipc::KIND_OUTPUT => {
                                let (session_id, data) = ipc::decode_output(&payload)?;
                                Message::Output { session_id, data }
                            }
                            _ => return Err(std::io::Error::other("unknown daemon frame")),
                        };
                        messages.try_send(message).map_err(|_| std::io::Error::other("dashboard view output backlog exceeded"))?;
                    }
                    std::thread::sleep(Duration::from_millis(10));
                }
            })();
            if let Err(error) = result { let _ = messages.try_send(Message::Disconnected(error.to_string())); }
        });
        Ok(Self { sender, receiver, daemon_pid })
    }

    pub fn daemon_pid(&self) -> u32 { self.daemon_pid }

    pub fn send(&mut self, command: &ClientMsg) -> std::io::Result<()> {
        command.validate()?;
        self.sender.try_send(command.clone()).map_err(|_| std::io::Error::other("dashboard daemon connection unavailable"))
    }

    pub fn receive(&mut self, timeout: Duration) -> std::io::Result<Vec<Message>> {
        let mut messages = Vec::new();
        match self.receiver.recv_timeout(timeout) {
            Ok(message) => messages.push(message),
            Err(mpsc::RecvTimeoutError::Timeout) => return Ok(messages),
            Err(mpsc::RecvTimeoutError::Disconnected) => return Err(std::io::Error::other("dashboard daemon disconnected")),
        }
        messages.extend(self.receiver.try_iter().take(255));
        Ok(messages)
    }
}

impl Drop for Connection {
    fn drop(&mut self) { let _ = self.sender.try_send(ClientMsg::Detach {}); }
}
