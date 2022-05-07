use super::PreprocessedPayload;

pub trait ApiProcessorUnit
{
 const API_ID: &'static str;
 fn process(pp: PreprocessedPayload) -> Option<PreprocessedPayload>;
}
