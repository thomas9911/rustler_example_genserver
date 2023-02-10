use rustler::error::Error;
use rustler::types::atom::{error, nil, ok};
use rustler::types::Encoder;
use rustler::NifResult;
use rustler::{Env, ResourceArc, Term};
use rustler::{NifMap, NifTaggedEnum};

use std::collections::HashMap;
use std::sync::RwLock;

rustler::atoms! {
    trainted_lock
}

#[derive(NifMap)]
pub struct InitResponse {
    r#ref: State,
}

pub struct GenServerResource {
    data: RwLock<HashMap<String, String>>,
}

pub type State = ResourceArc<GenServerResource>;

#[derive(NifTaggedEnum)]
pub enum HandleCallResponse<'a> {
    Noreply(InitResponse),
    Reply(Term<'a>, InitResponse),
}

#[derive(NifTaggedEnum)]
pub enum HandleCallRequest {
    Put(String, String),
    Get(String),
}

fn trainted_lock_error() -> Error {
    Error::Term(Box::new((error(), trainted_lock())))
}

#[rustler::nif]
pub fn init(_args: Vec<()>) -> Result<InitResponse, ()> {
    Ok(InitResponse {
        r#ref: ResourceArc::new(GenServerResource {
            data: RwLock::new(HashMap::new()),
        }),
    })
}

#[rustler::nif]
pub fn put(env: Env<'_>, state: State, key: String, value: String) -> NifResult<Term<'_>> {
    inner_put(env, &state, key, value)
}

pub fn inner_put<'a>(
    env: Env<'a>,
    state: &State,
    key: String,
    value: String,
) -> NifResult<Term<'a>> {
    state
        .data
        .write()
        .map_err(|_| trainted_lock_error())?
        .insert(key, value);
    Ok(ok().encode(env))
}

#[rustler::nif]
pub fn get<'a>(env: Env<'a>, state: State, key: &str) -> NifResult<Term<'a>> {
    inner_get(env, &state, key)
}

pub fn inner_get<'a>(env: Env<'a>, state: &State, key: &str) -> NifResult<Term<'a>> {
    if let Some(value) = state
        .data
        .read()
        .map_err(|_| trainted_lock_error())?
        .get(key)
        .cloned()
    {
        Ok(value.encode(env))
    } else {
        Ok(nil().encode(env))
    }
}

#[rustler::nif]
pub fn handle_call<'a>(
    env: Env<'a>,
    request: HandleCallRequest,
    _from: Term<'a>,
    state: InitResponse,
) -> NifResult<HandleCallResponse> {
    match request {
        HandleCallRequest::Put(key, value) => {
            let res = inner_put(env, &state.r#ref, key, value)?;
            Ok(HandleCallResponse::Reply(res, state))
        }
        HandleCallRequest::Get(key) => {
            let res = inner_get(env, &state.r#ref, &key)?;
            Ok(HandleCallResponse::Reply(res, state))
        }
    }
}

rustler::init!(
    "Elixir.RustlerExampleGenServer.Native",
    [init, put, get, handle_call],
    load = on_load
);

pub fn on_load(env: Env, _: rustler::Term) -> bool {
    rustler::resource!(GenServerResource, env);
    true
}
