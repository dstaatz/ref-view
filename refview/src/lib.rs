/* Copyright (C) 2022 Dylan Staatz - All Rights Reserved. */

//! Motivation: easily generate a "Viewer" struct that has some fields that
//! reference another struct.
//! 
//! Example input:
//! ```rust
//! #[derive(RefView)]
//! struct DataStruct {
//!     field0: f64,
//!     field1: u64,
//!     field2: isize,
//!     field3: Vec<Box<usize>>,
//! }
//! ```
//! 
//! This would (roughly) generate the following code:
//! ```rust
//! pub struct DataStructViewer<'a> {
//!     pub field0: &'a f64,
//!     pub field1: &'a u64,
//!     pub field2: &'a isize,
//!     pub field3: &'a Vec<Box<usize>>,
//! }
//! 
//! impl<'a> RefView<'a> for DataStruct {
//!     type Viewer = DataStructViewer<'a>;
//!     fn view(&'a self) -> Self::Viewer
//!     where
//!         Self::Viewer: 'a,
//!     {
//!         Self::Viewer {
//!             field0: &self.field0,
//!             field1: &self.field1,
//!             field2: &self.field2,
//!             field3: &self.field3,
//!         }
//!     }
//! }
//! ```
//! 
//! Mutable referenced structs can be generated with [`RefViewMut`].
//! 
//! ## Planned features
//! 
//! - Make some fields skip-able or include them though field attributes
//! - More control over visability of generated struct

pub use refview_derive::*;

/// Trait to "view" into a struct
/// 
/// See [module-level documentation]
/// 
/// [module-level documentation]: crate
pub trait RefView<'a> {
    type Viewer;

    fn view(&'a self) -> Self::Viewer
    where
        Self::Viewer: 'a;
}

/// Trait to mutably "view" into a struct
/// 
/// See [module-level documentation]
/// 
/// [module-level documentation]: crate
pub trait RefViewMut<'a>: RefView<'a> {
    type ViewerMut;

    fn view_mut(&'a mut self) -> Self::ViewerMut
    where
        Self::ViewerMut: 'a;
}

#[cfg(test)]
mod tests {

    use super::*;

    struct Data {
        field0: f64,
        field1: u64,
        field2: isize,
        field3: Vec<Box<usize>>,
    }

    struct DataViewer<'a> {
        field0: &'a f64,
        field1: &'a u64,
        field2: &'a isize,
        field3: &'a Vec<Box<usize>>,
    }

    impl<'a> RefView<'a> for Data {
        type Viewer = DataViewer<'a>;

        fn view(&'a self) -> Self::Viewer
        where
            Self::Viewer: 'a,
        {
            DataViewer {
                field0: &self.field0,
                field1: &self.field1,
                field2: &self.field2,
                field3: &self.field3,
            }
        }
    }

    struct DataViewerMut<'a> {
        field1: &'a mut u64,
        field2: &'a mut isize,
    }

    impl<'a> RefViewMut<'a> for Data {
        type ViewerMut = DataViewerMut<'a>;

        fn view_mut(&'a mut self) -> Self::ViewerMut
        where
            Self::ViewerMut: 'a,
        {
            DataViewerMut {
                field1: &mut self.field1,
                field2: &mut self.field2,
            }
        }
    }

    #[test]
    fn test_view() {
        let data = Data {
            field0: 1.0,
            field1: 2,
            field2: -5,
            field3: vec![],
        };

        let view = data.view();

        assert_eq!(view.field0, &data.field0);
        assert_eq!(view.field1, &data.field1);
        assert_eq!(view.field2, &data.field2);
        assert_eq!(view.field3, &data.field3);
    }

    #[test]
    fn test_view_mut() {
        let mut data = Data {
            field0: 1.0,
            field1: 2,
            field2: -5,
            field3: vec![],
        };

        // Modify data with mutable reference
        let view = data.view_mut();
        *view.field1 = 5;
        *view.field2 = 5;

        let view = data.view();

        assert_eq!(*view.field0, data.field0);
        assert_eq!(*view.field1, data.field1);
        assert_eq!(*view.field2, data.field2);
        assert_eq!(view.field3, &data.field3);
    }
}
