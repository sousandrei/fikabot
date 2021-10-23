mod fika;
mod song;

pub fn start() {
    async_std::task::spawn(fika::start());
    async_std::task::spawn(song::start());
}
