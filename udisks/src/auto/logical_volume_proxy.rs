// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use crate::{LogicalVolume};

glib::wrapper! {
    #[doc(alias = "UDisksLogicalVolumeProxy")]
    pub struct LogicalVolumeProxy(Object<ffi::UDisksLogicalVolumeProxy, ffi::UDisksLogicalVolumeProxyClass>) @implements LogicalVolume;

    match fn {
        type_ => || ffi::udisks_logical_volume_proxy_get_type(),
    }
}

impl LogicalVolumeProxy {
        pub const NONE: Option<&'static LogicalVolumeProxy> = None;
    

    //#[doc(alias = "udisks_logical_volume_proxy_new_for_bus_sync")]
    //#[doc(alias = "new_for_bus_sync")]
    //pub fn for_bus_sync(bus_type: /*Ignored*/gio::BusType, flags: /*Ignored*/gio::DBusProxyFlags, name: &str, object_path: &str, cancellable: /*Ignored*/Option<&gio::Cancellable>, error: /*Ignored*/Option<glib::Error>) -> LogicalVolumeProxy {
    //    unsafe { TODO: call ffi:udisks_logical_volume_proxy_new_for_bus_sync() }
    //}

    //#[doc(alias = "udisks_logical_volume_proxy_new_sync")]
    //pub fn new_sync(connection: /*Ignored*/&gio::DBusConnection, flags: /*Ignored*/gio::DBusProxyFlags, name: Option<&str>, object_path: &str, cancellable: /*Ignored*/Option<&gio::Cancellable>, error: /*Ignored*/Option<glib::Error>) -> LogicalVolumeProxy {
    //    unsafe { TODO: call ffi:udisks_logical_volume_proxy_new_sync() }
    //}

    //#[doc(alias = "udisks_logical_volume_proxy_new")]
    //pub fn new<P: FnOnce(Result<LogicalVolumeProxy, /*Ignored*/glib::Error>) + 'static>(connection: /*Ignored*/&gio::DBusConnection, flags: /*Ignored*/gio::DBusProxyFlags, name: Option<&str>, object_path: &str, cancellable: /*Ignored*/Option<&gio::Cancellable>, callback: P) {
    //    unsafe { TODO: call ffi:udisks_logical_volume_proxy_new() }
    //}

    //
    //pub fn new_future(connection: /*Ignored*/&gio::DBusConnection, flags: /*Ignored*/gio::DBusProxyFlags, name: Option<&str>, object_path: &str) -> Pin<Box_<dyn std::future::Future<Output = Result<LogicalVolumeProxy, /*Ignored*/glib::Error>> + 'static>> {

        //skip_assert_initialized!();
        //let connection = connection.clone();
        //let name = name.map(ToOwned::to_owned);
        //let object_path = String::from(object_path);
        //Box_::pin(gio::GioFuture::new(&(), move |_obj, cancellable, send| {
        //    Self::new(
        //        &connection,
        //        flags,
        //        name.as_ref().map(::std::borrow::Borrow::borrow),
        //        &object_path,
        //        Some(cancellable),
        //        move |res| {
        //            send.resolve(res);
        //        },
        //    );
        //}))
    //}

    //#[doc(alias = "udisks_logical_volume_proxy_new_for_bus")]
    //pub fn new_for_bus<P: FnOnce(Result<LogicalVolumeProxy, /*Ignored*/glib::Error>) + 'static>(bus_type: /*Ignored*/gio::BusType, flags: /*Ignored*/gio::DBusProxyFlags, name: &str, object_path: &str, cancellable: /*Ignored*/Option<&gio::Cancellable>, callback: P) {
    //    unsafe { TODO: call ffi:udisks_logical_volume_proxy_new_for_bus() }
    //}

    //
    //pub fn new_for_bus_future(bus_type: /*Ignored*/gio::BusType, flags: /*Ignored*/gio::DBusProxyFlags, name: &str, object_path: &str) -> Pin<Box_<dyn std::future::Future<Output = Result<LogicalVolumeProxy, /*Ignored*/glib::Error>> + 'static>> {

        //skip_assert_initialized!();
        //let name = String::from(name);
        //let object_path = String::from(object_path);
        //Box_::pin(gio::GioFuture::new(&(), move |_obj, cancellable, send| {
        //    Self::new_for_bus(
        //        bus_type,
        //        flags,
        //        &name,
        //        &object_path,
        //        Some(cancellable),
        //        move |res| {
        //            send.resolve(res);
        //        },
        //    );
        //}))
    //}
}
