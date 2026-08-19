//! 🧪️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-probe-spikes W6, S8). Does `reqwest` with
//! `rustls-tls` and no default features actually build and complete a real HTTPS GET on this
//! machine? Real network call, no mocking — resolves the platform trust-store question by hitting
//! a public HTTPS endpoint and checking the handshake + response actually succeed.

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let resp = reqwest::get("https://example.com").await?;
    let status = resp.status();
    let body = resp.text().await?;
    println!("[tlsprobe] GET https://example.com -> status = {status}, body_len = {}", body.len());
    if status.is_success() {
        println!("[tlsprobe] S8 PASS — rustls-tls (no default features) completed a real HTTPS GET");
    } else {
        println!("[tlsprobe] S8 FAIL — non-success status {status}");
    }
    Ok(())
}
