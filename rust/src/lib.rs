// Copyright 2019 Ivan Sorokin.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.


// This code is mostly based on Ivan Sorokin's work in IronBelly(https://github.com/cyclefortytwo/ironbelly/blob/master/rust/src/lib.rs). Original copyright has been retained.

use grin_wallet_libwallet::{
    slate_versions, InitTxArgs, NodeClient, WalletInst, WalletLCProvider,
};
use grin_wallet_util::grin_core::global::ChainTypes;
use grin_wallet_util::grin_keychain::{ExtKeychain, Keychain};
use grin_wallet_util::grin_util::file::get_first_line;
use grin_wallet_util::grin_util::Mutex;
use grin_wallet_util::grin_util::ZeroingString;

use grin_wallet_config::{WalletConfig};
use grin_wallet_impls::{
    DefaultLCProvider, DefaultWalletImpl, Error, ErrorKind, HTTPNodeClient, HttpSlateSender,
    PathToSlate, SlateGetter, SlateSender,
};

use grin_wallet_api::{Foreign, Owner};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::Arc;
use uuid::Uuid;
use grin_wallet_util::grin_util::logger::LoggingConfig;
use grin_wallet_util::grin_util::init_logger;
use log::{Level};
use std::path::PathBuf;

fn c_str_to_rust(s: *const c_char) -> String {
    unsafe { CStr::from_ptr(s).to_string_lossy().into_owned() }
}

#[no_mangle]
pub unsafe extern "C" fn cstr_free(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    CString::from_raw(s);
}

#[derive(Serialize, Deserialize, Clone)]
struct State {
    wallet_dir: String,
    check_node_api_http_addr: String,
    chain: String,
    minimum_confirmations: u64,
    account: Option<String>,
    password: String,
}

impl State {
    fn from_str(json: &str) -> Result<Self, Error> {
        serde_json::from_str::<State>(json)
            .map_err(|e| Error::from(ErrorKind::GenericError(e.to_string())))
    }
}


pub fn get_wallet_config(wallet_dir: &str, chain_type: &str, check_node_api_http_addr: &str) -> WalletConfig {
    let chain_type_config = match chain_type {
        "floonet" => ChainTypes::Floonet,
        "usernet" => ChainTypes::UserTesting,
        "mainnet" => ChainTypes::Mainnet,
        _ => ChainTypes::Mainnet,
    };
    WalletConfig {
        chain_type: Some(chain_type_config),
        api_listen_interface: "127.0.0.1".to_string(),
        api_listen_port: 13415,
        api_secret_path: Some(".api_secret".to_string()),
        node_api_secret_path: Some(wallet_dir.to_owned() + "/.api_secret"),
        check_node_api_http_addr: check_node_api_http_addr.to_string(),
        data_file_dir: wallet_dir.to_owned(),
        tls_certificate_file: None,
        tls_certificate_key: None,
        dark_background_color_scheme: Some(true),
        keybase_notify_ttl: Some(1),
        no_commit_cache: None,
        owner_api_include_foreign: None,
        owner_api_listen_port: Some(WalletConfig::default_owner_api_listen_port()),
    }
}

pub fn get_wallet_log_config(wallet_dir: &str) -> LoggingConfig{
    let mut path = PathBuf::from(wallet_dir);
    path.push("grin-wallet.log");

    LoggingConfig {
        log_to_stdout: false,
        stdout_log_level: Level::Warn,
        log_to_file: true,
        file_log_level: Level::Info,
        log_file_path: path.to_str().unwrap().to_owned(),
        log_file_append: true,
        log_max_size: Some(1024 * 1024 * 16), // 16 megabytes default
        log_max_files: Some(32),
        tui_running: None,
    }
}


fn get_wallet(
    path: &str,
    chain_type: &str,
    account: &str,
    password: &str,
    check_node_api_http_addr: &str,
) -> Result<
        Arc<
            Mutex<
                Box<
                    dyn WalletInst<
                        'static,
                        DefaultLCProvider<'static, HTTPNodeClient, ExtKeychain>,
                        HTTPNodeClient,
                        ExtKeychain,
                    >,
                >,
            >,
        >,
        Error,
