use std::collections::HashMap;

#[derive(Clone)]
pub struct Output {
    pub imgs: Vec<OutputImage>,
}

impl Output {
    pub fn join(self, other: Self) -> Self {
        let mut imgs = self.imgs;
        imgs.extend(other.imgs);
        Self { imgs }
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for img in self.imgs {
            let b = img.to_bytes();
            bytes.extend(b.len().to_le_bytes());
            bytes.extend(b);
        }
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        let mut imgs = Vec::new();
        let mut index = 0;

        while index < bytes.len() {
            let usize_len = std::mem::size_of::<usize>();
            if bytes.len() < index + usize_len {
                return Err("Not enough bytes for len".to_string());
            }
            let image_len =
                usize::from_le_bytes(bytes[index..index + usize_len].try_into().unwrap());
            index += usize_len;
            let img = OutputImage::from_bytes(&bytes[index..index + image_len])?;
            imgs.push(img);
            index += image_len;
        }
        Ok(Output { imgs })
    }
}

#[derive(Clone)]
pub struct OutputImage {
    pub areas: Vec<OutputElement>,
}

impl OutputImage {
    pub fn to_bytes(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for area in self.areas {
            bytes.extend(area.to_bytes());
        }
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        let mut areas = Vec::new();
        let mut index = 0;
        while index < bytes.len() {
            let area = OutputElement::from_bytes(&bytes[index..])?;
            index += area.size();
            areas.push(area);
        }
        Ok(OutputImage { areas })
    }
}

#[derive(Clone)]
pub struct OutputElement {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub rotation: u32,
    pub fg: (u32, u32, u32),
    pub bg: (u32, u32, u32),
    pub output_key: String,
    pub translations: HashMap<String, String>,
    pub background: Vec<u8>,
}

impl OutputElement {
    pub fn size(&self) -> usize {
        4 * 11
            + self.output_key.len()
            + self.background.len()
            + self
                .translations
                .iter()
                .map(|v| v.0.len() + v.1.len())
                .sum::<usize>()
            + std::mem::size_of::<usize>() * (3 + self.translations.len() * 2)
    }
    pub fn to_bytes(self) -> Vec<u8> {
        let bytes = vec![];
        let mut bytes = bytes;
        bytes.extend(self.x.to_le_bytes());
        bytes.extend(self.y.to_le_bytes());
        bytes.extend(self.width.to_le_bytes());
        bytes.extend(self.height.to_le_bytes());
        bytes.extend(self.rotation.to_le_bytes());
        bytes.extend(self.output_key.as_bytes());
        bytes.extend(self.fg.0.to_le_bytes());
        bytes.extend(self.fg.1.to_le_bytes());
        bytes.extend(self.fg.2.to_le_bytes());
        bytes.extend(self.bg.0.to_le_bytes());
        bytes.extend(self.bg.1.to_le_bytes());
        bytes.extend(self.bg.2.to_le_bytes());
        bytes.extend(self.translations.len().to_le_bytes());
        for (key, value) in self.translations {
            bytes.extend(key.len().to_le_bytes());
            bytes.extend(key.as_bytes());
            bytes.extend(value.len().to_le_bytes());
            bytes.extend(value.as_bytes());
        }
        bytes.extend(self.background.len().to_le_bytes());
        bytes.extend(self.background);
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        let mut index = 0;

        macro_rules! take_bytes {
            ( $count:expr) => {{
                if bytes.len() < index + $count {
                    return Err("".to_string());
                }
                let slice = &bytes[index..index + $count];
                index += $count;
                slice
            }};
        }

        macro_rules! take_u32 {
            () => {{
                let raw = take_bytes!(4);
                u32::from_le_bytes(raw.try_into().unwrap())
            }};
        }

        macro_rules! take_usize {
            () => {{
                let size = std::mem::size_of::<usize>();

                let raw = take_bytes!(size);
                usize::from_le_bytes(raw.try_into().unwrap())
            }};
        }

        macro_rules! take_string {
            () => {{
                let len = take_usize!();
                let raw = take_bytes!(len);
                String::from_utf8(raw.to_vec())
                    .map_err(|e| format!("Invalid UTF-8 for {}: {}", "", e))?
            }};
        }

        let x = take_u32!();
        let y = take_u32!();
        let width = take_u32!();
        let height = take_u32!();
        let rotation = take_u32!();
        let fg_0 = take_u32!();
        let fg_1 = take_u32!();
        let bg_0 = take_u32!();
        let bg_1 = take_u32!();
        let fg_2 = take_u32!();
        let bg_2 = take_u32!();
        let output_key = take_string!();

        let translations_len = take_usize!();
        let mut translations = HashMap::new();
        for _ in 0..translations_len {
            let key = take_string!();
            let value = take_string!();
            translations.insert(key, value);
        }

        let background_len = take_usize!();
        let background = take_bytes!(background_len).to_vec();
        let _ = index;

        Ok(OutputElement {
            x,
            y,
            width,
            height,
            rotation,
            output_key,
            translations,
            background,
            fg: (fg_0, fg_1, fg_2),
            bg: (bg_0, bg_1, bg_2),
        })
    }
}
