# RustySensor: A Remote Sensing Library for Rust

**This library is still WIP. Expected first release: Early December.**

RustySensor wants to be an efficient library implementing many remote sensing formulas and algorithms. Written in pure Rust, RustySensor is designed to be efficient, fast, and complete. It uses the `contracts` crate to ensure pre and postconditions for each function.

RustySensor is separated into five submodules:

1. `em`: Anything related to electromagnetics in general
2. `el_opt`: Functions related to electro optical systems
3. `muwave`: Functions related to microwave systems
4. `photographic`: Anything related to photographic systems
5. `ranged`: Anything related to ranged and scattering systems
