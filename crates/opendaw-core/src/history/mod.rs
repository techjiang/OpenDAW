#[derive(Debug)]
pub struct History<T> {
    undo: Vec<T>,
    redo: Vec<T>,
    limit: usize,
}

impl<T> History<T> {
    pub fn new(limit: usize) -> Self { Self { undo: Vec::new(), redo: Vec::new(), limit } }
    pub fn can_undo(&self) -> bool { !self.undo.is_empty() }
    pub fn can_redo(&self) -> bool { !self.redo.is_empty() }
    pub fn undo_len(&self) -> usize { self.undo.len() }
    pub fn redo_len(&self) -> usize { self.redo.len() }

    pub fn push(&mut self, state: T) {
        self.undo.push(state);
        self.redo.clear();
        if self.undo.len() > self.limit {
            let excess = self.undo.len() - self.limit;
            self.undo.drain(0..excess);
        }
    }

    pub fn undo(&mut self, current: T) -> Option<T> {
        let previous = self.undo.pop()?;
        self.redo.push(current);
        Some(previous)
    }

    pub fn redo(&mut self, current: T) -> Option<T> {
        let next = self.redo.pop()?;
        self.undo.push(current);
        Some(next)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn undo_redo_round_trip() {
        let mut history = History::new(3);
        history.push(1);
        history.push(2);
        assert_eq!(history.undo(3), Some(2));
        assert_eq!(history.redo(2), Some(3));
    }

    #[test]
    fn limit_keeps_recent_states() {
        let mut history = History::new(2);
        history.push(1);
        history.push(2);
        history.push(3);
        assert_eq!(history.undo_len(), 2);
        assert_eq!(history.undo(4), Some(2));
    }
}
