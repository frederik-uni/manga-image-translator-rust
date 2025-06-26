//! Comprehensive example demonstrating real-world usage of the text fitting library.
//!
//! This example shows how to:
//! - Load and manage multiple fonts
//! - Handle different languages and writing systems
//! - Implement various text wrapping strategies
//! - Create adaptive text layouts
//! - Handle errors gracefully
//! - Optimize performance for different use cases

use default_renderer::get_size::{
    ScriptType, TextFitter, TextFittingConfig, TextFittingError, TextStyle,
};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct FontManager {
    fitter: TextFitter,
    font_paths: HashMap<String, String>,
}

impl FontManager {
    pub fn new() -> Result<Self, TextFittingError> {
        Ok(Self {
            fitter: TextFitter::new()?,
            font_paths: HashMap::new(),
        })
    }

    pub fn load_font_family(
        &mut self,
        family_name: &str,
        font_dir: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Define common font weight variants
        let variants = vec![
            ("Regular", 400, "Regular.ttf"),
            ("Medium", 500, "Medium.ttf"),
            ("SemiBold", 600, "SemiBold.ttf"),
            ("Bold", 700, "Bold.ttf"),
        ];

        for (variant_name, weight, filename) in variants {
            let font_path = format!("{}/{}-{}", font_dir, family_name, filename);

            if Path::new(&font_path).exists() {
                match fs::read(&font_path) {
                    Ok(font_data) => {
                        self.fitter.add_font(&font_data, weight)?;
                        self.font_paths.insert(
                            format!("{}_{}", family_name, variant_name),
                            font_path.clone(),
                        );
                        println!(
                            "✓ Loaded {}-{} (weight: {})",
                            family_name, variant_name, weight
                        );
                    }
                    Err(e) => {
                        println!("⚠ Could not load {}: {}", font_path, e);
                    }
                }
            } else {
                println!("⚠ Font file not found: {}", font_path);
            }
        }

        Ok(())
    }

    pub fn load_cjk_fonts(&mut self, font_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Load CJK (Chinese, Japanese, Korean) fonts
        let cjk_fonts = vec![
            ("NotoSansCJK", "NotoSansCJK-Regular.ttf", 400),
            ("NotoSansCJK", "NotoSansCJK-Bold.ttf", 700),
        ];

        for (_family, filename, weight) in cjk_fonts {
            let font_path = format!("{}/{}", font_dir, filename);

            if Path::new(&font_path).exists() {
                match fs::read(&font_path) {
                    Ok(font_data) => {
                        self.fitter.add_font(&font_data, weight)?;
                        println!("✓ Loaded CJK font: {} (weight: {})", filename, weight);
                    }
                    Err(e) => {
                        println!("⚠ Could not load CJK font {}: {}", font_path, e);
                    }
                }
            }
        }

        Ok(())
    }

    pub fn fit_text(
        &self,
        text: &str,
        config: &TextFittingConfig,
        style: &TextStyle,
    ) -> Result<default_renderer::get_size::TextMetrics, TextFittingError> {
        self.fitter.fit_text(text, config, style)
    }
}

#[derive(Debug, Clone)]
pub struct UITextBox {
    pub id: String,
    pub text: String,
    pub script_type: ScriptType,
    pub max_width: f32,
    pub max_height: f32,
    pub preferred_font_size: f32,
    pub font_weight: u16,
    pub line_height: f32,
    pub letter_spacing: f32,
}

impl UITextBox {
    pub fn new(id: &str, text: &str) -> Self {
        Self {
            id: id.to_string(),
            text: text.to_string(),
            script_type: ScriptType::Latin,
            max_width: 300.0,
            max_height: 200.0,
            preferred_font_size: 16.0,
            font_weight: 400,
            line_height: 1.2,
            letter_spacing: 0.0,
        }
    }

    pub fn with_script_type(mut self, script_type: ScriptType) -> Self {
        self.script_type = script_type;
        self
    }

    pub fn with_dimensions(mut self, width: f32, height: f32) -> Self {
        self.max_width = width;
        self.max_height = height;
        self
    }

