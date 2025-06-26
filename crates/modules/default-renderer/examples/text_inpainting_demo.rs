//! # Text Inpainting Demo
//!
//! This example demonstrates how to use the ImageTextRenderer in conjunction with
//! the text size detection system to perform text inpainting - replacing detected
//! text regions with new rendered text.

use default_renderer::{
    get_size::{ScriptType, TextFittingConfig, TextStyle, WrapStrategy},
    image_renderer::{
        Color, DropShadowConfig, ImageTextRenderer, TextAlignment, TextBox, TextRenderConfig,
    },
};
use std::fs;

/// Represents a detected text region (simulating output from text detection)
#[derive(Debug, Clone)]
pub struct DetectedTextRegion {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub confidence: f32,
    pub original_text: String,
}

/// Configuration for text inpainting
#[derive(Debug, Clone)]
pub struct InpaintingConfig {
    pub replacement_text: String,
    pub render_config: TextRenderConfig,
    pub preserve_original_size: bool,
    pub background_fill_enabled: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎨 Text Inpainting Demonstration");
    println!("================================");

    // Create output directory
    fs::create_dir_all("output/inpainting")?;

    // Initialize the renderer and load font
    let mut renderer = ImageTextRenderer::new()?;
    let font_loaded = load_system_font(&mut renderer);

    if !font_loaded {
        println!("⚠️  No system font available - creating layout demonstration");
        create_layout_demonstration()?;
        return Ok(());
    }

    println!("✅ Font loaded successfully");

    // Create demonstration scenarios
    create_basic_inpainting_demo(&renderer)?;
    create_multilingual_demo(&renderer)?;
    create_style_variation_demo(&renderer)?;
    create_real_world_simulation(&renderer)?;

    println!("\n🎉 Text inpainting demonstration completed!");
    println!("📁 Check the 'output/inpainting' directory for generated images");

    Ok(())
}

/// Load a system font
fn load_system_font(renderer: &mut ImageTextRenderer) -> bool {
    let font_paths = vec![
        "/System/Library/Fonts/Arial.ttf",
        "/System/Library/Fonts/Helvetica.ttc",
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
        "/usr/share/fonts/TTF/arial.ttf",
    ];

    for font_path in font_paths {
        if let Ok(font_data) = fs::read(font_path) {
            if renderer.add_font(&font_data, 400).is_ok() {
                println!("📁 Using font: {}", font_path);
                return true;
            }
        }
    }
    false
}

/// Create a basic inpainting demonstration
fn create_basic_inpainting_demo(
    renderer: &ImageTextRenderer,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📝 Creating basic inpainting demo...");

    // Simulate an original image with text regions
    let mut image = create_simulated_document_image(800, 600);

    // Simulate detected text regions
    let detected_regions = vec![
        DetectedTextRegion {
            x: 50,
            y: 50,
            width: 700,
            height: 80,
            confidence: 0.95,
            original_text: "ORIGINAL TITLE TEXT".to_string(),
        },
        DetectedTextRegion {
            x: 50,
            y: 150,
            width: 650,
            height: 120,
            confidence: 0.88,
            original_text: "Original paragraph text that was detected...".to_string(),
        },
        DetectedTextRegion {
            x: 50,
            y: 290,
            width: 600,
            height: 90,
            confidence: 0.92,
            original_text: "Another text region".to_string(),
        },
    ];

    // Replacement texts
    let replacements = vec![
        "REPLACEMENT TITLE: Advanced Text Processing",
        "This is the replacement text that has been rendered using the advanced text fitting algorithm. The system automatically determines the optimal font size and line breaks.",
        "Final replacement text with proper formatting and spacing.",
    ];

    // Perform inpainting for each region
    for (i, (region, replacement)) in detected_regions.iter().zip(replacements.iter()).enumerate() {
        let _text_box = TextBox::new(region.x, region.y, region.width, region.height);

        let config = InpaintingConfig {
            replacement_text: replacement.to_string(),
            render_config: TextRenderConfig {
                text_color: Color::new(20, 20, 20, 255), // Dark gray text
                background_color: Color::new(245, 245, 245, 255), // Light background
                box_color: Color::new(255, 255, 255, 255), // White text box
                drop_shadow: DropShadowConfig {
                    enabled: i == 0, // Only title has shadow
                    color: Color::new(200, 200, 200, 120),
                    expansion: 1,
                    blur_radius: 0,
                },
                character_spacing: if i == 0 { 1.0 } else { 0.0 },
                line_height_multiplier: 1.2,
                alignment: TextAlignment::CenterCenter,
                fitting_config: TextFittingConfig {
                    script_type: ScriptType::Latin,
                    wrap_strategy: Some(WrapStrategy::Word),
                    max_width: region.width as f32,
                    max_height: region.height as f32,
                    min_font_size: 10.0,
                    max_font_size: if i == 0 { 36.0 } else { 20.0 },
                    font_size_step: 0.5,
                },
                text_style: TextStyle {
                    font_size: if i == 0 { 32.0 } else { 16.0 },
                    line_height: 1.2,
                    letter_spacing: 0.0,
                    font_weight: 400,
                },
            },
            preserve_original_size: true,
            background_fill_enabled: true,
        };

        inpaint_text_region(&mut image, region, &config, renderer)?;
    }

    ImageTextRenderer::save_image(&image, "output/inpainting/basic_inpainting.png")?;
    println!("  ✅ Basic inpainting demo created");

    Ok(())
}

