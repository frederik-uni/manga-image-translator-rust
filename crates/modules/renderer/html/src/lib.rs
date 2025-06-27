use base64::engine::general_purpose;
use base64::Engine as _;
use binary::Output;
use image::ImageEncoder;
use image::{ColorType, ExtendedColorType};
use interface::{image::RawImage, rederer::Renderer};
use serde::{Deserialize, Serialize};
use v_htmlescape::escape;

pub struct HtmlRenderer;
impl Renderer for HtmlRenderer {
    type Options;

    fn render(
        &self,
        image: RawImage,
        translations: interface::detectors::textlines::Quadrilateral,
        options: Self::Options,
    ) -> anyhow::Result<Box<dyn interface::rederer::Output>> {
        todo!()
    }
}

impl interface::rederer::Output for Data {
    fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }

    fn to_file(&self, path: &str) -> anyhow::Result<()> {
        todo!()
    }

    fn join(
        self: Box<Self>,
        other: Box<dyn interface::rederer::Output>,
    ) -> Box<dyn interface::rederer::Output> {
        todo!()
    }
}

struct Data {
    imgs: Vec<RawImage>,
    outputs: Output,
    configs: Vec<Config>,
}

pub struct Config {
    shadow: Option<(u8, u8, u8, f32)>,
    color: Option<(u8, u8, u8)>,
    font: Option<String>,
    archive: bool,
}

#[derive(Deserialize, Serialize)]
pub struct JsonData {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    rotation: u32,
    color: (u8, u8, u8),
    shadow: (u8, u8, u8, f32),
    text: String,
    background: String,
}

impl Data {
    fn process(self) -> (Vec<u8>, bool) {
        let mut output = vec![r#"<meta charset="UTF-8" />"#.to_owned()];
        let files = vec![];
        let archive = self.configs.first().unwrap().archive;
        let mut counter = 0;
        for ((img, data), config) in self
            .imgs
            .into_iter()
            .zip(self.outputs.imgs)
            .zip(self.configs)
        {
            let mut data_ = vec![];
            let mut temp_data = vec![];
            for (index, item) in data.areas.into_iter().enumerate() {
                match archive {
                    true => {
                        temp_data.push(item.background);
                        let shadow = config
                            .shadow
                            .unwrap_or_else(|| (item.bg.0, item.bg.1, item.bg.2, 1.0));
                        data_.push(JsonData {
                            x: item.x,
                            y: item.y,
                            width: item.width,
                            height: item.height,
                            rotation: item.rotation,
                            color: config.color.unwrap_or(item.fg),
                            shadow,
                            text: item.translations.get(&item.output_key).unwrap().to_owned(),
                            background: format!("./{}.png", counter + 1 + index),
                        });
                    }
                    false => {
                        let shadow = config
                            .shadow
                            .unwrap_or_else(|| (item.bg.0, item.bg.1, item.bg.2, 1.0));
                        let data = vec![];
                        let encoder = image::codecs::png::PngEncoder::new(&mut data);
                        encoder
                            .write_image(
                                &img.data,
                                img.width as u32,
                                img.height as u32,
                                ExtendedColorType::Rgb8,
                            )
                            .unwrap();
                        let base64_str = general_purpose::STANDARD.encode(&data);
                        data_.push(JsonData {
                            x: item.x,
                            y: item.y,
                            width: item.width,
                            height: item.height,
                            rotation: item.rotation,
                            color: config.color.unwrap_or(item.fg),
                            shadow,
                            text: item.translations.get(&item.output_key).unwrap().to_owned(),
                            background: format!(r#"data:image/png;base64,{}"#, base64_str),
                        });
                    }
                }
            }
            output.push(generate(config, data_, 0, &img, archive));
            if archive {
                counter += 1;
                files.push(img);
                counter += temp_data.len();
                files.extend(temp_data);
            }
        }
        output.push("<!--<script>var maxWidth = 300;</script> -->".to_owned());
        output.push(r#"<script src="/lazyInit.js" defer></script>"#.to_owned());
        (output.join("").as_bytes().to_vec(), archive)
    }
}

pub fn build_zip(files: &[(&str, &[u8])]) -> zip::result::ZipResult<Vec<u8>> {
    let mut buffer = Cursor::new(Vec::new());
    let mut zip = ZipWriter::new(&mut buffer);

    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o644);

    for (filename, contents) in files {
        zip.start_file(*filename, options)?;
        zip.write_all(contents)?;
    }

    zip.finish()?;
    Ok(buffer.into_inner())
}

impl Data {
    pub fn join(self, other: Self) -> Self {
        let mut imgs = self.imgs;
        imgs.extend(other.imgs);
        let mut configs = self.configs;
        configs.extend(other.configs);
        Self {
            imgs,
            outputs: self.outputs.join(other.outputs),
            configs,
        }
    }
}

fn generate(
    config: Config,
    data: Vec<JsonData>,
    index: usize,
    img: &RawImage,
    archive: bool,
) -> String {
    let data = serde_json::to_string(&data).unwrap();
    let data_str = escape(&data);
    let font_escaped = escape(&config.font.unwrap_or_else(|| "arial".to_owned()));
    let path = match archive {
        true => format!("./{index}.png"),
        false => {
            let data = vec![];
            let encoder = image::codecs::png::PngEncoder::new(&mut data);
            encoder
                .write_image(
                    &img.data,
                    img.width as u32,
                    img.height as u32,
                    ExtendedColorType::Rgb8,
                )
                .unwrap();
            let base64_str = general_purpose::STANDARD.encode(&data);
            format!(r#"data:image/png;base64,{}"#, base64_str)
        }
    };
    format!(
        r###"
        <div
            class="canvas-wrapper"
            style="
                --ui-font-family: {};
            "
            data-overlays='{}'
        >
            <img class="base-image" src="{}" alt="Image" />
        </div>
 "###,
        font_escaped, data_str, path
    )
}
