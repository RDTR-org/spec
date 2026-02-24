pub trait HostApi {
    fn send_network(&self, data: &[u8]);
    fn request_focus(&self, id: u32);
    fn open_overlay(&self);
    fn close_overlay(&self);
}
