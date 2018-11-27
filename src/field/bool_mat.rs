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

	fn set(&mut self, i: usize, j: usize) {
		self.contents[i * self.width + j] = true;
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
}
