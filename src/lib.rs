use std::time::{Duration, Instant};

/// Windowed Vector
///
/// A Collection that purges keys based on a fixed TTL.
///
/// Useful for rolling windows and other time based collections/caches.
///
/// We purge old keys on read, rather than on insert.
/// You can specify the duration via `with_duration()`.
/// Add elements with `push` or `push_with_timestamp`.
/// View elements via `iter` and `into_iter`
pub struct WinVec<T>(Vec<(Instant, T)>, Duration);

impl <'a, T> WinVec<T> {
    /// Create a new Windowed Vector with a set duration
    pub fn with_duration(dur: Duration) -> Self {
        WinVec(Vec::new(), dur)
    }

    /// Push an element into the windowed array
    pub fn push(&mut self, el: T) {
        self.0.push((Instant::now(), el));
    }

    /// Push an element with a specified timestamp
    pub fn push_with_timestamp(&mut self, el: T, instant: Instant) {
        self.0.push((instant, el));
    }
}

impl<'a, T: Clone> WinVec<T> {
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
        let vec = &mut self.0;

        let filtered = vec.into_iter().filter(|e| e.0.elapsed() < dur).map(|e| e.clone()).collect();

        self.0 = filtered;
    }

    /// Purges & Returns an Interator of the elements
    pub fn iter(&'a mut self) -> impl 'a + Iterator<Item = T> {
        self.purge();
        self.0.iter().map(|e| e.1.clone())
    }
}

impl<'a, T: Clone> IntoIterator for WinVec<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(mut self) ->  Self::IntoIter {
        self.purge();
        let mapped = self.0.into_iter().map(|e| e.1).collect::<Vec<_>>();
        mapped.into_iter()
    }
}
