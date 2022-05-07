/// [0]:unaf-svr > [1]:some-middleware > [2]:some-app のように開発する場合に some-svr レベルの
/// unaf カスタム製品でカスタムAPIを実装するためのAPI処理実装向けディスパッチヘルパーマクロです。
/// -----
/// 実装で使う際のコード例:
///  - ApiProcessorUnit(A.P.U.) の要件は trait unaf_svr::prelude::ApiProcessorUnit を参照してください。
///  - 実装例の AppTeapot は unaf_svr::prelude::example_impl::UnafTeapot を参照してください。
///  (この紹介ではApp層なので実装名はAppTeapotに変えていますが要件はまったく同じです。)
/// ```rust
/// // actix に service として登録する
/// // pub async fn app_api(req_data: Option<web::ReqData<ApiOutput>>) -> HttpResponse
/// // の中で、
/// process!(api_id, pp_in, req, svc, [AppTeapot]);
/// ```
#[macro_export]
macro_rules! process_for_app {
 ($req_data:expr,[$($pattern:ident),*]) => {{
  use unaf_svr::prelude::*;
  if let Some(either) = api_get_either($req_data)
  {
   if either.is_right()
   {
    // unaf レベルの API が応答済み
    let right = either.unwrap_right();
    return HttpResponse::Ok().content_type(right.content_type).body(right.serialized_params);
   }

   match either.unwrap_left()
   {
    Ok(left) =>
    {
     let (api_id, pp_in) = left;
     // アプリのカスタムAPIはここから定義
     match api_id.as_str()
     {
      // 動作試験用Teapot
      // - `curl 127.0.0.1:50000/api -v -X PUT -d '"neko"' -H Content-Type:application/json -H x-unaf-api-id:unaf/teapot`
      AppTeapot::API_ID =>
      {
       let pp_out = AppTeapot::process(pp_in);
       match pp_out
       {
        Some(pp_out) =>
        {
         return HttpResponse::Ok()
          .content_type(mime::TEXT_PLAIN_UTF_8)
          .body(pp_out.serialized_params)
        },
        None =>
        {
         return HttpResponse::BadRequest()
          .content_type(mime::TEXT_PLAIN_UTF_8)
          .body(format!("AppTeapot, api process was failed."))
        },
       }
      },
      _ => ()
     }
    },
    Err(le) =>
    {
     // unafレベルのAPIパラメーター解析でエラーがあり処理できない場合
     return HttpResponse::BadRequest()
      .content_type(mime::TEXT_PLAIN_UTF_8)
      .body(format!("bad request; {le:?}"));
    }
   }
  }
  // either が取れてない場合 = APIとしての様式は満たしているが何れのAPIにもフックされなかった場合は
  // つまるところ ID が未定義の bad request. またはどうしても /api で unaf 外の理の応答を入れたい場合は
  // ここで入れることになるのかもしれません。
  HttpResponse::NotFound()
   .content_type(mime::TEXT_PLAIN_UTF_8)
   .body(format!("bad request; API-ID is not found."))
 }};
}

pub use process_for_app;

// 念のため、↑を作成する元になったマクロを使わない状態でのコードを参考用に↓に残しておく。
// {
//  if let Some(either) = get_either(req_data)
//  {
//   if either.is_right()
//   {
//    let right = either.unwrap_right();
//    // unaf レベルの API が応答済み
//    return HttpResponse::Ok().content_type(right.content_type).body(right.serialized_params);
//   }
//   match either.unwrap_left()
//   {
//    Ok(left) =>
//    {
//     let (api_id, pp_in) = left;
//     // アプリのカスタムAPIはここから定義
//     match api_id.as_str()
//     {
//      // 動作試験用Teapot
//      // - `curl 127.0.0.1:50000/api -v -X PUT -d '"neko"' -H Content-Type:application/json -H x-unaf-api-id:unaf/teapot`
//      AppTeapot::API_ID =>
//      {
//       let pp_out = AppTeapot::process(pp_in);
//       match pp_out
//       {
//        Some(pp_out) =>
//        {
//         return HttpResponse::Ok()
//          .content_type(mime::TEXT_PLAIN_UTF_8)
//          .body(pp_out.serialized_params)
//        },
//        None =>
//        {
//         return HttpResponse::BadRequest()
//          .content_type(mime::TEXT_PLAIN_UTF_8)
//          .body(format!("AppTeapot, api process was failed."))
//        },
//       }
//      },
//      _ => ()
//     }
//    },
//    Err(le) =>
//    {
//     // unafレベルのAPIパラメーター解析でエラーがあり処理できない場合
//     return HttpResponse::BadRequest()
//      .content_type(mime::TEXT_PLAIN_UTF_8)
//      .body(format!("bad request; {le:?}"));
//    }
//   }
//  }
//  // APIとしての様式は満たしているが何れのAPIにもフックされなかった場合は ID が未定義の bad request.
//  HttpResponse::NotFound()
//   .content_type(mime::TEXT_PLAIN_UTF_8)
//   .body(format!("bad request; API-ID is not found."))
// }

use super::*;
use actix_web::web;

pub fn api_get_right(req_data: Option<web::ReqData<ApiOutput>>) -> Option<ApiOutputEitherRight>
{
 if let Some(req_data) = req_data
 {
  let ApiOutput(either) = req_data.into_inner();
  if let Some(right) = either.right()
  {
   return Some(right);
  }
 }
 None
}

pub fn api_get_left(req_data: Option<web::ReqData<ApiOutput>>) -> Option<ApiOutputEitherLeft>
{
 if let Some(req_data) = req_data
 {
  let ApiOutput(either) = req_data.into_inner();
  if let Some(left) = either.left()
  {
   return Some(left);
  }
 }
 None
}

pub fn api_get_either(
 req_data: Option<web::ReqData<ApiOutput>>
) -> Option<ApiOutputEither>
{
 if let Some(req_data) = req_data
 {
  let ApiOutput(either) = req_data.into_inner();
  return Some(either);
 }
 None
}
