#![allow(dead_code,non_camel_case_types,unused_unsafe,unused_variables)]
#![allow(non_upper_case_globals,non_snake_case,unused_imports)]

pub mod interfaces {
    use sys::wayland::interfaces::{wl_surface_interface, wl_seat_interface, wl_output_interface};
    include!(concat!(env!("OUT_DIR"), "/xdg_shell_interfaces.rs"));
}

pub mod server {
    use sys::wayland::server::{WlSeat, WlSurface, WlOutput};
    include!(concat!(env!("OUT_DIR"), "/xdg_shell_server_api.rs"));
}