/// Create multilingual inpainting demonstration
fn create_multilingual_demo(
    renderer: &ImageTextRenderer,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🌍 Creating multilingual demo...");

    let mut image = create_simulated_document_image(900, 500);

    let multilingual_regions = vec![
        (
            DetectedTextRegion {
                x: 50,
                y: 50,
                width: 800,
                height: 60,
                confidence: 0.94,
                original_text: "English Title".to_string(),
            },
            "Advanced Text Processing System",
            ScriptType::Latin,
        ),
        (
            DetectedTextRegion {
                x: 50,
                y: 130,
                width: 400,
                height: 100,
                confidence: 0.87,
                original_text: "English paragraph".to_string(),
            },
            "This system supports multiple languages and writing systems with intelligent text fitting.",
            ScriptType::Latin,
        ),
        (
            DetectedTextRegion {
                x: 470,
                y: 130,
                width: 380,
                height: 100,
                confidence: 0.89,
                original_text: "CJK text".to_string(),
            },
            "这是中文文本渲染示例，系统支持CJK文字的自动换行和字体调整功能。",
            ScriptType::CJK,
        ),
    ];

    for (i, (region, text, script_type)) in multilingual_regions.iter().enumerate() {
        let _text_box = TextBox::new(region.x, region.y, region.width, region.height);

        let config = InpaintingConfig {
            replacement_text: text.to_string(),
            render_config: TextRenderConfig {
                text_color: Color::new(40, 40, 40, 255),
                box_color: Color::new(250, 250, 250, 255),
                alignment: TextAlignment::CenterCenter,
                fitting_config: TextFittingConfig {
                    script_type: *script_type,
                    wrap_strategy: match script_type {
                        ScriptType::Latin => Some(WrapStrategy::Word),
                        ScriptType::CJK => Some(WrapStrategy::Anywhere),
                    },
                    max_width: region.width as f32,
                    max_height: region.height as f32,
                    min_font_size: 12.0,
                    max_font_size: if i == 0 { 28.0 } else { 18.0 },
                    font_size_step: 0.5,
                },
                ..Default::default()
            },
            preserve_original_size: true,
            background_fill_enabled: true,
        };

        inpaint_text_region(&mut image, region, &config, renderer)?;
    }

    ImageTextRenderer::save_image(&image, "output/inpainting/multilingual_demo.png")?;
    println!("  ✅ Multilingual demo created");

    Ok(())
}

/// Create style variation demonstration
fn create_style_variation_demo(
    renderer: &ImageTextRenderer,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🎨 Creating style variation demo...");

    let mut image = create_simulated_document_image(800, 700);

    let style_variations = vec![
        (
            "Normal Text Style",
            TextRenderConfig {
                text_color: Color::new(50, 50, 50, 255),
                box_color: Color::white(),
                ..Default::default()
            },
        ),
        (
            "High Contrast Style",
            TextRenderConfig {
                text_color: Color::white(),
                box_color: Color::new(30, 30, 30, 255),
                drop_shadow: DropShadowConfig {
                    enabled: true,
                    color: Color::new(100, 100, 100, 150),
                    expansion: 2,
                    blur_radius: 0,
                },
                ..Default::default()
            },
        ),
        (
            "Colorful Style with Spacing",
            TextRenderConfig {
                text_color: Color::new(0, 100, 200, 255),
                box_color: Color::new(240, 248, 255, 255),
                character_spacing: 1.5,
                line_height_multiplier: 1.4,
                ..Default::default()
            },
        ),
        (
            "Emphasized Style",
            TextRenderConfig {
                text_color: Color::new(150, 0, 0, 255),
                box_color: Color::new(255, 245, 245, 255),
                drop_shadow: DropShadowConfig {
                    enabled: true,
                    color: Color::new(255, 200, 200, 100),
                    expansion: 3,
                    blur_radius: 0,
                },
                character_spacing: 0.8,
                ..Default::default()
            },
        ),
    ];

    for (i, (style_name, render_config)) in style_variations.iter().enumerate() {
        let y_offset = 50 + (i as u32 * 150);
        let region = DetectedTextRegion {
            x: 50,
            y: y_offset,
            width: 700,
            height: 100,
            confidence: 0.9,
            original_text: format!("Original text {}", i + 1),
        };

        let mut updated_render_config = render_config.clone();
        updated_render_config.fitting_config = TextFittingConfig {
            max_width: region.width as f32,
            max_height: region.height as f32,
            min_font_size: 12.0,
            max_font_size: 24.0,
            ..Default::default()
        };

        let _text_box = TextBox::new(region.x, region.y, region.width, region.height);
        let config = InpaintingConfig {
            replacement_text: format!("Style Demo: {} - This text demonstrates different rendering styles and typography options.", style_name),
            render_config: updated_render_config,
            preserve_original_size: true,
            background_fill_enabled: true,
        };

        inpaint_text_region(&mut image, &region, &config, renderer)?;
    }

    ImageTextRenderer::save_image(&image, "output/inpainting/style_variations.png")?;
    println!("  ✅ Style variation demo created");

    Ok(())
}

