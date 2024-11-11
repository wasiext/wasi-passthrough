use crate::bindings::exports;
use crate::bindings::exports::wasi::blobstore::types::OutgoingValueBorrow;
use crate::bindings::wasi::blobstore::container::{Container, StreamObjectNames};
use crate::bindings::wasi::blobstore::types::Error;

impl From<exports::wasi::blobstore::container::StreamObjectNames> for StreamObjectNames {
    fn from(value: exports::wasi::blobstore::container::StreamObjectNames) -> Self {
        value.into_inner()
    }
}

impl From<StreamObjectNames> for exports::wasi::blobstore::container::StreamObjectNames {
    fn from(value: StreamObjectNames) -> Self {
        Self::new(value)
    }
}

impl From<exports::wasi::blobstore::container::Container> for Container {
    fn from(value: exports::wasi::blobstore::container::Container) -> Self {
        value.into_inner()
    }
}

impl From<Container> for exports::wasi::blobstore::container::Container {
    fn from(value: Container) -> Self {
        Self::new(value)
    }
}

impl exports::wasi::blobstore::container::GuestContainer for Container {
    fn name(&self) -> Result<String, Error> {
        Self::name(self)
    }

    fn info(&self) -> Result<exports::wasi::blobstore::types::ContainerMetadata, Error> {
        let res = Self::info(self)?;
        Ok(res.into())
    }

    fn get_data(
        &self,
        name: String,
        start: u64,
        end: u64,
    ) -> Result<exports::wasi::blobstore::types::IncomingValue, Error> {
        let res = Self::get_data(self, &name, start, end)?;
        Ok(res.into())
    }

    fn write_data(&self, name: String, data: OutgoingValueBorrow<'_>) -> Result<(), Error> {
        Self::write_data(self, &name, data.get())
    }

    fn list_objects(
        &self,
    ) -> Result<exports::wasi::blobstore::container::StreamObjectNames, Error> {
        let res = Self::list_objects(self)?;
        Ok(res.into())
    }

    fn delete_object(&self, name: String) -> Result<(), Error> {
        Self::delete_object(self, &name)
    }

    fn delete_objects(&self, names: Vec<String>) -> Result<(), Error> {
        Self::delete_objects(self, &names)
    }

    fn has_object(&self, name: String) -> Result<bool, Error> {
        Self::has_object(self, &name)
    }

    fn object_info(
        &self,
        name: String,
    ) -> Result<exports::wasi::blobstore::types::ObjectMetadata, Error> {
        let res = Self::object_info(self, &name)?;
        Ok(res.into())
    }

    fn clear(&self) -> Result<(), Error> {
        Self::clear(self)
    }
}

impl exports::wasi::blobstore::container::GuestStreamObjectNames for StreamObjectNames {
    fn read_stream_object_names(&self, len: u64) -> Result<(Vec<String>, bool), Error> {
        Self::read_stream_object_names(self, len)
    }

    fn skip_stream_object_names(&self, num: u64) -> Result<(u64, bool), Error> {
        Self::skip_stream_object_names(self, num)
    }
}

impl exports::wasi::blobstore::container::Guest for () {
    type Container = Container;
    type StreamObjectNames = StreamObjectNames;
}
