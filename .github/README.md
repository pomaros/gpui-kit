# gpui-kit for AROS

This is the [PomAROS](https://github.com/pomaros) port of
[GPUI Kit](https://github.com/longbridge/gpui-kit) to native AROS on AArch64.
The port lives on the default branch, **`aros-port-0.7.1`**: gpui-kit 0.7.1
with a small AROS delta.

- Async channels use `async-channel` in place of `smol`.
- Windows have no client-side window controls; AROS Intuition draws them.

It runs on [gpui-aros](https://github.com/pomaros/gpui-aros), the AROS
platform backend for GPUI, and is built with
[rust-aros](https://github.com/pomaros/rust-aros), Rust for native AROS.
[Ferail](https://github.com/jonx/Ferail), a file manager, uses it on AROS.

The library's own documentation is the upstream
[README](../README.md). Changes that are not specific to AROS belong
upstream in [longbridge/gpui-kit](https://github.com/longbridge/gpui-kit).
