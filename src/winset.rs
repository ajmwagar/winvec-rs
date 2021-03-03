
use std::{
    iter::FromIterator,
    time::{Duration, Instant},
};

use std::collections::HashSet;
use std::hash::Hash;

/// Windowed HashSet
///
/// A Collection that purges keys based on a fixed TTL.
///
/// Useful for rolling windows and other time based collections/caches.
///
/// We purge old keys on read, rather than on insert.
/// You can specify the duration via `with_duration()`.
/// Add elements with `insert` or `insert_with_timestamp`.
/// View elements via `iter` and `into_iter`
#[derive(Clone)]
pub struct WinSet<T>(HashSet<(Instant, T)>, Duration);

impl<'a, T: Eq + Hash> WinSet<T> {
    /// Create a new Windowed HashSet with a set duration
    pub fn with_duration(dur: Duration) -> Self {
        WinSet(HashSet::new(), dur)
    }

    /// insert an element into the windowed array
    pub fn insert(&mut self, el: T) {
        self.0.insert((Instant::now(), el));
    }

    /// insert an element with a specified timestamp
    pub fn insert_with_timestamp(&mut self, el: T, instant: Instant) {
        self.0.insert((instant, el));
    }

    pub fn from_set(set: HashSet<T>, dur: Duration) -> Self {
        let instant = Instant::now();
        let internal_set = set.into_iter().map(|el| (instant, el)).collect::<HashSet<_>>();

        WinSet(internal_set, dur)
    }

    pub fn duration(&self) -> Duration {
        self.1
    }
}

impl<'a, T: Clone + Eq + Hash> WinSet<T> {
    /// Returns the number of elements within the collection.
    /// We purge and then return the new length.
    pub fn len(&mut self) -> usize {
        self.purge();
        self.0.len()
    }

    /// Purge expired entries by calculating elapsed time and filtering values past our specified
    /// duration.
    fn purge(&mut self) {
        let dur = self.1;
        let set = &mut self.0;

        let filtered = set.clone()
            .into_iter()
            .filter(|e| e.0.elapsed() < dur)
            .map(|e| e.clone())
            .collect();

        self.0 = filtered;
    }

    /// Purges & Returns an Interator of the elements
    pub fn iter(&'a mut self) -> impl 'a + Iterator<Item = T> {
        self.purge();
        self.0.iter().map(|e| e.1.clone())
    }
}

impl<'a, T: Clone + Eq + Hash> IntoIterator for WinSet<T> {
    type Item = T;
    type IntoIter = std::collections::hash_set::IntoIter<Self::Item>;

    fn into_iter(mut self) -> Self::IntoIter {
        self.purge();
        let mapped = self.0.into_iter().map(|e| e.1).collect::<HashSet<_>>();
        mapped.into_iter()
    }
}

