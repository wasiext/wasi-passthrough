use crate::bindings::exports;
use crate::bindings::exports::wasi::keyvalue::store::BucketBorrow;
use crate::bindings::wasi::keyvalue::store::{Error, KeyResponse};
use crate::bindings::wasi::keyvalue::{atomics, batch, store};

impl From<Error> for exports::wasi::keyvalue::store::Error {
    fn from(value: Error) -> Self {
        match value {
            Error::NoSuchStore => Self::NoSuchStore,
            Error::AccessDenied => Self::AccessDenied,
            Error::Other(s) => Self::Other(s),
        }
    }
}

impl From<exports::wasi::keyvalue::store::Error> for Error {
    fn from(value: exports::wasi::keyvalue::store::Error) -> Self {
        match value {
            exports::wasi::keyvalue::store::Error::NoSuchStore => Self::NoSuchStore,
            exports::wasi::keyvalue::store::Error::AccessDenied => Self::AccessDenied,
            exports::wasi::keyvalue::store::Error::Other(s) => Self::Other(s),
        }
    }
}

impl From<KeyResponse> for exports::wasi::keyvalue::store::KeyResponse {
    fn from(value: KeyResponse) -> Self {
        Self {
            keys: value.keys,
            cursor: value.cursor,
        }
    }
}

impl From<exports::wasi::keyvalue::store::KeyResponse> for KeyResponse {
    fn from(value: exports::wasi::keyvalue::store::KeyResponse) -> Self {
        Self {
            keys: value.keys,
            cursor: value.cursor,
        }
    }
}

impl exports::wasi::keyvalue::store::Guest for () {
    type Bucket = store::Bucket;

    fn open(
        identifier: String,
    ) -> Result<exports::wasi::keyvalue::store::Bucket, exports::wasi::keyvalue::store::Error> {
        let res = store::open(&identifier)?;
        Ok(exports::wasi::keyvalue::store::Bucket::new(res))
    }
}

impl exports::wasi::keyvalue::store::GuestBucket for store::Bucket {
    fn get(&self, key: String) -> Result<Option<Vec<u8>>, exports::wasi::keyvalue::store::Error> {
        let res = Self::get(self, &key)?;
        Ok(res)
    }

    fn set(
        &self,
        key: String,
        value: Vec<u8>,
    ) -> Result<(), exports::wasi::keyvalue::store::Error> {
        Self::set(self, &key, &value)?;
        Ok(())
    }

    fn delete(&self, key: String) -> Result<(), exports::wasi::keyvalue::store::Error> {
        Self::delete(self, &key)?;
        Ok(())
    }

    fn exists(&self, key: String) -> Result<bool, exports::wasi::keyvalue::store::Error> {
        let res = Self::exists(self, &key)?;
        Ok(res)
    }

    fn list_keys(
        &self,
        cursor: Option<u64>,
    ) -> Result<exports::wasi::keyvalue::store::KeyResponse, exports::wasi::keyvalue::store::Error>
    {
        let res = Self::list_keys(self, cursor)?;
        Ok(res.into())
    }
}

impl exports::wasi::keyvalue::atomics::Guest for () {
    fn increment(
        bucket: BucketBorrow<'_>,
        key: String,
        delta: u64,
    ) -> Result<u64, exports::wasi::keyvalue::store::Error> {
        let res = atomics::increment(bucket.get(), &key, delta)?;
        Ok(res)
    }
}

impl exports::wasi::keyvalue::batch::Guest for () {
    fn get_many(
        bucket: BucketBorrow<'_>,
        keys: Vec<String>,
    ) -> Result<Vec<Option<(String, Vec<u8>)>>, exports::wasi::keyvalue::store::Error> {
        let res = batch::get_many(bucket.get(), &keys)?;
        Ok(res)
    }

    fn set_many(
        bucket: BucketBorrow<'_>,
        key_values: Vec<(String, Vec<u8>)>,
    ) -> Result<(), exports::wasi::keyvalue::store::Error> {
        batch::set_many(bucket.get(), &key_values)?;
        Ok(())
    }

    fn delete_many(
        bucket: BucketBorrow<'_>,
        keys: Vec<String>,
    ) -> Result<(), exports::wasi::keyvalue::store::Error> {
        batch::delete_many(bucket.get(), &keys)?;
        Ok(())
    }
}
