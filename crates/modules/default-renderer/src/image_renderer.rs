//! # Image Text Renderer
//!
//! A module for rendering text onto images with advanced typography features including
//! drop shadows, customizable colors, and intelligent text fitting.

use crate::get_size::{TextFitter, TextFittingConfig, TextMetrics, TextStyle};
use fontdue::{Font, FontSettings};
use image::{ImageBuffer, Rgba, RgbaImage};

use std::collections::HashMap;
use thiserror::Error;

/// Errors that can occur during image text rendering
#[derive(Error, Debug)]
pub enum ImageRenderError {
    #[error("Font loading error: {0}")]
    FontError(String),
    #[error("Image processing error: {0}")]
    ImageError(String),
    #[error("Text fitting error: {0}")]
    TextFittingError(#[from] crate::get_size::TextFittingError),
    #[error("Invalid color format: {0}")]
    InvalidColor(String),
}

/// RGBA color representation
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::new(r, g, b, 255)
    }

    pub fn white() -> Self {
        Self::rgb(255, 255, 255)
    }

    pub fn black() -> Self {
        Self::rgb(0, 0, 0)
    }

    pub fn green() -> Self {
        Self::rgb(0, 255, 0)
    }

    pub fn transparent() -> Self {
        Self::new(0, 0, 0, 0)
    }

    pub fn to_rgba(&self) -> Rgba<u8> {
        Rgba([self.r, self.g, self.b, self.a])
    }
}

/// Text alignment options
#[derive(Debug, Clone, Copy)]
pub enum TextAlignment {
    /// Top-left alignment
    TopLeft,
    /// Top-center alignment
    TopCenter,
    /// Top-right alignment
    TopRight,
    /// Center-left alignment
    CenterLeft,
    /// Center-center alignment
    CenterCenter,
    /// Center-right alignment
    CenterRight,
    /// Bottom-left alignment
    BottomLeft,
    /// Bottom-center alignment
    BottomCenter,
    /// Bottom-right alignment
    BottomRight,
}

impl Default for TextAlignment {
    fn default() -> Self {
        TextAlignment::TopLeft
    }
}

/// Drop shadow configuration
#[derive(Debug, Clone)]
pub struct DropShadowConfig {
    /// Enable drop shadow
    pub enabled: bool,
    /// Shadow color
    pub color: Color,
    /// Shadow expansion in pixels (how much to expand in all directions)
    pub expansion: u32,
    /// Shadow blur radius (0 for no blur, just expansion)
    pub blur_radius: u32,
}

impl Default for DropShadowConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            color: Color::black(),
            expansion: 2,
            blur_radius: 0,
        }
    }
}

/// Text rendering configuration
#[derive(Debug, Clone)]
pub struct TextRenderConfig {
    /// Text color
    pub text_color: Color,
    /// Background color (for areas not covered by text)
    pub background_color: Color,
    /// Text box background color
    pub box_color: Color,
    /// Drop shadow configuration
    pub drop_shadow: DropShadowConfig,
    /// Additional character spacing in pixels
    pub character_spacing: f32,
    /// Line height multiplier (1.0 = normal, 1.5 = 150% spacing)
    pub line_height_multiplier: f32,
    /// Text alignment within the text box
    pub alignment: TextAlignment,
    /// Text fitting configuration
    pub fitting_config: TextFittingConfig,
    /// Text style
    pub text_style: TextStyle,
}

impl Default for TextRenderConfig {
    fn default() -> Self {
        Self {
            text_color: Color::white(),
            background_color: Color::black(),
            box_color: Color::green(),
            drop_shadow: DropShadowConfig::default(),
            character_spacing: 0.0,
            line_height_multiplier: 1.0,
            alignment: TextAlignment::default(),
            fitting_config: TextFittingConfig::default(),
            text_style: TextStyle::default(),
        }
    }
}

