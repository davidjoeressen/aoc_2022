#[derive(PartialEq, Eq, Debug)]
pub struct Map<T> {
    pub width: usize,
    pub height: usize,
    pub values: Vec<T>,
}

pub struct Row<'a, T> {
    map: &'a Map<T>,
    row: usize,
    item: usize,
    item_back: usize,
}

impl<'a, T: Clone> Iterator for Row<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.item_back <= self.item {
            return None;
        }
        let result = self.map.get(self.row, self.item).map(T::clone);
        self.item += 1;
        result
    }
}

impl<'a, T: Clone> ExactSizeIterator for Row<'a, T> {
    fn len(&self) -> usize {
        self.item_back + 1
    }
}

impl<'a, T: Clone> DoubleEndedIterator for Row<'a, T> {
    fn next_back(&mut self) -> Option<T> {
        if self.item_back <= self.item {
            return None;
        }
        let result = self.map.get(self.row, self.item_back).map(T::clone);
        self.item_back -= 1;
        result
    }
}

pub struct Col<'a, T> {
    map: &'a Map<T>,
    col: usize,
    item: usize,
    item_back: usize,
}

impl<'a, T: Clone> Iterator for Col<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.item_back <= self.item {
            return None;
        }
        let result = self.map.get(self.item, self.col).map(T::clone);
        self.item += 1;
        result
    }
}

impl<'a, T: Clone> ExactSizeIterator for Col<'a, T> {
    fn len(&self) -> usize {
        self.item_back + 1
    }
}

impl<'a, T: Clone> DoubleEndedIterator for Col<'a, T> {
    fn next_back(&mut self) -> Option<T> {
        if self.item_back <= self.item {
            return None;
        }
        let result = self.map.get(self.item_back, self.col).map(T::clone);
        self.item_back -= 1;
        result
    }
}

impl<T> Map<T> {
    pub fn set(&mut self, row: usize, col: usize, value: T) {
        let index = self.width * row + col;
        self.values[index] = value
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        let index = self.width * row + col;
        self.values.get(index)
    }

    pub fn row(&self, n: usize) -> Row<T> {
        if n >= self.height {
            panic!("Index out of bounds");
        }
        Row {
            map: self,
            row: n,
            item: 0,
            item_back: self.width - 1,
        }
    }

    pub fn col(&self, n: usize) -> Col<T> {
        if n >= self.width {
            panic!("Index out of bounds");
        }
        Col {
            map: self,
            col: n,
            item: 0,
            item_back: self.height - 1,
        }
    }
}
