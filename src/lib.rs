use spin_sdk::http::{IntoResponse, Request, Router};
use spin_sdk::http_component;

/// A simple Spin HTTP component.
#[http_component]
fn handle_word_of_day(req: Request) -> anyhow::Result<impl IntoResponse> {
    // println!("Handling request to {:?}", req.header("spin-full-url"));
    // Ok(Response::builder()
    //     .status(200)
    //     .header("content-type", "text/plain")
    //     .body("Hello, Fermyon")
    //     .build())
    let mut router = Router::new();

    router.get("/set-password", api::set_password);
    router.get("/get-today", api::get_today);
    router.get("/set-today", api::set_today);
    Ok(router.handle(req))
}

mod api {
    use std::time::{Duration, SystemTime};

    use querystring::QueryParams;
    use serde::{Deserialize, Serialize};
    use sha3::{Digest, Sha3_256};
    use spin_sdk::{
        http::{IntoResponse, Params, Request, Response},
        key_value::Store,
    };

    const TIMEOUT: u128 = 30 * 1000;

    #[derive(Serialize, Deserialize)]
    enum ErrorResponse<'a> {
        MissingParameter { name: &'a str },
        Unauthenticated { cooldown: u128 },
        CooldownActive { cooldown: u128 },
        NothingToday,
    }
    #[derive(Serialize, Deserialize)]
    enum SuccessResponse<'a> {
        NewPassword { password: &'a str },
        NewWord { word: &'a str },
        Today { word: &'a str },
    }
    pub fn set_today(req: Request, params: Params) -> anyhow::Result<impl IntoResponse> {
        let query = querystring::querify(req.query());
        let Some(password) = get_from_query(&query, "password") else {
            return error_response(&ErrorResponse::MissingParameter { name: "password" });
        };
        let Some(new_word) = get_from_query(&query, "new_word") else {
            return error_response(&ErrorResponse::MissingParameter { name: "new_word" });
        };

        let store = Store::open_default()?;
        if !verify_password(&password)? {
            store.set(
                "timeout_start",
                &SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_millis()
                    .to_ne_bytes(),
            )?;
            return error_response(&ErrorResponse::Unauthenticated { cooldown: TIMEOUT });
        }

        let todays_word = get_todays_word(&store)?;
        if let Some(today) = todays_word {
            store.set(&format!("{:?}", SystemTime::now()), today.as_bytes())?;
        }

        store.set("today", new_word.as_bytes())?;

        Ok(Response::builder()
            .header("content-type", "application/json")
            .body(serde_json::to_string(&SuccessResponse::NewWord { word: &new_word }).unwrap())
            .build())
    }

    pub fn get_today(_req: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
        let store = Store::open_default()?;
        match get_todays_word(&store)? {
            Some(word) => Ok(Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&SuccessResponse::Today { word: &word }).unwrap())
                .build()),
            None => error_response(&ErrorResponse::NothingToday),
        }
    }

    fn get_todays_word(store: &Store) -> anyhow::Result<Option<String>> {
        let today = store.get("today")?;
        Ok(today.map(|word| String::from_utf8_lossy(&word).to_string()))
    }

    pub fn set_password(req: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
        let store = Store::open_default()?;
        let timeout_start = SystemTime::UNIX_EPOCH
            + Duration::from_millis(
                store
                    .get("timeout_start")?
                    .map(|timeout| u128::from_ne_bytes(timeout.try_into().unwrap()))
                    .unwrap_or_default()
                    .try_into()
                    .unwrap(),
            );
        if SystemTime::now()
            .duration_since(timeout_start)
            .map(|duration| duration.as_millis() < TIMEOUT)
            .unwrap_or_default()
        {
            return error_response(&ErrorResponse::CooldownActive { cooldown: TIMEOUT });
        }
        let query = querystring::querify(req.query());

        let Some(new_password) = get_from_query(&query, "new") else {
            return error_response(&ErrorResponse::MissingParameter { name: "new" });
        };
        let Some(old_password) = get_from_query(&query, "old") else {
            return error_response(&ErrorResponse::MissingParameter { name: "old" });
        };

        let mut response = Response::builder();

        let response = response.header("content-type", "application/json");
        Ok(if change_password(&old_password, &new_password)? {
            response
                .status(200)
                .body(
                    serde_json::to_string(&SuccessResponse::NewPassword {
                        password: &new_password,
                    })
                    .unwrap(),
                )
                .build()
        } else {
            store.set(
                "timeout_start",
                &SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_millis()
                    .to_ne_bytes(),
            )?;

            response
                .status(400)
                .body(
                    serde_json::to_string(&ErrorResponse::Unauthenticated { cooldown: TIMEOUT })
                        .unwrap(),
                )
                .build()
        })
    }
    fn get_from_query(query: &QueryParams, param: &str) -> Option<String> {
        query
            .iter()
            .filter(|(key, _)| *key == param)
            .last()
            .map(|(_, value)| urlencoding::decode(value).unwrap().to_string())
    }

    fn verify_password(password: &str) -> anyhow::Result<bool> {
        let store = Store::open_default()?;

        let Some(pass) = store.get("password")? else {
            return Ok(true);
        };

        let mut hasher = Sha3_256::new();
        hasher.update(password);
        let hashed_old = hasher.finalize().to_vec();
        Ok(hashed_old == pass)
    }

    fn change_password(old_password: &str, new_password: &str) -> anyhow::Result<bool> {
        Ok(if verify_password(old_password)? {
            let mut hasher = Sha3_256::new();
            hasher.update(new_password);
            let hashed_new = hasher.finalize().to_vec();

            update_password(hashed_new)?;
            true
        } else {
            false
        })
    }

    fn update_password(hashed_new: Vec<u8>) -> anyhow::Result<()> {
        let store = Store::open_default()?;
        store.set("password", &hashed_new)?;
        Ok(())
    }

    fn error_response(error: &ErrorResponse) -> anyhow::Result<Response> {
        return Ok(Response::builder()
            .status(400)
            .header("content-type", "application/json")
            .body(serde_json::to_string(error).unwrap())
            .build());
    }
}
