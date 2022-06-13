use crate::{MapState, Transformation};

pub struct MapState1D<T: Eq> {
    pub state: Vec<T>
}

impl<T: Eq> MapState1D<T> {
    pub fn new(state: Vec<T>) -> Self {
        Self {
            state
        }
    }
}

impl<T: Eq + Clone> MapState<Transformation1D<T>> for MapState1D<T> {
    type Pos = usize;
    type Dir = Direction1D;

    fn linear_match(&self, pattern: &Transformation1D<T>) -> Option<Vec<(Self::Pos, Self::Dir)>> {
        let item_pattern = pattern.get();

        'state_iter: for i in 0..(self.state.len() + 1 - item_pattern.len()) {
            for n in 0..item_pattern.len() {
                if self.state[i+n] != item_pattern[n] {
                    continue 'state_iter
                }
            }
            return Some(vec![(i, Self::Dir::Right)])
        }

        None
    }

    fn random_match(&self, pattern: &Transformation1D<T>, tries: u64) -> Option<Vec<(Self::Pos, Self::Dir)>> {
        todo!()
    }

    fn match_all_without_conflicts(&self, pattern: &Transformation1D<T>) -> Option<Vec<(Self::Pos, Self::Dir)>> {
        todo!()
    }

    fn match_all(&self, pattern: &Transformation1D<T>) -> Option<Vec<(Self::Pos, Self::Dir)>> {
        todo!()
    }

    fn set(&mut self, pattern: &Transformation1D<T>, pos: &Self::Pos, dir: &Self::Dir) {
        // NOTE: Dir not used since its traditional 1D markov
        self.state.splice(*pos..(pos+pattern.item.len()), pattern.item.to_vec());
    }
}

pub struct Transformation1D<T: Eq + Clone> {
    pub item: Vec<T>
}

impl<T: Eq + Clone> Transformation1D<T> {
    pub fn new(item: Vec<T>) -> Self {
        Self {
            item
        }
    }
}

impl<T: Eq + Clone> Transformation for Transformation1D<T> {
    type Item = Vec<T>;

    fn equal_size(&self, compared: &Self) -> bool {
        self.item.len() == compared.item.len()
    }

    fn get(&self) -> &Self::Item {
        &self.item
    }
}

pub enum Direction1D {
    Right
}

#[cfg(test)]
mod test_1d {
    use crate::{MapState, MarkovEngine, Match, Rule};
    use crate::markov1D::{Direction1D, MapState1D, Transformation1D};

    #[test]
    fn init_state() {
        let map = MapState1D::new(String::from("testing").chars().collect());

        assert_eq!(map.state, vec!['t','e','s','t','i','n','g']);
    }

    #[test]
    fn linear_find() {
        let map = MapState1D::new(String::from("testing").chars().collect());

        let find = map.linear_match(&Transformation1D {
            item: vec!['x']
        });

        assert!(find.is_none());

        let find = map.linear_match(&Transformation1D {
            item: vec!['s']
        });

        assert!(find.is_some());
        assert_eq!(2, find.unwrap()[0].0);
    }

    #[test]
    fn linear_rules() {
        let map = MapState1D::new(String::from("testing").chars().collect());
        // TODO: impl macros to make creation these transformation types easier
        let rules = vec![
            Rule::new(
                Transformation1D::new(vec!['t','e']),
                Transformation1D::new(vec![' ', 'H']),
                Match::Linear, None),
            Rule::new(
                Transformation1D::new(vec!['s']),
                Transformation1D::new(vec!['e']),
                Match::Linear, None),
            Rule::new(
                Transformation1D::new(vec!['t']),
                Transformation1D::new(vec!['l']),
                Match::Linear, None),
            Rule::new(
                Transformation1D::new(vec!['i']),
                Transformation1D::new(vec!['l']),
                Match::Linear, None),
            Rule::new(
                Transformation1D::new(vec!['n','g']),
                Transformation1D::new(vec!['o', ' ']),
                Match::Linear, None),
        ];

        let mut engine = MarkovEngine::new(map, rules);
        engine.finish();

        assert_eq!(engine.state.state, [' ', 'H', 'e', 'l', 'l', 'o', ' ']);
    }
}