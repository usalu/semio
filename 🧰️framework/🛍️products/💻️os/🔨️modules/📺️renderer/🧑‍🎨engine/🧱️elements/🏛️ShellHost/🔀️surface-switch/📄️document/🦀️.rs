//! 📄️ Complete document preparation before publishing a replacement surface.
pub(crate) trait DocumentSurfacePreparation {
    type Archive;
    type Surface;
    fn current(&self) -> bool;
    async fn capture(&mut self) -> Result<Self::Archive, String>;
    async fn create(&mut self) -> Result<Self::Surface, String>;
    async fn restore(&mut self, surface: &Self::Surface, archive: Self::Archive) -> Result<(), String>;
    async fn attach(&mut self, surface: &Self::Surface) -> Result<(), String>;
    async fn release(&mut self, surface: Self::Surface) -> Result<(), String>;
}

/// 🔐️ Keeps capture before activation and releases an unpublished successor on every later refusal.
pub(crate) async fn prepare_document_surface<P: DocumentSurfacePreparation>(ports: &mut P) -> Result<P::Surface, String> {
    if !ports.current() {
        return Err("surface-document.predecessor-stale".into());
    }
    let archive = ports.capture().await?;
    if !ports.current() {
        return Err("surface-document.predecessor-stale".into());
    }
    let surface = ports.create().await?;
    let outcome = async {
        if !ports.current() {
            return Err("surface-document.predecessor-stale".into());
        }
        ports.restore(&surface, archive).await?;
        if !ports.current() {
            return Err("surface-document.predecessor-stale".into());
        }
        ports.attach(&surface).await?;
        if !ports.current() {
            return Err("surface-document.predecessor-stale".into());
        }
        Ok(())
    }
    .await;
    match outcome {
        Ok(()) => Ok(surface),
        Err(error) => match ports.release(surface).await {
            Ok(()) => Err(error),
            Err(release_error) => Err(format!("surface-document.preparation-and-release-failed: {error}; {release_error}")),
        },
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
