# Error using serde and -fno-pic libraries in Rust

This repo has a demonstration of the issue when combining:

 - Using `-fno-pic` C++ libraries
 - and using serde::Deserialize (presumably any proc_macro based library)

It won't build, unless you edit `.cargo/config.toml` to use `relocation-model=dynamic-no-pic`

## Output

### Option 1) no .cargo/config.toml:

```
...
  = note: /usr/bin/ld: /path/cpplink/target/debug/build/rustapp-9411bc1e2c79bd91/out/libcpplib.a(cpplib.o): relocation R_X86_64_32 against `.rodata' can not be used when making a PIE object; recompile with -fPIE
      collect2: error: ld returned 1 exit status
```
This is the 'usual' -fPIC error. When building rust applications this is fairly common because cargo (sensibly) defaults to building position independent executables.


### Option 2) relocation-model=static
Let's try to create a -no-pie executable

```
$ cat .cargo/config.toml
[build]
rustflags = ["-C", "relocation-model=static"]
$ cargo build
...
/usr/bin/ld: /path/cpplink/rustapp/target/debug/deps/libproc_macro2-3f38899cd4eed5f7.rlib(proc_macro2-3f38899cd4eed5f7.proc_macro2.b5cdcd2e-cgu.11.rcgu.o): relocation R_X86_64_32S against `.rodata.cst16' can not be used when making a shared object; recompile with -fPIC
/usr/bin/ld: /path/cpplink/rustapp/target/debug/deps/libproc_macro2-3f38899cd4eed5f7.rlib(proc_macro2-3f38899cd4eed5f7.proc_macro2.b5cdcd2e-cgu.12.rcgu.o): relocation R_X86_64_32 against `.rodata..L__unnamed_1' can not be used when making a shared object; recompile with -fPIC
/usr/bin/ld: /path/cpplink/rustapp/target/debug/deps/libproc_macro2-3f38899cd4eed5f7.rlib(proc_macro2-3f38899cd4eed5f7.proc_macro2.b5cdcd2e-cgu.13.rcgu.o): relocation R_X86_64_32 against `.rodata..L__unnamed_1' can not be used when making a shared object; recompile with -fPIC
/usr/bin/ld: /path/cpplink/rustapp/target/debug/deps/libproc_macro2-3f38899cd4eed5f7.rlib(proc_macro2-3f38899cd4eed5f7.proc_macro2.b5cdcd2e-cgu.14.rcgu.o): relocation R_X86_64_32 against `.rodata..L__unnamed_1' can not be used when making a shared object; recompile with -fPIC
/usr/bin/ld: /path/cpplink/rustapp/target/debug/deps/libproc_macro2-3f38899cd4eed5f7.rlib(proc_macro2-3f38899cd4eed5f7.proc_macro2.b5cdcd2e-cgu.15.rcgu.o): relocation R_X86_64_32 against `.rodata..L__unnamed_1' can not be used when making a shared object; recompile with -fPIC
/usr/bin/ld: /path/cpplink/rustapp/target/debug/deps/libproc_macro2-3f38899cd4eed5f7.rlib(proc_macro2-3f38899cd4eed5f7.proc_macro2.b5cdcd2e-cgu.2.rcgu.o): relocation R_X86_64_32 against `.rodata.str.0' can not be used when making a shared object; recompile with -fPIC
/usr/bin/ld: /path/cpplink/rustapp/target/debug/deps/libproc_macro2-3f38899cd4eed5f7.rlib(proc_macro2-3f38899cd4eed5f7.proc_macro2.b5cdcd2e-cgu.3.rcgu.o): relocation R_X86_64_32 against `.rodata..L__unnamed_1' can not be used when making a shared object; recompile with -fPIC
/usr/bin/ld: /path/cpplink/rustapp/target/debug/deps/libproc_macro2-3f38899cd4eed5f7.rlib(proc_macro2-3f38899cd4eed5f7.proc_macro2.b5cdcd2e-cgu.4.rcgu.o): relocation R_X86_64_32 against `.rodata.str.0' can not be used when making a shared object; recompile with -fPIC
/usr/bin/ld: /path/cpplink/rustapp/target/debug/deps/libproc_macro2-3f38899cd4eed5f7.rlib(proc_macro2-3f38899cd4eed5f7.proc_macro2.b5cdcd2e-cgu.5.rcgu.o): relocation R_X86_64_32 against `.rodata..L__unnamed_1' can not be used when making a shared object; recompile with -fPIC
/usr/bin/ld: /path/cpplink/rustapp/target/debug/deps/libproc_macro2-3f38899cd4eed5f7.rlib(proc_macro2-3f38899cd4eed5f7.proc_macro2.b5cdcd2e-cgu.6.rcgu.o): relocation R_X86_64_32 against `.rodata..L__unnamed_1' can not be used when making a shared object; recompile with -fPIC
/usr/bin/ld: /path/cpplink/rustapp/target/debug/deps/libproc_macro2-3f38899cd4eed5f7.rlib(proc_macro2-3f38899cd4eed5f7.proc_macro2.b5cdcd2e-cgu.7.rcgu.o): relocation R_X86_64_32 against `.rodata..L__unnamed_2' can not be used when making a shared object; recompile with -fPIC
/usr/bin/ld: /path/cpplink/rustapp/target/debug/deps/libproc_macro2-3f38899cd4eed5f7.rlib(proc_macro2-3f38899cd4eed5f7.proc_macro2.b5cdcd2e-cgu.8.rcgu.o): relocation R_X86_64_32 against `.rodata.str.0' can not be used when making a shared object; recompile with -fPIC
/usr/bin/ld: /path/cpplink/rustapp/target/debug/deps/libproc_macro2-3f38899cd4eed5f7.rlib(proc_macro2-3f38899cd4eed5f7.proc_macro2.b5cdcd2e-cgu.9.rcgu.o): relocation R_X86_64_32 against `.rodata..L__unnamed_1' can not be used when making a shared object; recompile with -fPIC
/usr/bin/ld: /path/cpplink/rustapp/target/debug/deps/libproc_macro2-3f38899cd4eed5f7.rlib(proc_macro2-3f38899cd4eed5f7.proc_macro2.b5cdcd2e-cgu.0.rcgu.o): relocation R_X86_64_32 against `.rodata..L__unnamed_1' can not be used when making a shared object; recompile with -fPIC
/usr/bin/ld: /path/cpplink/rustapp/target/debug/deps/libunicode_xid-8bb8457dab823b6f.rlib(unicode_xid-8bb8457dab823b6f.unicode_xid.3b40f430-cgu.0.rcgu.o): relocation R_X86_64_32S against `.rodata..L__unnamed_1' can not be used when making a shared object; recompile with -fPIC
/usr/bin/ld: /path/cpplink/rustapp/target/debug/deps/libunicode_xid-8bb8457dab823b6f.rlib(unicode_xid-8bb8457dab823b6f.unicode_xid.3b40f430-cgu.2.rcgu.o): relocation R_X86_64_32 against `.rodata..L__unnamed_1' can not be used when making a shared object; recompile with -fPIC
          collect2: error: ld returned 1 exit status
```
I don't understand this issue - looking for a solution here

### Option 3) relocation-model=dynamic-no-pic

[The rustc book](https://doc.rust-lang.org/rustc/codegen-options/index.html#relocation-model) describes a relocation-model `dynamic-no-pic`:

> dynamic-no-pic - relocatable external references, non-relocatable code. 
> Only makes sense on Darwin and is rarely used.
If StackOverflow tells you to use this as an opt-out of PIC or PIE, don't believe it, use -C relocation-model=static instead.

But it turns out this mode *does* work in this case:

```
$ cat .cargo/config.toml
[build]
rustflags = ["-C", "relocation-model=dynamic-no-pic"]
$ cargo build
...
    Finished dev [unoptimized + debuginfo] target(s) in ...
```

But it doesn't seem great to use a mode which "only makes sense on Darwin" on Linux.