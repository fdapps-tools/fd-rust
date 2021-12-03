use neon::prelude::*;

fn attach_routes(mut cx: FunctionContext) -> JsResult<JsObject> {
    let router: Handle<JsObject> = cx.argument(0)?;
    
    let func = router
    .get(&mut cx, "get")?
    .downcast_or_throw::<JsFunction, _>(&mut cx)?;
    
    let null = cx.null();
    let route = cx.string("/rust-route-created");
    let param = cx.string("----param");

    let router = func.call(&mut cx, null, vec![route, param])?;

    let obj = cx.empty_object();
    obj.set(&mut cx, "router", router)?;

    Ok(obj)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("attach_routes", attach_routes)?;
    Ok(())
}
