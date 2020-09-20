/// Represents a fuzzy findable element
#[derive(Clone)]
struct FzfElem<T: Eq + Clone, U: Clone> {
    key: Vec<T>,
    index: isize,
    value: U,
}

/// Represents a fuzzy find of elements
pub struct Fzf<T: Eq + Clone, U: Clone> {
    elems: Vec<FzfElem<T, U>>,
}

impl<T: Eq + Clone, U: Clone> FzfElem<T, U> {
    /// Create a new fuzzy findable element
    fn new(key: Vec<T>, value: U) -> FzfElem<T, U> {
        FzfElem {
            key: key,
            index: -1,
            value: value,
        }
    }

    /// Goes to the next element of the fuzzy findable element. Returns false if no such element exists
    fn goto_next(&mut self, next: T) -> bool {
        self.index = self.index + 1;
        while self.index < self.key.len() as isize {
            if self.key[self.index as usize] == next {
                return true;
            }
            self.index = self.index + 1;
        }

        return false;
    }
}

impl<T: Eq + Clone, U: Clone> Fzf<T, U> {
    /// Creates a new fuzzy find
    pub fn new(vec: Vec<(Vec<T>, U)>) -> Fzf<T, U> {
        Fzf {
            elems: vec
                .iter()
                .map(|e| FzfElem::new(e.clone().0, e.clone().1))
                .collect(),
        }
    }

    /// Get the remaining elements of the fuzzy find
    pub fn get_remaining(&self) -> Vec<U> {
        self.elems
            .iter()
            .map(|fzfelem| fzfelem.value.clone())
            .collect()
    }

    /// Give the next element to fuzzy find for. Will return the remaining elements.
    pub fn next(&mut self, t: T) -> Vec<U> {
        let mut new_elems = vec![];
        for e in self.elems.iter_mut() {
            if e.goto_next(t.clone()) {
                new_elems.push(e.clone());
            }
        }

        self.elems = new_elems;

        return self.get_remaining();
    }
}
