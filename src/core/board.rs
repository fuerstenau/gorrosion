use super::bool_mat::*;

pub trait Board {
	type Index;

	fn index_to_num(&self, i: Self::Index) -> usize;
	fn num_to_index(&self, n: usize) -> Self::Index;
	fn adjacencies(&self) -> BoolMat;
	fn is_hoshi(&self, i: Self::Index) -> bool;
}