/// Bounding box for text rendering
#[derive(Debug, Clone)]
pub struct TextBox {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl TextBox {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

/// Result of text rendering operation
#[derive(Debug)]
pub struct RenderResult {
    /// Final text metrics after fitting
    pub metrics: TextMetrics,
    /// Actual text box used for rendering
    pub text_box: TextBox,
    /// Whether the text was successfully fitted
    pub fitted: bool,
}

/// Image text renderer
pub struct ImageTextRenderer {
    /// Text fitter for size detection and line breaking
    text_fitter: TextFitter,
    /// Loaded fonts mapped by weight
    fonts: HashMap<u16, Font>,
}

impl ImageTextRenderer {
    /// Create a new image text renderer
    pub fn new() -> Result<Self, ImageRenderError> {
        let text_fitter = TextFitter::new().map_err(|e| ImageRenderError::TextFittingError(e))?;

        Ok(Self {
            text_fitter,
            fonts: HashMap::new(),
        })
    }

    /// Add a font with specified weight
    pub fn add_font(&mut self, font_data: &[u8], weight: u16) -> Result<(), ImageRenderError> {
        // Add font to text fitter for size detection
        self.text_fitter
            .add_font(font_data, weight)
            .map_err(|e| ImageRenderError::TextFittingError(e))?;

        // Create fontdue font for rendering
        let font = Font::from_bytes(font_data, FontSettings::default())
            .map_err(|e| ImageRenderError::FontError(format!("Failed to load font: {}", e)))?;

        self.fonts.insert(weight, font);
        Ok(())
    }

    /// Render text onto an image within the specified text box
    pub fn render_text(
        &self,
        image: &mut RgbaImage,
        text: &str,
        text_box: TextBox,
        config: &TextRenderConfig,
    ) -> Result<RenderResult, ImageRenderError> {
        // Update fitting config with text box dimensions
        let mut fitting_config = config.fitting_config.clone();
        fitting_config.max_width = text_box.width as f32;
        fitting_config.max_height = text_box.height as f32;

        // Fit text to the box
        let metrics = self
            .text_fitter
            .fit_text(text, &fitting_config, &config.text_style)?;

        // Fill text box background
        self.fill_text_box(image, &text_box, config.box_color);

        // Get the font for rendering
        let font = self
            .fonts
            .get(&config.text_style.font_weight)
            .ok_or_else(|| {
                ImageRenderError::FontError(format!(
                    "Font with weight {} not loaded",
                    config.text_style.font_weight
                ))
            })?;

        // Render text with optional drop shadow
        if config.drop_shadow.enabled {
            self.render_text_with_shadow(image, &metrics, &text_box, font, config)?;
        } else {
            self.render_text_simple(image, &metrics, &text_box, font, config)?;
        }

        Ok(RenderResult {
            metrics,
            text_box,
            fitted: true, // TODO: Add proper fitting check
        })
    }

    /// Fill the text box area with the specified color
    fn fill_text_box(&self, image: &mut RgbaImage, text_box: &TextBox, color: Color) {
        let rgba = color.to_rgba();

        for y in text_box.y..text_box.y + text_box.height {
            for x in text_box.x..text_box.x + text_box.width {
                if x < image.width() && y < image.height() {
                    image.put_pixel(x, y, rgba);
                }
            }
        }
    }

    /// Render text without drop shadow
    fn render_text_simple(
        &self,
        image: &mut RgbaImage,
        metrics: &TextMetrics,
        text_box: &TextBox,
        font: &Font,
        config: &TextRenderConfig,
    ) -> Result<(), ImageRenderError> {
        let line_height = metrics.font_size * config.line_height_multiplier;
        let total_text_height = metrics.lines.len() as f32 * line_height;

        // Calculate starting positions based on alignment
        let (start_x, start_y) = self.calculate_text_position(
            text_box,
            metrics,
            total_text_height,
            line_height,
            config.alignment,
            font,
            config.character_spacing,
        );

        for (line_index, line) in metrics.lines.iter().enumerate() {
            let line_y = start_y + (line_index as f32 * line_height);
            let line_x = match config.alignment {
                TextAlignment::TopCenter
                | TextAlignment::CenterCenter
                | TextAlignment::BottomCenter => {
                    let line_width = self.calculate_line_width(
                        line,
                        font,
                        metrics.font_size,
                        config.character_spacing,
                    );
                    start_x - (line_width / 2.0)
                }
                TextAlignment::TopRight
                | TextAlignment::CenterRight
                | TextAlignment::BottomRight => {
                    let line_width = self.calculate_line_width(
                        line,
                        font,
                        metrics.font_size,
                        config.character_spacing,
                    );
                    start_x - line_width
                }
                _ => start_x, // Left alignment
            };

            self.render_line(
                image,
                line,
                line_x,
                line_y,
                font,
                metrics.font_size,
                config.character_spacing,
                config.text_color,
                text_box,
            )?;
        }

        Ok(())
    }

