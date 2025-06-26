//! # Baseline Alignment Comparison Example
//!
//! This example demonstrates proper text baseline alignment and positioning,
//! showing how text should sit on a consistent baseline rather than jumping
//! up and down between characters.

use default_renderer::{
    get_size::TextFittingConfig,
    image_renderer::{
        Color, DropShadowConfig, ImageTextRenderer, TextAlignment, TextBox, TextRenderConfig,
    },
};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📏 Baseline Alignment Comparison Example");
    println!("========================================");

    // Create output directory
    fs::create_dir_all("output/baseline")?;

    // Initialize the renderer and load font
    let mut renderer = ImageTextRenderer::new()?;
    let font_loaded = load_system_font(&mut renderer);

    if !font_loaded {
        println!("⚠️  No system font available");
        return Ok(());
    }

    println!("✅ Font loaded successfully");

    // Create comparison examples
    create_baseline_demonstration(&renderer)?;
    create_alignment_showcase(&renderer)?;
    create_professional_examples(&renderer)?;

    println!("\n🎉 Baseline comparison examples completed!");
    println!("📁 Check the 'output/baseline' directory for generated images");

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

/// Create baseline demonstration showing proper text alignment
fn create_baseline_demonstration(
    renderer: &ImageTextRenderer,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📏 Creating baseline demonstration...");

    let mut image = ImageTextRenderer::create_image(800, 600, Color::new(248, 248, 248, 255));

    // Draw baseline guides
    draw_baseline_guides(&mut image);

    // Example 1: Single line with mixed characters showing proper baseline
    let text_box1 = TextBox::new(50, 100, 700, 60);
    let config1 = TextRenderConfig {
        text_color: Color::new(40, 40, 40, 255),
        background_color: Color::new(248, 248, 248, 255),
        box_color: Color::new(255, 255, 255, 200),
        alignment: TextAlignment::CenterCenter,
        fitting_config: TextFittingConfig {
            max_width: 700.0,
            max_height: 60.0,
            min_font_size: 20.0,
            max_font_size: 32.0,
            ..Default::default()
        },
        ..Default::default()
    };

    renderer.render_text(
        &mut image,
        "Proper Baseline: gjpqy HELLO world 123",
        text_box1,
        &config1,
    )?;

    // Example 2: Multi-line text with consistent baselines
    let text_box2 = TextBox::new(50, 200, 700, 120);
    let config2 = TextRenderConfig {
        text_color: Color::new(40, 40, 40, 255),
        box_color: Color::new(255, 255, 255, 200),
        alignment: TextAlignment::CenterCenter,
        line_height_multiplier: 1.4,
        fitting_config: TextFittingConfig {
            max_width: 700.0,
            max_height: 120.0,
            min_font_size: 16.0,
            max_font_size: 24.0,
            ..Default::default()
        },
        ..Default::default()
    };

    renderer.render_text(
        &mut image,
        "Multiple lines of text should maintain\nconsistent baseline alignment throughout\nthe entire text block for readability",
        text_box2,
        &config2,
    )?;

    // Example 3: Different sizes but proper baselines
    let text_box3 = TextBox::new(50, 360, 700, 80);
    let config3 = TextRenderConfig {
        text_color: Color::new(0, 100, 200, 255),
        box_color: Color::new(240, 248, 255, 200),
        alignment: TextAlignment::CenterCenter,
        fitting_config: TextFittingConfig {
            max_width: 700.0,
            max_height: 80.0,
            min_font_size: 18.0,
            max_font_size: 28.0,
            ..Default::default()
        },
        ..Default::default()
    };

    renderer.render_text(
        &mut image,
        "Typography with proper descenders: gjpqy",
        text_box3,
        &config3,
    )?;

    // Add title
    let title_box = TextBox::new(50, 20, 700, 50);
    let title_config = TextRenderConfig {
        text_color: Color::new(20, 20, 20, 255),
        box_color: Color::transparent(),
        alignment: TextAlignment::CenterCenter,
        fitting_config: TextFittingConfig {
            max_width: 700.0,
            max_height: 50.0,
            min_font_size: 24.0,
            max_font_size: 36.0,
            ..Default::default()
        },
        ..Default::default()
    };

    renderer.render_text(
        &mut image,
        "Proper Baseline Alignment Demonstration",
        title_box,
        &title_config,
    )?;

    ImageTextRenderer::save_image(&image, "output/baseline/baseline_demo.png")?;
    println!("  ✅ Baseline demonstration created");

    Ok(())
}

