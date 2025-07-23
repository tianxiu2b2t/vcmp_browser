use std::{fmt::Display, net::SocketAddr};

#[derive(Debug)]
pub enum VCMPError {
    Error(String),
    FailedToResolveAddress(String),
    UnknownHost(SocketAddr),
    SocketRecvError(String),
    SocketSendError(String),
}

impl Display for VCMPError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VCMPError::Error(e) => write!(f, "VCMPError: {}", e),
            VCMPError::FailedToResolveAddress(e) => write!(f, "VCMPError: Failed to resolve address: {}", e),
            VCMPError::UnknownHost(e) => write!(f, "VCMPError: Unknown host: {}", e),
            VCMPError::SocketRecvError(e) => write!(f, "VCMPError: Socket recv error: {}", e),
            VCMPError::SocketSendError(e) => write!(f, "VCMPError: Socket send error: {}", e),
        }
    }
}

pub type VCMPResult<T> = Result<T, VCMPError>;
