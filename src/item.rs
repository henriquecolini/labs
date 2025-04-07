pub trait Store<'a, T: 'a> {
	type Inner;
	fn items(&'a self) -> &'a [Self::Inner];
	fn flatten(&'a self, item: &'a Self::Inner) -> T;
	fn len(&'a self) -> usize {
		self.items().len()
	}
	fn get(&'a self, id: usize) -> T {
		self.items().get(id).map(|it| self.flatten(it)).unwrap()
	}
	fn iter(&'a self) -> impl Iterator<Item = T> {
		(0..self.len()).map(|id| self.get(id))
	}
}
