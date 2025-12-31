let add = |x| x + 1;
🆚 Closures vs Functions
Functions
Require explicit type annotations
Types are part of a public interface
Cannot capture variables from their environment
fn add(x: u32) -> u32 {
    x + 1
}
Closures
Usually don’t need type annotations
Types are inferred from usage
Can capture values from surrounding scope
let add = |x| x + 1;
🔍 Type Inference in Closures
Rust infers one concrete type per closure.
let example = |x| x;

example(String::from("hello")); // works
example(5); // ❌ error
Once Rust infers the type (String here), it is locked.
📦 How Closures Capture Values
Closures can capture values in three ways:
Capture Type	What it does
Immutable borrow	Read-only access
Mutable borrow	Can modify value
Move ownership	Takes ownership
Rust automatically chooses the least restrictive capture needed.
🧘 Fn — Read-only Closures
Capture values immutably
Do not mutate or move values
Can be called multiple times
let list = vec![1, 2, 3];
let show = || println!("{list:?}");
🔁 FnMut — Mutable Closures
Mutate captured values
Can be called multiple times
Require mut
let mut count = 0;
let mut increment = || count += 1;
🔥 FnOnce — One-time Closures
Move ownership of captured values
Can be called only once
let s = String::from("hello");
let consume = || drop(s);
🧠 Closure Trait Rule (Very Important)
You don’t choose Fn, FnMut, or FnOnce —
the closure body decides.
Rust infers the correct trait automatically.
📚 Why unwrap_or_else Uses FnOnce
unwrap_or_else(|| self.most_stocked())
Closure may be called zero or one time
Needs maximum flexibility
FnOnce works for all closures
🔄 Why sort_by_key Uses FnMut
list.sort_by_key(|r| r.width);
Closure is called many times
May mutate internal state (like a counter)
Ownership must not be moved
🔍 iter().map() and Closures
iter() gives immutable references
map() does not modify original values
Uses FnMut because:
closure may be called many times
closure may mutate its own environment
let doubled: Vec<_> = v.iter().map(|x| x * 2).collect();
Original vector remains unchanged.
🧠 Final Mental Model
Fn     → read only
FnMut  → mutate state
FnOnce → move ownership
Or even simpler:
Read → Fn
Mut  → FnMut
Move → FnOnce
✅ Key Takeaways
Closures are functions with memory
Rust infers closure types from usage
Closure behavior determines which Fn trait is implemented
Standard library APIs choose the weakest trait needed
Understanding closures makes iterators and async Rust easier
🚀 Why This Matters
Closures are everywhere in Rust:
Iterators
Async code
Threads
Functional-style APIs
Understanding them deeply makes Rust feel predictable and powerful instead of confusing.

update thee data abve in this readme file. 