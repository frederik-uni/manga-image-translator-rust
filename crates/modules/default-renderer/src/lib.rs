//! # Default Renderer - Text Fitting Library
//!
//! A high-performance Rust library for fitting text into constrained boxes with support
//! for multiple languages, custom fonts, and various text wrapping strategies.
//!
//! ## Features
//!
//! - **Multi-script Support**: Latin scripts (word/syllable breaking) and CJK scripts (anywhere breaking)
//! - **Flexible Text Wrapping**: Word-based, syllable-based, and anywhere wrapping
//! - **Custom Font Support**: Multiple font weights and custom font loading
//! - **Advanced Typography**: Line height, letter spacing, font weight control
//! - **Performance Optimized**: Fast binary search for optimal font sizing
//! - **Image Rendering**: High-performance text rendering on images with inpainting support
//! - **GPU Acceleration**: Hardware-accelerated image processing and text rendering
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use default_renderer::get_size::{
//!     ScriptType, TextFitter, TextFittingConfig, TextStyle, WrapStrategy
//! };
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Initialize text fitter
//! let mut fitter = TextFitter::new()?;
//!
//! // Load font (replace with actual font data)
//! let font_data = std::fs::read("path/to/font.ttf")?;
//! fitter.add_font(&font_data, 400)?;
//!
//! // Configure text fitting
//! let config = TextFittingConfig {
//!     script_type: ScriptType::Latin,
//!     wrap_strategy: Some(WrapStrategy::Word),
//!     max_width: 300.0,
//!     max_height: 200.0,
//!     ..Default::default()
//! };
//!
//! let style = TextStyle {
//!     font_size: 16.0,
//!     line_height: 1.2,
//!     letter_spacing: 0.0,
//!     font_weight: 400,
//! };
//!
//! // Fit text to constraints
//! let text = "The quick brown fox jumps over the lazy dog";
//! let metrics = fitter.fit_text(text, &config, &style)?;
//!
//! println!("Optimal font size: {:.1}px", metrics.font_size);
//! println!("Final dimensions: {:.1}x{:.1}", metrics.width, metrics.height);
//! # Ok(())
//! # }
//! ```
//!
//! ## Script Support
//!
//! The library provides specialized support for different writing systems:
//!
//! - **Latin Scripts**: Word-based wrapping with optional syllable breaking (English, German, French, etc.)
//! - **CJK Scripts**: Character-based wrapping with Unicode line breaking rules (Chinese, Japanese, Korean)
//!
//! ## Examples
//!
//! See the `examples/` directory for comprehensive usage examples, including:
//! - Multi-script text handling
//! - Font management
//! - Image rendering and inpainting
//! - Performance optimization
//! - Error handling strategies

pub mod get_size;
pub mod image_renderer;

// Re-export commonly used types for convenience
pub use get_size::{
    ScriptType, TextFitter, TextFittingConfig, TextFittingError, TextMetrics, TextStyle,
    WrapStrategy,
};
pub use image_renderer::{
    Color, DropShadowConfig, ImageRenderError, ImageTextRenderer, RenderResult, TextAlignment,
    TextBox, TextRenderConfig,
};
