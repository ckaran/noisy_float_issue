# Illustration of difference between debug and release builds

**THIS CODE IS FOR A BUG REPORT.  DO NOT USE IT IN PRODUCTION CODE!**

This simplified project illustrates the difference in behavior between debug and
release builds.  When this code is executed using `cargo run`, it will execute
as expected, without any crashes.  When run with `cargo run --release`, it will
panic with the following message:

```
thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', src/libcore/option.rs:345:21
note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
```

Here is the backtrace:

```
$ RUST_BACKTRACE=full cargo run --release
    Finished release [optimized] target(s) in 0.02s
     Running `target/release/noisy_float_issue`
thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', src/libcore/option.rs:345:21
stack backtrace:
   0:     0x55817fea0963 - std::sys::unix::backtrace::tracing::imp::unwind_backtrace::hd71462e61ec5c76e
                               at src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:39
   1:     0x55817fe9ca6b - std::sys_common::backtrace::_print::hae2106692f421f30
                               at src/libstd/sys_common/backtrace.rs:70
   2:     0x55817fe9f946 - std::panicking::default_hook::{{closure}}::h4e372307f318ae92
                               at src/libstd/sys_common/backtrace.rs:58
                               at src/libstd/panicking.rs:200
   3:     0x55817fe9f6c4 - std::panicking::default_hook::hb6f777c87029e950
                               at src/libstd/panicking.rs:215
   4:     0x55817fe9ff9f - std::panicking::rust_panic_with_hook::h49d43efebcb63d45
                               at src/libstd/panicking.rs:478
   5:     0x55817fe9fb21 - std::panicking::continue_panic_fmt::h87cb4de2c26b0fa7
                               at src/libstd/panicking.rs:385
   6:     0x55817fe9fa05 - rust_begin_unwind
                               at src/libstd/panicking.rs:312
   7:     0x55817feafccc - core::panicking::panic_fmt::h472d766e4dff71a2
                               at src/libcore/panicking.rs:85
   8:     0x55817feafc0b - core::panicking::panic::h3c512c7c2bb6da25
                               at src/libcore/panicking.rs:49
   9:     0x55817fe97a5d - noisy_float_issue::main::h02b74e2a4af883fd
  10:     0x55817fe97a22 - std::rt::lang_start::{{closure}}::h1bc09f850a1cd95c
  11:     0x55817fe9f9f2 - std::panicking::try::do_call::hf75bca0b71ecc6e0
                               at src/libstd/rt.rs:49
                               at src/libstd/panicking.rs:297
  12:     0x55817fea18d9 - __rust_maybe_catch_panic
                               at src/libpanic_unwind/lib.rs:87
  13:     0x55817fea04ac - std::rt::lang_start_internal::h5a74da15a365d5aa
                               at src/libstd/panicking.rs:276
                               at src/libstd/panic.rs:388
                               at src/libstd/rt.rs:48
  14:     0x55817fe97a81 - main
  15:     0x7fd672c62b96 - __libc_start_main
  16:     0x55817fe97939 - _start
  17:                0x0 - <unknown>

```

# Meta

```
$ uname -a
Linux EMANE 4.15.0-47-generic #50-Ubuntu SMP Wed Mar 13 10:44:52 UTC 2019 x86_64 x86_64 x86_64 GNU/Linux

$ lsb_release -a
No LSB modules are available.
Distributor ID: Ubuntu
Description:    Ubuntu 18.04.2 LTS
Release:    18.04
Codename:   bionic

$ cargo -vV
cargo 1.34.0 (6789d8a0a 2019-04-01)
release: 1.34.0
commit-hash: 6789d8a0a54a96d95365c4e1fb01d47a5eed9937
commit-date: 2019-04-01

$ rustc -vV
rustc 1.34.0 (91856ed52 2019-04-10)
binary: rustc
commit-hash: 91856ed52c58aa5ba66a015354d1cc69e9779bdf
commit-date: 2019-04-10
host: x86_64-unknown-linux-gnu
release: 1.34.0
LLVM version: 8.0
```
