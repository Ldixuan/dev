use actix_web::{web, Error, HttpResponse, post};
use crate::obj::data::Data;

#[post("/chatgpt_predict")] 
pub async fn chatgpt_predict(_query: web::Json<Data>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}

#[cfg(test)]
mod predict_api_tests{
    use actix_web::{test, App, dev::Service, http::{StatusCode, header}};
    use crate::app_config::config_app;
    
    #[actix_web::test]
    pub async fn test_chatgpt_precit(){
        let app = test::init_service(App::new().configure(config_app)).await;
            
        let payload = r#"{"id":12345,"data_type":"fancy","name":"test"}"#.as_bytes();
            
        let req = test::TestRequest::post()
            .uri("/chatgpt_predict")
            .insert_header((header::CONTENT_TYPE, "application/json"))
            .set_payload(payload)
            .to_request();
            
            let resp = app.call(req).await.unwrap();
            
            assert_eq!(resp.status(), StatusCode::OK);
    }
}