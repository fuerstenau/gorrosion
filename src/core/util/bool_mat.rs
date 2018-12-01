use core::util::bool_vec::BoolVec;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct BoolMat {
	height: usize,
	width: usize,
	contents: BoolVec,
}

impl BoolMat {
	pub fn falses(height: usize, width: usize) -> BoolMat {
		let contents = BoolVec::falses(height * width);
		BoolMat {
			height,
			width,
			contents,
		}
	}

	pub fn trues(height: usize, width: usize) -> BoolMat {
		let contents = BoolVec::trues(height * width);
		BoolMat {
			height,
			width,
			contents,
		}
	}

	pub fn from_diag(diag: &BoolVec) -> BoolMat {
		let len = diag.len();
		let mut res = BoolMat::falses(len, len);
		for i in diag.set_positions() {
			res.set(i, i);
		}
		res
	}

	fn column(contents: &BoolVec) -> BoolMat {
		let contents = contents.clone();
		let height = contents.len();
		let width = 1;
		BoolMat { height, width, contents }
	}

	pub fn set(&mut self, i: usize, k: usize) {
		self.contents.set(i * self.width + k);
	}

	pub fn sym_set(&mut self, i: usize, k: usize) {
		self.set(i, k);
		self.set(k, i);
	}

	pub fn get(&self, i: usize, k: usize) -> bool {
		self.contents.get(i * self.width + k)
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
		let mut data = Vec::with_capacity(height * width);
		for i in 0..height {
			for k in 0..width {
				data.push(BoolMat::mult_get(a, b, i, k));
			}
		}
		let contents = BoolVec::from(data);
		BoolMat {
			height,
			width,
			contents,
		}
	}

	pub fn eval(&self, v: &BoolVec) -> BoolVec {
		assert_eq!(self.width, v.len());
		let mat = BoolMat::mult(self, &BoolMat::column(v));
		mat.contents
	}
}
