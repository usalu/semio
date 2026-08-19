//! 🧪️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-probe-spikes W6, S7). Compile-only `Send`
//! probes for the two async DB drivers `🛢️db` uses (`sqlx` postgres, `neo4rs`) — never connects to
//! anything; the two probe functions below are dead code, only ever type-checked. If a host-side
//! bridge ever wants to `.await` these query futures directly from inside `wasmtime::component`'s
//! `run_concurrent` (which requires `Send`, per S4), this is the load-bearing fact to know.

fn assert_send<T: Send>(_value: T) {}

#[allow(dead_code)]
fn sqlx_query_future_is_send(pool: &sqlx::PgPool) {
    let fut = sqlx::query("SELECT 1").execute(pool);
    assert_send(fut);
}

#[allow(dead_code)]
fn neo4rs_execute_future_is_send(graph: &neo4rs::Graph) {
    let fut = graph.execute(neo4rs::query("RETURN 1"));
    assert_send(fut);
}

fn main() {
    println!(
        "[driversend] S7 PASS (compile-only): sqlx::query(..).execute(&PgPool) future is Send, \
         neo4rs::Graph::execute(..) future is Send. This binary performs no I/O — see \
         sqlx_query_future_is_send / neo4rs_execute_future_is_send, both dead code, both had to \
         type-check for this to build at all."
    );
}
