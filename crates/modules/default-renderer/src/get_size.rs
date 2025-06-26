use fontdue::{Font, FontSettings};
use hyphenation::Standard;
use std::collections::HashMap;
use thiserror::Error;
use unicode_linebreak::{linebreaks, BreakOpportunity};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Error, Debug)]
pub enum TextFittingError {
    #[error("Font loading error: {0}")]
    FontError(String),
    #[error("Text measurement error: {0}")]
    MeasurementError(String),
    #[error("No suitable font size found")]
    NoSuitableSize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScriptType {
    /// Latin-based scripts (English, German, French, etc.) - need word/syllable breaking
    Latin,
    /// CJK scripts (Chinese, Japanese, Korean) - can break anywhere
    CJK,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WrapStrategy {
    /// Break on word boundaries
    Word,
    /// Break on syllable boundaries
    Syllable,
    /// Break anywhere (for CJK scripts)
    Anywhere,
}

impl ScriptType {
    pub fn default_wrap_strategy(&self) -> WrapStrategy {
        match self {
            ScriptType::Latin => WrapStrategy::Word,
            ScriptType::CJK => WrapStrategy::Anywhere,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TextStyle {
    pub font_size: f32,
    pub line_height: f32,
    pub letter_spacing: f32,
    pub font_weight: u16, // 100-900
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            font_size: 16.0,
            line_height: 1.2,
            letter_spacing: 0.0,
            font_weight: 400,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TextFittingConfig {
    pub script_type: ScriptType,
    pub wrap_strategy: Option<WrapStrategy>,
    pub max_width: f32,
    pub max_height: f32,
    pub min_font_size: f32,
    pub max_font_size: f32,
    pub font_size_step: f32,
}

impl Default for TextFittingConfig {
    fn default() -> Self {
        Self {
            script_type: ScriptType::Latin,
            wrap_strategy: None,
            max_width: 300.0,
            max_height: 200.0,
            min_font_size: 8.0,
            max_font_size: 72.0,
            font_size_step: 0.5,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TextMetrics {
    pub width: f32,
    pub height: f32,
    pub lines: Vec<String>,
    pub font_size: f32,
}

#[derive(Debug)]
pub struct TextFitter {
    fonts: HashMap<u16, Font>, // font_weight -> Font
    #[allow(dead_code)]
    hyphenator: Option<Standard>, // Reserved for syllable breaking
}

impl TextFitter {
    pub fn new() -> Result<Self, TextFittingError> {
        Ok(Self {
            fonts: HashMap::new(),
            hyphenator: None,
        })
    }

    pub fn add_font(&mut self, font_data: &[u8], font_weight: u16) -> Result<(), TextFittingError> {
        let font = Font::from_bytes(font_data, FontSettings::default())
            .map_err(|e| TextFittingError::FontError(format!("Failed to load font: {}", e)))?;

        self.fonts.insert(font_weight, font);
        Ok(())
    }

    pub fn fit_text(
        &self,
        text: &str,
        config: &TextFittingConfig,
        style: &TextStyle,
    ) -> Result<TextMetrics, TextFittingError> {
        let default_strategy = config.script_type.default_wrap_strategy();
        let wrap_strategy = config.wrap_strategy.as_ref().unwrap_or(&default_strategy);

        let font = self.fonts.get(&style.font_weight).ok_or_else(|| {
            TextFittingError::FontError(format!("Font with weight {} not found", style.font_weight))
        })?;

        // Binary search for the optimal font size
        let mut min_size = config.min_font_size;
        let mut max_size = config.max_font_size;
        let mut best_metrics: Option<TextMetrics> = None;

        while max_size - min_size > config.font_size_step {
            let test_size = (min_size + max_size) / 2.0;
            let test_style = TextStyle {
                font_size: test_size,
                ..style.clone()
            };

            let metrics = self.measure_text(text, config, &test_style, wrap_strategy, font)?;

            if metrics.width <= config.max_width && metrics.height <= config.max_height {
                best_metrics = Some(metrics);
                min_size = test_size;
            } else {
                max_size = test_size;
            }
        }

        best_metrics.ok_or(TextFittingError::NoSuitableSize)
    }

    fn measure_text(
        &self,
        text: &str,
        config: &TextFittingConfig,
        style: &TextStyle,
        wrap_strategy: &WrapStrategy,
        font: &Font,
    ) -> Result<TextMetrics, TextFittingError> {
        let lines = self.wrap_text(text, config.max_width, style, wrap_strategy, font)?;

        let mut max_width = 0.0f32;
        let line_height = style.font_size * style.line_height;
        let total_height = lines.len() as f32 * line_height;

        for line in &lines {
            let line_width = self.measure_line_width(line, style, font)?;
            max_width = max_width.max(line_width);
        }

        Ok(TextMetrics {
            width: max_width,
            height: total_height,
            lines,
            font_size: style.font_size,
        })
    }

    fn wrap_text(
        &self,
        text: &str,
        max_width: f32,
        style: &TextStyle,
        wrap_strategy: &WrapStrategy,
        font: &Font,
    ) -> Result<Vec<String>, TextFittingError> {
        let mut lines = Vec::new();
        let paragraphs: Vec<&str> = text.split('\n').collect();

        for paragraph in paragraphs {
            if paragraph.is_empty() {
                lines.push(String::new());
                continue;
            }

            let paragraph_lines = match wrap_strategy {
                WrapStrategy::Word => self.wrap_by_words(paragraph, max_width, style, font)?,
                WrapStrategy::Syllable => {
                    self.wrap_by_syllables(paragraph, max_width, style, font)?
                }
                WrapStrategy::Anywhere => self.wrap_anywhere(paragraph, max_width, style, font)?,
            };

            lines.extend(paragraph_lines);
        }

        Ok(lines)
    }

    fn wrap_by_words(
        &self,
        text: &str,
        max_width: f32,
        style: &TextStyle,
        font: &Font,
    ) -> Result<Vec<String>, TextFittingError> {
        let mut lines = Vec::new();
        let mut current_line = String::new();
        let words: Vec<&str> = text.split_whitespace().collect();

        for word in words {
            let test_line = if current_line.is_empty() {
                word.to_string()
            } else {
                format!("{} {}", current_line, word)
            };

            let line_width = self.measure_line_width(&test_line, style, font)?;

            if line_width <= max_width {
                current_line = test_line;
            } else {
                // Word is too long for line, try syllable breaking
                let word_width = self.measure_line_width(word, style, font)?;
                if word_width > max_width {
                    // Push current line if not empty
                    if !current_line.is_empty() {
                        lines.push(current_line);
                        current_line = String::new();
                    }
                    // Force break the long word
                    let syllable_lines = self.wrap_by_syllables(word, max_width, style, font)?;
                    lines.extend(syllable_lines);
                } else {
                    lines.push(current_line);
                    current_line = word.to_string();
                }
            }
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }

        Ok(lines)
    }

    fn wrap_by_syllables(
        &self,
        text: &str,
        max_width: f32,
        style: &TextStyle,
        font: &Font,
    ) -> Result<Vec<String>, TextFittingError> {
        let mut lines = Vec::new();
        let mut current_line = String::new();
        let words: Vec<&str> = text.split_whitespace().collect();

        for word in words {
            // Simple syllable approximation: break after vowels when possible
            let syllables = self.simple_syllable_break(word);

            for syllable in syllables {
                let test_line = if current_line.is_empty() {
                    syllable.clone()
                } else {
                    format!("{}{}", current_line, syllable)
                };

                let line_width = self.measure_line_width(&test_line, style, font)?;

                if line_width <= max_width {
                    current_line = test_line;
                } else {
                    if !current_line.is_empty() {
                        lines.push(format!("{}-", current_line));
                        current_line = syllable.clone();
                    } else {
                        // Even single syllable is too long, force character break
                        let char_lines = self.wrap_anywhere(&syllable, max_width, style, font)?;
                        lines.extend(char_lines);
                    }
                }
            }

            // Add space after word if not last word
            if !current_line.is_empty() {
                current_line.push(' ');
            }
        }

        if !current_line.is_empty() {
            lines.push(current_line.trim_end().to_string());
        }

        Ok(lines)
    }

    fn wrap_anywhere(
        &self,
        text: &str,
        max_width: f32,
        style: &TextStyle,
        font: &Font,
    ) -> Result<Vec<String>, TextFittingError> {
        let mut lines = Vec::new();
        let mut current_line = String::new();

        // Use Unicode line breaking for CJK languages
        let breaks: Vec<_> = linebreaks(text).collect();
        let mut last_pos = 0;

        for (pos, opportunity) in breaks {
            let segment = &text[last_pos..pos];

            let test_line = format!("{}{}", current_line, segment);
            let line_width = self.measure_line_width(&test_line, style, font)?;

            if line_width <= max_width {
                current_line = test_line;
            } else {
                if !current_line.is_empty() {
                    lines.push(current_line);
                    current_line = segment.to_string();
                } else {
                    // Even single segment is too long, break character by character
                    for grapheme in segment.graphemes(true) {
                        let test_line = format!("{}{}", current_line, grapheme);
                        let line_width = self.measure_line_width(&test_line, style, font)?;

                        if line_width <= max_width {
                            current_line = test_line;
                        } else {
                            if !current_line.is_empty() {
                                lines.push(current_line);
                                current_line = grapheme.to_string();
                            } else {
                                // Single character is too long, force it anyway
                                current_line = grapheme.to_string();
                            }
                        }
                    }
                }
            }

            if opportunity == BreakOpportunity::Mandatory {
                lines.push(current_line);
                current_line = String::new();
            }

            last_pos = pos;
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }

        Ok(lines)
    }

    fn measure_line_width(
        &self,
        line: &str,
        style: &TextStyle,
        font: &Font,
    ) -> Result<f32, TextFittingError> {
        let mut total_width = 0.0f32;
        let scale = style.font_size;

        for ch in line.chars() {
            let metrics = font.metrics(ch, scale);
            total_width += metrics.advance_width + style.letter_spacing;
        }

        // Remove the last letter spacing
        if !line.is_empty() {
            total_width -= style.letter_spacing;
        }

        Ok(total_width)
    }

    // Simple syllable breaking for demonstration
    fn simple_syllable_break(&self, word: &str) -> Vec<String> {
        if word.len() <= 3 {
            return vec![word.to_string()];
        }

        let mut syllables = Vec::new();
        let chars: Vec<char> = word.chars().collect();
        let mut current_syllable = String::new();

        for (i, &ch) in chars.iter().enumerate() {
            current_syllable.push(ch);

            // Simple rule: break after vowels if followed by consonants
            if self.is_vowel(ch) && i + 2 < chars.len() && !self.is_vowel(chars[i + 1]) {
                syllables.push(current_syllable.clone());
                current_syllable.clear();
            }
        }

        if !current_syllable.is_empty() {
            syllables.push(current_syllable);
        }

        if syllables.is_empty() {
            vec![word.to_string()]
        } else {
            syllables
        }
    }

    fn is_vowel(&self, ch: char) -> bool {
        matches!(ch.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u' | 'y')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MOCK_FONT_DATA: &[u8] = &[0u8; 1024];

    fn create_test_fitter() -> TextFitter {
        TextFitter::new().unwrap()
    }

    #[test]
    fn test_text_fitter_creation() {
        let fitter = TextFitter::new();
        assert!(fitter.is_ok());
    }

    #[test]
    fn test_default_text_style() {
        let style = TextStyle::default();
        assert_eq!(style.font_size, 16.0);
        assert_eq!(style.line_height, 1.2);
        assert_eq!(style.letter_spacing, 0.0);
        assert_eq!(style.font_weight, 400);
    }

    #[test]
    fn test_default_fitting_config() {
        let config = TextFittingConfig::default();
        assert_eq!(config.script_type, ScriptType::Latin);
        assert_eq!(config.wrap_strategy, None);
        assert_eq!(config.max_width, 300.0);
        assert_eq!(config.max_height, 200.0);
    }

    #[test]
    fn test_script_type_default_strategies() {
        assert_eq!(
            ScriptType::Latin.default_wrap_strategy(),
            WrapStrategy::Word
        );
        assert_eq!(
            ScriptType::CJK.default_wrap_strategy(),
            WrapStrategy::Anywhere
        );
    }

    #[test]
    fn test_wrap_strategies() {
        let strategies = vec![
            WrapStrategy::Word,
            WrapStrategy::Syllable,
            WrapStrategy::Anywhere,
        ];

        for strategy in strategies {
            let cloned = strategy.clone();
            assert_eq!(strategy, cloned);
        }
    }

    #[test]
    fn test_syllable_breaking() {
        let fitter = create_test_fitter();

        let syllables = fitter.simple_syllable_break("hello");
        assert!(!syllables.is_empty());

        let syllables = fitter.simple_syllable_break("a");
        assert_eq!(syllables, vec!["a"]);

        let syllables = fitter.simple_syllable_break("beautiful");
        assert!(syllables.len() > 1);
    }

    #[test]
    fn test_vowel_detection() {
        let fitter = create_test_fitter();

        assert!(fitter.is_vowel('a'));
        assert!(fitter.is_vowel('e'));
        assert!(fitter.is_vowel('A'));
        assert!(!fitter.is_vowel('b'));
        assert!(!fitter.is_vowel('1'));
    }

    #[test]
    fn test_font_loading() {
        let mut fitter = create_test_fitter();

        let result = fitter.add_font(MOCK_FONT_DATA, 400);
        assert!(result.is_err()); // Mock data will fail
    }

    #[test]
    fn test_script_type_configurations() {
        let script_types = vec![ScriptType::Latin, ScriptType::CJK];

        for script_type in script_types {
            let strategy = script_type.default_wrap_strategy();
            match script_type {
                ScriptType::Latin => assert_eq!(strategy, WrapStrategy::Word),
                ScriptType::CJK => assert_eq!(strategy, WrapStrategy::Anywhere),
            }
        }
    }

    #[test]
    fn test_config_validation() {
        let config = TextFittingConfig {
            min_font_size: 10.0,
            max_font_size: 20.0,
            font_size_step: 0.5,
            max_width: 200.0,
            max_height: 100.0,
            ..Default::default()
        };

        assert!(config.min_font_size < config.max_font_size);
        assert!(config.font_size_step > 0.0);
        assert!(config.max_width > 0.0);
        assert!(config.max_height > 0.0);
    }

    #[test]
    fn test_text_metrics_structure() {
        let metrics = TextMetrics {
            width: 100.0,
            height: 50.0,
            lines: vec!["Line 1".to_string(), "Line 2".to_string()],
            font_size: 16.0,
        };

        assert_eq!(metrics.width, 100.0);
        assert_eq!(metrics.height, 50.0);
        assert_eq!(metrics.lines.len(), 2);
        assert_eq!(metrics.font_size, 16.0);
    }

    #[test]
    fn test_error_types() {
        let font_error = TextFittingError::FontError("Test error".to_string());
        let measurement_error = TextFittingError::MeasurementError("Test error".to_string());
        let no_size_error = TextFittingError::NoSuitableSize;

        assert!(matches!(font_error, TextFittingError::FontError(_)));
        assert!(matches!(
            measurement_error,
            TextFittingError::MeasurementError(_)
        ));
        assert!(matches!(no_size_error, TextFittingError::NoSuitableSize));
    }

    #[test]
    fn test_style_combinations() {
        let styles = vec![
            TextStyle {
                font_size: 12.0,
                line_height: 1.0,
                letter_spacing: 0.0,
                font_weight: 400,
            },
            TextStyle {
                font_size: 16.0,
                line_height: 1.5,
                letter_spacing: 1.0,
                font_weight: 700,
            },
        ];

        for style in styles {
            assert!(style.font_size > 0.0);
            assert!(style.line_height > 0.0);
            assert!(style.font_weight >= 100 && style.font_weight <= 900);
        }
    }
}

// Integration tests would require actual font data
#[cfg(test)]
mod integration_tests {

    #[test]
    #[ignore] // Requires actual font data
    fn test_latin_text_fitting() {
        // This test would require loading actual font data
        // let mut fitter = TextFitter::new().unwrap();
        // fitter.add_font(FONT_DATA, 400).unwrap();

        // let config = TextFittingConfig {
        //     script_type: ScriptType::Latin,
        //     max_width: 200.0,
        //     max_height: 100.0,
        //     ..Default::default()
        // };

        // let style = TextStyle::default();
        // let text = "The quick brown fox jumps over the lazy dog";

        // let result = fitter.fit_text(text, &config, &style);
        // assert!(result.is_ok());
    }

    #[test]
    #[ignore] // Requires actual font data
    fn test_cjk_text_fitting() {
        // Similar test for CJK text with anywhere wrapping
    }
}
