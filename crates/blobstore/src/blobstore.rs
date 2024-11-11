use crate::bindings::exports;
use crate::bindings::wasi::blobstore::blobstore;
use crate::bindings::wasi::blobstore::types::Error;

impl exports::wasi::blobstore::blobstore::Guest for () {
    fn create_container(
        name: String,
    ) -> Result<exports::wasi::blobstore::container::Container, Error> {
        let res = blobstore::create_container(&name)?;
        Ok(res.into())
    }

    fn get_container(
        name: String,
    ) -> Result<exports::wasi::blobstore::container::Container, Error> {
        let res = blobstore::get_container(&name)?;
        Ok(res.into())
    }

    fn delete_container(name: String) -> Result<(), Error> {
        blobstore::delete_container(&name)
    }

    fn container_exists(name: String) -> Result<bool, Error> {
        blobstore::container_exists(&name)
    }

    fn copy_object(
        src: exports::wasi::blobstore::types::ObjectId,
        dest: exports::wasi::blobstore::types::ObjectId,
    ) -> Result<(), Error> {
        blobstore::copy_object(&src.into(), &dest.into())
    }

    fn move_object(
        src: exports::wasi::blobstore::types::ObjectId,
        dest: exports::wasi::blobstore::types::ObjectId,
    ) -> Result<(), Error> {
        blobstore::move_object(&src.into(), &dest.into())
    }
}
