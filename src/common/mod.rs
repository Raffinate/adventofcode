pub struct AocFlags {
    pub debug: bool,
}

pub trait AocTask {
    fn solve(&self, flags: AocFlags) -> String;
}
