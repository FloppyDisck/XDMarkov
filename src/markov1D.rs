use rand::{Rng, SeedableRng, thread_rng};
use rand_chacha::ChaCha8Rng;
use crate::{MapState, Transformation};

pub struct MapState1D<T: Eq> {
    pub state: Vec<T>,
    pub rand: ChaCha8Rng
}

impl<T: Eq> MapState1D<T> {
    pub fn new(state: Vec<T>, seed: Option<u64>) -> Self {
        Self {
            state,
            rand: match seed {
                None => ChaCha8Rng::from_rng(thread_rng()).unwrap(),
                Some(s) => ChaCha8Rng::seed_from_u64(s)
            }
        }
    }

    fn match_pattern(&self, pos: usize, pattern: &Vec<T>) -> bool {
        for n in 0..pattern.len() {
            if self.state[pos+n] != pattern[n] {
                return false
            }
        }
        true
    }
}

impl<T: Eq + Clone> MapState<Transformation1D<T>> for MapState1D<T> {
    type Pos = usize;
    type Dir = Direction1D;

    fn linear_match(&self, pattern: &Transformation1D<T>) -> Option<Vec<(Self::Pos, Self::Dir)>> {
        let item_pattern = pattern.get();

        'state_iter: for i in 0..(self.state.len() + 1 - item_pattern.len()) {
            if !self.match_pattern(i, item_pattern) {
                continue 'state_iter
            }
            return Some(vec![(i, Self::Dir::Right)])
        }

        None
    }

    fn random_match(&mut self, pattern: &Transformation1D<T>, tries: u64) -> Option<Vec<(Self::Pos, Self::Dir)>> {
        let item_pattern = pattern.get();

        let mut tried = 0;
        let max = self.state.len() - item_pattern.len();
        while tried != tries {
            let pos = self.rand.gen_range(0..=max);
            if self.match_pattern(pos, item_pattern) {
                return Some(vec![(pos, Self::Dir::Right)])
            }
            tried += 1;
        }

        None
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
        let map = MapState1D::new(String::from("testing").chars().collect(), None);

        assert_eq!(map.state, vec!['t','e','s','t','i','n','g']);
    }

    #[test]
    fn linear_find() {
        let map = MapState1D::new(String::from("testing").chars().collect(), None);

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
        let map = MapState1D::new(String::from("testing").chars().collect(), None);
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

    #[test]
    fn random_rule() {
        let map = MapState1D::new(String::from("          ").chars().collect(), None);
        let rules = vec![
            Rule::new(
                Transformation1D::new(vec![' ']),
                Transformation1D::new(vec!['o']),
                Match::Random{tries: 100}, None),
        ];

        let mut engine = MarkovEngine::new(map, rules);
        engine.finish();

        let mut found = 0;
        for item in engine.state.state.iter() {
            if item == &'o' {
                found += 1;
            }
        }

        assert_eq!(found, 10);
    }
}