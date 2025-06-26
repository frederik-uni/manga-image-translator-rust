//! # Simple Image Rendering Example
//!
//! A focused example demonstrating text rendering onto images with various features.

use default_renderer::{
    get_size::{ScriptType, TextFittingConfig, TextStyle, WrapStrategy},
    image_renderer::{
        Color, DropShadowConfig, ImageTextRenderer, TextAlignment, TextBox, TextRenderConfig,
    },
};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎨 Simple Image Text Rendering Example");
    println!("======================================");

    // Create output directory
    fs::create_dir_all("output")?;

    // Initialize the renderer and load font
    let mut renderer = ImageTextRenderer::new()?;
    let font_loaded = load_system_font(&mut renderer);

    if !font_loaded {
        println!("⚠️  No system font available - creating layout demonstration");
        create_layout_examples()?;
        return Ok(());
    }

    println!("✅ Font loaded successfully");

    // Create demonstration scenarios
    create_text_examples(&renderer)?;

    println!("\n🎉 Example completed!");
    println!("📁 Check the 'output' directory for generated images");

    Ok(())
}

/// Try to load a system font
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

/// Create text rendering examples with actual fonts
fn create_text_examples(renderer: &ImageTextRenderer) -> Result<(), Box<dyn std::error::Error>> {
    // Example 1: Basic white text on black background with green text box
    create_basic_example(renderer)?;

    // Example 2: Text with drop shadow
    create_shadow_example(renderer)?;

    // Example 3: Multi-line text with custom spacing
    create_multiline_example(renderer)?;

    // Example 4: Text alignment demonstrations
    create_alignment_examples(renderer)?;

    // Example 5: Color variations
    create_color_examples(renderer)?;

    Ok(())
}

/// Create a basic text rendering example
fn create_basic_example(renderer: &ImageTextRenderer) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📝 Creating basic example...");

    let mut image = ImageTextRenderer::create_image(800, 200, Color::black());
    let text_box = TextBox::new(50, 50, 700, 100);

    let config = TextRenderConfig {
        text_color: Color::white(),
        background_color: Color::black(),
        box_color: Color::green(),
        alignment: TextAlignment::CenterCenter,
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
            font_size: 32.0,
            line_height: 1.2,
            letter_spacing: 0.0,
            font_weight: 400,
        },
        ..Default::default()
    };

    let result = renderer.render_text(
        &mut image,
        "Hello, World! Simple Text Rendering",
        text_box,
        &config,
    )?;

    ImageTextRenderer::save_image(&image, "output/basic_text.png")?;

    println!(
        "  ✅ Basic example: font size {:.1}px",
        result.metrics.font_size
    );
    Ok(())
}

/// Create a drop shadow example
fn create_shadow_example(renderer: &ImageTextRenderer) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🌑 Creating drop shadow example...");

    let mut image = ImageTextRenderer::create_image(800, 200, Color::black());
    let text_box = TextBox::new(50, 50, 700, 100);

    let config = TextRenderConfig {
        text_color: Color::white(),
        box_color: Color::green(),
        alignment: TextAlignment::CenterCenter,
        drop_shadow: DropShadowConfig {
            enabled: true,
            color: Color::new(128, 128, 128, 180),
            expansion: 3,
            blur_radius: 0,
        },
        fitting_config: TextFittingConfig {
            max_width: 700.0,
            max_height: 100.0,
            ..Default::default()
        },
        text_style: TextStyle {
            font_size: 36.0,
            ..Default::default()
        },
        ..Default::default()
    };

    renderer.render_text(&mut image, "Text with Drop Shadow", text_box, &config)?;
    ImageTextRenderer::save_image(&image, "output/shadow_text.png")?;

    println!("  ✅ Drop shadow example created");
    Ok(())
}