/// Create alignment showcase showing all 9 alignment options
fn create_alignment_showcase(
    renderer: &ImageTextRenderer,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🎯 Creating alignment showcase...");

    let mut image = ImageTextRenderer::create_image(1200, 800, Color::new(245, 245, 245, 255));

    // Create 3x3 grid of alignment examples
    let alignments = vec![
        (TextAlignment::TopLeft, "Top Left", 100, 100),
        (TextAlignment::TopCenter, "Top Center", 450, 100),
        (TextAlignment::TopRight, "Top Right", 800, 100),
        (TextAlignment::CenterLeft, "Center Left", 100, 300),
        (TextAlignment::CenterCenter, "Center Center", 450, 300),
        (TextAlignment::CenterRight, "Center Right", 800, 300),
        (TextAlignment::BottomLeft, "Bottom Left", 100, 500),
        (TextAlignment::BottomCenter, "Bottom Center", 450, 500),
        (TextAlignment::BottomRight, "Bottom Right", 800, 500),
    ];

    for (alignment, name, x, y) in alignments {
        // Draw text box border to show alignment area
        draw_text_box_border(&mut image, x, y, 250, 150);

        let text_box = TextBox::new(x, y, 250, 150);
        let config = TextRenderConfig {
            text_color: Color::new(30, 30, 30, 255),
            box_color: Color::new(255, 255, 255, 180),
            alignment,
            line_height_multiplier: 1.3,
            fitting_config: TextFittingConfig {
                max_width: 250.0,
                max_height: 150.0,
                min_font_size: 12.0,
                max_font_size: 20.0,
                ..Default::default()
            },
            ..Default::default()
        };

        let text = format!("{}\nAlignment\nProper Baseline", name);
        renderer.render_text(&mut image, &text, text_box, &config)?;
    }

    // Add title
    let title_box = TextBox::new(100, 20, 1000, 60);
    let title_config = TextRenderConfig {
        text_color: Color::new(20, 20, 20, 255),
        box_color: Color::transparent(),
        alignment: TextAlignment::CenterCenter,
        fitting_config: TextFittingConfig {
            max_width: 1000.0,
            max_height: 60.0,
            min_font_size: 28.0,
            max_font_size: 42.0,
            ..Default::default()
        },
        ..Default::default()
    };

    renderer.render_text(
        &mut image,
        "Text Alignment Options with Proper Baselines",
        title_box,
        &title_config,
    )?;

    ImageTextRenderer::save_image(&image, "output/baseline/alignment_showcase.png")?;
    println!("  ✅ Alignment showcase created");

    Ok(())
}