    /// Render text with drop shadow
    fn render_text_with_shadow(
        &self,
        image: &mut RgbaImage,
        metrics: &TextMetrics,
        text_box: &TextBox,
        font: &Font,
        config: &TextRenderConfig,
    ) -> Result<(), ImageRenderError> {
        let line_height = metrics.font_size * config.line_height_multiplier;
        let total_text_height = metrics.lines.len() as f32 * line_height;

        // Calculate starting positions based on alignment
        let (start_x, start_y) = self.calculate_text_position(
            text_box,
            metrics,
            total_text_height,
            line_height,
            config.alignment,
            font,
            config.character_spacing,
        );

        // First render the shadow (expanded text)
        for (line_index, line) in metrics.lines.iter().enumerate() {
            let line_y = start_y + (line_index as f32 * line_height);
            let line_x = match config.alignment {
                TextAlignment::TopCenter
                | TextAlignment::CenterCenter
                | TextAlignment::BottomCenter => {
                    let line_width = self.calculate_line_width(
                        line,
                        font,
                        metrics.font_size,
                        config.character_spacing,
                    );
                    start_x - (line_width / 2.0)
                }
                TextAlignment::TopRight
                | TextAlignment::CenterRight
                | TextAlignment::BottomRight => {
                    let line_width = self.calculate_line_width(
                        line,
                        font,
                        metrics.font_size,
                        config.character_spacing,
                    );
                    start_x - line_width
                }
                _ => start_x, // Left alignment
            };

            // Render shadow in multiple directions for expansion effect
            for dx in -(config.drop_shadow.expansion as i32)..=(config.drop_shadow.expansion as i32)
            {
                for dy in
                    -(config.drop_shadow.expansion as i32)..=(config.drop_shadow.expansion as i32)
                {
                    if dx == 0 && dy == 0 {
                        continue;
                    } // Skip center (will be rendered as main text)

                    self.render_line(
                        image,
                        line,
                        line_x + dx as f32,
                        line_y + dy as f32,
                        font,
                        metrics.font_size,
                        config.character_spacing,
                        config.drop_shadow.color,
                        text_box,
                    )?;
                }
            }
        }

        // Then render the main text on top
        for (line_index, line) in metrics.lines.iter().enumerate() {
            let line_y = start_y + (line_index as f32 * line_height);
            let line_x = match config.alignment {
                TextAlignment::TopCenter
                | TextAlignment::CenterCenter
                | TextAlignment::BottomCenter => {
                    let line_width = self.calculate_line_width(
                        line,
                        font,
                        metrics.font_size,
                        config.character_spacing,
                    );
                    start_x - (line_width / 2.0)
                }
                TextAlignment::TopRight
                | TextAlignment::CenterRight
                | TextAlignment::BottomRight => {
                    let line_width = self.calculate_line_width(
                        line,
                        font,
                        metrics.font_size,
                        config.character_spacing,
                    );
                    start_x - line_width
                }
                _ => start_x, // Left alignment
            };

            self.render_line(
                image,
                line,
                line_x,
                line_y,
                font,
                metrics.font_size,
                config.character_spacing,
                config.text_color,
                text_box,
            )?;
        }

        Ok(())
    }

    /// Render a single line of text
    fn render_line(
        &self,
        image: &mut RgbaImage,
        text: &str,
        x: f32,
        baseline_y: f32,
        font: &Font,
        font_size: f32,
        character_spacing: f32,
        color: Color,
        text_box: &TextBox,
    ) -> Result<(), ImageRenderError> {
        let mut current_x = x;

        for ch in text.chars() {
            let (metrics, bitmap) = font.rasterize(ch, font_size);

            // Skip whitespace rendering but advance position
            if ch.is_whitespace() {
                current_x += metrics.advance_width + character_spacing;
                continue;
            }

            // Calculate position with proper baseline
            // baseline_y is where the text baseline should be
            // For fontdue: ymin is negative offset from baseline to glyph bottom
            // We need to position the glyph so its baseline aligns with baseline_y
            let glyph_x = current_x + metrics.xmin as f32;
            let glyph_y = baseline_y - metrics.height as f32 - metrics.ymin as f32;

            // Render glyph bitmap with bounds checking
            self.blend_glyph_with_bounds(
                image,
                &bitmap,
                glyph_x as i32,
                glyph_y as i32,
                metrics.width,
                metrics.height,
                color,
                text_box,
            );

            // Advance position
            current_x += metrics.advance_width + character_spacing;
        }

        Ok(())
    }

