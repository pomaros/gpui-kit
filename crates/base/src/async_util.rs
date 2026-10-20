//! Cross-platform async channel primitives for native and WASM targets.

#[cfg(not(any(target_family = "wasm", target_os = "aros")))]
pub use smol::channel::{Receiver, Sender, unbounded};

// `smol::channel` re-exports `async_channel`; depending on it directly is
// behaviour-identical and keeps smol's async-io reactor (no AROS arm) out of
// the graph.
#[cfg(any(target_family = "wasm", target_os = "aros"))]
pub use async_channel::{Receiver, Sender, unbounded};
