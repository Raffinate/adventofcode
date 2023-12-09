pub struct AocFlags {
    pub debug: bool,
    pub algorithm: String,
}

pub trait AocTask {
    fn solve(&self, flags: AocFlags) -> String;
}
