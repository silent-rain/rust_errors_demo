use crate::into_demo::ErrorMsg;

#[derive(thiserror::Error, Debug)]
#[repr(u16)]
pub enum Error {
    #[error("ok")]
    Ok = 10000,

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("unknown data store error")]
    Unknown,

    #[error(transparent)]
    Any(#[from] anyhow::Error),
    #[error(transparent)]
    ColorEyreReport(#[from] color_eyre::Report),
}

impl Error {
    /// 返回错误码
    pub fn code(&self) -> u16 {
        unsafe {
            let ptr = self as *const Error as *const u16;
            ptr.read_volatile()
        }
    }
    /// 返回错误码信息
    pub fn msg(&self) -> String {
        self.to_string()
    }
}

