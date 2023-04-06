use thiserror_no_std::Error;
pub type Result<T> = core::result::Result<T, SoilMoistureSensorError>;

#[derive(Error, Debug) ]
pub enum SoilMoistureSensorError {
    #[error("Write Read I2C Error")]
    WriteReadI2CError,
    #[error("Write I2C Error")]
    WriteI2CError,
    #[error("Read I2C Error")]
    ReadI2CError,
}
