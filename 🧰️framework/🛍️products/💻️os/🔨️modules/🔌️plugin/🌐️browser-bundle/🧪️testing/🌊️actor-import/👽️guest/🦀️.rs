//! 🌊️ Tiny standalone guest for canonical `semio:framework/host-async` import qualification.

wit_bindgen::generate!({
    path: "🧬️schema/📜️world.wit",
    world: "actor-import",
    async: true,
});

use exports::semio::framework::actor_import_probe::Guest as ActorImportProbeGuest;
use semio::framework::host_async;

struct Component;

impl ActorImportProbeGuest for Component {
    async fn link_roundtrip(link: Vec<u8>) -> Result<Vec<u8>, Vec<u8>> {
        host_async::link_resolve(link).await
    }

    async fn blob_collect(hash: String) -> Result<Vec<u8>, Vec<u8>> {
        let mut stream = host_async::blob_read(hash).await?;
        let mut bytes = Vec::new();
        while let Some(byte) = stream.next().await {
            bytes.push(byte);
        }
        Ok(bytes)
    }

    async fn blob_drop(hash: String) -> Result<(), Vec<u8>> {
        let stream = host_async::blob_read(hash).await?;
        drop(stream);
        Ok(())
    }

    async fn timer_roundtrip(delay_ms: u32) -> u32 {
        std::thread::sleep(std::time::Duration::from_millis(u64::from(delay_ms)));
        delay_ms
    }
}

export!(Component);
