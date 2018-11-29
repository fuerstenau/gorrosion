#[derive(Debug, Eq, PartialEq)]
pub struct BoolMat {
	height: usize,
	width: usize,
	contents: Vec<bool>,
}

pub struct BoolVec {
	mat: BoolMat,
}

impl BoolMat {
	pub fn falses(height: usize, width: usize) -> BoolMat {
		let contents = vec![false; height * width];
		BoolMat {
			height,
			width,
			contents,
		}
	}

	pub fn trues(height: usize, width: usize) -> BoolMat {
		let contents = vec![true; height * width];
		BoolMat {
			height,
			width,
			contents,
		}
	}

	pub fn from_diag(diag: BoolVec) -> BoolMat {
		let len = diag.len();
		let mut res = BoolMat::falses(len, len);
		for i in diag.set_positions() {
			res.set(i, i);
		}
		res
	}

	pub fn set(&mut self, i: usize, k: usize) {
		self.contents[i * self.width + k] = true;
	}

	pub fn sym_set(&mut self, i: usize, k: usize) {
		self.set(i, k);
		self.set(k, i);
	}

	pub fn get(&self, i: usize, k: usize) -> bool {
		self.contents[i * self.width + k]
	}

	fn mult_get(a: &BoolMat, b: &BoolMat, i: usize, k: usize) -> bool {
		assert_eq!(a.width, b.height);
		let len = a.width;
		(0..len).map(|n| a.get(i, n) & b.get(n, k)).any(|n| n)
	}

	pub fn mult(a: &BoolMat, b: &BoolMat) -> BoolMat {
		assert_eq!(a.width, b.height);
		let height = a.height;
		let width = b.width;
		let mut contents = Vec::with_capacity(height * width);
		for i in 0..height {
			for k in 0..width {
				contents.push(BoolMat::mult_get(a, b, i, k));
			}
		}
		BoolMat {
			height,
			width,
			contents,
		}
	}

	pub fn eval(&self, v: &BoolVec) -> BoolVec {
		assert_eq!(self.width, v.len());
		let mat = BoolMat::mult(self, &v.mat);
		BoolVec { mat }
	}
}

impl BoolVec {
	pub fn falses(size: usize) -> BoolVec {
		let mat = BoolMat::falses(size, 1);
		BoolVec { mat }
	}

	pub fn trues(size: usize) -> BoolVec {
		let mat = BoolMat::trues(size, 1);
		BoolVec { mat }
	}

	fn len(&self) -> usize {
		self.mat.height
	}

	pub fn get(&self, i: usize) -> bool {
		self.mat.get(i, 1)
	}

	fn set_positions(&self) -> impl Iterator<Item = usize> {
		let len = self.len();
		let all_indices = 0..len;
		let res: Vec<usize> =
			all_indices.filter(|n| self.get(*n)).collect();
		res.into_iter()
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn false_mat() {
		use super::BoolMat;
		assert_eq!(
			BoolMat::falses(3, 2),
			BoolMat {
				height: 3,
				width: 2,
				contents: vec![false; 6]
			}
		);
	}

	#[test]
	fn set() {
		use super::BoolMat;
		let mut bm = BoolMat::falses(1, 3);
		bm.set(0, 1);
		assert_eq!(
			bm,
			BoolMat {
				height: 1,
				width: 3,
				contents: vec![false, true, false]
			}
		);
	}

	#[test]
	fn mult() {
		use super::BoolMat;
		let mut a = BoolMat::falses(1, 2);
		let mut b = BoolMat::falses(2, 3);
		a.set(0, 0);
		b.set(0, 1);
		b.set(1, 0);
		b.set(1, 2);
		assert_eq!(
			BoolMat::mult(&a, &b),
			BoolMat {
				height: 1,
				width: 3,
				contents: vec![false, true, false]
			}
		)
	}
}
