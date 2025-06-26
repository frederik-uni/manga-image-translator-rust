//! # Baseline and Bounds Testing Example
//!
//! This example specifically tests proper baseline alignment and bounds checking
//! to ensure text renders correctly within text boxes with proper typography.

use default_renderer::{
    get_size::TextFittingConfig,
    image_renderer::{Color, ImageTextRenderer, TextAlignment, TextBox, TextRenderConfig},
};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Baseline and Bounds Testing");
    println!("==============================");

    // Create output directory
    fs::create_dir_all("output/baseline_test")?;

    // Initialize the renderer and load font
    let mut renderer = ImageTextRenderer::new()?;
    let font_loaded = load_system_font(&mut renderer);

    if !font_loaded {
        println!("⚠️  No system font available");
        return Ok(());
    }

    println!("✅ Font loaded successfully");

    // Run tests
    test_baseline_alignment(&renderer)?;
    test_bounds_checking(&renderer)?;
    test_descender_positioning(&renderer)?;
    test_edge_cases(&renderer)?;

    println!("\n🎉 All baseline and bounds tests completed!");
    println!("📁 Check the 'output/baseline_test' directory for test images");

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

/// Test proper baseline alignment with descenders
fn test_baseline_alignment(renderer: &ImageTextRenderer) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📏 Testing baseline alignment...");

    let mut image = ImageTextRenderer::create_image(800, 600, Color::new(250, 250, 250, 255));

    // Draw baseline reference lines
    draw_baseline_guides(&mut image, vec![120, 220, 320, 420, 520]);

    // Test 1: Letters with descenders should align properly on baseline
    let test_texts = vec![
        ("Baseline Test: gjpqy", 50, 100, "Descender letters"),
        ("UPPERCASE ONLY", 50, 200, "Uppercase letters"),
        ("Mixed Case: Hgjpqy", 50, 300, "Mixed case"),
        ("Numbers: 123456789", 50, 400, "Numbers and digits"),
        ("Symbols: ()[]{}!@#", 50, 500, "Symbols and punctuation"),
    ];

    for (text, x, y, description) in test_texts {
        // Draw text box outline
        draw_text_box_outline(&mut image, x, y, 700, 80);

        let text_box = TextBox::new(x, y, 700, 80);
        let config = TextRenderConfig {
            text_color: Color::new(40, 40, 40, 255),
            box_color: Color::new(255, 255, 255, 180),
            alignment: TextAlignment::CenterLeft,
            fitting_config: TextFittingConfig {
                max_width: 700.0,
                max_height: 80.0,
                min_font_size: 20.0,
                max_font_size: 32.0,
                ..Default::default()
            },
            ..Default::default()
        };

        renderer.render_text(&mut image, text, text_box, &config)?;

        // Add description label
        let label_box = TextBox::new(x + 710, y + 20, 80, 40);
        let label_config = TextRenderConfig {
            text_color: Color::new(100, 100, 100, 255),
            box_color: Color::transparent(),
            alignment: TextAlignment::CenterLeft,
            fitting_config: TextFittingConfig {
                max_width: 80.0,
                max_height: 40.0,
                min_font_size: 8.0,
                max_font_size: 12.0,
                ..Default::default()
            },
            ..Default::default()
        };

        renderer.render_text(&mut image, description, label_box, &label_config)?;
    }

    ImageTextRenderer::save_image(&image, "output/baseline_test/baseline_alignment.png")?;
    println!("  ✅ Baseline alignment test completed");

    Ok(())
}