    /// Alpha blend two RGBA colors
    fn alpha_blend(&self, background: Rgba<u8>, foreground: Rgba<u8>, alpha: u8) -> Rgba<u8> {
        let alpha_f = alpha as f32 / 255.0;
        let inv_alpha = 1.0 - alpha_f;

        let r = (foreground.0[0] as f32 * alpha_f + background.0[0] as f32 * inv_alpha) as u8;
        let g = (foreground.0[1] as f32 * alpha_f + background.0[1] as f32 * inv_alpha) as u8;
        let b = (foreground.0[2] as f32 * alpha_f + background.0[2] as f32 * inv_alpha) as u8;
        let a = ((foreground.0[3] as f32 * alpha_f + background.0[3] as f32 * inv_alpha).min(255.0))
            as u8;

        Rgba([r, g, b, a])
    }

    /// Calculate text position based on alignment
    fn calculate_text_position(
        &self,
        text_box: &TextBox,
        _metrics: &TextMetrics,
        total_text_height: f32,
        line_height: f32,
        alignment: TextAlignment,
        font: &Font,
        _character_spacing: f32,
    ) -> (f32, f32) {
        let (ascent, descent) = self.get_font_metrics(font, line_height);
        let box_center_x = text_box.x as f32 + (text_box.width as f32 / 2.0);
        let box_center_y = text_box.y as f32 + (text_box.height as f32 / 2.0);

        let start_x = match alignment {
            TextAlignment::TopLeft | TextAlignment::CenterLeft | TextAlignment::BottomLeft => {
                text_box.x as f32
            }
            TextAlignment::TopCenter
            | TextAlignment::CenterCenter
            | TextAlignment::BottomCenter => {
                box_center_x // Will be adjusted per line based on line width
            }
            TextAlignment::TopRight | TextAlignment::CenterRight | TextAlignment::BottomRight => {
                text_box.x as f32 + text_box.width as f32 // Will be adjusted per line based on line width
            }
        };

        let start_y = match alignment {
            TextAlignment::TopLeft | TextAlignment::TopCenter | TextAlignment::TopRight => {
                text_box.y as f32 + ascent
            }
            TextAlignment::CenterLeft
            | TextAlignment::CenterCenter
            | TextAlignment::CenterRight => box_center_y - (total_text_height / 2.0) + ascent,
            TextAlignment::BottomLeft
            | TextAlignment::BottomCenter
            | TextAlignment::BottomRight => {
                // For bottom alignment, ensure text doesn't go below text box
                let bottom_baseline = text_box.y as f32 + text_box.height as f32 - descent;
                let ideal_top_baseline = bottom_baseline - total_text_height + line_height;
                ideal_top_baseline.max(text_box.y as f32 + ascent)
            }
        };

        (start_x, start_y)
    }

    /// Calculate the width of a line of text
    fn calculate_line_width(
        &self,
        text: &str,
        font: &Font,
        font_size: f32,
        character_spacing: f32,
    ) -> f32 {
        let mut width = 0.0;
        for ch in text.chars() {
            let (metrics, _) = font.rasterize(ch, font_size);
            width += metrics.advance_width + character_spacing;
        }
        // Remove the last character spacing
        if !text.is_empty() {
            width -= character_spacing;
        }
        width
    }

    /// Get approximate font metrics from fontdue
    fn get_font_metrics(&self, font: &Font, font_size: f32) -> (f32, f32) {
        // Use a representative character to estimate font metrics
        let (metrics, _) = font.rasterize('H', font_size); // 'H' for ascent
        let ascent = metrics.height as f32 - metrics.ymin as f32;

        let (desc_metrics, _) = font.rasterize('g', font_size); // 'g' for descent
        let descent = desc_metrics.ymin.abs() as f32;

        (ascent, descent)
    }