/// Create a multi-line text example
fn create_multiline_example(
    renderer: &ImageTextRenderer,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📄 Creating multi-line example...");

    let mut image = ImageTextRenderer::create_image(600, 400, Color::black());
    let text_box = TextBox::new(50, 50, 500, 300);

    let long_text = "This demonstrates multi-line text rendering with automatic line breaking. \
                     The text fitting algorithm finds the optimal font size to fit all text \
                     within the specified dimensions while maintaining readability.";

    let config = TextRenderConfig {
        text_color: Color::white(),
        box_color: Color::green(),
        alignment: TextAlignment::CenterCenter,
        character_spacing: 1.0,
        line_height_multiplier: 1.4,
        fitting_config: TextFittingConfig {
            script_type: ScriptType::Latin,
            wrap_strategy: Some(WrapStrategy::Word),
            max_width: 500.0,
            max_height: 300.0,
            min_font_size: 10.0,
            max_font_size: 24.0,
            font_size_step: 0.5,
        },
        ..Default::default()
    };

    let result = renderer.render_text(&mut image, long_text, text_box, &config)?;
    ImageTextRenderer::save_image(&image, "output/multiline_text.png")?;

    println!(
        "  ✅ Multi-line example: {} lines",
        result.metrics.lines.len()
    );
    Ok(())
}

/// Create text alignment examples
fn create_alignment_examples(
    renderer: &ImageTextRenderer,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🎯 Creating text alignment examples...");

    // Create a grid showing all alignment options
    let mut image = ImageTextRenderer::create_image(900, 600, Color::black());

    let alignments = vec![
        (TextAlignment::TopLeft, "TopLeft", 50, 50),
        (TextAlignment::TopCenter, "TopCenter", 350, 50),
        (TextAlignment::TopRight, "TopRight", 650, 50),
        (TextAlignment::CenterLeft, "CenterLeft", 50, 200),
        (TextAlignment::CenterCenter, "CenterCenter", 350, 200),
        (TextAlignment::CenterRight, "CenterRight", 650, 200),
        (TextAlignment::BottomLeft, "BottomLeft", 50, 350),
        (TextAlignment::BottomCenter, "BottomCenter", 350, 350),
        (TextAlignment::BottomRight, "BottomRight", 650, 350),
    ];

    for (alignment, name, x, y) in alignments {
        let text_box = TextBox::new(x, y, 200, 120);

        let config = TextRenderConfig {
            text_color: Color::white(),
            box_color: Color::new(0, 100, 200, 100), // Semi-transparent blue
            alignment,
            fitting_config: TextFittingConfig {
                max_width: 200.0,
                max_height: 120.0,
                min_font_size: 10.0,
                max_font_size: 18.0,
                ..Default::default()
            },
            text_style: TextStyle {
                font_size: 16.0,
                ..Default::default()
            },
            ..Default::default()
        };

        let text = format!("{}\nAlignment\nExample", name);
        renderer.render_text(&mut image, &text, text_box, &config)?;
    }

    ImageTextRenderer::save_image(&image, "output/alignment_examples.png")?;
    println!("  ✅ Text alignment examples created");

    // Create a focused center-center example
    let mut center_image = ImageTextRenderer::create_image(600, 400, Color::black());
    let center_box = TextBox::new(100, 100, 400, 200);

    let center_config = TextRenderConfig {
        text_color: Color::white(),
        box_color: Color::green(),
        alignment: TextAlignment::CenterCenter,
        character_spacing: 1.0,
        line_height_multiplier: 1.3,
        fitting_config: TextFittingConfig {
            max_width: 400.0,
            max_height: 200.0,
            min_font_size: 12.0,
            max_font_size: 24.0,
            ..Default::default()
        },
        ..Default::default()
    };

    renderer.render_text(
        &mut center_image,
        "Perfectly Centered Text\nWith Proper Baseline\nAlignment",
        center_box,
        &center_config,
    )?;

    ImageTextRenderer::save_image(&center_image, "output/center_aligned_text.png")?;
    println!("  ✅ Center-aligned text example created");

    Ok(())
}

