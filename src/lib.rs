pub trait ViewRef<'a> {
    type RefView;

    fn view(&'a self) -> Self::RefView
    where
        Self::RefView: 'a;
}

pub trait ViewMut<'a>: ViewRef<'a> {
    type MutView;

    fn view_mut(&'a mut self) -> Self::MutView
    where
        Self::MutView: 'a;
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

    struct DataView<'a> {
        field0: &'a f64,
        field1: &'a u64,
        field2: &'a isize,
        field3: &'a Vec<Box<usize>>,
    }

    impl<'a> ViewRef<'a> for Data {
        type RefView = DataView<'a>;

        fn view(&'a self) -> Self::RefView
        where
            Self::RefView: 'a,
        {
            DataView {
                field0: &self.field0,
                field1: &self.field1,
                field2: &self.field2,
                field3: &self.field3,
            }
        }
    }

    struct DataViewMut<'a> {
        field1: &'a mut u64,
        field2: &'a mut isize,
    }

    impl<'a> ViewMut<'a> for Data {
        type MutView = DataViewMut<'a>;

        fn view_mut(&'a mut self) -> Self::MutView
        where
            Self::MutView: 'a,
        {
            DataViewMut {
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
