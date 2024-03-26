/// A square matrix with a fixed size
pub struct SquareMatrix<T> {
    size: usize,
    data: Vec<Vec<Option<T>>>,
}

impl<T: Clone> SquareMatrix<T> {
    /// Creates an empty square matrix with given size and initializes all values to None
    ///
    /// # Examples
    ///
    /// ```
    /// use linalgo_structs::SquareMatrix;
    ///
    /// let matrix: SquareMatrix<i32> = SquareMatrix::new(10);
    /// ```
    ///
    /// # Panics
    ///
    /// Zero dimensional matrices aren't allowed, so we panic if the size is less than 1.
    pub fn new(size: usize) -> Self {
        if size == 0 {
            panic!("Matrix size must be at least 1, got {}", size);
        }

        let mut row: Vec<Option<T>> = Vec::new();
        row.resize(size, None);

        let mut data: Vec<Vec<Option<T>>> = Vec::new();
        data.resize(size, row);

        Self { size, data }
    }

    /// Returns the size of the matrix
    ///
    /// # Examples
    ///
    /// ```
    /// use linalgo_structs::SquareMatrix;
    ///
    /// let matrix: SquareMatrix<i32> = SquareMatrix::new(10);
    ///
    /// assert_eq!(matrix.size(), 10);
    /// ```
    pub fn size(&self) -> usize {
        self.size
    }

    /// Returns a reference to the value at the given position. Returns None if nothing is found
    /// or if the position is not valid.
    ///
    /// # Examples
    ///
    /// ```
    /// use linalgo_structs::SquareMatrix;
    ///
    /// let matrix: SquareMatrix<i32> = SquareMatrix::new(10);
    ///
    /// assert_eq!(matrix.get(0, 0), None);
    /// ```
    ///
    /// ```
    /// use linalgo_structs::SquareMatrix;
    ///
    /// let mut matrix: SquareMatrix<i32> = SquareMatrix::new(10);
    /// matrix.set(5, 5, 10);
    ///
    /// assert_eq!(matrix.get(5, 5), Some(10).as_ref());
    /// ```
    pub fn get(&self, row: usize, column: usize) -> Option<&T> {
        if let Some(row) = self.data.get(row) {
            if let Some(entry) = row.get(column) {
                return entry.as_ref();
            }
        }

        None
    }

    /// Moves `value` into the matrix at the given position. Consumes `value`.
    ///
    /// # Examples
    ///
    /// ```
    /// use linalgo_structs::SquareMatrix;
    ///
    /// let mut matrix: SquareMatrix<i32> = SquareMatrix::new(10);
    ///
    /// matrix.set(0, 0, 5);
    /// matrix.set(5, 5, 0);
    ///
    /// assert_eq!(matrix.get(0, 0), Some(5).as_ref());
    /// assert_eq!(matrix.get(5, 5), Some(0).as_ref());
    /// assert_eq!(matrix.get(2, 2), None);
    pub fn set(&mut self, row: usize, column: usize, value: T) {
        if let Some(row) = self.data.get_mut(row) {
            if let Some(entry) = row.get_mut(column) {
                *entry = Some(value.to_owned());
            }
        }
    }

    /// Clones `value` into every valid position in the matrix. Does not consume `value`.
    ///
    /// # Examples
    ///
    /// ```
    /// use linalgo_structs::SquareMatrix;
    ///
    /// let mut matrix: SquareMatrix<i32> = SquareMatrix::new(10);
    /// matrix.set_all_to(&2);
    ///
    /// assert_eq!(matrix.get(0, 0), Some(2).as_ref());
    /// assert_eq!(matrix.get(1, 1), Some(2).as_ref());
    /// assert_eq!(matrix.get(2, 2), Some(2).as_ref());
    /// assert_eq!(matrix.get(9, 9), Some(2).as_ref());
    pub fn set_all_to(&mut self, value: &T) {
        for row in self.data.iter_mut() {
            for entry in row.iter_mut() {
                *entry = Some(value.clone());
            }
        }
    }
}

/// A vector with a fixed size
pub struct Vector<T> {
    size: usize,
    data: Vec<Option<T>>,
}

impl<T: Clone> Vector<T> {
    /// Creates an empty vector with given size and initializes all values to None
    ///
    /// # Examples
    ///
    /// ```
    /// use linalgo_structs::Vector;
    ///
    /// let vector: Vector<i32> = Vector::new(5);
    /// ```
    ///
    /// # Panics
    ///
    /// Zero dimensional vectors aren't allowed, so we panic if the size is less than 1.
    pub fn new(size: usize) -> Self {
        if size == 0 {
            panic!("Vector size must be at least 1, got {}", size);
        }

        let mut data: Vec<Option<T>> = Vec::new();
        data.resize(size, None);

        Self { size, data }
    }

    /// Returns the size of the vector
    ///
    /// # Examples
    ///
    /// ```
    /// use linalgo_structs::Vector;
    ///
    /// let vector: Vector<i32> = Vector::new(10);
    ///
    /// assert_eq!(vector.size(), 10);
    /// ```
    pub fn size(&self) -> usize {
        self.size
    }

    /// Returns a reference to the value at the given position. Returns None if nothing is found
    /// or if the position is not valid.
    ///
    /// # Examples
    ///
    /// ```
    /// use linalgo_structs::Vector;
    ///
    /// let vector: Vector<i32> = Vector::new(10);
    ///
    /// assert_eq!(vector.get(0), None);
    /// ```
    ///
    /// ```
    /// use linalgo_structs::Vector;
    ///
    /// let mut vector: Vector<i32> = Vector::new(10);
    /// vector.set(5, 10);
    ///
    /// assert_eq!(vector.get(5), Some(10).as_ref());
    /// ```
    pub fn get(&self, position: usize) -> Option<&T> {
        self.data.get(position)?.as_ref()
    }

    /// Moves `value` into the vector at the given position. Consumes `value`.
    ///
    /// # Examples
    ///
    /// ```
    /// use linalgo_structs::Vector;
    ///
    /// let mut vector: Vector<i32> = Vector::new(10);
    ///
    /// vector.set(0, 5);
    /// vector.set(5, 0);
    ///
    /// assert_eq!(vector.get(0), Some(5).as_ref());
    /// assert_eq!(vector.get(5), Some(0).as_ref());
    /// assert_eq!(vector.get(2), None);
    pub fn set(&mut self, position: usize, value: T) {
        if let Some(entry) = self.data.get_mut(position) {
            *entry = Some(value.to_owned());
        }
    }

    /// Clones `value` into every valid position in the matrix. Does not consume `value`.
    ///
    /// # Examples
    ///
    /// ```
    /// use linalgo_structs::Vector;
    ///
    /// let mut vector: Vector<i32> = Vector::new(10);
    /// vector.set_all_to(&2);
    ///
    /// assert_eq!(vector.get(0), Some(2).as_ref());
    /// assert_eq!(vector.get(1), Some(2).as_ref());
    /// assert_eq!(vector.get(2), Some(2).as_ref());
    /// assert_eq!(vector.get(9), Some(2).as_ref());
    pub fn set_all_to(&mut self, value: &T) {
        for entry in self.data.iter_mut() {
            *entry = Some(value.clone());
        }
    }
}
