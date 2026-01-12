# resource-bound-core

`resource-bound-core` is a **part of `rsource-bound` crate** and not designed to work solely,the main usage is in `resource-bound` crate, refer to.


# resource-bound

`resource-bound` is a **procedural macro crate** that enforces **compile-time resource constraints** on Rust structs.

It allows you to:

* enforce a **maximum struct size** at compile time
* **disallow heap allocation by default**
* explicitly **opt in to heap usage** when required
* catch violations **early**, with **zero runtime cost**

This crate is intentionally **conservative**, **explicit**, and **honest** about what Rust can and cannot guarantee at compile time.
