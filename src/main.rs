use std::{collections::HashMap, future::pending, sync::Arc, time::Duration};

use fake::{faker::name::en::{FirstName, LastName}, Fake};
use lettre::{message::header::ContentType, transport::smtp::authentication::Credentials, AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use thirtyfour::{prelude::ElementQueryable, By, ChromiumLikeCapabilities, DesiredCapabilities, WebDriver};
use tokio::sync::{mpsc, oneshot};
use tracing::{error, info};
use tracing_subscriber::fmt;
use uuid::Uuid;


#[tokio::main]
async fn main() {
    let _ = fmt().init();

    let sender = create_sub_email().await;


    for _ in [0;300] {

        let email = random_generate_email_account("onew197482", "@2925.com");
        let (first_name, last_name) = random_generate_name();
        let _  = sender.send((email, first_name, last_name)).await;
    }

    pending::<()>().await

}



async fn submit_white_list() -> mpsc::Sender<((String, String, String), oneshot::Sender<Result<(String, String, String), (String, String, String)>>)>{


    let (tx, mut rx) = mpsc::channel::<((String, String, String), oneshot::Sender<Result<(String, String, String), (String, String, String)>>)>(1024);

    tokio::spawn(async move {
        while let Some(((email, first_name, last_name), cb)) = rx.recv().await {
            let mut caps = DesiredCapabilities::chrome();
            let _  = caps.set_headless();

            let driver = match WebDriver::new("http://172.20.64.1:2287", caps).await {
                Ok(driver) => driver,
                Err(e) => {
                    error!("connect http://172.20.64.1:2287 error. {}", e);
                    let _  = cb.send(Err((email, first_name, last_name)));
                    continue;
                }
            };
    
            match driver.goto("https://saharalabs.ai/").await {
                Ok(_) => {},
                Err(e) => {
                    error!("driver goto https://saharalabs.ai/ error. {}", e);
                    let _  = cb.send(Err((email, first_name, last_name)));
                    continue;
                }
            };
    
            let start_button = match driver.find(By::ClassName("framer-y548na-container")).await {
                Ok(start_button) => start_button,
                Err(e) => {
                    error!("can't find start button. {}", e);
                    let _  = cb.send(Err((email, first_name, last_name)));
                    continue;
                }
            };
            
            info!("waiting 2 second");
            tokio::time::sleep(Duration::from_secs(2)).await;
            let _ = match start_button.click().await {
                Ok(_) => {},
                Err(e) => {
                    error!("click start button error. {}", e);
                    let _  = cb.send(Err((email, first_name, last_name)));
                    continue;
                }
            };
    
            
            let first_name_input = match driver.query(By::Id("firstname-b8ef3646-4f6a-4389-a9b1-3958123c4778_67")).and_enabled().first().await {
                Ok(first_name_input) => first_name_input,
                Err(e) => {
                    error!("can't find first name input. {}", e);
                    let _  = cb.send(Err((email, first_name, last_name)));
                    continue;
                }
            };
            let _ = match first_name_input.send_keys(first_name.clone()).await {
                Err(e) => {
                    error!("input first name error. {}", e);
                    let _  = cb.send(Err((email, first_name, last_name)));
                    continue;
                },
                _ => {}
            };
    
    
    
            let last_name_input = match driver.query(By::Id("lastname-b8ef3646-4f6a-4389-a9b1-3958123c4778_67")).and_enabled().first().await {
                Ok(last_name_input) => last_name_input,
                Err(e) => {
                    error!("can't find last name input. {}", e);
                    let _  = cb.send(Err((email, first_name, last_name)));
                    continue;
                }
            };
    
            let _  = match last_name_input.send_keys(last_name.clone()).await {
                Err(e) => {
                    error!("input last name error. {}", e);
                    let _  = cb.send(Err((email, first_name, last_name)));
                    continue;
                },
                _ => {}
            };
    
    
            let email_input = match driver.query(By::Id("email-b8ef3646-4f6a-4389-a9b1-3958123c4778_67")).and_enabled().first().await {
                Ok(email_input) => email_input,
                Err(e) => {
                    error!("can't find email input. {}", e);
                    let _  = cb.send(Err((email, first_name, last_name)));
                    continue;
                }
            };
            let _  = match email_input.send_keys(email.clone()).await {
                Err(e) => {
                    error!("input email error. {}", e);
                    let _  = cb.send(Err((email, first_name, last_name)));
                    continue;
                },
                _ => {}
            };
    
    
            let first_option = match driver.query(By::Css("label.hs-form-radio-display[for=user_role0-b8ef3646-4f6a-4389-a9b1-3958123c4778_67]")).and_enabled().first().await {
                Ok(first_option) => first_option,
                Err(e) => {
                    error!("can't find first option. {}", e);
                    let _  = cb.send(Err((email, first_name, last_name)));
                    continue;
                }
            };
    
            let _  = match first_option.click().await {
                Err(e) => {
                    error!("click first option error. {}", e);
                    let _  = cb.send(Err((email, first_name, last_name)));
                    continue;
                },
                _ => {}
            };
    
    
    
            let first_check_box = match driver.query(By::Css("label.hs-form-checkbox-display[for=knowledge_types0-b8ef3646-4f6a-4389-a9b1-3958123c4778_67]")).and_enabled().first().await {
                Ok(first_check_box) => first_check_box,
                Err(e) => {
                    error!("can't find first checkbox. {}", e);
                    let _  = cb.send(Err((email, first_name, last_name)));
                    continue;
                }
            };
    
            let _  = match first_check_box.click().await {
                Err(e) => {
                    error!("click first checkbox error. {}", e);
                    let _  = cb.send(Err((email, first_name, last_name)));
                    continue;
                },
                _ => {}
            };
    
    
            let subscribe_option = match driver.query(By::Css("label.hs-form-booleancheckbox-display[for=LEGAL_CONSENT\\.subscription_type_304964235-b8ef3646-4f6a-4389-a9b1-3958123c4778_67]")).and_enabled().first().await {
                Ok(subscribe_option) => subscribe_option,
                Err(e) => {
                    error!("can't find subscribe option. {}", e);
                    let _  = cb.send(Err((email, first_name, last_name)));
                    continue;
                }
            };
            let _ = match subscribe_option.click().await {
                Err(e) => {
                    error!("click subscribe option error. {}", e);
                    let _  = cb.send(Err((email, first_name, last_name)));
                    continue;
                },
                _ => {}
            };
    
    
            let submit_button = match driver.query(By::Css("input.hs-button.primary.large")).and_displayed().first().await {
                Ok(submit_button) => submit_button,
                Err(e) => {
                    error!("can't find submitbutton. {}", e);
                    let _  = cb.send(Err((email, first_name, last_name)));
                    continue;
                }
            };
    
            let _ = match submit_button.click().await {
                Err(e) => {
                    error!("click submit button error. {}", e);
                    let _  = cb.send(Err((email, first_name, last_name)));
                    continue;
                },
                _ => {}
            };



            tokio::time::sleep(Duration::from_secs(2)).await;
            let _  = driver.query(By::Css("div#hs_cos_wrapper_widget_1723464463495")).and_enabled().first().await;

            let _ = cb.send(Ok((email, first_name, last_name)));
        }

    });

    tx

}




fn random_generate_email_account(prefix: &str, suffix: &str) -> String {

    let uuid  = Uuid::new_v4();
    let uuid = uuid.as_simple().to_string();
    let uuid  = &uuid[0..6];
    format!("{}{}{}", prefix, uuid, suffix)
}

fn random_generate_name() -> (String, String){
    let first_name: String = FirstName().fake();
    let last_name: String = LastName().fake();
    (first_name, last_name)
}



async fn create_sub_email() -> mpsc::Sender<(String, String, String)>{
    
    let (tx, mut rx) = mpsc::channel::<(String, String, String)>(1024);

    let retry_rx = tx.clone();
    tokio::spawn(async move {
        let retry_rx = retry_rx;
        let email_sender = Arc::new(EmailSender::new());
        let mut map = HashMap::new();

        let sub_tx = submit_white_list().await;


        while let Some((email_account, first_name, last_name)) = rx.recv().await {
            if !map.contains_key(&email_account) {
                let ret = email_sender.send_email(email_account.clone(), format!("open sub eamil account: {}", email_account), format!("{} {}", first_name, last_name)).await;
                tokio::time::sleep(Duration::from_secs(1)).await;
                if ret.is_some() {
                    map.insert(email_account.clone(), (first_name.clone(), last_name.clone()));
                    
                    let (cb, ecb) = oneshot::channel::<Result<(String, String, String), (String, String, String)>>();
                    let email_sender = email_sender.clone();
                    let retry_rx = retry_rx.clone();
                    let join_handle = tokio::spawn(async move {
                        match ecb.await {
                            Ok(Ok((email, first_name, last_name))) => {
                                let _  = email_sender.send_email(email.clone(), format!("{} submit white list successfully", email), format!("{} {} {} successfully", email, first_name, last_name)).await;
                                info!("{} {} {} successfully", email, first_name, last_name);
                            },
                            Ok(Err((email, first_name, last_name))) => {
                                let _  = email_sender.send_email(email.clone(), format!("{} submit white list failed", email), format!("{} {} {} failed", email, first_name, last_name)).await;
                                let _  = retry_rx.send((email, first_name, last_name)).await;
                            }
                            Err(e) => {
                                error!("callback error. {}", e);
                            }
                        }
                    });
                    let _ = sub_tx.send(((email_account, first_name, last_name), cb)).await;
                    let _  = join_handle.await;
                }
                
            }
        }
    });
    tx
}



pub struct EmailSender {
    transport: AsyncSmtpTransport<Tokio1Executor>
}


impl EmailSender {

    pub fn new() -> Self {

        let creds = Credentials::new("15922896272@163.com".to_owned(), "MGhnttiy2FSFmXg7".to_owned());
        let transport = AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.163.com")
                                                    .unwrap()
                                                    .credentials(creds)
                                                    .build();
        Self {
            transport
        }                        
    }

    pub async fn send_email(&self, to: String, subject: String, body: String) -> Option<()>{

        info!("sending email, from: {}, to: {}, subject: {}, body: {}", "15922896272@163.com", to, subject, body);

       let email =  Message::builder()
                                    .from("15922896272@163.com".parse().unwrap())
                                    .reply_to("15922896272@163.com".parse().unwrap())
                                    .to(to.parse().unwrap())
                                    .header(ContentType::TEXT_PLAIN)
                                    .subject(subject.clone())
                                    .body(body.clone())
                                    .unwrap();

       match self.transport.send(email).await {
        Ok(_) => {
            info!("sent email to {} with subject {} and body {} successfully!", to, subject, body);
            Some(())
        },
        Err(e) => {
            error!("send email failed, from: {}, to: {}, subject: {}, body: {}, cause: {}", "15922896272@163.com", to, subject, body, e);
            None
        }
       }
    }
}