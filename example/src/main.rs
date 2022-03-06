/* Copyright (C) 2022 Dylan Staatz - All Rights Reserved. */

use refview::{RefView, RefViewMut};

#[derive(RefView, RefViewMut)]
struct DataStruct {
    field0: f64,
    field1: u64,
    field2: isize,
    field3: Vec<Box<usize>>,
}

fn test_view_struct() {
    let data = DataStruct {
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

fn test_view_mut_struct() {
    let mut data = DataStruct {
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

#[derive(RefView, RefViewMut)]
struct DataTuple(f64, u64, isize, Vec<Box<usize>>);

fn test_view_tuple() {
    let data = DataTuple(1.0, 2, -5, vec![]);

    let view = data.view();

    assert_eq!(view.0, &data.0);
    assert_eq!(view.1, &data.1);
    assert_eq!(view.2, &data.2);
    assert_eq!(view.3, &data.3);
}

fn test_view_mut_tuple() {
    let mut data = DataTuple(1.0, 2, -5, vec![]);

    // Modify data with mutable reference
    let view = data.view_mut();
    *view.1 = 5;
    *view.2 = 5;

    let view = data.view();

    assert_eq!(*view.0, data.0);
    assert_eq!(*view.1, data.1);
    assert_eq!(*view.2, data.2);
    assert_eq!(view.3, &data.3);
}

fn main() {
    test_view_struct();
    test_view_mut_struct();
    test_view_tuple();
    test_view_mut_tuple();
}
