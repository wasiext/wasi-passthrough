use crate::bindings::exports;
use crate::bindings::exports::wasi::io::poll::PollableBorrow;
use crate::bindings::exports::wasi::io::streams::InputStreamBorrow;
use crate::bindings::wasi::io::error::Error;
use crate::bindings::wasi::io::poll::{self, Pollable};
use crate::bindings::wasi::io::streams::{InputStream, OutputStream, StreamError};

impl From<exports::wasi::io::error::Error> for Error {
    fn from(value: exports::wasi::io::error::Error) -> Self {
        value.into_inner()
    }
}

impl From<Error> for exports::wasi::io::error::Error {
    fn from(value: Error) -> Self {
        Self::new(value)
    }
}

impl From<exports::wasi::io::poll::Pollable> for Pollable {
    fn from(value: exports::wasi::io::poll::Pollable) -> Self {
        value.into_inner()
    }
}

impl From<Pollable> for exports::wasi::io::poll::Pollable {
    fn from(value: Pollable) -> Self {
        Self::new(value)
    }
}

impl From<exports::wasi::io::streams::InputStream> for InputStream {
    fn from(value: exports::wasi::io::streams::InputStream) -> Self {
        value.into_inner()
    }
}

impl From<InputStream> for exports::wasi::io::streams::InputStream {
    fn from(value: InputStream) -> Self {
        Self::new(value)
    }
}

impl From<exports::wasi::io::streams::OutputStream> for OutputStream {
    fn from(value: exports::wasi::io::streams::OutputStream) -> Self {
        value.into_inner()
    }
}

impl From<OutputStream> for exports::wasi::io::streams::OutputStream {
    fn from(value: OutputStream) -> Self {
        Self::new(value)
    }
}

impl From<exports::wasi::io::streams::StreamError> for StreamError {
    fn from(value: exports::wasi::io::streams::StreamError) -> Self {
        match value {
            exports::wasi::io::streams::StreamError::LastOperationFailed(err) => {
                Self::LastOperationFailed(err.into())
            }
            exports::wasi::io::streams::StreamError::Closed => Self::Closed,
        }
    }
}

impl From<StreamError> for exports::wasi::io::streams::StreamError {
    fn from(value: StreamError) -> Self {
        match value {
            StreamError::LastOperationFailed(err) => Self::LastOperationFailed(err.into()),
            StreamError::Closed => Self::Closed,
        }
    }
}

impl exports::wasi::io::error::Guest for () {
    type Error = Error;
}

impl exports::wasi::io::error::GuestError for Error {
    fn to_debug_string(&self) -> String {
        Self::to_debug_string(self)
    }
}

impl exports::wasi::io::streams::Guest for () {
    type InputStream = InputStream;
    type OutputStream = OutputStream;
}

impl exports::wasi::io::streams::GuestInputStream for InputStream {
    fn read(&self, len: u64) -> Result<Vec<u8>, exports::wasi::io::streams::StreamError> {
        let ret = Self::read(self, len)?;
        Ok(ret)
    }

    fn blocking_read(&self, len: u64) -> Result<Vec<u8>, exports::wasi::io::streams::StreamError> {
        let ret = Self::blocking_read(self, len)?;
        Ok(ret)
    }

    fn skip(&self, len: u64) -> Result<u64, exports::wasi::io::streams::StreamError> {
        let ret = Self::skip(self, len)?;
        Ok(ret)
    }

    fn blocking_skip(&self, len: u64) -> Result<u64, exports::wasi::io::streams::StreamError> {
        let ret = Self::blocking_skip(self, len)?;
        Ok(ret)
    }

    fn subscribe(&self) -> exports::wasi::io::poll::Pollable {
        exports::wasi::io::poll::Pollable::new(Self::subscribe(self))
    }
}

impl exports::wasi::io::streams::GuestOutputStream for OutputStream {
    fn check_write(&self) -> Result<u64, exports::wasi::io::streams::StreamError> {
        let ret = Self::check_write(self)?;
        Ok(ret)
    }

    fn write(&self, contents: Vec<u8>) -> Result<(), exports::wasi::io::streams::StreamError> {
        Self::write(self, &contents)?;
        Ok(())
    }

    fn blocking_write_and_flush(
        &self,
        contents: Vec<u8>,
    ) -> Result<(), exports::wasi::io::streams::StreamError> {
        Self::blocking_write_and_flush(self, &contents)?;
        Ok(())
    }

    fn flush(&self) -> Result<(), exports::wasi::io::streams::StreamError> {
        Self::flush(self)?;
        Ok(())
    }

    fn blocking_flush(&self) -> Result<(), exports::wasi::io::streams::StreamError> {
        Self::blocking_flush(self)?;
        Ok(())
    }

    fn subscribe(&self) -> exports::wasi::io::poll::Pollable {
        exports::wasi::io::poll::Pollable::new(Self::subscribe(self))
    }

    fn write_zeroes(&self, len: u64) -> Result<(), exports::wasi::io::streams::StreamError> {
        Self::write_zeroes(self, len)?;
        Ok(())
    }

    fn blocking_write_zeroes_and_flush(
        &self,
        len: u64,
    ) -> Result<(), exports::wasi::io::streams::StreamError> {
        Self::blocking_write_zeroes_and_flush(self, len)?;
        Ok(())
    }

    fn splice(
        &self,
        src: InputStreamBorrow<'_>,
        len: u64,
    ) -> Result<u64, exports::wasi::io::streams::StreamError> {
        let ret = Self::splice(self, src.get(), len)?;
        Ok(ret)
    }

    fn blocking_splice(
        &self,
        src: InputStreamBorrow<'_>,
        len: u64,
    ) -> Result<u64, exports::wasi::io::streams::StreamError> {
        let ret = Self::blocking_splice(self, src.get(), len)?;
        Ok(ret)
    }
}

impl exports::wasi::io::poll::Guest for () {
    type Pollable = Pollable;

    fn poll(in_: Vec<PollableBorrow<'_>>) -> Vec<u32> {
        poll::poll(
            &in_.iter()
                .map(exports::wasi::io::poll::PollableBorrow::get)
                .collect::<Vec<_>>(),
        )
    }
}

impl exports::wasi::io::poll::GuestPollable for Pollable {
    fn ready(&self) -> bool {
        Self::ready(self)
    }

    fn block(&self) {
        Self::block(self);
    }
}
