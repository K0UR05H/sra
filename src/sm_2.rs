//! The SM-2 algorithm
//!
//! SM-2 is a simple and popular spaced repetition algorithm.
//! It takes four inputs and returns the number of days to wait
//! before doing a review again.
//!
//! # Inputs
//!
//! * Repetitions: The number of times the item has been
//! successfully recalled in a row since the last time
//! it was not.
//!
//! * Interval: The inter-repetition interval, which is
//! the number of days to wait before doing a review again
//!
//! * Ease factor: The easiness factor, which determines
//! how quickly the inter-repetition interval grows
//!
//! * [`Quality`]: The quality of the response
//!
//! # Examples
//!
//! Doing a review
//!
//! ```
//! use sra::sm_2::{Quality, SM2};
//!
//! let item = SM2::new().review(Quality::Grade4);
//!
//! assert_eq!(1, item.interval());
//! ```
//!
//! Doing a review with a custom repetition, interval, and ease factor
//!
//! ```
//! use sra::sm_2::{Quality, SM2};
//!
//! let item = SM2::new()
//!     .set_repetitions(7)
//!     .set_interval(12)
//!     .set_ease_factor(2.0)
//!     .review(Quality::Grade5);
//!
//! assert_eq!(24, item.interval());
//! ```
//!
//! Doing multiple reviews
//!
//! ```
//! use sra::sm_2::{Quality, SM2};
//!
//! let item = SM2::new()
//!     .set_ease_factor(2.0)
//!     .review(Quality::Grade5)
//!     .review(Quality::Grade4);
//!
//! assert_eq!(6, item.interval());
//! ```

/// The quality of the response
#[derive(Debug, Copy, Clone)]
pub enum Quality {
    /// perfect response
    Grade5,
    /// correct response after a hesitation
    Grade4,
    /// correct response recalled with serious difficulty
    Grade3,
    /// incorrect response; where the correct one seemed easy to recall
    Grade2,
    /// incorrect response; the correct one remembered
    Grade1,
    /// complete blackout
    Grade0,
}

impl Quality {
    fn is_correct(&self) -> bool {
        matches!(self, Quality::Grade3 | Quality::Grade4 | Quality::Grade5)
    }
}

/// An implementation of the SM-2 algorithm
#[derive(Debug, Copy, Clone)]
pub struct SM2 {
    /// The number of times the item has been successfully recalled
    repetitions: usize,
    /// The number of days to wait before doing a review again
    interval: usize,
    /// The easiness of memorizing and retaining the item
    ease_factor: f32,
}

impl SM2 {
    /// Creates a new instance of the SM-2 algorithm
    pub fn new() -> Self {
        SM2 {
            repetitions: 0,
            interval: 0,
            ease_factor: 2.5,
        }
    }

    /// Returns the number of successful recalls in a row
    pub fn repetitions(&self) -> usize {
        self.repetitions
    }

    /// Sets the number of successful recalls in a row
    ///
    /// # Examples
    ///
    /// Doing a review with a repetition of 10
    ///
    /// ```
    /// use sra::sm_2::{Quality, SM2};
    /// let item = SM2::new().set_repetitions(10).review(Quality::Grade4);
    /// ```
    pub fn set_repetitions(mut self, reviews: usize) -> Self {
        self.repetitions = reviews;
        self
    }

    /// Returns the number of days to wait before doing a review again
    pub fn interval(&self) -> usize {
        self.interval
    }

    /// Sets the number of days to wait before doing a review again
    ///
    /// # Examples
    ///
    /// Doing a review with an interval of 5 days
    ///
    /// ```
    /// use sra::sm_2::{Quality, SM2};
    /// let item = SM2::new().set_interval(5).review(Quality::Grade4);
    /// ```
    pub fn set_interval(mut self, days: usize) -> Self {
        self.interval = days;
        self
    }

    /// Returns the easiness of memorizing and retaining the item
    pub fn ease_factor(&self) -> f32 {
        self.ease_factor
    }

