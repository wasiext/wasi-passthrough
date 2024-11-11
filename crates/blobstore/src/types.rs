use crate::bindings::exports;
use crate::bindings::wasi::blobstore::types::{
    ContainerMetadata, Error, IncomingValue, ObjectId, ObjectMetadata, OutgoingValue,
};

impl From<exports::wasi::blobstore::types::IncomingValue> for IncomingValue {
    fn from(value: exports::wasi::blobstore::types::IncomingValue) -> Self {
        value.into_inner()
    }
}

impl From<IncomingValue> for exports::wasi::blobstore::types::IncomingValue {
    fn from(value: IncomingValue) -> Self {
        Self::new(value)
    }
}

impl From<exports::wasi::blobstore::types::OutgoingValue> for OutgoingValue {
    fn from(value: exports::wasi::blobstore::types::OutgoingValue) -> Self {
        value.into_inner()
    }
}

impl From<OutgoingValue> for exports::wasi::blobstore::types::OutgoingValue {
    fn from(value: OutgoingValue) -> Self {
        Self::new(value)
    }
}

impl From<ContainerMetadata> for exports::wasi::blobstore::types::ContainerMetadata {
    fn from(value: ContainerMetadata) -> Self {
        Self {
            name: value.name,
            created_at: value.created_at,
        }
    }
}

impl From<exports::wasi::blobstore::types::ContainerMetadata> for ContainerMetadata {
    fn from(value: exports::wasi::blobstore::types::ContainerMetadata) -> Self {
        Self {
            name: value.name,
            created_at: value.created_at,
        }
    }
}

impl From<ObjectMetadata> for exports::wasi::blobstore::types::ObjectMetadata {
    fn from(value: ObjectMetadata) -> Self {
        Self {
            name: value.name,
            container: value.container,
            created_at: value.created_at,
            size: value.size,
        }
    }
}

impl From<exports::wasi::blobstore::types::ObjectMetadata> for ObjectMetadata {
    fn from(value: exports::wasi::blobstore::types::ObjectMetadata) -> Self {
        Self {
            name: value.name,
            container: value.container,
            created_at: value.created_at,
            size: value.size,
        }
    }
}

impl From<ObjectId> for exports::wasi::blobstore::types::ObjectId {
    fn from(value: ObjectId) -> Self {
        Self {
            container: value.container,
            object: value.object,
        }
    }
}

impl From<exports::wasi::blobstore::types::ObjectId> for ObjectId {
    fn from(value: exports::wasi::blobstore::types::ObjectId) -> Self {
        Self {
            container: value.container,
            object: value.object,
        }
    }
}

impl exports::wasi::blobstore::types::GuestOutgoingValue for OutgoingValue {
    fn new_outgoing_value() -> exports::wasi::blobstore::types::OutgoingValue {
        Self::new_outgoing_value().into()
    }

    fn outgoing_value_write_body(
        &self,
    ) -> Result<exports::wasi::blobstore::types::OutputStream, ()> {
        let res = Self::outgoing_value_write_body(self)?;
        Ok(res.into())
    }

    fn finish(this: exports::wasi::blobstore::types::OutgoingValue) -> Result<(), Error> {
        let res = Self::finish(this.into())?;
        Ok(res.into())
    }
}

impl exports::wasi::blobstore::types::GuestIncomingValue for IncomingValue {
    fn incoming_value_consume_sync(
        this: exports::wasi::blobstore::types::IncomingValue,
    ) -> Result<exports::wasi::blobstore::types::IncomingValueSyncBody, Error> {
        let res = Self::incoming_value_consume_sync(this.into_inner())?;
        Ok(res)
    }

    fn incoming_value_consume_async(
        this: exports::wasi::blobstore::types::IncomingValue,
    ) -> Result<exports::wasi::blobstore::types::InputStream, Error> {
        let res = Self::incoming_value_consume_async(this.into_inner())?;
        Ok(res)
    }

    fn size(&self) -> u64 {
        Self::size(self)
    }
}

impl exports::wasi::blobstore::types::Guest for () {
    type OutgoingValue = OutgoingValue;
    type IncomingValue = IncomingValue;
}
