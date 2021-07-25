pub mod run;
mod url;
mod analyze;

pub struct ParamNameVal<'a> {
    pub parameter: &'a str,
    pub value: &'a str,
    pub index: usize
}

