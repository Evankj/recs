# recs
This is a simple Rust implementation of an Entity Component System. It differs greatly from my C implementation [`ecc`](https://github.com/Evankj/ecc) by storing components in hashmaps inside entity structs rather than relying on fixed size arrays for components and indexing into them to get the component data for a specific entity.