/// Create real-world simulation
fn create_real_world_simulation(
    renderer: &ImageTextRenderer,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📄 Creating real-world simulation...");

    // Simulate a document with various text elements
    let mut image = create_simulated_document_image(1000, 800);

    // Header
    let header_region = DetectedTextRegion {
        x: 100,
        y: 50,
        width: 800,
        height: 80,
        confidence: 0.96,
        original_text: "Document Header".to_string(),
    };

    // Paragraphs
    let paragraph_regions = vec![
        DetectedTextRegion {
            x: 100,
            y: 160,
            width: 800,
            height: 120,
            confidence: 0.91,
            original_text: "First paragraph".to_string(),
        },
        DetectedTextRegion {
            x: 100,
            y: 300,
            width: 800,
            height: 140,
            confidence: 0.89,
            original_text: "Second paragraph".to_string(),
        },
        DetectedTextRegion {
            x: 100,
            y: 460,
            width: 800,
            height: 100,
            confidence: 0.93,
            original_text: "Third paragraph".to_string(),
        },
    ];

    // Footer
    let footer_region = DetectedTextRegion {
        x: 100,
        y: 600,
        width: 800,
        height: 60,
        confidence: 0.88,
        original_text: "Footer text".to_string(),
    };

    // Inpaint header
    let header_config = InpaintingConfig {
        replacement_text: "ADVANCED TEXT PROCESSING SYSTEM".to_string(),
        render_config: TextRenderConfig {
            text_color: Color::new(20, 20, 80, 255),
            box_color: Color::new(240, 240, 255, 255),
            drop_shadow: DropShadowConfig {
                enabled: true,
                color: Color::new(180, 180, 220, 120),
                expansion: 2,
                blur_radius: 0,
            },
            character_spacing: 2.0,
            alignment: TextAlignment::CenterCenter,
            fitting_config: TextFittingConfig {
                max_width: header_region.width as f32,
                max_height: header_region.height as f32,
                min_font_size: 20.0,
                max_font_size: 48.0,
                ..Default::default()
            },
            text_style: TextStyle {
                font_size: 36.0,
                ..Default::default()
            },
            ..Default::default()
        },
        preserve_original_size: true,
        background_fill_enabled: true,
    };

    inpaint_text_region(&mut image, &header_region, &header_config, renderer)?;

    // Inpaint paragraphs
    let paragraph_texts = vec![
        "This document demonstrates advanced text inpainting capabilities using intelligent size detection and rendering. The system automatically determines optimal font sizes and line breaks for each text region.",
        "The text fitting algorithm considers multiple factors including available space, text length, and readability requirements. It supports various wrapping strategies including word-based, syllable-based, and character-based breaking for different languages.",
        "Integration with image processing allows for seamless text replacement in documents, supporting multiple color schemes, typography options, and shadow effects for enhanced visual presentation.",
    ];

    for (region, text) in paragraph_regions.iter().zip(paragraph_texts.iter()) {
        let config = InpaintingConfig {
            replacement_text: text.to_string(),
            render_config: TextRenderConfig {
                text_color: Color::new(40, 40, 40, 255),
                box_color: Color::new(255, 255, 255, 255),
                line_height_multiplier: 1.3,
                alignment: TextAlignment::TopLeft,
                fitting_config: TextFittingConfig {
                    script_type: ScriptType::Latin,
                    wrap_strategy: Some(WrapStrategy::Word),
                    max_width: region.width as f32,
                    max_height: region.height as f32,
                    min_font_size: 12.0,
                    max_font_size: 18.0,
                    font_size_step: 0.25,
                },
                ..Default::default()
            },
            preserve_original_size: true,
            background_fill_enabled: true,
        };

        inpaint_text_region(&mut image, region, &config, renderer)?;
    }

    // Inpaint footer
    let footer_config = InpaintingConfig {
        replacement_text: "Generated by Advanced Text Processing System - Version 1.0".to_string(),
        render_config: TextRenderConfig {
            text_color: Color::new(100, 100, 100, 255),
            box_color: Color::new(250, 250, 250, 255),
            alignment: TextAlignment::CenterCenter,
            fitting_config: TextFittingConfig {
                max_width: footer_region.width as f32,
                max_height: footer_region.height as f32,
                min_font_size: 10.0,
                max_font_size: 16.0,
                ..Default::default()
            },
            text_style: TextStyle {
                font_size: 14.0,
                ..Default::default()
            },
            ..Default::default()
        },
        preserve_original_size: true,
        background_fill_enabled: true,
    };

    inpaint_text_region(&mut image, &footer_region, &footer_config, renderer)?;

    ImageTextRenderer::save_image(&image, "output/inpainting/real_world_simulation.png")?;
    println!("  ✅ Real-world simulation created");

    Ok(())
}

