use actix_web::error::ResponseError;
use macros_create_app::make_app71;
use macros_make_error::make_error2;
use macros_make_model::make_model23;
use macros_make_model::make_model35;
use macros_make_scope::make_scope1;
use serde::*;
use serde_repr::{Deserialize_repr, Serialize_repr};
use url::Url;

make_error2!({{crate_name | upper_camel_case }}Error);

make_model23!(
    Q{{crate_name | upper_camel_case }},
    I{{crate_name | upper_camel_case }},
    O{{crate_name | upper_camel_case }},
    name: String,
);

make_model35!(
    Q,
    I,
    O,
    {
        {
            crate_name
        }
    },
    name: String
);

make_app71!(
    [name: String],
    route,
    "/{{crate_name}}",
    "/{{crate_name | replace_first: "lib_", ""}}/{id}",
    "",
    "{id}",
    O,
    Q,
    I,
    {{crate_name}},
    [
        |s: actix_web::web::Data<my_state::MyState>,
         json: actix_web::web::Json<route::IRequest>,
         wallet: lib_wallet::QWallet,
         http_request: actix_web::HttpRequest| async move { handle(s, json, wallet).await }
    ],
    ProductError
);

make_scope1!("product", [put, route]);

async fn handle(
    s: actix_web::web::Data<my_state::MyState>,
    json: actix_web::web::Json<route::IRequest>,
    _: lib_wallet::QWallet,
) -> Result<Q, {{crate_name | upper_camel_case }}Error> {
    tokio::spawn(async {
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
        tracing::info!("good day mat");
        ()
    });
    if json.data.search.is_empty() {
        Ok(Q::default())
    } else {
        product::postgres_query::insert(&s.sqlx_pool, &json.data)
            .await
            .map_err({{crate_name | upper_camel_case }}Error::from_general)
    }
}
