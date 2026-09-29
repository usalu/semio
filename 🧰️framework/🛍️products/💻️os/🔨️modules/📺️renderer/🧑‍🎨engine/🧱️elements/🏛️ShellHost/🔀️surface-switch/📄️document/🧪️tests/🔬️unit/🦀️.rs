//! 📄️ Native execution of the shared document surface handoff traces.
use super::*;
struct Ports {
    law: serde_json::Value,
    source: serde_json::Value,
    target: Option<serde_json::Value>,
    current: bool,
    attached: bool,
    events: Vec<String>,
}
impl Ports {
    fn step(&mut self, name: &str) -> Result<(), String> {
        self.events.push(name.into());
        if self.law["failure"].as_str() == Some(name) {
            return Err(name.into());
        }
        if self.law["staleAfter"].as_str() == Some(name) {
            self.current = false;
        }
        Ok(())
    }
}
impl DocumentSurfacePreparation for Ports {
    type Archive = serde_json::Value;
    type Surface = u64;
    fn current(&self) -> bool {
        self.current
    }
    async fn capture(&mut self) -> Result<Self::Archive, String> {
        self.step("capture")?;
        Ok(self.source.clone())
    }
    async fn create(&mut self) -> Result<Self::Surface, String> {
        self.step("create")?;
        Ok(17)
    }
    async fn restore(&mut self, surface: &Self::Surface, archive: Self::Archive) -> Result<(), String> {
        assert_eq!(*surface, 17);
        self.step("restore")?;
        self.target = Some(archive);
        Ok(())
    }
    async fn attach(&mut self, surface: &Self::Surface) -> Result<(), String> {
        assert_eq!(*surface, 17);
        self.step("attach")?;
        self.attached = true;
        Ok(())
    }
    async fn release(&mut self, surface: Self::Surface) -> Result<(), String> {
        assert_eq!(surface, 17);
        self.events.push("release".into());
        self.target = None;
        self.attached = false;
        Ok(())
    }
}
#[test]
fn document_surface_preparation_preserves_complete_archive_and_refuses_stale_owners() {
    semio_framework_async::block_on(async {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
        for law in fixture["cases"].as_array().unwrap() {
            let mut ports = Ports { law: law.clone(), source: fixture["archive"].clone(), target: None, current: law["initiallyCurrent"].as_bool().unwrap_or(true), attached: false, events: Vec::new() };
            let ready = law["ready"].as_bool().unwrap();
            let outcome = prepare_document_surface(&mut ports).await;
            assert_eq!(outcome.is_ok(), ready, "{law}");
            assert_eq!(ports.attached, ready, "{law}");
            assert_eq!(ports.target, ready.then(|| fixture["archive"].clone()), "{law}");
            assert_eq!(serde_json::to_value(ports.events).unwrap(), law["events"], "{law}");
            assert_eq!(ports.source, fixture["archive"]);
        }
    });
}
