
<div style="text-align: center;">
  <h1>˚ˋঌ˖ Fury Tracer ˋঌ </h1>
</div>


This is a ray tracer developed for METU graduate level course CENG 795.

Author: Bartu

---
For debugging:
``$ RUST_LOG=debug cargo run``

For fastest run:
``$ cargo run --release``

For unit tests:
``$ cargo test``

For more suggestions to improve code:
``$ cargo clippy``

---

> [!IMPORTANT]
> Binaries are placed under ./target/debug/ or ./target/release depending on cargo run commands above. By default it is under debug but for faster runs compiling with --release is recommended.

![Elmo Fire](./assets/elmofire.png)

# TODO 
---
- [x] Vertex indices start from 1, not 0, make sure to handle that correctly
- [x] Ambient light is declared only once, change implementation to return a single vec3, Not vector of vec3. 
- [ ] Consider utilizing CoordLike implementation in geometry.rs  
- [ ] Unit tests missing
- [ ] Consider explicitly marking your function with #[inline] or #[inline(always)] to see if it improves performance. (source: https://softwaremill.com/rust-static-vs-dynamic-dispatch/)
- [ ] Fix Fresnel computations done twice, see comments or commits for thoughts on it.
- [ ] Every shape in this class has material_idx so there could be a struct
    dedicated to such data unrelated to shapes but required for HitRecord. But
    Rust does not allow implicit inheritance so we cannot just:
    ```
    struct Shape { // better naming here? 
        material: Index,    
    }

    struct SomeShape : Shape {
        some_other_member: SomeType,
    }
```
    What is recommended is to use composition, e.g.

    ```struct ShapeData {
        material: Index,
    }

    struct SomeShape {
        _data: ShapeData,
        some_other_member: SomeType,
    }```

    however I'm not sure if this is helpful in our case or if it adds unnecessary complexity.
    In case this becomes necessary, check out this crate https://docs.rs/delegate/latest/delegate/. 
    Update: I completely forgot why I had this discussion, now I remember another reason: some materials share reflectance coefficients and I'd like to have specular( ) diffuse( ) functions taking material.specular_rf but traits do not allow holding data, rather we could use composition to have that information (I guess a struct like BRDF?)

    ---
    ## How to add more material? 
    - Declare your CustomMaterialStruct
    - impl material::Material for CustomMaterialStruct 
    - Add match arm to scene::parse_single_material( ) using _type value of JSON (TODO: automatize that?)