    /// Blend a glyph bitmap onto the image with text box bounds checking
    fn blend_glyph_with_bounds(
        &self,
        image: &mut RgbaImage,
        bitmap: &[u8],
        x: i32,
        y: i32,
        width: usize,
        height: usize,
        color: Color,
        text_box: &TextBox,
    ) {
        // Calculate text box bounds
        let box_left = text_box.x as i32;
        let box_top = text_box.y as i32;
        let box_right = (text_box.x + text_box.width) as i32;
        let box_bottom = (text_box.y + text_box.height) as i32;

        for row in 0..height {
            for col in 0..width {
                let bitmap_index = row * width + col;
                if bitmap_index >= bitmap.len() {
                    continue;
                }

                let alpha = bitmap[bitmap_index];
                if alpha == 0 {
                    continue;
                }

                let pixel_x = x + col as i32;
                let pixel_y = y + row as i32;

                // Check bounds: must be within both image and text box
                if pixel_x >= 0
                    && pixel_y >= 0
                    && pixel_x < image.width() as i32
                    && pixel_y < image.height() as i32
                    && pixel_x >= box_left
                    && pixel_y >= box_top
                    && pixel_x < box_right
                    && pixel_y < box_bottom
                {
                    let px = pixel_x as u32;
                    let py = pixel_y as u32;

                    // Alpha blend the glyph
                    let existing = image.get_pixel(px, py);
                    let blended = self.alpha_blend(*existing, color.to_rgba(), alpha);
                    image.put_pixel(px, py, blended);
                }
            }
        }
    }

    /// Create a new image with the specified dimensions and background color
    pub fn create_image(width: u32, height: u32, background_color: Color) -> RgbaImage {
        ImageBuffer::from_pixel(width, height, background_color.to_rgba())
    }

