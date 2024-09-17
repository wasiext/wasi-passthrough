use crate::bindings::exports;
use crate::bindings::wasi::clocks::monotonic_clock::Duration;
use crate::bindings::wasi::sockets::ip_name_lookup::{self, ResolveAddressStream};
use crate::bindings::wasi::sockets::network::{
    ErrorCode, IpAddress, IpAddressFamily, IpSocketAddress, Network,
};
use crate::bindings::wasi::sockets::tcp::{ShutdownType, TcpSocket};
use crate::bindings::wasi::sockets::udp::{
    IncomingDatagram, IncomingDatagramStream, OutgoingDatagram, OutgoingDatagramStream, UdpSocket,
};
use crate::bindings::wasi::sockets::{instance_network, tcp_create_socket, udp_create_socket};

impl From<exports::wasi::sockets::tcp::TcpSocket> for TcpSocket {
    fn from(value: exports::wasi::sockets::tcp::TcpSocket) -> Self {
        value.into_inner()
    }
}

impl From<TcpSocket> for exports::wasi::sockets::tcp::TcpSocket {
    fn from(value: TcpSocket) -> Self {
        Self::new(value)
    }
}

impl From<exports::wasi::sockets::udp::UdpSocket> for UdpSocket {
    fn from(value: exports::wasi::sockets::udp::UdpSocket) -> Self {
        value.into_inner()
    }
}

impl From<UdpSocket> for exports::wasi::sockets::udp::UdpSocket {
    fn from(value: UdpSocket) -> Self {
        Self::new(value)
    }
}

impl From<exports::wasi::sockets::udp::IncomingDatagramStream> for IncomingDatagramStream {
    fn from(value: exports::wasi::sockets::udp::IncomingDatagramStream) -> Self {
        value.into_inner()
    }
}

impl From<IncomingDatagramStream> for exports::wasi::sockets::udp::IncomingDatagramStream {
    fn from(value: IncomingDatagramStream) -> Self {
        Self::new(value)
    }
}

impl From<exports::wasi::sockets::udp::OutgoingDatagramStream> for OutgoingDatagramStream {
    fn from(value: exports::wasi::sockets::udp::OutgoingDatagramStream) -> Self {
        value.into_inner()
    }
}

impl From<OutgoingDatagramStream> for exports::wasi::sockets::udp::OutgoingDatagramStream {
    fn from(value: OutgoingDatagramStream) -> Self {
        Self::new(value)
    }
}

impl From<exports::wasi::sockets::tcp::ShutdownType> for ShutdownType {
    fn from(value: exports::wasi::sockets::tcp::ShutdownType) -> Self {
        match value {
            exports::wasi::sockets::tcp::ShutdownType::Receive => ShutdownType::Receive,
            exports::wasi::sockets::tcp::ShutdownType::Send => ShutdownType::Send,
            exports::wasi::sockets::tcp::ShutdownType::Both => ShutdownType::Both,
        }
    }
}

impl From<exports::wasi::sockets::udp::OutgoingDatagram> for OutgoingDatagram {
    fn from(value: exports::wasi::sockets::udp::OutgoingDatagram) -> Self {
        Self {
            data: value.data,
            remote_address: value.remote_address,
        }
    }
}

impl From<OutgoingDatagram> for exports::wasi::sockets::udp::OutgoingDatagram {
    fn from(value: OutgoingDatagram) -> Self {
        Self {
            data: value.data,
            remote_address: value.remote_address,
        }
    }
}

impl From<exports::wasi::sockets::udp::IncomingDatagram> for IncomingDatagram {
    fn from(value: exports::wasi::sockets::udp::IncomingDatagram) -> Self {
        Self {
            data: value.data,
            remote_address: value.remote_address,
        }
    }
}

impl From<IncomingDatagram> for exports::wasi::sockets::udp::IncomingDatagram {
    fn from(value: IncomingDatagram) -> Self {
        Self {
            data: value.data,
            remote_address: value.remote_address,
        }
    }
}

impl exports::wasi::sockets::ip_name_lookup::Guest for () {
    type ResolveAddressStream = ResolveAddressStream;

    fn resolve_addresses(
        network: &Network,
        name: String,
    ) -> Result<exports::wasi::sockets::ip_name_lookup::ResolveAddressStream, ErrorCode> {
        let res = ip_name_lookup::resolve_addresses(network, &name)?;
        Ok(exports::wasi::sockets::ip_name_lookup::ResolveAddressStream::new(res))
    }
}

