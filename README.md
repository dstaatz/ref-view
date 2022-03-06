# ref-view

A derive macro that generates a "view" into some fields of a struct.

- [`refview/src/lib.rs`](refview/src/lib.rs)
- [`refview_derive/src/lib.rs`](refview_derive/src/lib.rs)
- [`example/src/main.rs`](example/src/main.rs)

## Usage

Motivation: easily generate a "Viewer" struct that has some fields that
reference another struct.

Example input:
```rust
#[derive(RefView)]
struct DataStruct {
    field0: f64,
    field1: u64,
    field2: isize,
    field3: Vec<Box<usize>>,
}
```

This would (roughly) generate the following code:
```rust
pub struct DataStructViewer<'a> {
    pub field0: &'a f64,
    pub field1: &'a u64,
    pub field2: &'a isize,
    pub field3: &'a Vec<Box<usize>>,
}

impl<'a> RefView<'a> for DataStruct {
    type Viewer = DataStructViewer<'a>;
    fn view(&'a self) -> Self::Viewer
    where
        Self::Viewer: 'a,
    {
        Self::Viewer {
            field0: &self.field0,
            field1: &self.field1,
            field2: &self.field2,
            field3: &self.field3,
        }
    }
}
```

Mutable referenced structs can be generated with `RefViewMut`.

## Planned features

- Make some fields skip-able or include them though field attributes
- More control over visability of generated struct


#### License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.


Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
