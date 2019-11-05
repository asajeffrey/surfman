// surfman/surfman/src/implementation/connection.rs
//
//! This is an included private module that automatically produces the implementation of the
//! `Connection` trait for a backend.

use crate::Error;
use crate::connection::Connection as ConnectionInterface;
use super::super::adapter::Adapter;
use super::super::connection::Connection;

#[cfg(feature = "sm-winit")]
use winit::Window;

impl ConnectionInterface for Connection {
    type Adapter = Adapter;

    #[inline]
    fn new() -> Result<Connection, Error> {
        Connection::new()
    }

    #[inline]
    fn create_adapter(&self) -> Result<Adapter, Error> {
        Connection::create_adapter(self)
    }

    #[inline]
    fn create_hardware_adapter(&self) -> Result<Adapter, Error> {
        Connection::create_hardware_adapter(self)
    }

    #[inline]
    fn create_software_adapter(&self) -> Result<Adapter, Error> {
        Connection::create_software_adapter(self)
    }

    #[inline]
    #[cfg(feature = "sm-winit")]
    fn from_winit_window(window: &Window) -> Result<Connection, Error> {
        Connection::from_winit_window(window)
    }
}