use crate::bindings::exports;
use crate::bindings::exports::wasi::filesystem::types::DescriptorBorrow;
use crate::bindings::wasi::filesystem::types::{
    Advice, Descriptor, DescriptorFlags, DescriptorStat, DescriptorType, DirectoryEntry,
    DirectoryEntryStream, ErrorCode, Filesize, MetadataHashValue, NewTimestamp, OpenFlags,
    PathFlags,
};
use crate::bindings::wasi::filesystem::{preopens, types};

impl From<exports::wasi::filesystem::types::DirectoryEntry> for DirectoryEntry {
    fn from(value: exports::wasi::filesystem::types::DirectoryEntry) -> Self {
        Self {
            type_: value.type_.into(),
            name: value.name,
        }
    }
}

impl From<DirectoryEntry> for exports::wasi::filesystem::types::DirectoryEntry {
    fn from(value: DirectoryEntry) -> Self {
        Self {
            type_: value.type_.into(),
            name: value.name,
        }
    }
}

impl From<exports::wasi::filesystem::types::NewTimestamp> for NewTimestamp {
    fn from(value: exports::wasi::filesystem::types::NewTimestamp) -> Self {
        match value {
            exports::wasi::filesystem::types::NewTimestamp::NoChange => NewTimestamp::NoChange,
            exports::wasi::filesystem::types::NewTimestamp::Now => NewTimestamp::Now,
            exports::wasi::filesystem::types::NewTimestamp::Timestamp(v) => {
                NewTimestamp::Timestamp(v)
            }
        }
    }
}

impl From<exports::wasi::filesystem::types::Advice> for Advice {
    fn from(value: exports::wasi::filesystem::types::Advice) -> Self {
        match value {
            exports::wasi::filesystem::types::Advice::Normal => Advice::Normal,
            exports::wasi::filesystem::types::Advice::Sequential => Advice::Sequential,
            exports::wasi::filesystem::types::Advice::Random => Advice::Random,
            exports::wasi::filesystem::types::Advice::WillNeed => Advice::WillNeed,
            exports::wasi::filesystem::types::Advice::DontNeed => Advice::DontNeed,
            exports::wasi::filesystem::types::Advice::NoReuse => Advice::NoReuse,
        }
    }
}

impl From<exports::wasi::filesystem::types::DescriptorType> for DescriptorType {
    fn from(value: exports::wasi::filesystem::types::DescriptorType) -> Self {
        match value {
            exports::wasi::filesystem::types::DescriptorType::Unknown => Self::Unknown,
            exports::wasi::filesystem::types::DescriptorType::BlockDevice => Self::BlockDevice,
            exports::wasi::filesystem::types::DescriptorType::CharacterDevice => {
                Self::CharacterDevice
            }
            exports::wasi::filesystem::types::DescriptorType::Directory => Self::Directory,
            exports::wasi::filesystem::types::DescriptorType::Fifo => Self::Fifo,
            exports::wasi::filesystem::types::DescriptorType::SymbolicLink => Self::SymbolicLink,
            exports::wasi::filesystem::types::DescriptorType::RegularFile => Self::RegularFile,
            exports::wasi::filesystem::types::DescriptorType::Socket => Self::Socket,
        }
    }
}

impl From<DescriptorType> for exports::wasi::filesystem::types::DescriptorType {
    fn from(value: DescriptorType) -> Self {
        match value {
            DescriptorType::Unknown => Self::Unknown,
            DescriptorType::BlockDevice => Self::BlockDevice,
            DescriptorType::CharacterDevice => Self::CharacterDevice,
            DescriptorType::Directory => Self::Directory,
            DescriptorType::Fifo => Self::Fifo,
            DescriptorType::SymbolicLink => Self::SymbolicLink,
            DescriptorType::RegularFile => Self::RegularFile,
            DescriptorType::Socket => Self::Socket,
        }
    }
}

