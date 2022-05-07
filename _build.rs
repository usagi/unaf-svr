use once_cell::sync::OnceCell;
use std::fmt::Write;

fn main() -> Result<(), Box<dyn std::error::Error>>
{
 // ref: https://docs.rs/tonic-build/latest/tonic_build/index.html

 const PROTO_DIR: &str = "../proto";
 static PROTO_FILES: OnceCell<Vec<String>> = OnceCell::new();

 build_pretty::build_pretty!()
  .enque_fn(
   "retrieve .proto files",
   Box::new(move |o| {
    let dir = std::fs::read_dir(PROTO_DIR).unwrap();
    let files = dir
     .into_iter()
     .filter_map(|entry| {
      let dir_entry = entry.ok()?;
      let file_type = dir_entry.file_type().ok()?;
      file_type.is_file().then(|| ())?;
      let path = dir_entry.path();
      path
       .extension()
       .and_then(|ext| ext.to_str()?.eq_ignore_ascii_case("proto").then(|| ()))?;
      let path = path.to_str()?.to_string();
      o.write_fmt(format_args!("found 👉 {path:?}\n")).unwrap();
      Some(path)
     })
     .collect::<Vec<_>>();
    PROTO_FILES.get_or_init(move || files);
    Ok(())
   })
  )
  .enque_fn(
   "compile .proto files",
   Box::new(|o| {
    for proto_file in PROTO_FILES.get().unwrap()
    {
     o.write_fmt(format_args!("begin compile {proto_file:?}"))?;
     tonic_build::configure().build_client(false).compile(&[proto_file], &[PROTO_DIR])?;
     o.write_fmt(format_args!(" 👈 [Succeeded] ⭐"))?;
    }
    Ok(())
   })
  );

 Ok(())
}
