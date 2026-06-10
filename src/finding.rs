//! PII finding structs and overlap resolution.

/// A single detected PII span within a text.
#[derive(Debug, Clone)]
pub struct Finding {
    /// Entity type name (e.g. `"CREDIT_CARD"`).
    pub entity: String,
    /// ISO-3166-1 alpha-2 country code, or `None` for global entities.
    pub country: Option<String>,
    /// Byte offset of the start of the match in the original text.
    pub start: usize,
    /// Byte offset one past the end of the match.
    pub end: usize,
    /// Confidence score in 0.0..=1.0.
    pub score: f32,
    /// Index into the `Engine`'s recognizer list that produced this finding.
    pub recognizer_idx: usize,
}

impl Finding {
    /// Length of the matched span in bytes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    /// Returns `true` if the span is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
}

/// Resolve overlapping findings by keeping the highest-scoring, longest, or
/// lexicographically-first entity for each non-overlapping byte region.
///
/// Sort order: start asc, score desc, len desc, entity asc. Greedy: keep a
/// finding only when its start >= the end of the last kept finding.
#[must_use]
pub fn resolve_overlaps(mut hits: Vec<Finding>) -> Vec<Finding> {
    hits.sort_unstable_by(|a, b| {
        a.start
            .cmp(&b.start)
            .then_with(|| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal))
            .then_with(|| b.len().cmp(&a.len()))
            .then_with(|| a.entity.cmp(&b.entity))
    });
    let mut result: Vec<Finding> = Vec::with_capacity(hits.len());
    let mut last_end: usize = 0;
    for hit in hits {
        if hit.start >= last_end {
            last_end = hit.end;
            result.push(hit);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn f(entity: &str, start: usize, end: usize, score: f32) -> Finding {
        Finding { entity: entity.to_string(), country: None, start, end, score, recognizer_idx: 0 }
    }

    #[test]
    fn non_overlapping_both_kept() {
        let hits = vec![f("A", 0, 5, 0.9), f("B", 6, 10, 0.8)];
        let result = resolve_overlaps(hits);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn higher_score_wins() {
        let hits = vec![f("A", 0, 10, 0.5), f("B", 0, 10, 0.9)];
        let result = resolve_overlaps(hits);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].entity, "B");
    }

    #[test]
    fn adjacent_spans_both_kept() {
        let hits = vec![f("A", 0, 5, 0.9), f("B", 5, 10, 0.9)];
        let result = resolve_overlaps(hits);
        assert_eq!(result.len(), 2);
    }
}
