use crate::domain;

use chrono::{DateTime, Utc};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use uuid::Uuid;

#[derive(Serialize)]
pub struct AuthenticatedUser {
    user: domain::User,
    authenticated: RwLock<DateTime<Utc>>,
}

#[derive(Serialize, Clone)]
pub struct AuthenticationResponse {
    token: Uuid,
    auth_info: Arc<AuthenticatedUser>,
}

use base64::{decode, encode};
use ring::{digest, pbkdf2};
use std::num::NonZeroU32;

#[derive(Clone)]
pub struct Identity {
    iterations: NonZeroU32,
    users_by_uuid: Arc<RwLock<HashMap<Uuid, Arc<AuthenticatedUser>>>>,
    users_by_personnel_nr: Arc<Mutex<HashMap<i16, AuthenticationResponse>>>,
}

static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA256;

const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
type Credential = [u8; CREDENTIAL_LEN];

impl Identity {
    pub fn new() -> Identity {
        Identity {
            iterations: NonZeroU32::new(1000).unwrap(),
            users_by_uuid: Arc::new(RwLock::new(HashMap::new())),
            users_by_personnel_nr: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn authenticate(
        &self,
        user: User,
        attempted_password: &str,
    ) -> Result<AuthenticationResponse, actix_web::Error> {
        self.verify_password(&user.salt, &user.password, attempted_password)?;

        // guard hashmap for write
        let mut guard = self.users_by_personnel_nr.lock().unwrap();
        if let Some(auth_response) = guard.get_mut(&user.personnel_nr) {
            // if auth record exists, renew auth timestamp
            {
                let mut guard = auth_response.auth_info.authenticated.write().unwrap();
                *guard = Utc::now();
            }
            return Ok(auth_response.clone());
        }

        // user not exists in hashmaps
        // generate new auth info

        let key = user.personnel_nr;
        let auth_info = Arc::new(AuthenticatedUser {
            user,
            authenticated: RwLock::new(Utc::now()),
        });

        let token = Uuid::new_v4();

        let auth_response = AuthenticationResponse {
            auth_info: auth_info.clone(),
            token,
        };

        // insert into hashmap by personnel nr
        guard.insert(key, auth_response.clone());

        // insert into hashmap by uuid token
        let mut guard = self.users_by_uuid.write().unwrap();
        guard.insert(token, auth_info);

        Ok(auth_response)
    }

    pub fn authorization_info(
        &self,
        token: &str,
    ) -> Result<Arc<AuthenticatedUser>, actix_web::Error> {
        let key = Uuid::parse_str(token)
            .map_err(|_| actix_web::error::ErrorBadRequest("invalid auth token"))?;

        let guard = self.users_by_uuid.read().unwrap();

        let info = guard.get(&key).map(|it| it.clone());

        match info {
            Some(info) => {
                // TODO: check duration; maximal session time must be 12Hours
                {
                    let hours = -{
                        let now = Utc::now();
                        let authenticated = info.authenticated.read().unwrap();
                        let duration = authenticated.signed_duration_since(now);
                        duration.num_hours()
                    };
                    if hours > 12 {
                        // session is outdated
                        Err(actix_web::error::ErrorUnauthorized("Session expired"))
                    } else {
                        Ok(info)
                    }
                }
            }
            None => Err(actix_web::error::ErrorUnauthorized(
                "You are not authenticated; invalid token",
            )),
        }
    }

    pub fn logout(&self, token: &str) -> Result<(), actix_web::Error> {
        let key = Uuid::parse_str(token)
            .map_err(|_| actix_web::error::ErrorBadRequest("invalid auth token"))?;

        let mut guard = self.users_by_uuid.write().unwrap();

        let auth_user = guard.remove(&key);
        if let Some(auth_user) = auth_user {
            let personnel_nr = auth_user.user.personnel_nr;
            let mut guard = self.users_by_personnel_nr.lock().unwrap();
            guard.remove(&personnel_nr).unwrap();
        }

        Ok(())
    }

    fn verify_password(
        &self,
        salt: &str,
        actual_password: &str,
        attempted_password: &str,
    ) -> Result<(), actix_web::Error> {
        let decoded_salt = decode(salt).unwrap();
        let decoded_actual_password = decode(actual_password).unwrap();

        pbkdf2::verify(
            PBKDF2_ALG,
            self.iterations,
            decoded_salt.as_slice(),
            attempted_password.as_bytes(),
            decoded_actual_password.as_slice(),
        )
        .map_err(|_| actix_web::error::ErrorUnauthorized("Parola este incorecta"))
    }

    pub fn generate_password_hash(&self, password: &str, salt: &str) -> String {
        let iterations = NonZeroU32::new(1000).unwrap();

        log::info!("Credentials len: {:}", CREDENTIAL_LEN);

        let decoded_salt = decode(salt).unwrap();

        let mut to_store: Credential = [0u8; CREDENTIAL_LEN];
        pbkdf2::derive(
            PBKDF2_ALG,
            iterations,
            decoded_salt.as_slice(),
            password.as_bytes(),
            &mut to_store,
        );
        encode(to_store)
    }
}
