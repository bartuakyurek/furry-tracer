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


TODO 
- Vertex indices start from 1, not 0, make sure to handle that correctly
- Ambient light is declared only once, change implementation to return a single vec3, 
Not vector of vec3. 