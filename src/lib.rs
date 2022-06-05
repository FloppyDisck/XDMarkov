use std::marker::PhantomData;
use rand::SeedableRng;

/// Easy to use interface that handles the map transformations
pub struct MarkovEngine<M: MapState<T, P, D>, T: Transformation, P: Pos, D: Dir> {
    pub state: M,
    pub rules: Vec<Rule<T>>,
    pos: PhantomData<P>,
    dir: PhantomData<D>
    // TODO: implement random in the update functions
    //pub seed: Box<dyn SeedableRng>
}

impl<M: MapState<T, P, D>, T: Transformation, P: Pos, D: Dir> MarkovEngine<M, T, P, D> {
    /// Initializer since we have phantom types
    pub fn new(state: M, rules: Vec<Rule<T>>) -> Self {
        Self {
            state,
            rules,
            pos: Default::default(),
            dir: Default::default()
        }
    }

    /// Updates the map as if it were going by steps
    pub fn update(&mut self) -> Option<(&Rule<T>, Vec<(P, D)>)> {
        for rule in self.rules.iter_mut() {
            // Skip rule if loops was reached
            if let Some(times) = rule.repeat {
                if times == 0 {
                    continue;
                }
            }
            let result = self.state.update(rule);
            if let Some(result) = result {

                // Tick one rule
                rule.use_repeat();

                return Some((rule, result))
            }
        }

        None
    }

    /// Runs until no rule is satisfied
    pub fn finish(&mut self) {
        loop {
            let result = self.update();
            if result.is_none() {
                break
            }
        }
    }
}


/// Stores the state of the canvas being worked on
pub trait MapState<T: Transformation, P: Pos, D: Dir> {
    /// Tries to update the map state given one rule
    fn update(&mut self, rule: &Rule<T>) -> Option<Vec<(P, D)>> {
        // Get the match result according to rules
        let matching = self.matching(&rule.comp, &rule.rule_type);

        // Process the match is there is some result
        return if let Some(matching) = matching {
            let rule_type = rule.rule_type.clone();
            match rule_type {
                Match::Linear | Match::Random { .. } | Match::AllWithoutConflicts => {
                    // Go through all the matches and replace the item
                    for pos in matching.iter() {
                        self.set(&rule.result, &pos.0, &pos.1)
                    }
                }

                Match::All => {
                    // Use all's set function
                    todo!();
                }
            }

            Some(matching)
        } else {
            None
        }
    }

    /// Rule logic router
    fn matching(&self, pattern: &T, rule_type: &Match) -> Option<Vec<(P, D)>> {
        match rule_type {
            Match::Linear => self.linear_match(pattern),
            Match::Random { tries } => self.random_match(pattern, *tries),
            Match::AllWithoutConflicts => self.match_all_without_conflicts(pattern),
            Match::All => self.match_all(pattern),
        }
    }

    /// Follows a linear "left to right" finding style until it finds one item that satisfied the rule
    fn linear_match(&self, pattern: &T) -> Option<Vec<(P, D)>>;

    /// Randomly picks for `tries` times until it finds an item that satisfies the rule or runs out of tries
    fn random_match(&self, pattern: &T, tries: u64) -> Option<Vec<(P, D)>>;

    /// Matches all items that dont conflict, if conflicting it may pick the "leftmost" item
    fn match_all_without_conflicts(&self, pattern: &T) -> Option<Vec<(P, D)>>;

    /// Matches all items, if there are conflicts then it runs a supplied superposition function on top of the conflicts
    fn match_all(&self, pattern: &T) -> Option<Vec<(P, D)>>;

    /// Sets the pattern, given the direction and position
    fn set(&mut self, pattern: &T, pos: &P, dir: &D);
}

/// Individual Markov rule logic
pub struct Rule<T: Transformation> {
    pub comp: T,
    pub result: T,
    pub rule_type: Match,
    pub repeat: Option<u64>
}

impl<T: Transformation> Rule<T> {
    /// Creates a new rule and verifies that the logic is correct
    pub fn new(comp: T, result: T, rule_type: Match, repeat: Option<u64>) -> Self {
        if !comp.equal_size(&result) {
            // TODO: return a Result instead
            assert!(false, "Items must be of equal size");
        }
        Self {
            comp,
            result,
            rule_type,
            repeat
        }
    }

    /// If the rule has a set amount of repeats then it will try to bring it to 0
    pub fn use_repeat(&mut self) {
        if let Some(times) = self.repeat {
            match times.checked_sub(1) {
                None => {}
                Some(sub) => self.repeat = Some(sub)
            }
        }
    }
}

/// Contains all supported rule-types
#[derive(Clone)]
pub enum Match {
    Linear,
    Random{ tries: u64 },
    AllWithoutConflicts,
    All //TODO: maybe ask for a custom fn containing logic on how to deal with superposition
}

/// Used to replace / compare with parts of MapState
pub trait Transformation {
    /// Verifies that both transformations are of the same size
    /// Used for rules
    fn equal_size(&self, compared: &Self) -> bool;
}

/// Represents the position for MapState
pub trait Pos {}

/// Represents the direction for MapState
pub trait Dir {}