    pub fn with_typography(mut self, font_size: f32, font_weight: u16, line_height: f32) -> Self {
        self.preferred_font_size = font_size;
        self.font_weight = font_weight;
        self.line_height = line_height;
        self
    }

    pub fn with_letter_spacing(mut self, spacing: f32) -> Self {
        self.letter_spacing = spacing;
        self
    }
}

pub struct TextLayoutEngine {
    font_manager: FontManager,
}

impl TextLayoutEngine {
    pub fn new() -> Result<Self, TextFittingError> {
        Ok(Self {
            font_manager: FontManager::new()?,
        })
    }

    pub fn initialize_fonts(
        &mut self,
        font_directory: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("Initializing font system...");

        // Load primary font family (e.g., Roboto, Inter, etc.)
        self.font_manager
            .load_font_family("Roboto", font_directory)?;

        // Load CJK fonts for international support
        self.font_manager.load_cjk_fonts(font_directory)?;

        println!("Font system initialized successfully!");
        Ok(())
    }

    pub fn layout_text_box(&self, text_box: &UITextBox) -> Result<LayoutResult, TextFittingError> {
        let config = TextFittingConfig {
            script_type: text_box.script_type,
            wrap_strategy: None, // Use script default
            max_width: text_box.max_width,
            max_height: text_box.max_height,
            min_font_size: 8.0,
            max_font_size: text_box.preferred_font_size * 1.5,
            font_size_step: 0.5,
        };

        let style = TextStyle {
            font_size: text_box.preferred_font_size,
            line_height: text_box.line_height,
            letter_spacing: text_box.letter_spacing,
            font_weight: text_box.font_weight,
        };

        let start_time = std::time::Instant::now();
        let metrics = self
            .font_manager
            .fit_text(&text_box.text, &config, &style)?;
        let layout_time = start_time.elapsed();

        Ok(LayoutResult {
            text_box_id: text_box.id.clone(),
            metrics,
            layout_time,
            overflow: false, // Would be determined by comparing with constraints
        })
    }

    pub fn layout_multiple_text_boxes(
        &self,
        text_boxes: Vec<UITextBox>,
    ) -> Vec<Result<LayoutResult, TextFittingError>> {
        println!("Laying out {} text boxes...", text_boxes.len());

        let start_time = std::time::Instant::now();
        let results: Vec<_> = text_boxes
            .iter()
            .map(|text_box| self.layout_text_box(text_box))
            .collect();
        let total_time = start_time.elapsed();

        let successful_layouts = results.iter().filter(|r| r.is_ok()).count();
        println!(
            "Completed {} layouts in {:?} (avg: {:?}/layout)",
            successful_layouts,
            total_time,
            total_time / text_boxes.len() as u32
        );

        results
    }
}

#[derive(Debug)]
pub struct LayoutResult {
    pub text_box_id: String,
    pub metrics: default_renderer::get_size::TextMetrics,
    pub layout_time: std::time::Duration,
    pub overflow: bool,
}

