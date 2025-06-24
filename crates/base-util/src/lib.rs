pub mod error;
pub mod onnx;
pub mod project;

pub trait RawSerializable: Sized {
    fn dump(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                (self as *const Self) as *const u8,
                std::mem::size_of::<Self>(),
            )
        }
    }

    fn parse(bytes: &[u8]) -> anyhow::Result<Self> {
        if bytes.len() < std::mem::size_of::<Self>() {
            anyhow::bail!(
                "Byte slice too small for type {}",
                std::any::type_name::<Self>()
            );
        }

        let ptr = bytes.as_ptr() as *const Self;
        let val = unsafe { std::ptr::read(ptr) };
        Ok(val)
    }
}
