#[derive(Clone, Eq, PartialEq, Debug)]
pub struct BoolVec {
	data: Vec<bool>,
}

// TODO: There is some renaming to be done.
// TODO: Implement a Set trait.
impl BoolVec {
	/// Create a new boolean vector with all positions being unset.
	pub fn falses(size: usize) -> BoolVec {
		let data = vec![false; size];
		BoolVec { data }
	}

	/// Create a new boolean vector with all positions being set.
	pub fn trues(size: usize) -> BoolVec {
		let data = vec![true; size];
		BoolVec { data }
	}

	/// This is deprecated.
	pub fn len(&self) -> usize {
		self.data.len()
	}

	/// Retrieve an entry.
	pub fn get(&self, i: usize) -> bool {
		self.data[i]
	}

	/// Set an entry (to “true”).
	pub fn set(&mut self, i: usize) {
		self.data[i] = true;
	}

	/// Get an iterator over all set / true positions.
	pub fn set_positions(&self) -> impl Iterator<Item = usize> {
		let len = self.len();
		let all_indices = 0..len;
		let res: Vec<usize> =
			all_indices.filter(|n| self.get(*n)).collect();
		res.into_iter()
	}

	/// Intersect two vectors considered as sets.
	pub fn intersection(a: &BoolVec, b: &BoolVec) -> BoolVec {
		assert_eq!(a.len(), b.len());
		let len = a.len();
		let mut res = BoolVec::falses(len);
		for i in 0..len {
			if a.get(i) & b.get(i) {
				res.set(i);
			}
		}
		res
	}

	/// Unite two vectors considered as sets.
	pub fn union(a: &BoolVec, b: &BoolVec) -> BoolVec {
		assert_eq!(a.len(), b.len());
		let len = a.len();
		let mut res = BoolVec::falses(len);
		for i in 0..len {
			if a.get(i) | b.get(i) {
				res.set(i);
			}
		}
		res
	}

	/// Take the complement of a vector
	/// considered as subset of the all-true vector of the same length.
	pub fn complement(&self) -> BoolVec {
		let len = self.len();
		let mut res = BoolVec::falses(len);
		for i in 0..len {
			if !self.get(i) {
				res.set(i);
			}
		}
		res
	}
}

impl From<Vec<bool>> for BoolVec {
	fn from(data: Vec<bool>) -> BoolVec {
		BoolVec { data }
	}
}
