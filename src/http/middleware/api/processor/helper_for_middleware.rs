/// [0]:unaf-svr > [1]:some-middleware > [2]:some-app のように開発する場合に some-middleware または unaf-svr レベルの
/// unaf の中での低中レベル層でのAPI処理実装向けディスパッチヘルパーマクロです。
/// -----
/// 実装で使う際のコード例:
///  - ApiProcessorUnit(A.P.U.) の要件は trait unaf_svr::prelude::ApiProcessorUnit を参照してください。
///  - 実装例の AppTeapot は unaf_svr::prelude::example_impl::UnafTeapot を参照してください。
/// -----
/// ```rust
/// // pub struct ApiMiddleware<S> の impl<S, B> Service<ServiceRequest> for ApiMiddleware<S> の
/// // fn call(&self, mut req: ServiceRequest) -> Self::Future で let svc = self.service.clone(); して、
/// // Box::pin(async move { した中で let api_id = match req.headers().get("x-unaf-api-id") かつ
/// // let pp_in = PreprocessedPayload { ... な実装状態で有効です。
/// // たぶん unaf-svr/src/http/middleware/api/processor/mod.rs のソースを読むのが手っ取り早い。
/// process_for_mw!(api_id, pp_in, req, svc, [UnafTeapot]);
/// ```
#[macro_export]
macro_rules! process_for_mw {
 ($api_id:expr, $pp_in:expr, $req:expr, $svc:expr, [$( $pattern:ident ),*] ) => {{
  match $api_id.as_str()
  {
   $( $pattern::API_ID =>
   {
    let pp_out = processor::$pattern::process($pp_in);
    match pp_out
    {
     Some(pp_out) =>
     {
      $req.extensions_mut().insert(ApiOutput(Either::Right(pp_out)));
      let r = $svc.call($req).await?;
      return Ok(r);
     },
     None =>
     {
      $req
       .extensions_mut()
       .insert(ApiOutput(Either::Left(Err(ApiOutputError::FailedToProcess))));
      let r = $svc.call($req).await?;
      return Ok(r);
     }
    }
   },)*
   _ =>
   {
    let r = ($api_id, $pp_in);
    $req.extensions_mut().insert(ApiOutput(Either::Left(Ok(r))));
    let r = $svc.call($req).await?;
    Ok(r)
   }
  }
 }}
}

pub use process_for_mw;

// 念のため、↑を作成する元になったマクロを使わない状態でのコードを参考用に↓に残しておく。
//
// ここから unaf レベルの API 処理
// match api_id.as_str()
// {
//  ----- ここから API プロセッサー1つぶん -----
//  動作試験用Teapot
//  - `curl 127.0.0.1:50000/api -v -X PUT -d '"neko"' -H Content-Type:application/json -H x-unaf-api-id:unaf/teapot`
//  UnafTeapot::API_ID =>
//  {
//   let pp_out = processor::UnafTeapot::process(pp_in);
//   match pp_out
//   {
//    Some(pp_out) =>
//    {
//     req.extensions_mut().insert(ApiOutput(Either::Right(pp_out)));
//     let r = svc.call(req).await?;
//     return Ok(r);
//    },
//    None =>
//    {
//     req
//      .extensions_mut()
//      .insert(ApiOutput(Either::Left(Err(ApiOutputError::FailedToProcess))));
//     let r = svc.call(req).await?;
//     return Ok(r);
//    }
//   }
//  },
//  ------ ここまで API プロセッサー1つぶん -----
//  _ =>
//  {
//   ここまで来たら API として文法上は正しいが unaf 本体ではなくユーザーコードの API なので Either::Left して後は任せる
//   let r = (api_id, pp_in);
//   req.extensions_mut().insert(ApiOutput(Either::Left(Ok(r))));
//   let r = svc.call(req).await?;
//   Ok(r)
//  }
// }
