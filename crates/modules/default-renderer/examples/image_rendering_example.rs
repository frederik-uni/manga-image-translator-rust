//! # Image Rendering Example
//!
//! This example demonstrates how to render text onto images using the ImageTextRenderer
//! with various configurations including drop shadows, different colors, and text fitting.

use default_renderer::{
    get_size::{ScriptType, TextFittingConfig, TextStyle, WrapStrategy},
    image_renderer::{
        Color, DropShadowConfig, ImageTextRenderer, TextAlignment, TextBox, TextRenderConfig,
    },
};

// Mock font data for demonstration (a minimal valid TTF header)
// In real usage, you would load actual font files
const MOCK_FONT_DATA: &[u8] = &[
    0x00, 0x01, 0x00, 0x00, // sfnt version
    0x00, 0x0C, // numTables
    0x00, 0x80, // searchRange
    0x00, 0x03, // entrySelector
    0x00,
    0x20, // rangeShift
          // Table directory entries would follow...
          // This is a minimal mock - in practice, use real font files
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎨 Image Text Rendering Example");
    println!("================================");

    // Create output directory
    std::fs::create_dir_all("output")?;

    // Initialize the renderer
    let mut renderer = ImageTextRenderer::new()?;

    // Load a mock font (in real usage, load actual font files)
    match renderer.add_font(MOCK_FONT_DATA, 400) {
        Ok(_) => println!("✅ Font loaded successfully"),
        Err(e) => {
            println!("⚠️  Font loading failed: {}", e);
            println!("   Creating examples without font rendering...");
            create_color_examples()?;
            return Ok(());
        }
    }

    // Create various text rendering examples
    create_basic_examples(&renderer)?;
    create_drop_shadow_examples(&renderer)?;
    create_multi_line_examples(&renderer)?;
    create_color_variation_examples(&renderer)?;
    create_spacing_examples(&renderer)?;

    println!("\n🎉 All examples created successfully!");
    println!("📁 Check the 'output' directory for generated images");

    Ok(())
}

/// Create basic text rendering examples
fn create_basic_examples(renderer: &ImageTextRenderer) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📝 Creating basic text examples...");

    // Example 1: Simple white text on black background with green text box
    let mut image = ImageTextRenderer::create_image(800, 600, Color::black());
    let text_box = TextBox::new(50, 50, 700, 100);

    let config = TextRenderConfig {
        text_color: Color::white(),
        background_color: Color::black(),
        box_color: Color::green(),
        drop_shadow: DropShadowConfig::default(),
        character_spacing: 0.0,
        line_height_multiplier: 1.0,
        alignment: TextAlignment::default(),
        fitting_config: TextFittingConfig {
            script_type: ScriptType::Latin,
            wrap_strategy: Some(WrapStrategy::Word),
            max_width: 700.0,
            max_height: 100.0,
            min_font_size: 12.0,
            max_font_size: 48.0,
            font_size_step: 1.0,
        },
        text_style: TextStyle {
            font_size: 24.0,
            line_height: 1.2,
            letter_spacing: 0.0,
            font_weight: 400,
        },
    };

    let result = renderer.render_text(
        &mut image,
        "Hello, World! This is a basic text rendering example.",
        text_box,
        &config,
    )?;
    ImageTextRenderer::save_image(&image, "output/basic_example.png")?;

    println!(
        "  ✅ Basic example: {} lines, font size: {:.1}px",
        result.metrics.lines.len(),
        result.metrics.font_size
    );

    Ok(())
}