impl exports::wasi::sockets::ip_name_lookup::GuestResolveAddressStream for ResolveAddressStream {
    fn resolve_next_address(&self) -> Result<Option<IpAddress>, ErrorCode> {
        Self::resolve_next_address(self)
    }

    fn subscribe(&self) -> exports::wasi::io::poll::Pollable {
        Self::subscribe(self).into()
    }
}

impl exports::wasi::sockets::tcp::Guest for () {
    type TcpSocket = TcpSocket;
}

impl exports::wasi::sockets::tcp::GuestTcpSocket for TcpSocket {
    fn start_bind(
        &self,
        network: &Network,
        local_address: IpSocketAddress,
    ) -> Result<(), ErrorCode> {
        Self::start_bind(self, network, local_address)
    }

    fn finish_bind(&self) -> Result<(), ErrorCode> {
        Self::finish_bind(self)
    }

    fn start_connect(
        &self,
        network: &Network,
        remote_address: IpSocketAddress,
    ) -> Result<(), ErrorCode> {
        Self::start_connect(self, network, remote_address)
    }

    fn finish_connect(
        &self,
    ) -> Result<
        (
            exports::wasi::io::streams::InputStream,
            exports::wasi::io::streams::OutputStream,
        ),
        ErrorCode,
    > {
        let (a, b) = Self::finish_connect(self)?;
        Ok((a.into(), b.into()))
    }

    fn start_listen(&self) -> Result<(), ErrorCode> {
        Self::start_listen(self)
    }

    fn finish_listen(&self) -> Result<(), ErrorCode> {
        Self::finish_listen(self)
    }

    fn accept(
        &self,
    ) -> Result<
        (
            exports::wasi::sockets::tcp::TcpSocket,
            exports::wasi::io::streams::InputStream,
            exports::wasi::io::streams::OutputStream,
        ),
        ErrorCode,
    > {
        let (a, b, c) = Self::accept(self)?;
        Ok((a.into(), b.into(), c.into()))
    }

    fn local_address(&self) -> Result<IpSocketAddress, ErrorCode> {
        Self::local_address(self)
    }

    fn remote_address(&self) -> Result<IpSocketAddress, ErrorCode> {
        Self::remote_address(self)
    }

    fn is_listening(&self) -> bool {
        Self::is_listening(self)
    }

    fn address_family(&self) -> IpAddressFamily {
        Self::address_family(self)
    }

    fn set_listen_backlog_size(&self, value: u64) -> Result<(), ErrorCode> {
        Self::set_listen_backlog_size(self, value)
    }

    fn keep_alive_enabled(&self) -> Result<bool, ErrorCode> {
        Self::keep_alive_enabled(self)
    }

    fn set_keep_alive_enabled(&self, value: bool) -> Result<(), ErrorCode> {
        Self::set_keep_alive_enabled(self, value)
    }

    fn keep_alive_idle_time(&self) -> Result<Duration, ErrorCode> {
        Self::keep_alive_idle_time(self)
    }

    fn set_keep_alive_idle_time(&self, value: Duration) -> Result<(), ErrorCode> {
        Self::set_keep_alive_idle_time(self, value)
    }

    fn keep_alive_interval(&self) -> Result<Duration, ErrorCode> {
        Self::keep_alive_interval(self)
    }

    fn set_keep_alive_interval(&self, value: Duration) -> Result<(), ErrorCode> {
        Self::set_keep_alive_interval(self, value)
    }

    fn keep_alive_count(&self) -> Result<u32, ErrorCode> {
        Self::keep_alive_count(self)
    }

    fn set_keep_alive_count(&self, value: u32) -> Result<(), ErrorCode> {
        Self::set_keep_alive_count(self, value)
    }

    fn hop_limit(&self) -> Result<u8, ErrorCode> {
        Self::hop_limit(self)
    }

    fn set_hop_limit(&self, value: u8) -> Result<(), ErrorCode> {
        Self::set_hop_limit(self, value)
    }

    fn receive_buffer_size(&self) -> Result<u64, ErrorCode> {
        Self::receive_buffer_size(self)
    }

    fn set_receive_buffer_size(&self, value: u64) -> Result<(), ErrorCode> {
        Self::set_receive_buffer_size(self, value)
    }

    fn send_buffer_size(&self) -> Result<u64, ErrorCode> {
        Self::send_buffer_size(self)
    }

    fn set_send_buffer_size(&self, value: u64) -> Result<(), ErrorCode> {
        Self::set_send_buffer_size(self, value)
    }

    fn subscribe(&self) -> exports::wasi::io::poll::Pollable {
        Self::subscribe(self).into()
    }