impl From<exports::wasi::filesystem::types::ErrorCode> for ErrorCode {
    fn from(value: exports::wasi::filesystem::types::ErrorCode) -> Self {
        match value {
            exports::wasi::filesystem::types::ErrorCode::Access => Self::Access,
            exports::wasi::filesystem::types::ErrorCode::WouldBlock => Self::WouldBlock,
            exports::wasi::filesystem::types::ErrorCode::Already => Self::Already,
            exports::wasi::filesystem::types::ErrorCode::BadDescriptor => Self::BadDescriptor,
            exports::wasi::filesystem::types::ErrorCode::Busy => Self::Busy,
            exports::wasi::filesystem::types::ErrorCode::Deadlock => Self::Deadlock,
            exports::wasi::filesystem::types::ErrorCode::Quota => Self::Quota,
            exports::wasi::filesystem::types::ErrorCode::Exist => Self::Exist,
            exports::wasi::filesystem::types::ErrorCode::FileTooLarge => Self::FileTooLarge,
            exports::wasi::filesystem::types::ErrorCode::IllegalByteSequence => {
                Self::IllegalByteSequence
            }
            exports::wasi::filesystem::types::ErrorCode::InProgress => Self::InProgress,
            exports::wasi::filesystem::types::ErrorCode::Interrupted => Self::Interrupted,
            exports::wasi::filesystem::types::ErrorCode::Invalid => Self::Invalid,
            exports::wasi::filesystem::types::ErrorCode::Io => Self::Io,
            exports::wasi::filesystem::types::ErrorCode::IsDirectory => Self::IsDirectory,
            exports::wasi::filesystem::types::ErrorCode::Loop => Self::Loop,
            exports::wasi::filesystem::types::ErrorCode::TooManyLinks => Self::TooManyLinks,
            exports::wasi::filesystem::types::ErrorCode::MessageSize => Self::MessageSize,
            exports::wasi::filesystem::types::ErrorCode::NameTooLong => Self::NameTooLong,
            exports::wasi::filesystem::types::ErrorCode::NoDevice => Self::NoDevice,
            exports::wasi::filesystem::types::ErrorCode::NoEntry => Self::NoEntry,
            exports::wasi::filesystem::types::ErrorCode::NoLock => Self::NoLock,
            exports::wasi::filesystem::types::ErrorCode::InsufficientMemory => {
                Self::InsufficientMemory
            }
            exports::wasi::filesystem::types::ErrorCode::InsufficientSpace => {
                Self::InsufficientSpace
            }
            exports::wasi::filesystem::types::ErrorCode::NotDirectory => Self::NotDirectory,
            exports::wasi::filesystem::types::ErrorCode::NotEmpty => Self::NotEmpty,
            exports::wasi::filesystem::types::ErrorCode::NotRecoverable => Self::NotRecoverable,
            exports::wasi::filesystem::types::ErrorCode::Unsupported => Self::Unsupported,
            exports::wasi::filesystem::types::ErrorCode::NoTty => Self::NoTty,
            exports::wasi::filesystem::types::ErrorCode::NoSuchDevice => Self::NoSuchDevice,
            exports::wasi::filesystem::types::ErrorCode::Overflow => Self::Overflow,
            exports::wasi::filesystem::types::ErrorCode::NotPermitted => Self::NotPermitted,
            exports::wasi::filesystem::types::ErrorCode::Pipe => Self::Pipe,
            exports::wasi::filesystem::types::ErrorCode::ReadOnly => Self::ReadOnly,
            exports::wasi::filesystem::types::ErrorCode::InvalidSeek => Self::InvalidSeek,
            exports::wasi::filesystem::types::ErrorCode::TextFileBusy => Self::TextFileBusy,
            exports::wasi::filesystem::types::ErrorCode::CrossDevice => Self::CrossDevice,
        }
    }
}

