# repro_crash_import_error

Reproduction for [Issue #91](https://github.com/DelSkayn/rquickjs/issues/91) in the https://github.com/DelSkayn/rquickjs Rust wrapper for QuickJS.

It appears that the following sequence of events can lead to a memory corruption that eventually crashes the program:

1. Compile a module A that import another module B.
2. Instantiating module B causes an error, e.g. a JS exception.

To run the reproduction:

```
cargo run --bin repro
```

To see that this only happens when the error occurs in the imported module:

```
cargo run --bin no-repro-without-imports
```

Valgrind has this to say.

```
==1664== Memcheck, a memory error detector
==1664== Copyright (C) 2002-2017, and GNU GPL'd, by Julian Seward et al.
==1664== Using Valgrind-3.18.1 and LibVEX; rerun with -h for copyright info
==1664== Command: target/debug/repro
==1664== 
--1664-- WARNING: unhandled amd64-linux syscall: 334
--1664-- You may be able to write your own handler.
--1664-- Read the file README_MISSING_SYSCALL_OR_IOCTL.
--1664-- Nevertheless we consider this a bug.  Please report
--1664-- it at http://valgrind.org/support/bug_reports.html.
Attempt 1
==1664== Invalid read of size 4
==1664==    at 0x131F85: rquickjs_sys::JS_FreeValueRef (common.rs:104)
==1664==    by 0x131DA5: rquickjs_sys::JS_FreeValue (common.rs:113)
==1664==    by 0x12CEF7: <rquickjs_core::value::Value as core::ops::drop::Drop>::drop (value.rs:60)
==1664==    by 0x12F43A: core::ptr::drop_in_place<rquickjs_core::value::Value> (mod.rs:448)
==1664==    by 0x11EBAA: core::ptr::drop_in_place<rquickjs_core::value::module::Module<rquickjs_core::value::module::Loaded<rquickjs_core::value::module::Script>>> (mod.rs:448)
==1664==    by 0x11D1C6: rquickjs_core::value::module::Module<rquickjs_core::value::module::Loaded<S>>::eval (module.rs:382)
==1664==    by 0x11ECED: rquickjs_core::context::ctx::Ctx::compile (ctx.rs:149)
==1664==    by 0x11D6E4: repro::testit::{{closure}} (repro.rs:10)
==1664==    by 0x11CEA1: rquickjs_core::context::Context::with (context.rs:131)
==1664==    by 0x11E811: repro::testit (repro.rs:9)
==1664==    by 0x11E96B: repro::main (repro.rs:23)
==1664==    by 0x11EB9A: core::ops::function::FnOnce::call_once (function.rs:227)
==1664==  Address 0x4ba2b30 is 0 bytes inside a block of size 168 free'd
==1664==    at 0x484717B: free (in /usr/libexec/valgrind/vgpreload_memcheck-amd64-linux.so)
==1664==    by 0x14330F: js_def_free (quickjs.c:1717)
==1664==    by 0x14265F: js_free_rt (quickjs.c:1292)
==1664==    by 0x142808: js_free (quickjs.c:1349)
==1664==    by 0x18B764: js_free_module_def (quickjs.c:27105)
==1664==    by 0x143D19: js_free_modules (quickjs.c:2221)
==1664==    by 0x19C5D6: JS_EvalFunctionInternal (quickjs.c:33573)
==1664==    by 0x19C684: JS_EvalFunction (quickjs.c:33585)
==1664==    by 0x11CFB4: rquickjs_core::value::module::Module<rquickjs_core::value::module::Loaded<S>>::eval (module.rs:378)
==1664==    by 0x11ECED: rquickjs_core::context::ctx::Ctx::compile (ctx.rs:149)
==1664==    by 0x11D6E4: repro::testit::{{closure}} (repro.rs:10)
==1664==    by 0x11CEA1: rquickjs_core::context::Context::with (context.rs:131)
==1664==  Block was alloc'd at
==1664==    at 0x48447B5: malloc (in /usr/libexec/valgrind/vgpreload_memcheck-amd64-linux.so)
==1664==    by 0x143267: js_def_malloc (quickjs.c:1701)
==1664==    by 0x142631: js_malloc_rt (quickjs.c:1287)
==1664==    by 0x1426DB: js_mallocz_rt (quickjs.c:1308)
==1664==    by 0x1427B4: js_mallocz (quickjs.c:1339)
==1664==    by 0x18B347: js_new_module_def (quickjs.c:27037)
==1664==    by 0x19C9FD: __JS_EvalInternal (quickjs.c:33655)
==1664==    by 0x19CEE6: JS_EvalInternal (quickjs.c:33732)
==1664==    by 0x19D090: JS_EvalThis (quickjs.c:33763)
==1664==    by 0x19D0F4: JS_Eval (quickjs.c:33771)
==1664==    by 0x11F05D: rquickjs_core::context::ctx::Ctx::eval_raw (ctx.rs:94)
==1664==    by 0x11D411: rquickjs_core::value::module::Module::new (module.rs:193)
==1664== 
==1664== Invalid write of size 4
==1664==    at 0x131F9D: rquickjs_sys::JS_FreeValueRef (common.rs:104)
==1664==    by 0x131DA5: rquickjs_sys::JS_FreeValue (common.rs:113)
==1664==    by 0x12CEF7: <rquickjs_core::value::Value as core::ops::drop::Drop>::drop (value.rs:60)
==1664==    by 0x12F43A: core::ptr::drop_in_place<rquickjs_core::value::Value> (mod.rs:448)
==1664==    by 0x11EBAA: core::ptr::drop_in_place<rquickjs_core::value::module::Module<rquickjs_core::value::module::Loaded<rquickjs_core::value::module::Script>>> (mod.rs:448)
==1664==    by 0x11D1C6: rquickjs_core::value::module::Module<rquickjs_core::value::module::Loaded<S>>::eval (module.rs:382)
==1664==    by 0x11ECED: rquickjs_core::context::ctx::Ctx::compile (ctx.rs:149)
==1664==    by 0x11D6E4: repro::testit::{{closure}} (repro.rs:10)
==1664==    by 0x11CEA1: rquickjs_core::context::Context::with (context.rs:131)
==1664==    by 0x11E811: repro::testit (repro.rs:9)
==1664==    by 0x11E96B: repro::main (repro.rs:23)
==1664==    by 0x11EB9A: core::ops::function::FnOnce::call_once (function.rs:227)
==1664==  Address 0x4ba2b30 is 0 bytes inside a block of size 168 free'd
==1664==    at 0x484717B: free (in /usr/libexec/valgrind/vgpreload_memcheck-amd64-linux.so)
==1664==    by 0x14330F: js_def_free (quickjs.c:1717)
==1664==    by 0x14265F: js_free_rt (quickjs.c:1292)
==1664==    by 0x142808: js_free (quickjs.c:1349)
==1664==    by 0x18B764: js_free_module_def (quickjs.c:27105)
==1664==    by 0x143D19: js_free_modules (quickjs.c:2221)
==1664==    by 0x19C5D6: JS_EvalFunctionInternal (quickjs.c:33573)
==1664==    by 0x19C684: JS_EvalFunction (quickjs.c:33585)
==1664==    by 0x11CFB4: rquickjs_core::value::module::Module<rquickjs_core::value::module::Loaded<S>>::eval (module.rs:378)
==1664==    by 0x11ECED: rquickjs_core::context::ctx::Ctx::compile (ctx.rs:149)
==1664==    by 0x11D6E4: repro::testit::{{closure}} (repro.rs:10)
==1664==    by 0x11CEA1: rquickjs_core::context::Context::with (context.rs:131)
==1664==  Block was alloc'd at
==1664==    at 0x48447B5: malloc (in /usr/libexec/valgrind/vgpreload_memcheck-amd64-linux.so)
==1664==    by 0x143267: js_def_malloc (quickjs.c:1701)
==1664==    by 0x142631: js_malloc_rt (quickjs.c:1287)
==1664==    by 0x1426DB: js_mallocz_rt (quickjs.c:1308)
==1664==    by 0x1427B4: js_mallocz (quickjs.c:1339)
==1664==    by 0x18B347: js_new_module_def (quickjs.c:27037)
==1664==    by 0x19C9FD: __JS_EvalInternal (quickjs.c:33655)
==1664==    by 0x19CEE6: JS_EvalInternal (quickjs.c:33732)
==1664==    by 0x19D090: JS_EvalThis (quickjs.c:33763)
==1664==    by 0x19D0F4: JS_Eval (quickjs.c:33771)
==1664==    by 0x11F05D: rquickjs_core::context::ctx::Ctx::eval_raw (ctx.rs:94)
==1664==    by 0x11D411: rquickjs_core::value::module::Module::new (module.rs:193)
==1664== 
==1664== Invalid read of size 4
==1664==    at 0x131F9F: rquickjs_sys::JS_FreeValueRef (common.rs:105)
==1664==    by 0x131DA5: rquickjs_sys::JS_FreeValue (common.rs:113)
==1664==    by 0x12CEF7: <rquickjs_core::value::Value as core::ops::drop::Drop>::drop (value.rs:60)
==1664==    by 0x12F43A: core::ptr::drop_in_place<rquickjs_core::value::Value> (mod.rs:448)
==1664==    by 0x11EBAA: core::ptr::drop_in_place<rquickjs_core::value::module::Module<rquickjs_core::value::module::Loaded<rquickjs_core::value::module::Script>>> (mod.rs:448)
==1664==    by 0x11D1C6: rquickjs_core::value::module::Module<rquickjs_core::value::module::Loaded<S>>::eval (module.rs:382)
==1664==    by 0x11ECED: rquickjs_core::context::ctx::Ctx::compile (ctx.rs:149)
==1664==    by 0x11D6E4: repro::testit::{{closure}} (repro.rs:10)
==1664==    by 0x11CEA1: rquickjs_core::context::Context::with (context.rs:131)
==1664==    by 0x11E811: repro::testit (repro.rs:9)
==1664==    by 0x11E96B: repro::main (repro.rs:23)
==1664==    by 0x11EB9A: core::ops::function::FnOnce::call_once (function.rs:227)
==1664==  Address 0x4ba2b30 is 0 bytes inside a block of size 168 free'd
==1664==    at 0x484717B: free (in /usr/libexec/valgrind/vgpreload_memcheck-amd64-linux.so)
==1664==    by 0x14330F: js_def_free (quickjs.c:1717)
==1664==    by 0x14265F: js_free_rt (quickjs.c:1292)
==1664==    by 0x142808: js_free (quickjs.c:1349)
==1664==    by 0x18B764: js_free_module_def (quickjs.c:27105)
==1664==    by 0x143D19: js_free_modules (quickjs.c:2221)
==1664==    by 0x19C5D6: JS_EvalFunctionInternal (quickjs.c:33573)
==1664==    by 0x19C684: JS_EvalFunction (quickjs.c:33585)
==1664==    by 0x11CFB4: rquickjs_core::value::module::Module<rquickjs_core::value::module::Loaded<S>>::eval (module.rs:378)
==1664==    by 0x11ECED: rquickjs_core::context::ctx::Ctx::compile (ctx.rs:149)
==1664==    by 0x11D6E4: repro::testit::{{closure}} (repro.rs:10)
==1664==    by 0x11CEA1: rquickjs_core::context::Context::with (context.rs:131)
==1664==  Block was alloc'd at
==1664==    at 0x48447B5: malloc (in /usr/libexec/valgrind/vgpreload_memcheck-amd64-linux.so)
==1664==    by 0x143267: js_def_malloc (quickjs.c:1701)
==1664==    by 0x142631: js_malloc_rt (quickjs.c:1287)
==1664==    by 0x1426DB: js_mallocz_rt (quickjs.c:1308)
==1664==    by 0x1427B4: js_mallocz (quickjs.c:1339)
==1664==    by 0x18B347: js_new_module_def (quickjs.c:27037)
==1664==    by 0x19C9FD: __JS_EvalInternal (quickjs.c:33655)
==1664==    by 0x19CEE6: JS_EvalInternal (quickjs.c:33732)
==1664==    by 0x19D090: JS_EvalThis (quickjs.c:33763)
==1664==    by 0x19D0F4: JS_Eval (quickjs.c:33771)
==1664==    by 0x11F05D: rquickjs_core::context::ctx::Ctx::eval_raw (ctx.rs:94)
==1664==    by 0x11D411: rquickjs_core::value::module::Module::new (module.rs:193)
==1664== 
```
