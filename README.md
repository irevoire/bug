```
% cargo build
   Compiling bug v0.1.0 (/home/irevoire/bug)
thread 'rustc' panicked at 'expected `NodeId` to be lowered already for res Local(
    NodeId(28),
)', compiler/rustc_ast_lowering/src/lib.rs:714:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (8e54a2113 2021-02-13) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
error: could not compile `bug`

To learn more, run the command again with --verbose.
```