impl From<ErrorCode> for exports::wasi::filesystem::types::ErrorCode {
    fn from(value: ErrorCode) -> Self {
        match value {
            ErrorCode::Access => Self::Access,
            ErrorCode::WouldBlock => Self::WouldBlock,
            ErrorCode::Already => Self::Already,
            ErrorCode::BadDescriptor => Self::BadDescriptor,
            ErrorCode::Busy => Self::Busy,
            ErrorCode::Deadlock => Self::Deadlock,
            ErrorCode::Quota => Self::Quota,
            ErrorCode::Exist => Self::Exist,
            ErrorCode::FileTooLarge => Self::FileTooLarge,
            ErrorCode::IllegalByteSequence => Self::IllegalByteSequence,
            ErrorCode::InProgress => Self::InProgress,
            ErrorCode::Interrupted => Self::Interrupted,
            ErrorCode::Invalid => Self::Invalid,
            ErrorCode::Io => Self::Io,
            ErrorCode::IsDirectory => Self::IsDirectory,
            ErrorCode::Loop => Self::Loop,
            ErrorCode::TooManyLinks => Self::TooManyLinks,
            ErrorCode::MessageSize => Self::MessageSize,
            ErrorCode::NameTooLong => Self::NameTooLong,
            ErrorCode::NoDevice => Self::NoDevice,
            ErrorCode::NoEntry => Self::NoEntry,
            ErrorCode::NoLock => Self::NoLock,
            ErrorCode::InsufficientMemory => Self::InsufficientMemory,
            ErrorCode::InsufficientSpace => Self::InsufficientSpace,
            ErrorCode::NotDirectory => Self::NotDirectory,
            ErrorCode::NotEmpty => Self::NotEmpty,
            ErrorCode::NotRecoverable => Self::NotRecoverable,
            ErrorCode::Unsupported => Self::Unsupported,
            ErrorCode::NoTty => Self::NoTty,
            ErrorCode::NoSuchDevice => Self::NoSuchDevice,
            ErrorCode::Overflow => Self::Overflow,
            ErrorCode::NotPermitted => Self::NotPermitted,
            ErrorCode::Pipe => Self::Pipe,
            ErrorCode::ReadOnly => Self::ReadOnly,
            ErrorCode::InvalidSeek => Self::InvalidSeek,
            ErrorCode::TextFileBusy => Self::TextFileBusy,
            ErrorCode::CrossDevice => Self::CrossDevice,
        }
    }
}

impl From<exports::wasi::filesystem::types::DescriptorStat> for DescriptorStat {
    fn from(value: exports::wasi::filesystem::types::DescriptorStat) -> Self {
        Self {
            type_: value.type_.into(),
            link_count: value.link_count,
            size: value.size,
            data_access_timestamp: value.data_modification_timestamp,
            data_modification_timestamp: value.data_modification_timestamp,
            status_change_timestamp: value.status_change_timestamp,
        }
    }
}

impl From<DescriptorStat> for exports::wasi::filesystem::types::DescriptorStat {
    fn from(value: DescriptorStat) -> Self {
        Self {
            type_: value.type_.into(),
            link_count: value.link_count,
            size: value.size,
            data_access_timestamp: value.data_modification_timestamp,
            data_modification_timestamp: value.data_modification_timestamp,
            status_change_timestamp: value.status_change_timestamp,
        }
    }
}

impl From<exports::wasi::filesystem::types::MetadataHashValue> for MetadataHashValue {
    fn from(value: exports::wasi::filesystem::types::MetadataHashValue) -> Self {
        Self {
            upper: value.upper,
            lower: value.lower,
        }
    }
}

impl From<MetadataHashValue> for exports::wasi::filesystem::types::MetadataHashValue {
    fn from(value: MetadataHashValue) -> Self {
        Self {
            upper: value.upper,
            lower: value.lower,
        }
    }
}

impl exports::wasi::filesystem::preopens::Guest for () {
    fn get_directories() -> Vec<(exports::wasi::filesystem::types::Descriptor, String)> {
        preopens::get_directories()
            .into_iter()
            .map(|(k, v)| (exports::wasi::filesystem::types::Descriptor::new(k), v))
            .collect()
    }
}

impl exports::wasi::filesystem::types::Guest for () {
    type Descriptor = Descriptor;
    type DirectoryEntryStream = DirectoryEntryStream;

    fn filesystem_error_code(
        err: exports::wasi::io::error::ErrorBorrow<'_>,
    ) -> Option<exports::wasi::filesystem::types::ErrorCode> {
        types::filesystem_error_code(err.get()).map(Into::into)
    }
}

impl exports::wasi::filesystem::types::GuestDescriptor for Descriptor {
    fn read_via_stream(
        &self,
        offset: Filesize,
    ) -> Result<exports::wasi::io::streams::InputStream, exports::wasi::filesystem::types::ErrorCode>
    {
        let res = Self::read_via_stream(self, offset)?;
        Ok(res.into())
    }

