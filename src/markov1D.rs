use crate::{Dir, MapState, Transformation};

pub struct MapState1D<T: Eq> {
    pub state: Vec<T>
}

impl MapState<Transformation1D<T>, Vec<u64>, Direction1D, T> for MapState1D<T> {
    fn linear_match(&self, pattern: &T) -> Option<Vec<(P, D)>> {
        todo!()
    }

    fn random_match(&self, pattern: &T, tries: u64) -> Option<Vec<(P, D)>> {
        todo!()
    }

    fn match_all_without_conflicts(&self, pattern: &T) -> Option<Vec<(P, D)>> {
        todo!()
    }

    fn match_all(&self, pattern: &T) -> Option<Vec<(P, D)>> {
        todo!()
    }

    fn set(&mut self, pattern: &T, pos: &P, dir: &D) {
        todo!()
    }
}

pub struct Transformation1D<T: Eq> {
    pub item: Vec<T>
}

impl<T: Eq> Transformation<T> for Transformation1D<T> {
    fn equal_size(&self, compared: &Self) -> bool {
        self.item == compared.item
    }

    fn get(&self) -> &T {
        &self.item
    }
}

pub enum Direction1D {
    Left,
    Right
}