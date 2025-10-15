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

impl Error {
    pub fn into_err(self) -> ErrorMsg {
        ErrorMsg::new(self.code(), &self.msg())
    }

    pub fn into_err_with_msg(self, msg: &str) -> ErrorMsg {
        ErrorMsg::new(self.code(), msg)
    }

    pub fn into_err_with_appended_msg(self, msg: &str) -> ErrorMsg {
        let msg = format!("{}, {}", self.msg(), msg.to_string());
        ErrorMsg::new(self.code(), &msg)
    }
}

pub trait IntoErrorMsg<T> {
    fn into_err(self) -> Result<T, ErrorMsg>;
    fn into_err_with_msg(self, msg: &str) -> Result<T, ErrorMsg>;
    fn into_err_with_appended_msg(self, msg: &str) -> Result<T, ErrorMsg>;
}

impl<T> IntoErrorMsg<T> for Result<T, Error> {
    fn into_err(self) -> Result<T, ErrorMsg> {
        self.map_err(ErrorMsg::form_err)
    }

    fn into_err_with_msg(self, msg: &str) -> Result<T, ErrorMsg> {
        self.map_err(|e| ErrorMsg::form_err(e).with_msg(msg))
    }

    fn into_err_with_appended_msg(self, msg: &str) -> Result<T, ErrorMsg> {
        self.map_err(|e| ErrorMsg::form_err(e).with_append_msg(msg))
    }
}

impl<T> IntoErrorMsg<T> for Result<T, anyhow::Error> {
    fn into_err(self) -> Result<T, ErrorMsg> {
        self.map_err(|e| {
            let mut code = Error::Ok.code();
            let mut msg = Error::Ok.msg();

            if let Some(e) = e.downcast_ref::<Error>() {
                code = e.code();
                msg = e.msg();
            }

            ErrorMsg::form_err(Error::Any(e))
                .with_code(code)
                .with_msg(&msg)
        })
    }

    fn into_err_with_msg(self, msg: &str) -> Result<T, ErrorMsg> {
        self.map_err(|e| {
            let mut code = Error::Ok.code();

            if let Some(e) = e.downcast_ref::<Error>() {
                code = e.code();
            }

            ErrorMsg::form_err(Error::Any(e))
                .with_code(code)
                .with_msg(msg)
        })
    }

    fn into_err_with_appended_msg(self, msg: &str) -> Result<T, ErrorMsg> {
        self.map_err(|e| {
            let mut code = Error::Ok.code();

            if let Some(e) = e.downcast_ref::<Error>() {
                code = e.code();
            }

            ErrorMsg::form_err(Error::Any(e))
                .with_code(code)
                .with_append_msg(msg)
        })
    }
}

impl<T> IntoErrorMsg<T> for Result<T, color_eyre::Report> {
    fn into_err(self) -> Result<T, ErrorMsg> {
        self.map_err(|e| {
            let mut code = Error::Ok.code();
            let mut msg = Error::Ok.msg();

            if let Some(e) = e.downcast_ref::<Error>() {
                code = e.code();
                msg = e.msg();
            }

            ErrorMsg::form_err(Error::ColorEyreReport(e))
                .with_code(code)
                .with_msg(&msg)
        })
    }

    fn into_err_with_msg(self, msg: &str) -> Result<T, ErrorMsg> {
        self.map_err(|e| {
            let mut code = Error::Ok.code();

            if let Some(e) = e.downcast_ref::<Error>() {
                code = e.code();
            }

            ErrorMsg::form_err(Error::ColorEyreReport(e))
                .with_code(code)
                .with_msg(msg)
        })
    }

    fn into_err_with_appended_msg(self, msg: &str) -> Result<T, ErrorMsg> {
        self.map_err(|e| {
            let mut code = Error::Ok.code();

            if let Some(e) = e.downcast_ref::<Error>() {
                code = e.code();
            }

            ErrorMsg::form_err(Error::ColorEyreReport(e))
                .with_code(code)
                .with_append_msg(msg)
        })
    }
}
