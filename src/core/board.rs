use super::bool_mat::*;

pub trait Board: Clone {
	type Index: Copy;

	fn index_to_num(&self, i: Self::Index) -> usize;
	fn num_to_index(&self, n: usize) -> Self::Index;

	fn size(&self) -> usize;
	fn adjacencies(&self) -> &BoolMat;

	fn is_hoshi(&self, i: Self::Index) -> bool;
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct Rect {
	height: usize,
	width: usize,
	graph: Graph,
}

impl Rect {
	fn index_to_num(width: usize, (j, k): (usize, usize)) -> usize {
		j * width + k
	}

	pub fn new(height: usize, width: usize) -> Rect {
		let size = width * height;
		let mut adj = BoolMat::from_diag(&BoolVec::trues(size));
		let index_to_num = |j, k| Rect::index_to_num(width, (j, k));
		for j in 0..height {
			for k in 1..width {
				let a = index_to_num(j, k - 1);
				let b = index_to_num(j, k);
				adj.sym_set(a, b);
			}
		}
		for j in 1..height {
			for k in 0..width {
				let a = index_to_num(j - 1, k);
				let b = index_to_num(j, k);
				adj.sym_set(a, b);
			}
		}
		let graph = Graph { size, adj };
		Rect {
			height,
			width,
			graph,
		}
	}
}

impl Board for Rect {
	type Index = (usize, usize);

	fn index_to_num(&self, i: Self::Index) -> usize {
		Rect::index_to_num(self.width, i)
	}

	fn num_to_index(&self, n: usize) -> Self::Index {
		let j = n / self.width;
		let k = n - j * self.width;
		(j, k)
	}

	fn size(&self) -> usize {
		self.graph.size()
	}

	fn adjacencies(&self) -> &BoolMat {
		self.graph.adjacencies()
	}

	fn is_hoshi(&self, _i: Self::Index) -> bool {
		false
	}
}

#[derive(Clone)]
pub struct Square {
	rect: Rect,
}

impl Square {
	pub fn new(length: usize) -> Square {
		let rect = Rect::new(length, length);
		Square { rect }
	}
}

impl Board for Square {
	type Index = (usize, usize);

	fn index_to_num(&self, i: Self::Index) -> usize {
		self.rect.index_to_num(i)
	}

	fn num_to_index(&self, n: usize) -> Self::Index {
		self.rect.num_to_index(n)
	}

	fn size(&self) -> usize {
		self.rect.size()
	}

	fn adjacencies(&self) -> &BoolMat {
		&self.rect.adjacencies()
	}

	fn is_hoshi(&self, _i: Self::Index) -> bool {
		false
	}
}

// TODO: Implement the three standard boards with their hoshi