/// Perform text inpainting on a specific region
fn inpaint_text_region(
    image: &mut image::RgbaImage,
    region: &DetectedTextRegion,
    config: &InpaintingConfig,
    renderer: &ImageTextRenderer,
) -> Result<(), Box<dyn std::error::Error>> {
    let text_box = TextBox::new(region.x, region.y, region.width, region.height);

    // Render the replacement text
    let result = renderer.render_text(
        image,
        &config.replacement_text,
        text_box,
        &config.render_config,
    )?;

    // Log the result for debugging
    println!(
        "    Inpainted region {}x{} with {} lines at {:.1}px font size",
        region.width,
        region.height,
        result.metrics.lines.len(),
        result.metrics.font_size
    );

    Ok(())
}

/// Create a simulated document image background
fn create_simulated_document_image(width: u32, height: u32) -> image::RgbaImage {
    let mut image = ImageTextRenderer::create_image(width, height, Color::new(248, 248, 248, 255));

    // Add some subtle background texture/pattern
    for y in 0..height {
        for x in 0..width {
            if (x + y) % 50 == 0 {
                let pixel = Color::new(240, 240, 240, 255).to_rgba();
                image.put_pixel(x, y, pixel);
            }
        }
    }

    image
}

/// Create layout demonstration when fonts are not available
fn create_layout_demonstration() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📐 Creating layout demonstration...");

    let mut image = create_simulated_document_image(800, 600);

    // Show where text regions would be inpainted
    let regions = vec![
        (50, 50, 700, 80, Color::new(255, 200, 200, 255)), // Header
        (50, 150, 650, 120, Color::new(200, 255, 200, 255)), // Paragraph 1
        (50, 290, 600, 90, Color::new(200, 200, 255, 255)), // Paragraph 2
    ];

    for (x, y, width, height, color) in regions {
        for py in y..y + height {
            for px in x..x + width {
                if px < image.width() && py < image.height() {
                    image.put_pixel(px, py, color.to_rgba());
                }
            }
        }
    }

    ImageTextRenderer::save_image(&image, "output/inpainting/layout_demo.png")?;
    println!("  ✅ Layout demonstration created");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detected_text_region_creation() {
        let region = DetectedTextRegion {
            x: 10,
            y: 20,
            width: 300,
            height: 100,
            confidence: 0.95,
            original_text: "Test text".to_string(),
        };

        assert_eq!(region.x, 10);
        assert_eq!(region.y, 20);
        assert_eq!(region.width, 300);
        assert_eq!(region.height, 100);
        assert_eq!(region.confidence, 0.95);
        assert_eq!(region.original_text, "Test text");
    }

    #[test]
    fn test_inpainting_config_creation() {
        let config = InpaintingConfig {
            replacement_text: "New text".to_string(),
            render_config: TextRenderConfig::default(),
            preserve_original_size: true,
            background_fill_enabled: false,
        };

        assert_eq!(config.replacement_text, "New text");
        assert!(config.preserve_original_size);
        assert!(!config.background_fill_enabled);
    }

    #[test]
    fn test_simulated_document_creation() {
        let image = create_simulated_document_image(200, 100);
        assert_eq!(image.width(), 200);
        assert_eq!(image.height(), 100);
    }
}
