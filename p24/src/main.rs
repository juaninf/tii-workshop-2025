fn main() {
    let arr = [2, 4, 8, 16];

    let mut n = 2;
    let nth = nth_item(&arr, &n);
    let increased = increased_by_first_item(&arr, &mut n);

    let value = {
        let values = TwoValues::new(&arr[3], increased);

        assert_eq!(*values.get_first(), 16);

        values.get_second()
    };

    assert_eq!(*value, 4);
    assert_eq!(*nth, 8);
}

fn nth_item<'d>(data: &'d [usize], n: &usize) -> &'d usize {
    &data[*n]
}

fn increased_by_first_item<'d>(data: &[usize], n: &'d mut usize) -> &'d mut usize {
    *n += data[0];
    n
}

struct TwoValues<'d> {
    first: &'d usize,
    second: &'d usize,
}

impl<'d> TwoValues<'d> {
    pub fn new(first: &'d usize, second: &'d usize) -> Self {
        Self { first, second }
    }

    pub fn get_first(&self) -> &usize {
        self.first
    }

    pub fn get_second(&self) -> &'d usize {
        self.second
    }
}