    /// Save image to file
    pub fn save_image(image: &RgbaImage, path: &str) -> Result<(), ImageRenderError> {
        image
            .save(path)
            .map_err(|e| ImageRenderError::ImageError(format!("Failed to save image: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_size::{ScriptType, WrapStrategy};

    #[test]
    fn test_color_creation() {
        let color = Color::rgb(255, 128, 64);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 64);
        assert_eq!(color.a, 255);
    }

    #[test]
    fn test_color_alpha_blend() {
        let renderer = ImageTextRenderer::new().unwrap();
        let bg = Color::white().to_rgba();
        let fg = Color::black().to_rgba();
        let result = renderer.alpha_blend(bg, fg, 128); // 50% alpha

        // Should be roughly gray (halfway between white and black)
        assert!(result.0[0] > 100 && result.0[0] < 155); // R
        assert!(result.0[1] > 100 && result.0[1] < 155); // G
        assert!(result.0[2] > 100 && result.0[2] < 155); // B
    }

    #[test]
    fn test_text_box_creation() {
        let text_box = TextBox::new(10, 20, 300, 100);
        assert_eq!(text_box.x, 10);
        assert_eq!(text_box.y, 20);
        assert_eq!(text_box.width, 300);
        assert_eq!(text_box.height, 100);
    }

    #[test]
    fn test_drop_shadow_config_default() {
        let config = DropShadowConfig::default();
        assert!(!config.enabled);
        assert_eq!(config.expansion, 2);
        assert_eq!(config.blur_radius, 0);
    }

    #[test]
    fn test_drop_shadow_config_enabled() {
        let config = DropShadowConfig {
            enabled: true,
            color: Color::new(128, 128, 128, 200),
            expansion: 5,
            blur_radius: 2,
        };
        assert!(config.enabled);
        assert_eq!(config.color.r, 128);
        assert_eq!(config.expansion, 5);
        assert_eq!(config.blur_radius, 2);
    }

    #[test]
    fn test_image_renderer_creation() {
        let renderer = ImageTextRenderer::new();
        assert!(renderer.is_ok());
    }

    #[test]
    fn test_create_image() {
        let image = ImageTextRenderer::create_image(100, 50, Color::black());
        assert_eq!(image.width(), 100);
        assert_eq!(image.height(), 50);

        // Check that all pixels are black
        let black_rgba = Color::black().to_rgba();
        for pixel in image.pixels() {
            assert_eq!(*pixel, black_rgba);
        }
    }

    #[test]
    fn test_text_render_config_default() {
        let config = TextRenderConfig::default();
        assert_eq!(config.text_color.r, 255); // White
        assert_eq!(config.background_color.r, 0); // Black
        assert_eq!(config.box_color.g, 255); // Green
        assert!(!config.drop_shadow.enabled);
        assert_eq!(config.character_spacing, 0.0);
        assert_eq!(config.line_height_multiplier, 1.0);
        assert!(matches!(config.alignment, TextAlignment::TopLeft));
    }

    #[test]
    fn test_fill_text_box() {
        let renderer = ImageTextRenderer::new().unwrap();
        let mut image = ImageTextRenderer::create_image(200, 200, Color::black());
        let text_box = TextBox::new(50, 50, 100, 100);

        renderer.fill_text_box(&mut image, &text_box, Color::green());

        // Check that the text box area is green
        let green_rgba = Color::green().to_rgba();
        for y in 50..150 {
            for x in 50..150 {
                assert_eq!(image.get_pixel(x, y), &green_rgba);
            }
        }

        // Check that areas outside the text box are still black
        let black_rgba = Color::black().to_rgba();
        assert_eq!(image.get_pixel(0, 0), &black_rgba);
        assert_eq!(image.get_pixel(199, 199), &black_rgba);
    }

    #[test]
    fn test_render_result_structure() {
        let text_box = TextBox::new(0, 0, 100, 100);
        let metrics = crate::get_size::TextMetrics {
            width: 80.0,
            height: 20.0,
            lines: vec!["Test".to_string()],
            font_size: 16.0,
        };

        let result = RenderResult {
            metrics,
            text_box: text_box.clone(),
            fitted: true,
        };

        assert_eq!(result.text_box.width, 100);
        assert_eq!(result.metrics.font_size, 16.0);
        assert!(result.fitted);
        assert_eq!(result.metrics.lines.len(), 1);
    }

    #[test]
    fn test_color_constants() {
        let white = Color::white();
        assert_eq!(white.r, 255);
        assert_eq!(white.g, 255);
        assert_eq!(white.b, 255);
        assert_eq!(white.a, 255);

        let black = Color::black();
        assert_eq!(black.r, 0);
        assert_eq!(black.g, 0);
        assert_eq!(black.b, 0);

        let green = Color::green();
        assert_eq!(green.g, 255);
        assert_eq!(green.r, 0);
        assert_eq!(green.b, 0);

        let transparent = Color::transparent();
        assert_eq!(transparent.a, 0);
    }

    #[test]
    fn test_integration_text_fitting_config() {
        let config = TextRenderConfig {
            fitting_config: TextFittingConfig {
                script_type: ScriptType::Latin,
                wrap_strategy: Some(WrapStrategy::Word),
                max_width: 500.0,
                max_height: 300.0,
                min_font_size: 12.0,
                max_font_size: 48.0,
                font_size_step: 1.0,
            },
            ..Default::default()
        };

        assert_eq!(config.fitting_config.max_width, 500.0);
        assert_eq!(config.fitting_config.max_height, 300.0);
        assert!(matches!(
            config.fitting_config.script_type,
            ScriptType::Latin
        ));
        assert!(matches!(
            config.fitting_config.wrap_strategy,
            Some(WrapStrategy::Word)
        ));
    }

    #[test]
    fn test_text_alignment_options() {
        // Test all alignment variants exist
        let alignments = vec![
            TextAlignment::TopLeft,
            TextAlignment::TopCenter,
            TextAlignment::TopRight,
            TextAlignment::CenterLeft,
            TextAlignment::CenterCenter,
            TextAlignment::CenterRight,
            TextAlignment::BottomLeft,
            TextAlignment::BottomCenter,
            TextAlignment::BottomRight,
        ];

        assert_eq!(alignments.len(), 9);
        assert!(matches!(TextAlignment::default(), TextAlignment::TopLeft));
    }

    #[test]
    fn test_line_width_calculation() {
        let _renderer = ImageTextRenderer::new().unwrap();

        // This test would need a real font to work properly
        // For now just test that the method exists and doesn't panic
        // let width = renderer.calculate_line_width("test", &font, 16.0, 0.0);
        // In practice, this would be tested with actual font data
    }
}