    fn write_via_stream(
        &self,
        offset: Filesize,
    ) -> Result<exports::wasi::io::streams::OutputStream, exports::wasi::filesystem::types::ErrorCode>
    {
        let res = Self::write_via_stream(self, offset)?;
        Ok(res.into())
    }

    fn append_via_stream(
        &self,
    ) -> Result<exports::wasi::io::streams::OutputStream, exports::wasi::filesystem::types::ErrorCode>
    {
        let res = Self::append_via_stream(self)?;
        Ok(res.into())
    }

    fn advise(
        &self,
        offset: Filesize,
        length: Filesize,
        advice: exports::wasi::filesystem::types::Advice,
    ) -> Result<(), exports::wasi::filesystem::types::ErrorCode> {
        Self::advise(self, offset, length, advice.into())?;
        Ok(())
    }

    fn sync_data(&self) -> Result<(), exports::wasi::filesystem::types::ErrorCode> {
        Self::sync_data(self)?;
        Ok(())
    }

    fn get_flags(
        &self,
    ) -> Result<
        exports::wasi::filesystem::types::DescriptorFlags,
        exports::wasi::filesystem::types::ErrorCode,
    > {
        let res = Self::get_flags(self)?;
        Ok(exports::wasi::filesystem::types::DescriptorFlags::from_bits_retain(res.bits()))
    }

    fn get_type(
        &self,
    ) -> Result<
        exports::wasi::filesystem::types::DescriptorType,
        exports::wasi::filesystem::types::ErrorCode,
    > {
        let res = Self::get_type(self)?;
        Ok(res.into())
    }

    fn set_size(&self, size: Filesize) -> Result<(), exports::wasi::filesystem::types::ErrorCode> {
        Self::set_size(self, size)?;
        Ok(())
    }

    fn set_times(
        &self,
        data_access_timestamp: exports::wasi::filesystem::types::NewTimestamp,
        data_modification_timestamp: exports::wasi::filesystem::types::NewTimestamp,
    ) -> Result<(), exports::wasi::filesystem::types::ErrorCode> {
        Self::set_times(
            self,
            data_access_timestamp.into(),
            data_modification_timestamp.into(),
        )?;
        Ok(())
    }

    fn read(
        &self,
        length: Filesize,
        offset: Filesize,
    ) -> Result<(Vec<u8>, bool), exports::wasi::filesystem::types::ErrorCode> {
        let res = Self::read(self, length, offset)?;
        Ok(res.into())
    }

    fn write(
        &self,
        buffer: Vec<u8>,
        offset: Filesize,
    ) -> Result<Filesize, exports::wasi::filesystem::types::ErrorCode> {
        let res = Self::write(self, &buffer, offset)?;
        Ok(res.into())
    }

    fn read_directory(
        &self,
    ) -> Result<
        exports::wasi::filesystem::types::DirectoryEntryStream,
        exports::wasi::filesystem::types::ErrorCode,
    > {
        let res = Self::read_directory(self)?;
        Ok(exports::wasi::filesystem::types::DirectoryEntryStream::new(
            res,
        ))
    }

    fn sync(&self) -> Result<(), exports::wasi::filesystem::types::ErrorCode> {
        Self::sync(self)?;
        Ok(())
    }

    fn create_directory_at(
        &self,
        path: String,
    ) -> Result<(), exports::wasi::filesystem::types::ErrorCode> {
        Self::create_directory_at(self, &path)?;
        Ok(())
    }

    fn stat(
        &self,
    ) -> Result<
        exports::wasi::filesystem::types::DescriptorStat,
        exports::wasi::filesystem::types::ErrorCode,
    > {
        let res = Self::stat(self)?;
        Ok(res.into())
    }

    fn stat_at(
        &self,
        path_flags: exports::wasi::filesystem::types::PathFlags,
        path: String,
    ) -> Result<
        exports::wasi::filesystem::types::DescriptorStat,
        exports::wasi::filesystem::types::ErrorCode,
    > {
        let res = Self::stat_at(self, PathFlags::from_bits_retain(path_flags.bits()), &path)?;
        Ok(res.into())
    }