/// Create color variation examples
fn create_color_examples(renderer: &ImageTextRenderer) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🎨 Creating color examples...");

    // Blue theme with center alignment
    let mut image1 = ImageTextRenderer::create_image(600, 150, Color::new(25, 25, 112, 255));
    let text_box1 = TextBox::new(50, 25, 500, 100);

    let blue_config = TextRenderConfig {
        text_color: Color::white(),
        background_color: Color::new(25, 25, 112, 255),
        box_color: Color::new(70, 130, 180, 255),
        alignment: TextAlignment::CenterCenter,
        text_style: TextStyle {
            font_size: 28.0,
            ..Default::default()
        },
        fitting_config: TextFittingConfig {
            max_width: 500.0,
            max_height: 100.0,
            ..Default::default()
        },
        ..Default::default()
    };

    renderer.render_text(&mut image1, "Blue Theme Example", text_box1, &blue_config)?;
    ImageTextRenderer::save_image(&image1, "output/blue_theme.png")?;

    // Red theme
    let mut image2 = ImageTextRenderer::create_image(600, 150, Color::new(139, 0, 0, 255));
    let text_box2 = TextBox::new(50, 25, 500, 100);

    let red_config = TextRenderConfig {
        text_color: Color::white(),
        background_color: Color::new(139, 0, 0, 255),
        box_color: Color::new(220, 20, 60, 255),
        alignment: TextAlignment::CenterCenter,
        text_style: TextStyle {
            font_size: 28.0,
            ..Default::default()
        },
        fitting_config: TextFittingConfig {
            max_width: 500.0,
            max_height: 100.0,
            ..Default::default()
        },
        ..Default::default()
    };

    renderer.render_text(&mut image2, "Red Theme Example", text_box2, &red_config)?;
    ImageTextRenderer::save_image(&image2, "output/red_theme.png")?;

    println!("  ✅ Color examples created");
    Ok(())
}

/// Create layout examples when no fonts are available
fn create_layout_examples() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📐 Creating layout examples...");

    // Create an image showing the intended layout structure
    let mut image = ImageTextRenderer::create_image(800, 600, Color::black());

    // Draw text boxes to show layout
    let text_boxes = vec![
        TextBox::new(50, 50, 700, 80),   // Header box
        TextBox::new(50, 150, 700, 120), // Content box 1
        TextBox::new(50, 290, 700, 120), // Content box 2
        TextBox::new(50, 430, 700, 120), // Content box 3
    ];

    for (i, text_box) in text_boxes.iter().enumerate() {
        let color = match i {
            0 => Color::new(0, 255, 0, 255),     // Green
            1 => Color::new(0, 200, 255, 255),   // Blue
            2 => Color::new(255, 200, 0, 255),   // Yellow
            3 => Color::new(255, 100, 100, 255), // Red
            _ => Color::green(),
        };

        // Fill the text box area
        for y in text_box.y..text_box.y + text_box.height {
            for x in text_box.x..text_box.x + text_box.width {
                if x < image.width() && y < image.height() {
                    image.put_pixel(x, y, color.to_rgba());
                }
            }
        }
    }

    ImageTextRenderer::save_image(&image, "output/layout_example.png")?;
    println!("  ✅ Layout example created (colored rectangles show text box areas)");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_renderer_creation() {
        let renderer = ImageTextRenderer::new();
        assert!(renderer.is_ok());
    }

    #[test]
    fn test_image_creation() {
        let image = ImageTextRenderer::create_image(100, 100, Color::black());
        assert_eq!(image.width(), 100);
        assert_eq!(image.height(), 100);
    }

    #[test]
    fn test_text_box() {
        let text_box = TextBox::new(10, 20, 300, 200);
        assert_eq!(text_box.x, 10);
        assert_eq!(text_box.y, 20);
        assert_eq!(text_box.width, 300);
        assert_eq!(text_box.height, 200);
    }

    #[test]
    fn test_alignment_options() {
        let alignments = vec![
            TextAlignment::TopLeft,
            TextAlignment::CenterCenter,
            TextAlignment::BottomRight,
        ];
        assert_eq!(alignments.len(), 3);
    }
}
