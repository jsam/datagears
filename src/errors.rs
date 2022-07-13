use std::result;
use thiserror::Error;


pub type Result<T> = result::Result<T, DGError>;

#[derive(Error, Debug)]
pub enum DGError {
    #[error("DataGears: General Error: {0}")]
    General(String),
    #[error("DataGears: Preprocessing Error: {0}")]
    Preprocessing(String),
    #[error("DataGears: Model Not Found Error: {0}")]
    ModelNotFound(String),
    #[error("DataGears: Request Error: {0}")]
    RequestError(String),
    #[error("DataGears: Request Kind Error: {0}")]
    RequestKindError(String),
    #[error("DataGears: Acquire GIL Error: {0}")]
    AcquireGILError(String),
    #[error("DataGears: Python Module Error: {0}")]
    PyModuleError(String),
    #[error("DataGears: Python Callee Error: {0}")]
    PyCallError(String),
    #[error("DataGears: Python Callee Error: {0}")]
    OsStringCnvError(String),
    #[error("OrkDataGearshon: Model Backend Error: {0}")]
    ModelBackendError(#[from] tract_core::TractError),
    #[error("DataGears: IO Error: {0}")]
    IOError(#[from] std::io::Error),
}