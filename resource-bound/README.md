[![Crates.io](https://img.shields.io/crates/v/resource-bound)](https://crates.io/crates/resource-bound)
[![GitHub stars](https://img.shields.io/github/stars/oOp995/resource-bound)](https://github.com/oOp995/resource-bound)
[![License](https://img.shields.io/github/license/oOp995/resource-bound)](https://github.com/oOp995/resource-bound/blob/main/LICENSE)

 

# resource-bound

`resource-bound` is a **procedural macro crate** that enforces **compile-time resource constraints** on Rust structs.

It allows you to:

* enforce a **maximum struct size** at compile time
* **disallow heap allocation by default**
* explicitly **opt in to heap usage** when required
* catch violations **early**, with **zero runtime cost**

This crate is intentionally **conservative**, **explicit**, and **honest** about what Rust can and cannot guarantee at compile time.

---


## 📜 Changelog
See the full [CHANGELOG.md](./CHANGELOG.md) for details on all updates and fixes.

## Motivation

In embedded, systems, and performance-critical Rust code, it is often necessary to ensure that certain data structures:

* have a **known and bounded size**
* do **not allocate on the heap** unless explicitly allowed
* fail **at compile time**, not at runtime

Rust itself does not provide a built-in way to enforce these constraints declaratively.

`resource-bound` fills this gap by providing a derive macro that performs **static checks** during compilation.

---

## What this crate guarantees

When you write:

```rust
#[derive(ResourceBound)]
#[size_limit = 32]
struct MyStruct { /* ... */ }
```

`resource-bound` guarantees at **compile time** that:

* `std::mem::size_of::<MyStruct>() <= 32`
* all field types are **explicitly approved stack-only types**
* no heap-allocating types are used **unless explicitly allowed**

All checks are performed **at compile time**. There is **no runtime overhead**.

---

## What this crate does NOT guarantee

It is important to be explicit about limitations:

* This crate does **not** detect runtime or indirect heap allocations
* This crate does **not** perform escape or alias analysis
* This crate does **not** infer allocation behavior of generic types
* This crate does **not** track allocator behavior

Heap usage is approximated by **explicit type allow-listing**, not by analysis.

If you need runtime memory tracking or allocator instrumentation, this crate is **not** the right tool.

---

## Usage

Add the dependency:

```toml
[dependencies]
resource-bound = "0.1.2"
```

Import the derive macro and StackOnly trait:

```rust
use resource_bound::ResourceBound;
use resource_bound::StackOnly;
//or
use resource_bound::*;
```

---

## Example: stack-only struct

```rust
// in case of size overflow you will get compile-time error
// -> attempt to compute `0_usize - 1_usize`, which would overflow


#[derive(ResourceBound)] 
#[size_limit=32] // 32 Bytes size struct, ,if the size of fields exceeds
                 // explictly defined value, you will get compile-time error.
struct StackStruct{

    // size is exact value returned by std::mem::size_of()
    // size is 32 bytes on 64-bit targets, 24 bytes on 32-bit targets

    unit:(),

    i32value:i32,  //signed-integer   i8,i16,i32,i64,i128 are StackOnly marked

    u32value:u32,  //unsigned-integer u8,u16,u32,u64,u128 are StackOnly marked

    f32value:f32,  //flaots f32,f64 are StackOnly marked

    char_value:char,//char is StackOnly marked

    bool_value:bool, //bool is StackOnly marked

    usize_value:usize

    //std::mem::size_of::<StackStruct>() is:

    //32 bytes on 64-bit

    //24 bytes on 32-bit

    //if you try to set #[size_limit] less than the actual
    //size depending on your structure ,you will trigger a
    //compile-time error


}

```

On common platforms:

* **64-bit targets** → `size_of::<StackStruct>() == 32`
* **32-bit targets** → `size_of::<StackStruct>() == 24`

If the actual size exceeds `#[size_limit]`, compilation will fail.

---

## Example: heap-enabled struct

Heap allocation is **disallowed by default**.

To allow heap usage, you must opt in explicitly:

```rust
#[derive(ResourceBound)]
#[size_limit = 48] // in Bytes
#[allow_heap = true] //default is false, if you want heap explicilty opt-in 
                     // #[allow_heap = true] ,
                     //othetwise #[allow_heap = false] is the default

//lifetimes are allowed for the sake of storing ref

//generics is disable intentionally , because generic heap behaviour 
//is not guaranted at compile-time neither at runtime
//actually using generics with ResourceBound will trigger compile-error

struct HeapStruct<'a>{

    string_value:String,

    str_value:&'a str,

    box_value:Box<i32>,
}
```

This makes heap usage **visible and intentional**, while still enforcing a maximum struct size.

---

## Default behavior

* `#[allow_heap]` defaults to **false**
* heap-allocating types such as `Vec`, `String`, and `Box` are **rejected by default**
* `#[size_limit]` enforces a hard upper bound on `size_of::<Self>()`
* `#[size_limit]` enforced by the compiler, **compile-time** error if **missing**.


All violations result in **compile-time errors**.

---

## Allowed primitive types (v0.1.1)

By default, the following **primitive scalar types** are considered stack-only and allowed:

* `()`
* `bool`
* `char`
* Signed integers: `i8`, `i16`, `i32`, `i64`, `i128`
* Unsigned integers: `u8`, `u16`, `u32`, `u64`, `u128`
* Pointer-sized integers: `isize`, `usize`
* Floating-point types: `f32`, `f64`

All other types are rejected unless heap usage is explicitly enabled.

---

## Generics and lifetimes

### Lifetimes

Lifetimes are allowed:

```rust
struct RefStruct<'a> {
    value: &'a u32,
}
```

### Generics

Generic type parameters are **intentionally disallowed**:

```rust
struct Generic<T> {
    value: T,
}
```

Generic heap behavior cannot be reliably verified at compile time, so `ResourceBound` rejects generic structs by design.

---

## Diagnostics

When a constraint is violated, `resource-bound` emits **clear, targeted compile-time errors** that:

* explain what rule was violated
* point to the offending field or item
* suggest how to fix the issue

No runtime panics. No silent failures.

---

## Design philosophy

`resource-bound` follows a few core principles:

* **Explicit over implicit**
* **Conservative by default**
* **Compile-time enforcement**
* **Zero runtime cost**
* **No false guarantees**

If a property cannot be proven at compile time, it is **not assumed**.

---

## Versioning and stability

This crate is intentionally strict in its initial release.

Future versions may:

* expand the allow-list deliberately
* introduce explicit opt-in traits
* add additional compile-time checks

Breaking changes will follow semantic versioning.

---

## License

Licensed under the MIT license.

---

## Final note

`resource-bound` is designed for developers who care deeply about **predictability**, **clarity**, and **compile-time guarantees**.

If that matches your use case, this crate was built for you.
#