    fn shutdown(
        &self,
        shutdown_type: exports::wasi::sockets::tcp::ShutdownType,
    ) -> Result<(), ErrorCode> {
        Self::shutdown(self, shutdown_type.into())
    }
}

impl exports::wasi::sockets::udp::Guest for () {
    type UdpSocket = UdpSocket;
    type IncomingDatagramStream = IncomingDatagramStream;
    type OutgoingDatagramStream = OutgoingDatagramStream;
}

impl exports::wasi::sockets::udp::GuestUdpSocket for UdpSocket {
    fn start_bind(
        &self,
        network: &Network,
        local_address: IpSocketAddress,
    ) -> Result<(), ErrorCode> {
        Self::start_bind(self, network, local_address)
    }

    fn finish_bind(&self) -> Result<(), ErrorCode> {
        Self::finish_bind(self)
    }

    fn stream(
        &self,
        remote_address: Option<IpSocketAddress>,
    ) -> Result<
        (
            exports::wasi::sockets::udp::IncomingDatagramStream,
            exports::wasi::sockets::udp::OutgoingDatagramStream,
        ),
        ErrorCode,
    > {
        let (a, b) = Self::stream(self, remote_address)?;
        Ok((a.into(), b.into()))
    }

    fn local_address(&self) -> Result<IpSocketAddress, ErrorCode> {
        Self::local_address(self)
    }

    fn remote_address(&self) -> Result<IpSocketAddress, ErrorCode> {
        Self::remote_address(self)
    }

    fn address_family(&self) -> IpAddressFamily {
        Self::address_family(self)
    }

    fn unicast_hop_limit(&self) -> Result<u8, ErrorCode> {
        Self::unicast_hop_limit(self)
    }

    fn set_unicast_hop_limit(&self, value: u8) -> Result<(), ErrorCode> {
        Self::set_unicast_hop_limit(self, value)
    }

    fn receive_buffer_size(&self) -> Result<u64, ErrorCode> {
        Self::receive_buffer_size(self)
    }

    fn set_receive_buffer_size(&self, value: u64) -> Result<(), ErrorCode> {
        Self::set_receive_buffer_size(self, value)
    }

    fn send_buffer_size(&self) -> Result<u64, ErrorCode> {
        Self::send_buffer_size(self)
    }

    fn set_send_buffer_size(&self, value: u64) -> Result<(), ErrorCode> {
        Self::set_send_buffer_size(self, value)
    }

    fn subscribe(&self) -> exports::wasi::io::poll::Pollable {
        Self::subscribe(self).into()
    }
}

impl exports::wasi::sockets::udp::GuestIncomingDatagramStream for IncomingDatagramStream {
    fn receive(
        &self,
        max_results: u64,
    ) -> Result<Vec<exports::wasi::sockets::udp::IncomingDatagram>, ErrorCode> {
        let res = Self::receive(self, max_results)?;
        Ok(res.into_iter().map(Into::into).collect())
    }

    fn subscribe(&self) -> exports::wasi::io::poll::Pollable {
        Self::subscribe(self).into()
    }
}

impl exports::wasi::sockets::udp::GuestOutgoingDatagramStream for OutgoingDatagramStream {
    fn check_send(&self) -> Result<u64, ErrorCode> {
        Self::check_send(self)
    }

    fn send(
        &self,
        datagrams: Vec<exports::wasi::sockets::udp::OutgoingDatagram>,
    ) -> Result<u64, ErrorCode> {
        Self::send(
            self,
            &datagrams.into_iter().map(Into::into).collect::<Vec<_>>(),
        )
    }

    fn subscribe(&self) -> exports::wasi::io::poll::Pollable {
        Self::subscribe(self).into()
    }
}

impl exports::wasi::sockets::tcp_create_socket::Guest for () {
    fn create_tcp_socket(
        address_family: IpAddressFamily,
    ) -> Result<exports::wasi::sockets::tcp::TcpSocket, ErrorCode> {
        let res = tcp_create_socket::create_tcp_socket(address_family)?;
        Ok(res.into())
    }
}

impl exports::wasi::sockets::udp_create_socket::Guest for () {
    fn create_udp_socket(
        address_family: IpAddressFamily,
    ) -> Result<exports::wasi::sockets::udp::UdpSocket, ErrorCode> {
        let res = udp_create_socket::create_udp_socket(address_family)?;
        Ok(res.into())
    }
}

impl exports::wasi::sockets::instance_network::Guest for () {
    fn instance_network() -> Network {
        instance_network::instance_network()
    }
}
