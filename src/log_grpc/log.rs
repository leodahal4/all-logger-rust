use sqlx::{Pool, Postgres};
//use sqlx::{Pool, Postgres};
use tonic::{Request, Response, Status};
use crate::logsaveservice::{log_save_service_server::LogSaveService, MessageRequest, EmptyResponse};
use crate::models::{LogMac, LogPatch};

#[derive(Debug)]
pub struct LogSaveStruct {
    db: Pool<Postgres>
}

#[tonic::async_trait]
impl LogSaveService for LogSaveStruct {
    async fn save_log(&self, req: Request<MessageRequest>) -> Result<Response<EmptyResponse>, Status> {
        let ac_req = grpc_converter(req);
        let _ = match LogMac::create(&self.db ,ac_req).await {
            Ok(_) => {},
            Err(e) => {
                println!("the error is {}", e);
                panic!("{}", e)
            }
        };
        Ok(Response::new(EmptyResponse{}))
    }
}

fn grpc_converter(req: Request<MessageRequest>) -> LogPatch {
    LogPatch {
        log: req.get_ref().log.clone(),
        error: req.get_ref().error,
        priority: req.get_ref().priority,
        send: req.get_ref().send,
        extra: req.get_ref().extra.clone(),
        subdomain: req.get_ref().subdomain.clone(),
        appname: req.get_ref().appname.clone()
    }
}


pub fn newgrpchandler(db: Pool<Postgres>) -> LogSaveStruct {
    LogSaveStruct{db}
}