/// Test bounds checking to ensure text stays within text boxes
fn test_bounds_checking(renderer: &ImageTextRenderer) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔲 Testing bounds checking...");

    let mut image = ImageTextRenderer::create_image(800, 600, Color::new(240, 240, 240, 255));

    // Test different positions that might cause overflow
    let bounds_tests = vec![
        (TextAlignment::TopLeft, 50, 50, 200, 100, "Top Left"),
        (TextAlignment::TopRight, 300, 50, 200, 100, "Top Right"),
        (TextAlignment::BottomLeft, 550, 50, 200, 100, "Bottom Left"),
        (
            TextAlignment::BottomRight,
            50,
            200,
            200,
            100,
            "Bottom Right",
        ),
        (TextAlignment::CenterCenter, 300, 200, 200, 100, "Center"),
        (TextAlignment::TopCenter, 550, 200, 200, 100, "Top Center"),
        // Edge cases - small boxes that would normally cause overflow
        (TextAlignment::BottomLeft, 50, 350, 150, 60, "Small Box 1"),
        (TextAlignment::BottomRight, 250, 350, 150, 60, "Small Box 2"),
        (
            TextAlignment::CenterCenter,
            450,
            350,
            150,
            60,
            "Small Box 3",
        ),
        // Very constrained boxes
        (TextAlignment::TopLeft, 50, 450, 100, 40, "Tiny 1"),
        (TextAlignment::CenterCenter, 200, 450, 100, 40, "Tiny 2"),
        (TextAlignment::BottomRight, 350, 450, 100, 40, "Tiny 3"),
    ];

    for (alignment, x, y, width, height, label) in bounds_tests {
        // Draw text box with bright border to show bounds
        draw_text_box_border(&mut image, x, y, width, height, Color::new(255, 0, 0, 255));

        let text_box = TextBox::new(x, y, width, height);
        let config = TextRenderConfig {
            text_color: Color::new(20, 20, 20, 255),
            box_color: Color::new(255, 255, 255, 200),
            alignment,
            fitting_config: TextFittingConfig {
                max_width: width as f32,
                max_height: height as f32,
                min_font_size: 8.0,
                max_font_size: 24.0,
                ..Default::default()
            },
            ..Default::default()
        };

        let text = format!("{}:\nText with gjpqy", label);
        renderer.render_text(&mut image, &text, text_box, &config)?;
    }

    ImageTextRenderer::save_image(&image, "output/baseline_test/bounds_checking.png")?;
    println!("  ✅ Bounds checking test completed");

    Ok(())
}

/// Test specific descender positioning
fn test_descender_positioning(
    renderer: &ImageTextRenderer,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📐 Testing descender positioning...");

    let mut image = ImageTextRenderer::create_image(800, 400, Color::white());

    // Create a detailed test of descender alignment
    let descender_tests = vec![
        ("BASELINE", 50, 80, "No descenders"),
        ("baseline", 250, 80, "All lowercase"),
        ("BaseLinE", 450, 80, "Mixed case"),
        ("gjpqy", 650, 80, "Heavy descenders"),
        ("Typography", 50, 180, "Normal word"),
        ("programming", 250, 180, "With 'g' descender"),
        ("Pygmy", 450, 180, "Mixed with 'gy'"),
        ("()[]{}jp", 650, 180, "Symbols + descenders"),
    ];

    for (text, x, y, description) in descender_tests {
        // Draw precise baseline guide
        let baseline_y = y + 40;
        for guide_x in x..(x + 150) {
            if guide_x < image.width() && baseline_y < image.height() {
                image.put_pixel(guide_x, baseline_y, Color::new(255, 0, 0, 255).to_rgba());
            }
        }

        // Draw text box
        draw_text_box_border(&mut image, x, y, 150, 80, Color::new(0, 0, 255, 128));

        let text_box = TextBox::new(x, y, 150, 80);
        let config = TextRenderConfig {
            text_color: Color::new(0, 0, 0, 255),
            box_color: Color::new(255, 255, 255, 100),
            alignment: TextAlignment::TopLeft,
            fitting_config: TextFittingConfig {
                max_width: 150.0,
                max_height: 80.0,
                min_font_size: 16.0,
                max_font_size: 20.0,
                ..Default::default()
            },
            ..Default::default()
        };

        renderer.render_text(&mut image, text, text_box, &config)?;

        // Add description below
        let desc_box = TextBox::new(x, y + 85, 150, 30);
        let desc_config = TextRenderConfig {
            text_color: Color::new(100, 100, 100, 255),
            box_color: Color::transparent(),
            alignment: TextAlignment::TopCenter,
            fitting_config: TextFittingConfig {
                max_width: 150.0,
                max_height: 30.0,
                min_font_size: 8.0,
                max_font_size: 12.0,
                ..Default::default()
            },
            ..Default::default()
        };

        renderer.render_text(&mut image, description, desc_box, &desc_config)?;
    }

    ImageTextRenderer::save_image(&image, "output/baseline_test/descender_positioning.png")?;
    println!("  ✅ Descender positioning test completed");

    Ok(())
}