    /// Sets the easiness of memorizing and retaining the item
    ///
    /// # Panics
    ///
    /// This method will panics if `easiness` is less than 1.3
    ///
    /// # Examples
    ///
    /// Doing a review with an ease factor of 2.1
    ///
    /// ```
    /// use sra::sm_2::{Quality, SM2};
    /// let item = SM2::new().set_ease_factor(2.1).review(Quality::Grade4);
    /// ```
    pub fn set_ease_factor(mut self, easiness: f32) -> Self {
        if easiness < 1.3 {
            panic!("ease factor must be greater than or equal to 1.3");
        }

        self.ease_factor = easiness;
        self
    }

    /// Updates the repetitions, the interval, and the ease factor based on the quality of the response
    ///
    /// # Examples
    ///
    /// ```
    /// use sra::sm_2::{Quality, SM2};
    ///
    /// let item = SM2::new()
    ///     .set_repetitions(3)
    ///     .set_interval(5)
    ///     .set_ease_factor(2.0)
    ///     .review(Quality::Grade3);
    ///
    /// assert_eq!(4, item.repetitions());
    /// assert_eq!(10, item.interval());
    /// assert_eq!(1.86, item.ease_factor());
    /// ```
    pub fn review(mut self, quality: Quality) -> Self {
        self.update_interval(&quality);
        self.update_ease_factor(&quality);
        self.update_repetitions(&quality);
        self
    }

    fn update_interval(&mut self, quality: &Quality) {
        self.interval = if quality.is_correct() {
            match self.repetitions {
                0 => 1,
                1 => 6,
                _ => ((self.interval as f32) * self.ease_factor) as usize,
            }
        } else {
            1
        }
    }

    fn update_ease_factor(&mut self, quality: &Quality) {
        if quality.is_correct() {
            let quality = (*quality as u8) as f32;
            self.ease_factor =
                (self.ease_factor + (0.1 - quality * (0.08 + quality * 0.02))).max(1.3);
        }
    }

    fn update_repetitions(&mut self, quality: &Quality) {
        if quality.is_correct() {
            self.repetitions += 1;
        } else {
            self.repetitions = 0;
        }
    }
}

impl Default for SM2 {
    fn default() -> Self {
        SM2::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_review(expected: (usize, usize, f32), input: (usize, usize, f32, Quality)) {
        let sm2 = SM2::new()
            .set_repetitions(input.0)
            .set_interval(input.1)
            .set_ease_factor(input.2)
            .review(input.3);

        assert_eq!(expected.0, sm2.repetitions());
        assert_eq!(expected.1, sm2.interval());
        assert_eq!(expected.2, sm2.ease_factor());
    }

    #[test]
    fn new() {
        let sm2 = SM2::new();

        assert_eq!(0, sm2.repetitions());
        assert_eq!(0, sm2.interval());
        assert_eq!(2.5, sm2.ease_factor());
    }

    #[test]
    #[should_panic(expected = "ease factor must be greater than or equal to 1.3")]
    fn ease_factor_out_of_range() {
        SM2::new().set_ease_factor(1.2);
    }

    #[test]
    fn correct_review() {
        assert_review((1, 1, 2.6), (0, 0, 2.5, Quality::Grade5));
        assert_review((1, 1, 2.5), (0, 0, 2.5, Quality::Grade4));
        assert_review((1, 1, 2.3600001), (0, 0, 2.5, Quality::Grade3));
    }

    #[test]
    fn second_review() {
        assert_review((2, 6, 2.6), (1, 0, 2.5, Quality::Grade5));
    }

    #[test]
    fn subsequent_reviews() {
        assert_review((3, 7, 2.6), (2, 3, 2.5, Quality::Grade5));
    }

    #[test]
    fn review_ease_factor_less_than_1_3() {
        assert_review((1, 1, 1.3), (0, 0, 1.3, Quality::Grade3));
    }

    #[test]
    fn incorrect_review() {
        assert_review((0, 1, 2.5), (3, 5, 2.5, Quality::Grade2));
    }
}