> {
    let wallet_log_config = get_wallet_log_config(path);

    // Load logging config
    init_logger(Some(wallet_log_config), None);

    let wallet_config = get_wallet_config(path, chain_type, check_node_api_http_addr);
    let node_api_secret = get_first_line(wallet_config.node_api_secret_path.clone());
    let node_client = HTTPNodeClient::new(&wallet_config.check_node_api_http_addr, node_api_secret);

    let wallet = inst_wallet::<
        DefaultLCProvider<HTTPNodeClient, ExtKeychain>,
        HTTPNodeClient,
        ExtKeychain,
    >(wallet_config.clone(), node_client)
    .unwrap_or_else(|e| {
        println!("{}", e);
        std::process::exit(1);
    });

    {
        let mut wallet_lock = wallet.lock();
        let lc = wallet_lock.lc_provider()?;
        if let Ok(open_wallet) = lc.wallet_exists(None) {
            if open_wallet {
                lc.open_wallet(None, ZeroingString::from(password), false, false)?;
                let wallet_inst = lc.wallet_inst()?;
                wallet_inst.set_parent_key_id_by_name(&account)?;
            }
        }
    }
    return Ok(wallet);
}

fn inst_wallet<L, C, K>(
    config: WalletConfig,
    node_client: C,
) -> Result<Arc<Mutex<Box<dyn WalletInst<'static, L, C, K>>>>, Error>
where
    DefaultWalletImpl<'static, C>: WalletInst<'static, L, C, K>,
    L: WalletLCProvider<'static, C, K>,
    C: NodeClient + 'static,
    K: Keychain + 'static,
{
    let mut wallet = Box::new(DefaultWalletImpl::<'static, C>::new(node_client.clone()).unwrap())
        as Box<dyn WalletInst<'static, L, C, K>>;
    let lc = wallet.lc_provider().unwrap();
    let _ = lc.set_top_level_directory(&config.data_file_dir);
    Ok(Arc::new(Mutex::new(wallet)))
}



// fn wallet_init(
//     path: &str,
//     chain_type: &str,
//     password: &str,
//     check_node_api_http_addr: &str,
// ) -> Result<String, Error> {
//     let wallet_config = get_wallet_config(path, chain_type, check_node_api_http_addr);
//     let node_api_secret = get_first_line(wallet_config.node_api_secret_path.clone());
//     let seed = WalletSeed::init_file(&wallet_config, 24, None, &password)?;
//     let client_n = HTTPNodeClient::new(
//         &wallet_config.check_node_api_http_addr,
//         node_api_secret.clone(),
//     );
//     let _: LMDBBackend<HTTPNodeClient, ExtKeychain> =
//         LMDBBackend::new(wallet_config.clone(), &password, client_n)?;
//     seed.to_mnemonic()
// }

macro_rules! unwrap_to_c (
	($func:expr, $error:expr) => (
	match $func {
        Ok(res) => {
            *$error = 0;
            CString::new(res.to_owned()).unwrap().into_raw()
        }
        Err(e) => {
            *$error = 1;
            CString::new(
                serde_json::to_string(&format!("{}",e)).unwrap()).unwrap().into_raw()
        }
    }
));

// #[no_mangle]
// pub unsafe extern "C" fn grin_wallet_init(
//     path: *const c_char,
//     chain_type: *const c_char,
//     password: *const c_char,
//     check_node_api_http_addr: *const c_char,
//     error: *mut u8,
// ) -> *const c_char {
//     unwrap_to_c!(
//         wallet_init(
//             &c_str_to_rust(path),
//             &c_str_to_rust(chain_type),
//             &c_str_to_rust(password),
//             &c_str_to_rust(check_node_api_http_addr),
//         ),
//         error
//     )
// }

fn wallet_recovery(
    path: &str,
    chain_type: &str,
    account: &str,
    phrase: &str,
    password: &str,
    check_node_api_http_addr: &str,
) -> Result<String, Error> {
    let wallet = get_wallet(path, chain_type, account, password, check_node_api_http_addr)?;
    let mut wallet_lock = wallet.lock();
    let lc = wallet_lock.lc_provider()?;
    lc.create_wallet(
        None,
        Some(ZeroingString::from(phrase)),
        32,
        ZeroingString::from(password),
        false,
    )?;
    let mut api = Owner::new(wallet.clone());
     api.scan(None,None,true)?;
     Ok("".to_owned())
}

#[no_mangle]
pub unsafe extern "C" fn grin_wallet_recovery(
    path: *const c_char,
    chain_type: *const c_char,
    account: *const c_char,
    phrase: *const c_char,
    password: *const c_char,
    check_node_api_http_addr: *const c_char,
    error: *mut u8,
) -> *const c_char {
    unwrap_to_c!(
        wallet_recovery(
            &c_str_to_rust(path),
            &c_str_to_rust(chain_type),
            &c_str_to_rust(account),
            &c_str_to_rust(phrase),
            &c_str_to_rust(password),
            &c_str_to_rust(check_node_api_http_addr),
        ),
        error
    )
}

// fn wallet_phrase(
//     path: &str,
//     chain_type: &str,
//     password: &str,
//     check_node_api_http_addr: &str,
// ) -> Result<String, Error> {
//     let seed = WalletSeed::from_file(
//         &format!("{}/{}", &path, GRIN_WALLET_DIR),
//         ZeroingString::from(password),
//     )?;
//     seed.to_mnemonic()
// }

// #[no_mangle]
// pub unsafe extern "C" fn grin_wallet_phrase(
//     path: *const c_char,
//     chain_type: *const c_char,
//     password: *const c_char,
//     check_node_api_http_addr: *const c_char,
//     error: *mut u8,
// ) -> *const c_char {
//     unwrap_to_c!(
//         wallet_phrase(
//             &c_str_to_rust(path),
//             &c_str_to_rust(chain_type),
//             &c_str_to_rust(password),
//             &c_str_to_rust(check_node_api_http_addr),
//         ),
//         error
//     )
// }

fn tx_get(
    path: &str,
    chain_type: &str,
    account: &str,
    password: &str,
    check_node_api_http_addr: &str,
    refresh_from_node: bool,
    tx_id: u32,
) -> Result<String, Error> {
    let wallet = get_wallet(path, chain_type, account, password, check_node_api_http_addr)?;
    let api = Owner::new(wallet.clone());
    let txs = api.retrieve_txs(None,refresh_from_node, Some(tx_id), None)?;
    Ok(serde_json::to_string(&txs).unwrap())
}

#[no_mangle]
pub unsafe extern "C" fn grin_tx_get(
    path: *const c_char,
    chain_type: *const c_char,
    account: *const c_char,
    password: *const c_char,
    check_node_api_http_addr: *const c_char,
    refresh_from_node: bool,
    tx_id: u32,
    error: *mut u8,
) -> *const c_char {
    unwrap_to_c!(
        tx_get(
            &c_str_to_rust(path),
            &c_str_to_rust(chain_type),
            &c_str_to_rust(account),
            &c_str_to_rust(password),
            &c_str_to_rust(check_node_api_http_addr),
            refresh_from_node,
            tx_id,
        ),
        error
    )
}

fn txs_get(
    path: &str,
    chain_type: &str,
    account: &str,
    password: &str,
    check_node_api_http_addr: &str,
    refresh_from_node: bool,
) -> Result<String, Error> {
    let wallet = get_wallet(path, chain_type, account, password, check_node_api_http_addr)?;
    let api = Owner::new(wallet.clone());

    match api.retrieve_txs(None,refresh_from_node, None, None) {
        Ok(txs) => Ok(serde_json::to_string(&txs).unwrap()),
        Err(e) => Err(Error::from(e)),
    }
}

#[no_mangle]
pub unsafe extern "C" fn grin_txs_get(
    path: *const c_char,
    chain_type: *const c_char,
    account: *const c_char,
    password: *const c_char,
    check_node_api_http_addr: *const c_char,
    refresh_from_node: bool,
    error: *mut u8,
) -> *const c_char {
    unwrap_to_c!(
        txs_get(
            &c_str_to_rust(path),
            &c_str_to_rust(chain_type),
            &c_str_to_rust(account),
            &c_str_to_rust(password),
            &c_str_to_rust(check_node_api_http_addr),
            refresh_from_node,
        ),
        error
    )
}

fn outputs_get(
    path: &str,
    chain_type: &str,
    account: &str,
    password: &str,
    check_node_api_http_addr: &str,
    refresh_from_node: bool,
) -> Result<String, Error> {
    let wallet = get_wallet(path, chain_type, account, password, check_node_api_http_addr)?;
    let api = Owner::new(wallet.clone());
    let outputs = api.retrieve_outputs(None,true,refresh_from_node, None)?;
    Ok(serde_json::to_string(&outputs).unwrap())
}

#[no_mangle]
pub unsafe extern "C" fn grin_outputs_get(
    path: *const c_char,
    chain_type: *const c_char,
    account: *const c_char,
    password: *const c_char,
    check_node_api_http_addr: *const c_char,
    refresh_from_node: bool,
    error: *mut u8,
) -> *const c_char {
    unwrap_to_c!(
        outputs_get(
            &c_str_to_rust(path),
            &c_str_to_rust(chain_type),
            &c_str_to_rust(account),
            &c_str_to_rust(password),
            &c_str_to_rust(check_node_api_http_addr),
            refresh_from_node,
        ),
        error
    )
}

fn output_get(
    path: &str,
    chain_type: &str,
    account: &str,
    password: &str,
    check_node_api_http_addr: &str,
    refresh_from_node: bool,
    tx_id: u32,
) -> Result<String, Error> {
    let wallet = get_wallet(path, chain_type, account, password, check_node_api_http_addr)?;
    let api = Owner::new(wallet.clone());
    let outputs = api.retrieve_outputs(None,true,refresh_from_node, Some(tx_id))?;
    Ok(serde_json::to_string(&outputs).unwrap())
}

#[no_mangle]
pub unsafe extern "C" fn grin_output_get(
    path: *const c_char,
    chain_type: *const c_char,
    account: *const c_char,
    password: *const c_char,
    check_node_api_http_addr: *const c_char,
    refresh_from_node: bool,
    tx_id: u32,
    error: *mut u8,
) -> *const c_char {
    unwrap_to_c!(
        output_get(
            &c_str_to_rust(path),
            &c_str_to_rust(chain_type),
            &c_str_to_rust(account),
            &c_str_to_rust(password),
            &c_str_to_rust(check_node_api_http_addr),
            refresh_from_node,
            tx_id,
        ),
        error
    )
}


fn balance(
    path: &str,
    chain_type: &str,
    account: &str,
    password: &str,
    check_node_api_http_addr: &str,
    refresh_from_node: bool,
) -> Result<String, Error> {
    let wallet = get_wallet(path, chain_type, account, password, check_node_api_http_addr)?;
    let mut api = Owner::new(wallet.clone());
    let (_validated, wallet_info) = api.retrieve_summary_info(None,refresh_from_node, 10)?;
    Ok(serde_json::to_string(&wallet_info).unwrap())
}

#[no_mangle]
pub unsafe extern "C" fn grin_balance(
    path: *const c_char,
    chain_type: *const c_char,
    account: *const c_char,
    password: *const c_char,
    check_node_api_http_addr: *const c_char,
    refresh_from_node: bool,
    error: *mut u8,
) -> *const c_char {
    unwrap_to_c!(
        balance(
            &c_str_to_rust(path),
            &c_str_to_rust(chain_type),
            &c_str_to_rust(account),
            &c_str_to_rust(password),
            &c_str_to_rust(check_node_api_http_addr),
            refresh_from_node,
        ),
        error
    )
}

fn height(
    path: &str,
    chain_type: &str,
    account: &str,
    password: &str,
    check_node_api_http_addr: &str,
) -> Result<String, Error> {
    let wallet = get_wallet(path, chain_type, account, password, check_node_api_http_addr)?;
    let mut api = Owner::new(wallet.clone());
    let height = api.node_height(None)?;
    Ok(serde_json::to_string(&height).unwrap())
}

#[no_mangle]
pub unsafe extern "C" fn grin_height(
    path: *const c_char,
    chain_type: *const c_char,
    account: *const c_char,
    password: *const c_char,
    check_node_api_http_addr: *const c_char,
    error: *mut u8,
) -> *const c_char {
    unwrap_to_c!(
        height(
            &c_str_to_rust(path),
            &c_str_to_rust(chain_type),
            &c_str_to_rust(account),
            &c_str_to_rust(password),
            &c_str_to_rust(check_node_api_http_addr),
        ),
        error
    )
}


#[derive(Serialize, Deserialize)]
struct Strategy {
    selection_strategy_is_use_all: bool,
    total: u64,
    fee: u64,
}

fn tx_strategies(
    path: &str,
    chain_type: &str,
    account: &str,
    password: &str,
    check_node_api_http_addr: &str,
    amount: u64,
) -> Result<String, Error> {
    let wallet = get_wallet(path, chain_type, account, password, check_node_api_http_addr)?;
    let api = Owner::new(wallet.clone());
    let mut result = vec![];
    let mut args = InitTxArgs {
        src_acct_name: None,
        amount,
        minimum_confirmations: 10,
        max_outputs: 500,
        num_change_outputs: 1,
        selection_strategy_is_use_all: false,
        message: None,
        target_slate_version: Some(2),
        estimate_only: Some(true),
        send_args: None,
        payment_proof_recipient_address: None,
        ttl_blocks: None,
    };
    if let Ok(smallest) = api.init_send_tx(None,args.clone()) {
        result.push(Strategy {
            selection_strategy_is_use_all: false,
            total: smallest.amount,
            fee: smallest.fee,
        })
    }
    args.selection_strategy_is_use_all = true;
    let all = api.init_send_tx(None, args).map_err(|e| Error::from(e))?;
    result.push(Strategy {
        selection_strategy_is_use_all: true,
        total: all.amount,
        fee: all.fee,
    });
    Ok(serde_json::to_string(&result).unwrap())
}

#[no_mangle]
pub unsafe extern "C" fn grin_tx_strategies(
    path: *const c_char,
    chain_type: *const c_char,
    account: *const c_char,
    password: *const c_char,
    check_node_api_http_addr: *const c_char,
    amount: u64,
    error: *mut u8,
) -> *const c_char {
    unwrap_to_c!(
        tx_strategies(
            &c_str_to_rust(path),
            &c_str_to_rust(chain_type),
            &c_str_to_rust(account),
            &c_str_to_rust(password),
            &c_str_to_rust(check_node_api_http_addr),
            amount,
        ),
        error
    )
}

fn tx_create(
    path: &str,
    chain_type: &str,
    account: &str,
    password: &str,
    check_node_api_http_addr: &str,
    message: &str,
    amount: u64,
    selection_strategy_is_use_all: bool,
) -> Result<String, Error> {
    let wallet = get_wallet(path, chain_type, account, password, check_node_api_http_addr)?;
    let mut api = Owner::new(wallet.clone());
     let args = InitTxArgs {
        src_acct_name: None,
        amount,
        minimum_confirmations: 10,
        max_outputs: 500,
        num_change_outputs: 1,
        selection_strategy_is_use_all,
        message: Some(message.to_owned()),
        target_slate_version: Some(2),
        estimate_only: Some(false),
        send_args: None,
        payment_proof_recipient_address: None,
        ttl_blocks: None,
    };
    let mut slate = api.init_send_tx(None, args).unwrap();
    slate.version_info.version = 2;
    slate.version_info.orig_version = 2;
    api.tx_lock_outputs(None, &slate, 0)?;
    Ok(
        serde_json::to_string(&slate_versions::VersionedSlate::into_version(
            slate.clone(),
            slate_versions::SlateVersion::V2,
        ))
        .map_err(|e| ErrorKind::GenericError(e.to_string()))?,
    )
}

#[no_mangle]
pub unsafe extern "C" fn grin_tx_create(
    path: *const c_char,
    chain_type: *const c_char,
    account: *const c_char,
    password: *const c_char,
    check_node_api_http_addr: *const c_char,
    amount: u64,
    selection_strategy_is_use_all: bool,
    message: *const c_char,
    error: *mut u8,
) -> *const c_char {
    unwrap_to_c!(
        tx_create(
            &c_str_to_rust(path),
            &c_str_to_rust(chain_type),
            &c_str_to_rust(account),
            &c_str_to_rust(password),
            &c_str_to_rust(check_node_api_http_addr),
            &c_str_to_rust(message),
            amount,
            selection_strategy_is_use_all,
        ),
        error
    )
}

fn tx_cancel(
    path: &str,
    chain_type: &str,
    account: &str,
    password: &str,
    check_node_api_http_addr: &str,
    id: u32,
) -> Result<String, Error> {
    let wallet = get_wallet(path, chain_type, account, password, check_node_api_http_addr)?;
    let mut api = Owner::new(wallet.clone());
    api.cancel_tx(None, Some(id), None)?;
    Ok("".to_owned())
}

#[no_mangle]
pub unsafe extern "C" fn grin_tx_cancel(
    path: *const c_char,
    chain_type: *const c_char,
    account: *const c_char,
    password: *const c_char,
    check_node_api_http_addr: *const c_char,
    id: u32,
    error: *mut u8,
) -> *const c_char {
    unwrap_to_c!(
        tx_cancel(
            &c_str_to_rust(path),
            &c_str_to_rust(chain_type),
            &c_str_to_rust(account),
            &c_str_to_rust(password),
            &c_str_to_rust(check_node_api_http_addr),
            id,
        ),
        error
    )
}

fn tx_receive(
    path: &str,
    chain_type: &str,
    account: &str,
    password: &str,
    check_node_api_http_addr: &str,
    slate_path: &str,
    message: &str,
) -> Result<String, Error> {
    let wallet = get_wallet(path, chain_type, account, password, check_node_api_http_addr)?;
    let api = Foreign::new(wallet.clone(), None, None);
    let mut slate = PathToSlate((&slate_path).into()).get_tx()?;
    api.verify_slate_messages(&slate)?;
    slate = api.receive_tx(&slate, Some(&account), Some(message.to_owned()))?;
    Ok(serde_json::to_string(&slate).map_err(|e| ErrorKind::GenericError(e.to_string()))?)
}

#[no_mangle]
pub unsafe extern "C" fn grin_tx_receive(
    path: *const c_char,
    chain_type: *const c_char,
    account: *const c_char,
    password: *const c_char,
    check_node_api_http_addr: *const c_char,
    slate_path: *const c_char,
    message: *const c_char,
    error: *mut u8,
) -> *const c_char {
    unwrap_to_c!(
        tx_receive(
            &c_str_to_rust(path),
            &c_str_to_rust(chain_type),
            &c_str_to_rust(account),
            &c_str_to_rust(password),
            &c_str_to_rust(check_node_api_http_addr),
            &c_str_to_rust(slate_path),
            &c_str_to_rust(message),
        ),
        error
    )
}

fn tx_finalize(
    path: &str,
    chain_type: &str,
    account: &str,
    password: &str,
    check_node_api_http_addr: &str,
    slate_path: &str,
) -> Result<String, Error> {
    let wallet = get_wallet(path, chain_type, account, password, check_node_api_http_addr)?;
    let api = Owner::new(wallet.clone());
    let mut slate = PathToSlate((&slate_path).into()).get_tx()?;
    api.verify_slate_messages(None, &slate)?;
    match api.finalize_tx(None, &slate){
        Ok(mut slate) => {
            Ok(
                serde_json::to_string(&slate_versions::VersionedSlate::into_version(
                    slate.clone(),
                    slate_versions::SlateVersion::V2,
                ))
                .map_err(|e| ErrorKind::GenericError(e.to_string()))?,
            )
        }
        Err(e) => {
            Err(Error::from(e))
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn grin_tx_finalize(
    path: *const c_char,
    chain_type: *const c_char,
    account: *const c_char,
    password: *const c_char,
    check_node_api_http_addr: *const c_char,
    slate_path: *const c_char,
    error: *mut u8,
) -> *const c_char {
    unwrap_to_c!(
        tx_finalize(
            &c_str_to_rust(path),
            &c_str_to_rust(chain_type),
            &c_str_to_rust(account),
            &c_str_to_rust(password),
            &c_str_to_rust(check_node_api_http_addr),
            &c_str_to_rust(slate_path),
        ),
        error
    )
}

fn tx_send_http(
    path: &str,
    chain_type: &str,
    account: &str,
    password: &str,
    check_node_api_http_addr: &str,
    amount: u64,
    selection_strategy_is_use_all: bool,
    message: &str,
    dest: &str,
) -> Result<String, Error> {
    let wallet = get_wallet(path, chain_type, account, password, check_node_api_http_addr)?;
    let api = Owner::new(wallet.clone());
    let args = InitTxArgs {
        src_acct_name: None,
        amount,
        minimum_confirmations: 10,
        max_outputs: 500,
        num_change_outputs: 1,
        selection_strategy_is_use_all,
        message: Some(message.to_owned()),
        target_slate_version: Some(2),
        estimate_only: Some(false),
        send_args: None,
        payment_proof_recipient_address: None,
        ttl_blocks: None,
    };
    let mut slate = api.init_send_tx(None, args)?;
    slate.version_info.version = 2;
    slate.version_info.orig_version = 2;
    let sender = Box::new(
        HttpSlateSender::new(dest)
            .map_err(|_| ErrorKind::GenericError(format!("Invalid destination URL: {}", dest)))?,
    );
    api.tx_lock_outputs(None, &slate, 0)?;
    match sender.send_tx(&slate) {
        Ok(mut slate) => {
            api.verify_slate_messages(None, &slate)?;
            api.finalize_tx(None, &mut slate)?;
            Ok(
                serde_json::to_string(&slate_versions::VersionedSlate::into_version(
                    slate.clone(),
                    slate_versions::SlateVersion::V2,
                ))
                .map_err(|e| ErrorKind::GenericError(e.to_string()))?,
            )
        }
        Err(e) => {
            api.cancel_tx(None, None, Some(slate.id))?;
            Err(Error::from(e))
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn grin_tx_send_http(
    path: *const c_char,
    chain_type: *const c_char,
    account: *const c_char,
    password: *const c_char,
    check_node_api_http_addr: *const c_char,
    amount: u64,
    selection_strategy_is_use_all: bool,
    message: *const c_char,
    dest: *const c_char,
    error: *mut u8,
) -> *const c_char {
    unwrap_to_c!(
        tx_send_http(
            &c_str_to_rust(path),
            &c_str_to_rust(chain_type),
            &c_str_to_rust(account),
            &c_str_to_rust(password),
            &c_str_to_rust(check_node_api_http_addr),
            amount,
            selection_strategy_is_use_all,
            &c_str_to_rust(message),
            &c_str_to_rust(dest),
        ),
        error
    )
}

fn tx_post(
    path: &str,
    chain_type: &str,
    account: &str,
    password: &str,
    check_node_api_http_addr: &str,
    tx_slate_id: &str,
) -> Result<String, Error> {
    let wallet = get_wallet(path, chain_type, account, password, check_node_api_http_addr)?;
    let api = Owner::new(wallet.clone());
    let uuid = Uuid::parse_str(tx_slate_id).map_err(|e| ErrorKind::GenericError(e.to_string()))?;
    let (_, txs) = api.retrieve_txs(None, true, None, Some(uuid))?;
    if txs[0].confirmed {
        return Err(Error::from(ErrorKind::GenericError(format!(
            "Transaction with id {} is already confirmed. Not posting.",
            tx_slate_id
        ))));
    }
    let stored_tx = api.get_stored_tx(None, &txs[0])?;
    match stored_tx {
        Some(stored_tx) => {
            api.post_tx(None, &stored_tx, true)?;
            Ok("".to_owned())
        }
        None => Err(Error::from(ErrorKind::GenericError(format!(
            "Transaction with id {} does not have transaction data. Not posting.",
            tx_slate_id
        )))),
    }
}

#[no_mangle]
pub unsafe extern "C" fn grin_tx_post(
    path: *const c_char,
    chain_type: *const c_char,
    account: *const c_char,
    password: *const c_char,
    check_node_api_http_addr: *const c_char,
    tx_slate_id: *const c_char,
    error: *mut u8,
) -> *const c_char {
    unwrap_to_c!(
        tx_post(
            &c_str_to_rust(path),
            &c_str_to_rust(chain_type),
            &c_str_to_rust(account),
            &c_str_to_rust(password),
            &c_str_to_rust(check_node_api_http_addr),
            &c_str_to_rust(tx_slate_id),
        ),
        error
    )
}

fn wallet_restore(
    path: &str,
    chain_type: &str,
    account: &str,
    password: &str,
    check_node_api_http_addr: &str,
) -> Result<String, Error> {
    let wallet = get_wallet(path, chain_type, account, password, check_node_api_http_addr)?;
    let mut api = Owner::new(wallet.clone());
    match api.scan( None, None, true) {
        Ok(_) => Ok("".to_owned()),
        Err(e) => Err(Error::from(e)),
    }
}

#[no_mangle]
pub unsafe extern "C" fn grin_wallet_restore(
    path: *const c_char,
    chain_type: *const c_char,
    account: *const c_char,
    password: *const c_char,
    check_node_api_http_addr: *const c_char,
    error: *mut u8,
) -> *const c_char {
    unwrap_to_c!(
        wallet_restore(
            &c_str_to_rust(path),
            &c_str_to_rust(chain_type),
            &c_str_to_rust(account),
            &c_str_to_rust(password),
            &c_str_to_rust(check_node_api_http_addr),
        ),
        error
    )
}

fn wallet_check(
    path: &str,
    chain_type: &str,
    account: &str,
    password: &str,
    check_node_api_http_addr: &str,
    delete_unconfirmed: bool,
) -> Result<String, Error> {
    let wallet = get_wallet(path, chain_type, account, password, check_node_api_http_addr)?;
    let mut api = Owner::new(wallet.clone());
    match api.scan(None,None,false)
     {
        Ok(_) => Ok("".to_owned()),
        Err(e) => Err(Error::from(e)),
    }
}

#[no_mangle]
pub unsafe extern "C" fn grin_wallet_check(
    path: *const c_char,
    chain_type: *const c_char,
    account: *const c_char,
    password: *const c_char,
    check_node_api_http_addr: *const c_char,
    delete_unconfirmed: bool,
    error: *mut u8,
) -> *const c_char {
    unwrap_to_c!(
        wallet_check(
            &c_str_to_rust(path),
            &c_str_to_rust(chain_type),
            &c_str_to_rust(account),
            &c_str_to_rust(password),
            &c_str_to_rust(check_node_api_http_addr),
            delete_unconfirmed,
        ),
        error
    )
}