/// Create professional examples showing real-world usage
fn create_professional_examples(
    renderer: &ImageTextRenderer,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n💼 Creating professional examples...");

    // Example 1: Document header
    let mut header_image = ImageTextRenderer::create_image(800, 200, Color::white());
    let header_box = TextBox::new(50, 50, 700, 100);
    let header_config = TextRenderConfig {
        text_color: Color::new(20, 20, 80, 255),
        box_color: Color::new(240, 240, 255, 255),
        alignment: TextAlignment::CenterCenter,
        drop_shadow: DropShadowConfig {
            enabled: true,
            color: Color::new(200, 200, 200, 100),
            expansion: 1,
            blur_radius: 0,
        },
        character_spacing: 1.5,
        fitting_config: TextFittingConfig {
            max_width: 700.0,
            max_height: 100.0,
            min_font_size: 24.0,
            max_font_size: 40.0,
            ..Default::default()
        },
        ..Default::default()
    };

    renderer.render_text(
        &mut header_image,
        "PROFESSIONAL DOCUMENT HEADER",
        header_box,
        &header_config,
    )?;

    ImageTextRenderer::save_image(&header_image, "output/baseline/professional_header.png")?;

    // Example 2: Business card layout
    let mut card_image = ImageTextRenderer::create_image(600, 350, Color::new(250, 250, 250, 255));

    // Company name
    let company_box = TextBox::new(50, 50, 500, 80);
    let company_config = TextRenderConfig {
        text_color: Color::new(0, 70, 140, 255),
        box_color: Color::transparent(),
        alignment: TextAlignment::CenterCenter,
        character_spacing: 2.0,
        fitting_config: TextFittingConfig {
            max_width: 500.0,
            max_height: 80.0,
            min_font_size: 24.0,
            max_font_size: 36.0,
            ..Default::default()
        },
        ..Default::default()
    };

    renderer.render_text(
        &mut card_image,
        "ACME CORPORATION",
        company_box,
        &company_config,
    )?;

    // Contact info
    let contact_box = TextBox::new(50, 150, 500, 120);
    let contact_config = TextRenderConfig {
        text_color: Color::new(60, 60, 60, 255),
        box_color: Color::transparent(),
        alignment: TextAlignment::CenterCenter,
        line_height_multiplier: 1.4,
        fitting_config: TextFittingConfig {
            max_width: 500.0,
            max_height: 120.0,
            min_font_size: 12.0,
            max_font_size: 18.0,
            ..Default::default()
        },
        ..Default::default()
    };

    renderer.render_text(
        &mut card_image,
        "John Smith\nSenior Developer\nphone: (555) 123-4567\nemail: john@acme.com",
        contact_box,
        &contact_config,
    )?;

    ImageTextRenderer::save_image(&card_image, "output/baseline/business_card.png")?;

    // Example 3: Poster with multiple alignments
    let mut poster_image =
        ImageTextRenderer::create_image(600, 800, Color::new(245, 245, 250, 255));

    // Main title
    let main_title_box = TextBox::new(50, 50, 500, 100);
    let main_title_config = TextRenderConfig {
        text_color: Color::new(150, 0, 0, 255),
        box_color: Color::new(255, 240, 240, 200),
        alignment: TextAlignment::CenterCenter,
        drop_shadow: DropShadowConfig {
            enabled: true,
            color: Color::new(200, 150, 150, 120),
            expansion: 2,
            blur_radius: 0,
        },
        character_spacing: 1.0,
        fitting_config: TextFittingConfig {
            max_width: 500.0,
            max_height: 100.0,
            min_font_size: 28.0,
            max_font_size: 42.0,
            ..Default::default()
        },
        ..Default::default()
    };

    renderer.render_text(
        &mut poster_image,
        "EVENT POSTER",
        main_title_box,
        &main_title_config,
    )?;

    // Event details
    let details_box = TextBox::new(50, 200, 500, 300);
    let details_config = TextRenderConfig {
        text_color: Color::new(40, 40, 40, 255),
        box_color: Color::new(255, 255, 255, 180),
        alignment: TextAlignment::CenterCenter,
        line_height_multiplier: 1.5,
        fitting_config: TextFittingConfig {
            max_width: 500.0,
            max_height: 300.0,
            min_font_size: 12.0,
            max_font_size: 20.0,
            ..Default::default()
        },
        ..Default::default()
    };

    renderer.render_text(
        &mut poster_image,
        "Join us for an amazing event!\n\nDate: July 15, 2024\nTime: 7:00 PM\nLocation: Grand Hall\n\nFeaturing live music,\nfood, and entertainment\n\nTickets available online",
        details_box,
        &details_config,
    )?;

    // Footer
    let footer_box = TextBox::new(50, 550, 500, 60);
    let footer_config = TextRenderConfig {
        text_color: Color::new(100, 100, 100, 255),
        box_color: Color::transparent(),
        alignment: TextAlignment::CenterCenter,
        fitting_config: TextFittingConfig {
            max_width: 500.0,
            max_height: 60.0,
            min_font_size: 10.0,
            max_font_size: 16.0,
            ..Default::default()
        },
        ..Default::default()
    };

    renderer.render_text(
        &mut poster_image,
        "www.example.com | info@example.com",
        footer_box,
        &footer_config,
    )?;

    ImageTextRenderer::save_image(&poster_image, "output/baseline/event_poster.png")?;

    println!("  ✅ Professional examples created");

    Ok(())
}

/// Draw baseline guides to show text positioning
fn draw_baseline_guides(image: &mut image::RgbaImage) {
    let guide_color = Color::new(200, 200, 200, 255).to_rgba();

    // Draw horizontal lines at baseline positions
    let baseline_positions = vec![130, 250, 320, 390];
    for y in baseline_positions {
        for x in 50..750 {
            if x < image.width() && y < image.height() {
                image.put_pixel(x, y, guide_color);
            }
        }
    }
}

/// Draw text box border to show alignment area
fn draw_text_box_border(image: &mut image::RgbaImage, x: u32, y: u32, width: u32, height: u32) {
    let border_color = Color::new(180, 180, 180, 255).to_rgba();

    // Draw border lines
    for i in 0..width {
        // Top and bottom borders
        if x + i < image.width() {
            if y < image.height() {
                image.put_pixel(x + i, y, border_color);
            }
            if y + height < image.height() {
                image.put_pixel(x + i, y + height, border_color);
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
                image.put_pixel(x + width, y + i, border_color);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_baseline_example_creation() {
        let renderer = ImageTextRenderer::new();
        assert!(renderer.is_ok());
    }

    #[test]
    fn test_alignment_options() {
        let center = TextAlignment::CenterCenter;
        assert!(matches!(center, TextAlignment::CenterCenter));
    }
}
