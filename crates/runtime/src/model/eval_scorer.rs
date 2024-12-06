/*
Copyright 2024 The Spice.ai OSS Authors

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

     https://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use std::sync::Arc;

use super::eval::{DatasetInput, DatasetOutput};
use async_trait::async_trait;

#[async_trait]
pub trait Scorer: Sync + Send {
    async fn score(
        &self,
        input: &DatasetInput,
        actual: &DatasetOutput,
        ideal: &DatasetOutput,
    ) -> f32;

    /// Compute the relevant metrics for this [`Scorer`], given a precomputed scores.
    fn metrics(&self, scores: &[f32]) -> Vec<(String, f32)>;
}

/// [`MatchScorer`] checks for equality between the actual and ideal values.
///
/// The score is an exact match, but it only compare the less detailed of the two [`DatasetOutput`]. ([`DatasetOutput::AssistantResponse`] < [`DatasetOutput::Messages`]).
pub struct MatchScorer {}

#[async_trait]
impl Scorer for MatchScorer {
    async fn score(
        &self,
        _input: &DatasetInput,
        actual: &DatasetOutput,
        ideal: &DatasetOutput,
    ) -> f32 {
        let is_equal = match (actual, ideal) {
            (DatasetOutput::AssistantResponse(a), DatasetOutput::AssistantResponse(b)) => *a == *b,
            (DatasetOutput::Choices(a), DatasetOutput::Choices(b)) => a == b,
            (DatasetOutput::AssistantResponse(a), DatasetOutput::Choices(m))
            | (DatasetOutput::Choices(m), DatasetOutput::AssistantResponse(a)) => {
                let b = m
                    .first()
                    .map(|m| m.message.content.clone().unwrap_or_default())
                    .unwrap_or_default();
                *a == b
            }
        };
        if is_equal {
            1.0_f32
        } else {
            0.0_f32
        }
    }

    #[allow(clippy::cast_precision_loss)]
    fn metrics(&self, scores: &[f32]) -> Vec<(String, f32)> {
        let n = scores.len();
        if n == 0 {
            // Return default metrics for empty input
            return vec![("mean".to_string(), 0.0), ("std_dev".to_string(), 0.0)];
        }

        let sum: f32 = scores.iter().copied().sum();
        let mean = sum / n as f32;

        vec![
            ("mean".to_string(), mean),
            // For  Bernoulli r.v., the variance is p(1-p).
            ("std_dev".to_string(), (mean * (1.0 - mean)).sqrt()),
        ]
    }
}

#[must_use]
pub fn builtin_scorer() -> Vec<(&'static str, Arc<dyn Scorer>)> {
    vec![("match", Arc::new(MatchScorer {}))]
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_score_assistant_response_match() {
        assert!(
            (MatchScorer {}
                .score(
                    &DatasetInput::UserInput("What is your name?".to_string()),
                    &DatasetOutput::from_raw("Hello"),
                    &DatasetOutput::from_raw("Hello")
                )
                .await
                - 1.0_f32)
                .abs()
                < f32::EPSILON
        );
    }

    #[tokio::test]
    async fn test_score_assistant_response_mismatch() {
        assert!(
            MatchScorer {}
                .score(
                    &DatasetInput::UserInput("What is your name?".to_string()),
                    &DatasetOutput::from_raw("Hello"),
                    &DatasetOutput::from_raw("Hi")
                )
                .await
                < f32::EPSILON
        );
    }

    #[tokio::test]
    async fn test_score_choices_match() {
        assert!(
            (MatchScorer {}
                .score(
                    &DatasetInput::Messages(vec![]),
                    &DatasetOutput::try_from_value(json!([
                        {
                            "index": 0,
                            "message": {
                                "role": "assistant",
                                "content": "Hello"
                            }
                        }
                    ]))
                    .expect("Failed to parse actual DatasetOutput")
                    .expect("Failed to parse ideal DatasetOutput"),
                    &DatasetOutput::try_from_value(json!([
                        {
                            "index": 0,
                            "message": {
                                "role": "assistant",
                                "content": "Hello"
                            }
                        }
                    ]))
                    .expect("Failed to parse ideal DatasetOutput")
                    .expect("Failed to parse ideal DatasetOutput")
                )
                .await
                - 1.0_f32)
                .abs()
                < f32::EPSILON
        );
    }

    #[tokio::test]
    async fn test_score_choices_mismatch() {
        assert!(
            MatchScorer {}
                .score(
                    &DatasetInput::Messages(vec![]),
                    &DatasetOutput::try_from_value(json!([
                        {
                            "index": 0,
                            "message": {
                                "role": "assistant",
                                "content": "Hello"
                            }
                        }
                    ]))
                    .expect("Failed to parse actual DatasetOutput")
                    .expect("Failed to parse ideal DatasetOutput"),
                    &DatasetOutput::try_from_value(json!([
                        {
                            "index": 0,
                            "message": {
                                "role": "assistant",
                                "content": "Hi"
                            }
                        }
                    ]))
                    .expect("Failed to parse ideal DatasetOutput")
                    .expect("Failed to parse ideal DatasetOutput")
                )
                .await
                < f32::EPSILON
        );
    }

    #[tokio::test]
    async fn test_score_mixed_match() {
        assert!(
            (MatchScorer {}
                .score(
                    &DatasetInput::UserInput("What is your name?".to_string()),
                    &DatasetOutput::from_raw("Hello"),
                    &DatasetOutput::try_from_value(json!([
                        {
                            "index": 0,
                            "message": {
                                "role": "assistant",
                                "content": "Hello"
                            }
                        }
                    ]))
                    .expect("Failed to parse ideal DatasetOutput")
                    .expect("Failed to parse ideal DatasetOutput")
                )
                .await
                - 1.0_f32)
                .abs()
                < f32::EPSILON
        );
    }

    #[tokio::test]
    async fn test_score_mixed_mismatch() {
        assert!(
            MatchScorer {}
                .score(
                    &DatasetInput::UserInput("What is your name?".to_string()),
                    &DatasetOutput::from_raw("Hi"),
                    &DatasetOutput::try_from_value(json!([
                        {
                            "index": 0,
                            "message": {
                                "role": "assistant",
                                "content": "Hello"
                            }
                        }
                    ]))
                    .expect("Failed to parse ideal DatasetOutput")
                    .expect("Failed to parse ideal DatasetOutput")
                )
                .await
                < f32::EPSILON
        );
    }

    #[test]
    fn test_metrics_non_empty_scores() {
        let stats = MatchScorer {}.metrics(&[1.0, 0.0, 1.0, 1.0]);
        assert_eq!(stats.len(), 2);
        assert_eq!(stats[0].0, "mean".to_string());
        assert!((stats[0].1 - 0.75_f32).abs() < f32::EPSILON);
        assert_eq!(stats[1].0, "std_dev".to_string());
        assert!((stats[1].1 - 0.433_012_7_f32).abs() < f32::EPSILON);
    }
}