/// Test edge cases and error conditions
fn test_edge_cases(renderer: &ImageTextRenderer) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🚨 Testing edge cases...");

    let mut image = ImageTextRenderer::create_image(600, 500, Color::new(245, 245, 245, 255));

    // Edge case tests
    let edge_cases = vec![
        // Bottom positioning that might overflow
        (
            TextAlignment::BottomLeft,
            50,
            400,
            200,
            80,
            "Bottom positioning",
        ),
        (
            TextAlignment::BottomCenter,
            300,
            400,
            200,
            80,
            "Bottom center",
        ),
        // Very small text boxes
        (
            TextAlignment::CenterCenter,
            50,
            50,
            80,
            30,
            "Very small box",
        ),
        (TextAlignment::TopLeft, 200, 50, 80, 30, "Small top-left"),
        // Large font in small box (should be constrained)
        (
            TextAlignment::CenterCenter,
            350,
            50,
            150,
            50,
            "Constrained font",
        ),
        // Multi-line in small vertical space
        (TextAlignment::TopLeft, 50, 150, 200, 60, "Multi-line small"),
        (
            TextAlignment::CenterCenter,
            300,
            150,
            200,
            60,
            "Multi-line center",
        ),
        // Text at image edges
        (TextAlignment::TopRight, 400, 250, 180, 80, "Near edge"),
        (
            TextAlignment::BottomRight,
            50,
            250,
            180,
            80,
            "Bottom near edge",
        ),
    ];

    for (alignment, x, y, width, height, description) in edge_cases {
        // Draw text box with warning color for edge cases
        draw_text_box_border(
            &mut image,
            x,
            y,
            width,
            height,
            Color::new(255, 165, 0, 255),
        );

        let text_box = TextBox::new(x, y, width, height);
        let config = TextRenderConfig {
            text_color: Color::new(50, 50, 50, 255),
            box_color: Color::new(255, 255, 255, 180),
            alignment,
            line_height_multiplier: 1.2,
            fitting_config: TextFittingConfig {
                max_width: width as f32,
                max_height: height as f32,
                min_font_size: 8.0,
                max_font_size: 20.0,
                ..Default::default()
            },
            ..Default::default()
        };

        let test_text = match description {
            "Multi-line small" | "Multi-line center" => "Multiple\nlines of\ntext here",
            "Constrained font" => "BIG TEXT",
            _ => "Test gjpqy",
        };

        renderer.render_text(&mut image, test_text, text_box, &config)?;

        // Add label
        let label_y = if y > 350 { y - 20 } else { y + height + 5 };
        let label_box = TextBox::new(x, label_y, width, 15);
        let label_config = TextRenderConfig {
            text_color: Color::new(80, 80, 80, 255),
            box_color: Color::transparent(),
            alignment: TextAlignment::TopCenter,
            fitting_config: TextFittingConfig {
                max_width: width as f32,
                max_height: 15.0,
                min_font_size: 8.0,
                max_font_size: 10.0,
                ..Default::default()
            },
            ..Default::default()
        };

        renderer.render_text(&mut image, description, label_box, &label_config)?;
    }

    ImageTextRenderer::save_image(&image, "output/baseline_test/edge_cases.png")?;
    println!("  ✅ Edge cases test completed");

    Ok(())
}

/// Draw baseline guide lines
fn draw_baseline_guides(image: &mut image::RgbaImage, y_positions: Vec<u32>) {
    let guide_color = Color::new(255, 0, 0, 180).to_rgba(); // Red baseline guides

    for y in y_positions {
        if y < image.height() {
            for x in 0..image.width() {
                // Draw dotted line
                if x % 10 < 5 {
                    image.put_pixel(x, y + 20, guide_color); // Offset for proper baseline position
                }
            }
        }
    }
}

/// Draw text box outline
fn draw_text_box_outline(image: &mut image::RgbaImage, x: u32, y: u32, width: u32, height: u32) {
    let outline_color = Color::new(200, 200, 200, 255).to_rgba();
    draw_text_box_border(image, x, y, width, height, Color::new(200, 200, 200, 255));
}

/// Draw text box border with specified color
fn draw_text_box_border(
    image: &mut image::RgbaImage,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    color: Color,
) {
    let border_color = color.to_rgba();

    // Draw border lines
    for i in 0..width {
        // Top and bottom borders
        if x + i < image.width() {
            if y < image.height() {
                image.put_pixel(x + i, y, border_color);
            }
            if y + height < image.height() {
                image.put_pixel(x + i, y + height - 1, border_color);
            }
        }
    }

    for i in 0..height {
        // Left and right borders
        if y + i < image.height() {
            if x < image.width() {
                image.put_pixel(x, y + i, border_color);
            }
            if x + width < image.width() {
                image.put_pixel(x + width - 1, y + i, border_color);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_baseline_test_creation() {
        let renderer = ImageTextRenderer::new();
        assert!(renderer.is_ok());
    }

    #[test]
    fn test_bounds_helpers() {
        let mut image = ImageTextRenderer::create_image(100, 100, Color::white());
        draw_text_box_border(&mut image, 10, 10, 50, 50, Color::new(255, 0, 0, 255));
        // Should not panic
    }
}
