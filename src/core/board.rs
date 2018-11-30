use super::bool_mat::*;

pub trait Board {
	type Index;

	fn index_to_num(&self, i: Self::Index) -> usize;
	fn num_to_index(&self, n: usize) -> Self::Index;

	fn size(&self) -> usize;
	fn adjacencies(&self) -> &BoolMat;

	fn is_hoshi(&self, i: Self::Index) -> bool;
}

pub struct Graph {
	size: usize,
	adj: BoolMat,
}

impl Board for Graph {
	type Index = usize;

	fn index_to_num(&self, n: Self::Index) -> usize {
		n
	}

	fn num_to_index(&self, n: usize) -> Self::Index {
		n
	}

	fn size(&self) -> usize {
		self.size
	}

	fn adjacencies(&self) -> &BoolMat {
		&self.adj
	}

	fn is_hoshi(&self, _i: Self::Index) -> bool {
		false
	}
}
