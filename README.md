# pteropus: a slow and stupid programming language (that eats fruit)

Hey! I felt like doing some programming language stuff that wasn't super overambitious or super hacky. If I go anywhere fun with this, I'll probably use pteropus bytecode as a codegen target for other projects.

This is definitely still a little hacky (because doing things right takes a lot of time and isn't fun) but not in ways that make it prone to falling over in a stiff breeze.

Your only hope of understanding what's going on is to read the code, because this project is functionally useless and real hacky. There's probably a demo of whatever I was testing sitting in main.rs. 

Goals are:

- pretty fast compiles
- a term representation to enable IPC and stuff (like what JSON is doing for Python)
- a good codegen target for my other projects (which mostly hackily target Rust via Jinja2)

Right now I have:

- interning for every string
- a stack address for every variable
- a stack-based bytecode (flat -- labels only exist as instruction pointers)
- a really simple calling convention that will remind you of Prolog
- pause/resume functionality (but you have to bring your own scheduler)
- assignment (but no assignment syntax except for functional call syntax)
- parsing (with Nom)

Things I'd need to add to consider this "usable":

- an ffi to rust (should be easy)
- a store that lasts between function invocations
- a proper tracer, pref. with an interactive debugger
- an on-disk format that can be loaded pretty fast (pref. not strictly dependent on Serde)
- any concurrency ops at all (even if the VM is effectively single-threaded)
- internally linearly-typed operations on vectors and compounds -- ex, a proper append that doesn't copy the vector first
- world sim operations -- esp a good partition, a good sort, conversions between vecs and sets
- a UI layer to hide all the interning (so pteropus instances can talk to each other)
- the ability to recover from failure on destructuring (see `pattern.rs` for some thoughts on how I might do that)

Things on my Would Be Nice list:

- a stack layout without the extra layer of indirection imposed by my frames/stack object
- unit test coverage
- a good solution to the problem of how copying variably-sized terms is expensive
- a language server
- turn off bounds checks in the interpreter once I stop hitting them
- codegen to rust (even really inefficient heavily sum-typed rust)

Consider this a big gist that happens to be multiple files! It's your fault if you seriously attempt to use this for anything.