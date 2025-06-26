use binary::Output;
use interface::image::RawImage;
use v_htmlescape::escape;

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

pub struct JsonData {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    rotation: u32,
    text: String,
    background: String,
}

impl Data {
    fn process(self, config: Config) -> Vec<u8> {
        let mut output = vec![r#"<meta charset="UTF-8" />"#.to_owned()];
        for ((img, data), config) in self
            .imgs
            .into_iter()
            .zip(self.outputs.imgs)
            .zip(self.configs)
        {
            config
                .data
                .output
                .push(generate(shadow, color, font, path, data));
        }
        output.push("<!--<script>var maxWidth = 300;</script> -->".to_owned());
        output.push(r#"<script src="/lazyInit.js" defer></script>"#.to_owned());
        output.join("").as_bytes().to_vec()
    }
}

impl Data {
    pub fn join(self, other: Self) -> Self {
        let mut imgs = self.imgs;
        imgs.extend(other.imgs);
        Self {
            imgs,
            outputs: self.outputs.join(other.outputs),
        }
    }
}

fn generate(
    shadow: (u8, u8, u8, f32),
    color: (u8, u8, u8),
    font: &str,
    path: &str,
    data: JsonData,
) -> String {
    let (r_s, g_s, b_s, a_s) = shadow;
    let (r_c, g_c, b_c) = color;
    let data = data.to_string();
    let data_str = escape(&data);
    let font_escaped = escape(font);
    let path_escaped = escape(path);

    format!(
        r###"
        <div
            class="canvas-wrapper"
            style="
                --text-shadow: rgba({}, {}, {}, {});
                --text-color: rgb({}, {}, {});
                --ui-font-family: {};
            "
            data-overlays='{}'
        >
            <img class="base-image" src="{}" alt="Image" />
        </div>
 "###,
        r_s, g_s, b_s, a_s, r_c, g_c, b_c, font_escaped, data_str, path_escaped
    )
}
