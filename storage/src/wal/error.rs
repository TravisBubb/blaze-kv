use bincode::error::{DecodeError, EncodeError};
use std::io;
use tokio::sync::{mpsc, oneshot};

use super::message::WalMessage;

/// Represents an error in the WAL layer
#[derive(Debug)]
pub enum WalError {
    Encode(EncodeError),
    Decode(DecodeError),
    Io(io::Error),
    OneshotRecv(oneshot::error::RecvError),
    MpscRecv(mpsc::error::TryRecvError),
    MpscSend(mpsc::error::SendError<WalMessage>),
}

impl std::fmt::Display for WalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WalError::Encode(e) => write!(f, "Encode error: {}", e),
            WalError::Decode(e) => write!(f, "Decode error: {}", e),
            WalError::Io(e) => write!(f, "IO error: {}", e),
            WalError::OneshotRecv(e) => write!(f, "Receive error: {}", e),
            WalError::MpscRecv(e) => write!(f, "Receive error: {}", e),
            WalError::MpscSend(e) => write!(f, "Send error: {}", e),
        }
    }
}

impl std::error::Error for WalError {}

impl From<EncodeError> for WalError {
    fn from(err: EncodeError) -> Self {
        WalError::Encode(err)
    }
}

impl From<DecodeError> for WalError {
    fn from(err: DecodeError) -> Self {
        WalError::Decode(err)
    }
}

impl From<io::Error> for WalError {
    fn from(err: io::Error) -> Self {
        WalError::Io(err)
    }
}

impl From<oneshot::error::RecvError> for WalError {
    fn from(err: oneshot::error::RecvError) -> Self {
        WalError::OneshotRecv(err)
    }
}

impl From<mpsc::error::TryRecvError> for WalError {
    fn from(err: mpsc::error::TryRecvError) -> Self {
        WalError::MpscRecv(err)
    }
}

impl From<mpsc::error::SendError<WalMessage>> for WalError {
    fn from(err: mpsc::error::SendError<WalMessage>) -> Self {
        WalError::MpscSend(err)
    }
}
