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
}
