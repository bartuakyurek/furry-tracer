# furry-tracer

This is a ray tracer developed for METU graduate level course CENG 795.

Author: Bartu

---
For debug:
``$ RUST_LOG=debug cargo run``

For fastest:
``$ cargo run --release``

For unit tests:
``$ cargo test``

For more suggestions to improve code:
``$ cargo clippy``


[NOTE]
Binaries are placed under ./target/debug/ or ./target/release
depending on cargo run commands above. By default it is under
debug but for faster runs compiling with --release is recommended.

TODO 
- Vertex indices start from 1, not 0, make sure to handle that correctly
- Ambient light is declared only once, change implementation to return a single vec3, 
Not vector of vec3. 
