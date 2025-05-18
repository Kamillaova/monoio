use std::{io, net::SocketAddr};
use io_uring::{opcode, types};
use crate::driver::ready::Direction;
use super::{MaybeFd, Op, OpAble};

pub(crate) struct Socket {
    domain: libc::c_int,
    socket_type: libc::c_int,
}

impl Op<Socket> {
    /// Submit a request to socket(2).
    pub(crate) fn socket(
        domain: libc::c_int,
        socket_type: libc::c_int,
    ) -> io::Result<Op<Socket>> {
        Op::submit_with(Socket {
            domain,
            socket_type,
        })
    }
}

impl OpAble for Socket {
    #[cfg(all(target_os = "linux", feature = "iouring"))]
    fn uring_op(&mut self) -> io_uring::squeue::Entry {
        opcode::Socket::new(
            self.domain,
            self.socket_type | libc::SOCK_CLOEXEC,
            0,
        )
            .build()
    }

    #[cfg(any(feature = "legacy", feature = "poll-io"))]
    #[inline]
    fn legacy_interest(&self) -> Option<(Direction, usize)> {
        panic!("no legacy here");
    }

    #[cfg(any(feature = "legacy", feature = "poll-io"))]
    fn legacy_call(&mut self) -> io::Result<MaybeFd> {
        panic!("no legacy here");
    }
}
