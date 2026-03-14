pub trait IDProvider {
    fn provide_id(&self) -> String;
}

pub struct NanoIdProvider;

impl IDProvider for NanoIdProvider {
    fn provide_id(&self) -> String {
        nanoid::nanoid!(7)
    }
}