/// Create drop shadow examples
fn create_drop_shadow_examples(
    renderer: &ImageTextRenderer,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🌑 Creating drop shadow examples...");

    // Example 1: Text with subtle drop shadow
    let mut image = ImageTextRenderer::create_image(800, 400, Color::black());
    let text_box = TextBox::new(50, 50, 700, 150);

    let config = TextRenderConfig {
        text_color: Color::white(),
        box_color: Color::green(),
        drop_shadow: DropShadowConfig {
            enabled: true,
            color: Color::new(128, 128, 128, 200), // Semi-transparent gray
            expansion: 2,
            blur_radius: 0,
        },
        fitting_config: TextFittingConfig {
            max_width: 700.0,
            max_height: 150.0,
            ..Default::default()
        },
        text_style: TextStyle {
            font_size: 32.0,
            ..Default::default()
        },
        ..Default::default()
    };

    renderer.render_text(&mut image, "Drop Shadow Example", text_box, &config)?;
    ImageTextRenderer::save_image(&image, "output/drop_shadow_example.png")?;

    // Example 2: Text with heavy drop shadow
    let mut image2 = ImageTextRenderer::create_image(800, 400, Color::black());
    let text_box2 = TextBox::new(50, 200, 700, 150);

    let config2 = TextRenderConfig {
        drop_shadow: DropShadowConfig {
            enabled: true,
            color: Color::new(255, 0, 0, 150), // Semi-transparent red
            expansion: 5,
            blur_radius: 0,
        },
        ..config
    };

    renderer.render_text(&mut image2, "Heavy Shadow Effect", text_box2, &config2)?;
    ImageTextRenderer::save_image(&image2, "output/heavy_shadow_example.png")?;

    println!("  ✅ Drop shadow examples created");

    Ok(())
}

/// Create multi-line text examples
fn create_multi_line_examples(
    renderer: &ImageTextRenderer,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📄 Creating multi-line text examples...");

    let mut image = ImageTextRenderer::create_image(600, 800, Color::black());
    let text_box = TextBox::new(50, 50, 500, 700);

    let long_text = "This is a comprehensive example of multi-line text rendering. \
                     The text fitting algorithm automatically determines the optimal font size \
                     and breaks the text into multiple lines to fit within the specified constraints. \
                     This demonstrates word wrapping, line height control, and proper text fitting \
                     for longer passages of text.";

    let config = TextRenderConfig {
        text_color: Color::white(),
        box_color: Color::green(),
        line_height_multiplier: 1.4, // Increased line spacing
        fitting_config: TextFittingConfig {
            script_type: ScriptType::Latin,
            wrap_strategy: Some(WrapStrategy::Word),
            max_width: 500.0,
            max_height: 700.0,
            min_font_size: 10.0,
            max_font_size: 24.0,
            font_size_step: 0.5,
        },
        ..Default::default()
    };

    let result = renderer.render_text(&mut image, long_text, text_box, &config)?;
    ImageTextRenderer::save_image(&image, "output/multiline_example.png")?;

    println!(
        "  ✅ Multi-line example: {} lines, font size: {:.1}px",
        result.metrics.lines.len(),
        result.metrics.font_size
    );

    Ok(())
}

/// Create color variation examples
fn create_color_variation_examples(
    renderer: &ImageTextRenderer,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🎨 Creating color variation examples...");

    let color_schemes = vec![
        (
            "blue_theme",
            Color::new(0, 100, 200, 255),
            Color::new(173, 216, 230, 255),
            Color::white(),
        ),
        (
            "red_theme",
            Color::new(139, 0, 0, 255),
            Color::new(255, 182, 193, 255),
            Color::white(),
        ),
        (
            "purple_theme",
            Color::new(75, 0, 130, 255),
            Color::new(221, 160, 221, 255),
            Color::white(),
        ),
        (
            "dark_theme",
            Color::new(40, 40, 40, 255),
            Color::new(100, 100, 100, 255),
            Color::new(220, 220, 220, 255),
        ),
    ];

    for (_i, (name, bg_color, box_color, text_color)) in color_schemes.iter().enumerate() {
        let mut image = ImageTextRenderer::create_image(600, 200, *bg_color);
        let text_box = TextBox::new(50, 50, 500, 100);

        let config = TextRenderConfig {
            text_color: *text_color,
            background_color: *bg_color,
            box_color: *box_color,
            fitting_config: TextFittingConfig {
                max_width: 500.0,
                max_height: 100.0,
                ..Default::default()
            },
            text_style: TextStyle {
                font_size: 28.0,
                ..Default::default()
            },
            ..Default::default()
        };

        let text = format!("Color Theme: {}", name.replace('_', " ").to_uppercase());
        renderer.render_text(&mut image, &text, text_box, &config)?;

        let filename = format!("output/color_{}.png", name);
        ImageTextRenderer::save_image(&image, &filename)?;
    }

    println!("  ✅ Color variation examples created");

    Ok(())
}

