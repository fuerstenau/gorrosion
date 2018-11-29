#[derive(Debug, Eq, PartialEq)]
struct BoolMat {
	height: usize,
	width: usize,
	contents: Vec<bool>,
}

impl BoolMat {
	fn new(height: usize, width: usize) -> BoolMat {
		let contents = vec![false; height * width];
		BoolMat {
			height,
			width,
			contents,
		}
	}

	fn set(&mut self, i: usize, k: usize) {
		self.contents[i * self.width + k] = true;
	}

	fn get(&self, i: usize, k: usize) -> bool {
		self.contents[i * self.width + k]
	}

	fn mult_get(A: &BoolMat, B: &BoolMat, i: usize, k: usize) -> bool {
		assert_eq!(A.width, B.height);
		let len = A.width;
		(0..len).map(|n| A.get(i, n) & B.get(n, k))
			.fold(false, |a, b| a | b)
	}

	fn mult(A: &BoolMat, B: &BoolMat) -> BoolMat {
		assert_eq!(A.width, B.height);
		let height = A.height;
		let width = B.width;
		let mut contents = Vec::with_capacity(height * width);
		for i in 0..height {
			for k in 0..width {
				contents.push(BoolMat::mult_get(A, B, i, k));
			}
		}
		BoolMat {
			height,
			width,
			contents,
		}
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn simple_mat() {
		use super::BoolMat;
		assert_eq!(
			BoolMat::new(3, 2),
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
		let mut bm = BoolMat::new(1, 3);
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
		let mut A = BoolMat::new(1, 2);
		let mut B = BoolMat::new(2, 3);
		A.set(0, 0);
		B.set(0, 1);
		B.set(1, 0);
		B.set(1, 2);
		assert_eq!(
			BoolMat::mult(&A, &B),
			BoolMat {
				height: 1,
				width: 3,
				contents: vec![false, true, false]
			}
		)
	}
}