impl LayoutResult {
    pub fn print_summary(&self) {
        println!("Layout Result for '{}':", self.text_box_id);
        println!("  Font size: {:.1}px", self.metrics.font_size);
        println!(
            "  Dimensions: {:.1}x{:.1}px",
            self.metrics.width, self.metrics.height
        );
        println!("  Lines: {}", self.metrics.lines.len());
        println!("  Layout time: {:?}", self.layout_time);
        println!("  Overflow: {}", if self.overflow { "Yes" } else { "No" });

        if self.metrics.lines.len() <= 5 {
            for (i, line) in self.metrics.lines.iter().enumerate() {
                println!("    Line {}: '{}'", i + 1, line);
            }
        } else {
            for (i, line) in self.metrics.lines.iter().take(3).enumerate() {
                println!("    Line {}: '{}'", i + 1, line);
            }
            println!("    ... and {} more lines", self.metrics.lines.len() - 3);
        }
        println!();
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Comprehensive Text Fitting Example ===");
    println!();

    // Initialize the text layout engine
    let mut layout_engine = TextLayoutEngine::new()?;

    // In a real application, you would specify the actual font directory
    let font_directory = "assets/fonts"; // This would contain your font files

    // Attempt to initialize fonts (will show warnings if fonts are not available)
    if let Err(e) = layout_engine.initialize_fonts(font_directory) {
        println!(
            "Warning: Could not load fonts from {}: {}",
            font_directory, e
        );
        println!("This demo will show the API structure without actual font rendering.");
        println!();
    }

    // Example 1: Simple UI text boxes
    println!("=== Example 1: Simple UI Text Boxes ===");

    let ui_text_boxes = vec![
        UITextBox::new("button_1", "Click Me")
            .with_dimensions(80.0, 30.0)
            .with_typography(14.0, 600, 1.0),

        UITextBox::new("title", "Welcome to Our Application")
            .with_dimensions(400.0, 60.0)
            .with_typography(24.0, 700, 1.1),

        UITextBox::new("description", "This is a longer description that needs to be wrapped across multiple lines to fit within the designated area.")
            .with_dimensions(300.0, 120.0)
            .with_typography(16.0, 400, 1.4),
    ];

    demonstrate_text_box_layouts(&layout_engine, ui_text_boxes);

    // Example 2: Multi-script content
    println!("=== Example 2: Multi-script Content ===");

    let multiscript_content = vec![
        UITextBox::new("latin_paragraph", "The quick brown fox jumps over the lazy dog. This sentence contains every letter of the alphabet.")
            .with_script_type(ScriptType::Latin)
            .with_dimensions(250.0, 100.0),

        UITextBox::new("latin_compound", "Antidisestablishmentarianism and pneumonoultramicroscopicsilicovolcanoconiosis are very long compound words.")
            .with_script_type(ScriptType::Latin)
            .with_dimensions(200.0, 120.0),

        UITextBox::new("chinese_text", "这是一个中文文本的例子。中文可以在任何字符处换行，不需要考虑单词边界。")
            .with_script_type(ScriptType::CJK)
            .with_dimensions(180.0, 100.0),

        UITextBox::new("japanese_text", "これは日本語のテキストの例です。ひらがな、カタカナ、漢字が混在しています。")
            .with_script_type(ScriptType::CJK)
            .with_dimensions(160.0, 120.0),
    ];

    demonstrate_text_box_layouts(&layout_engine, multiscript_content);

    // Example 3: Typography variations
    println!("=== Example 3: Typography Variations ===");

    let typography_examples = vec![
        UITextBox::new("light_text", "Light weight text with generous spacing")
            .with_typography(16.0, 300, 1.6)
            .with_letter_spacing(0.5)
            .with_dimensions(200.0, 80.0),
        UITextBox::new("regular_text", "Regular weight text with normal spacing")
            .with_typography(16.0, 400, 1.4)
            .with_dimensions(200.0, 80.0),
        UITextBox::new("bold_text", "Bold weight text with tight spacing")
            .with_typography(16.0, 700, 1.2)
            .with_letter_spacing(-0.2)
            .with_dimensions(200.0, 80.0),
        UITextBox::new("black_text", "Black weight text with very tight spacing")
            .with_typography(16.0, 900, 1.1)
            .with_letter_spacing(-0.5)
            .with_dimensions(200.0, 80.0),
    ];

    demonstrate_text_box_layouts(&layout_engine, typography_examples);

    // Example 4: Responsive text sizing
    println!("=== Example 4: Responsive Text Sizing ===");

    let responsive_examples = vec![
        UITextBox::new("mobile_title", "Mobile App Title")
            .with_dimensions(320.0, 40.0)
            .with_typography(18.0, 600, 1.2),
        UITextBox::new("tablet_title", "Tablet Application Title")
            .with_dimensions(768.0, 60.0)
            .with_typography(24.0, 600, 1.2),
        UITextBox::new("desktop_title", "Desktop Application Title with More Space")
            .with_dimensions(1200.0, 80.0)
            .with_typography(32.0, 600, 1.2),
    ];

    demonstrate_text_box_layouts(&layout_engine, responsive_examples);

    // Example 5: Performance benchmark
    println!("=== Example 5: Performance Benchmark ===");

    let benchmark_text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. ".repeat(10);
    let mut benchmark_boxes = Vec::new();

    for i in 0..100 {
        benchmark_boxes.push(
            UITextBox::new(&format!("perf_test_{}", i), &benchmark_text)
                .with_dimensions(200.0 + (i as f32 * 2.0), 100.0)
                .with_typography(14.0, 400, 1.3),
        );
    }

    let start_time = std::time::Instant::now();
    let results = layout_engine.layout_multiple_text_boxes(benchmark_boxes);
    let total_time = start_time.elapsed();

    let successful_count = results.iter().filter(|r| r.is_ok()).count();
    println!("Performance Results:");
    println!("  Total layouts: {}", results.len());
    println!("  Successful layouts: {}", successful_count);
    println!("  Total time: {:?}", total_time);
    println!(
        "  Average time per layout: {:?}",
        total_time / results.len() as u32
    );
    println!(
        "  Layouts per second: {:.0}",
        results.len() as f64 / total_time.as_secs_f64()
    );

    // Example 6: Error handling
    println!("=== Example 6: Error Handling ===");

    demonstrate_error_handling(&layout_engine);

    println!("=== Example Complete ===");
    println!("This example demonstrates comprehensive usage of the text fitting library.");
    println!(
        "For production use, ensure you have the appropriate font files in your assets directory."
    );

    Ok(())
}

fn demonstrate_text_box_layouts(layout_engine: &TextLayoutEngine, text_boxes: Vec<UITextBox>) {
    let results = layout_engine.layout_multiple_text_boxes(text_boxes);

    for result in results {
        match result {
            Ok(layout_result) => layout_result.print_summary(),
            Err(e) => println!("Layout failed: {}", e),
        }
    }
}

fn demonstrate_error_handling(layout_engine: &TextLayoutEngine) {
    // Test various error conditions
    let error_test_cases = vec![
        // Extremely small box
        UITextBox::new("tiny_box", "This text is way too long for such a tiny box")
            .with_dimensions(10.0, 5.0),
        // Zero dimensions
        UITextBox::new("zero_box", "Text in zero-sized box").with_dimensions(0.0, 0.0),
        // Very long text with small box
        UITextBox::new("overflow_test", &"Very long text ".repeat(100)).with_dimensions(50.0, 20.0),
    ];

    for text_box in error_test_cases {
        match layout_engine.layout_text_box(&text_box) {
            Ok(result) => {
                println!(
                    "Unexpectedly succeeded for '{}': {:.1}x{:.1} at {:.1}px",
                    text_box.id,
                    result.metrics.width,
                    result.metrics.height,
                    result.metrics.font_size
                );
            }
            Err(e) => {
                println!("Expected error for '{}': {}", text_box.id, e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ui_text_box_creation() {
        let text_box = UITextBox::new("test", "Hello World");
        assert_eq!(text_box.id, "test");
        assert_eq!(text_box.text, "Hello World");
        assert_eq!(text_box.script_type, ScriptType::Latin);
    }

    #[test]
    fn test_ui_text_box_builder_pattern() {
        let text_box = UITextBox::new("test", "Hello")
            .with_script_type(ScriptType::CJK)
            .with_dimensions(100.0, 50.0)
            .with_typography(18.0, 600, 1.5)
            .with_letter_spacing(1.0);

        assert_eq!(text_box.script_type, ScriptType::CJK);
        assert_eq!(text_box.max_width, 100.0);
        assert_eq!(text_box.max_height, 50.0);
        assert_eq!(text_box.preferred_font_size, 18.0);
        assert_eq!(text_box.font_weight, 600);
        assert_eq!(text_box.line_height, 1.5);
        assert_eq!(text_box.letter_spacing, 1.0);
    }

    #[test]
    fn test_font_manager_creation() {
        let font_manager = FontManager::new();
        assert!(font_manager.is_ok());
    }

    #[test]
    fn test_text_layout_engine_creation() {
        let layout_engine = TextLayoutEngine::new();
        assert!(layout_engine.is_ok());
    }

    #[test]
    fn test_font_loading_graceful_failure() {
        let mut font_manager = FontManager::new().unwrap();
        // This should not panic even with non-existent font directory
        let result = font_manager.load_font_family("NonExistent", "/nonexistent/path");
        // Should complete without crashing (might log warnings)
        assert!(result.is_ok());
    }
}