    fn set_times_at(
        &self,
        path_flags: exports::wasi::filesystem::types::PathFlags,
        path: String,
        data_access_timestamp: exports::wasi::filesystem::types::NewTimestamp,
        data_modification_timestamp: exports::wasi::filesystem::types::NewTimestamp,
    ) -> Result<(), exports::wasi::filesystem::types::ErrorCode> {
        Self::set_times_at(
            self,
            PathFlags::from_bits_retain(path_flags.bits()),
            &path,
            data_access_timestamp.into(),
            data_modification_timestamp.into(),
        )?;
        Ok(())
    }

    fn link_at(
        &self,
        old_path_flags: exports::wasi::filesystem::types::PathFlags,
        old_path: String,
        new_descriptor: DescriptorBorrow<'_>,
        new_path: String,
    ) -> Result<(), exports::wasi::filesystem::types::ErrorCode> {
        Self::link_at(
            self,
            PathFlags::from_bits_retain(old_path_flags.bits()),
            &old_path,
            new_descriptor.get(),
            &new_path,
        )?;
        Ok(())
    }

    fn open_at(
        &self,
        path_flags: exports::wasi::filesystem::types::PathFlags,
        path: String,
        open_flags: exports::wasi::filesystem::types::OpenFlags,
        flags: exports::wasi::filesystem::types::DescriptorFlags,
    ) -> Result<
        exports::wasi::filesystem::types::Descriptor,
        exports::wasi::filesystem::types::ErrorCode,
    > {
        let res = Self::open_at(
            self,
            PathFlags::from_bits_retain(path_flags.bits()),
            &path,
            OpenFlags::from_bits_retain(open_flags.bits()),
            DescriptorFlags::from_bits_retain(flags.bits()),
        )?;
        Ok(exports::wasi::filesystem::types::Descriptor::new(res))
    }

    fn readlink_at(
        &self,
        path: String,
    ) -> Result<String, exports::wasi::filesystem::types::ErrorCode> {
        let res = Self::readlink_at(self, &path)?;
        Ok(res)
    }

    fn remove_directory_at(
        &self,
        path: String,
    ) -> Result<(), exports::wasi::filesystem::types::ErrorCode> {
        Self::remove_directory_at(self, &path)?;
        Ok(())
    }

    fn rename_at(
        &self,
        old_path: String,
        new_descriptor: DescriptorBorrow<'_>,
        new_path: String,
    ) -> Result<(), exports::wasi::filesystem::types::ErrorCode> {
        Self::rename_at(self, &old_path, new_descriptor.get(), &new_path)?;
        Ok(())
    }

    fn symlink_at(
        &self,
        old_path: String,
        new_path: String,
    ) -> Result<(), exports::wasi::filesystem::types::ErrorCode> {
        Self::symlink_at(self, &old_path, &new_path)?;
        Ok(())
    }

    fn unlink_file_at(
        &self,
        path: String,
    ) -> Result<(), exports::wasi::filesystem::types::ErrorCode> {
        Self::unlink_file_at(self, &path)?;
        Ok(())
    }

    fn is_same_object(&self, other: DescriptorBorrow<'_>) -> bool {
        Self::is_same_object(self, other.get())
    }

    fn metadata_hash(
        &self,
    ) -> Result<
        exports::wasi::filesystem::types::MetadataHashValue,
        exports::wasi::filesystem::types::ErrorCode,
    > {
        let res = Self::metadata_hash(self)?;
        Ok(res.into())
    }

    fn metadata_hash_at(
        &self,
        path_flags: exports::wasi::filesystem::types::PathFlags,
        path: String,
    ) -> Result<
        exports::wasi::filesystem::types::MetadataHashValue,
        exports::wasi::filesystem::types::ErrorCode,
    > {
        let res =
            Self::metadata_hash_at(self, PathFlags::from_bits_retain(path_flags.bits()), &path)?;
        Ok(res.into())
    }
}

impl exports::wasi::filesystem::types::GuestDirectoryEntryStream for DirectoryEntryStream {
    fn read_directory_entry(
        &self,
    ) -> Result<
        Option<exports::wasi::filesystem::types::DirectoryEntry>,
        exports::wasi::filesystem::types::ErrorCode,
    > {
        let res = Self::read_directory_entry(self)?;
        Ok(res.map(Into::into))
    }
}