/// Create character and line spacing examples
fn create_spacing_examples(renderer: &ImageTextRenderer) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📏 Creating spacing examples...");

    let spacing_configs = vec![
        ("normal", 0.0, 1.0),
        ("wide_chars", 3.0, 1.0),
        ("tall_lines", 0.0, 1.8),
        ("wide_and_tall", 2.0, 1.6),
    ];

    for (name, char_spacing, line_height) in spacing_configs {
        let mut image = ImageTextRenderer::create_image(800, 300, Color::black());
        let text_box = TextBox::new(50, 50, 700, 200);

        let config = TextRenderConfig {
            text_color: Color::white(),
            box_color: Color::green(),
            character_spacing: char_spacing,
            line_height_multiplier: line_height,
            fitting_config: TextFittingConfig {
                max_width: 700.0,
                max_height: 200.0,
                ..Default::default()
            },
            text_style: TextStyle {
                font_size: 20.0,
                ..Default::default()
            },
            ..Default::default()
        };

        let text = format!(
            "Spacing Example: {}\nCharacter spacing: {:.1}px\nLine height: {:.1}x",
            name.replace('_', " "),
            char_spacing,
            line_height
        );

        renderer.render_text(&mut image, &text, text_box, &config)?;

        let filename = format!("output/spacing_{}.png", name);
        ImageTextRenderer::save_image(&image, &filename)?;
    }

    println!("  ✅ Spacing examples created");

    Ok(())
}

/// Create color examples without font rendering (fallback for when fonts fail to load)
fn create_color_examples() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🎨 Creating color demonstration examples...");

    // Create a simple color pattern to show the intended layout
    let mut image = ImageTextRenderer::create_image(800, 600, Color::black());

    // Draw green rectangles to show where text boxes would be
    let text_boxes = vec![
        TextBox::new(50, 50, 700, 100),
        TextBox::new(50, 200, 700, 150),
        TextBox::new(50, 400, 700, 150),
    ];

    for text_box in text_boxes {
        for y in text_box.y..text_box.y + text_box.height {
            for x in text_box.x..text_box.x + text_box.width {
                if x < image.width() && y < image.height() {
                    image.put_pixel(x, y, Color::green().to_rgba());
                }
            }
        }
    }

    ImageTextRenderer::save_image(&image, "output/color_layout_example.png")?;
    println!("  ✅ Color layout example created (showing intended text box areas)");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_execution() {
        // Test that the example code compiles and basic functions work
        let renderer = ImageTextRenderer::new();
        assert!(renderer.is_ok());

        let image = ImageTextRenderer::create_image(100, 100, Color::black());
        assert_eq!(image.width(), 100);
        assert_eq!(image.height(), 100);
    }

    #[test]
    fn test_color_creation() {
        let color = Color::new(255, 128, 64, 200);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 64);
        assert_eq!(color.a, 200);
    }

    #[test]
    fn test_text_box_creation() {
        let text_box = TextBox::new(10, 20, 300, 200);
        assert_eq!(text_box.x, 10);
        assert_eq!(text_box.y, 20);
        assert_eq!(text_box.width, 300);
        assert_eq!(text_box.height, 200);
    }
}
