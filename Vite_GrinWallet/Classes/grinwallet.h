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

#include <stdint.h>

void cstr_free (const char* s);

// Basic wallet contents summary
const char* grin_balance(const char* path, const char* chain_type, const char* account, const char* password, const char* check_node_api_http_addr, const bool refresh_from_node, const uint8_t* error);

//Display transaction information
const char* grin_txs_get(const char* path, const char* chain_type, const char* account,const char* password, const char* check_node_api_http_addr, const bool refresh_from_node, const uint8_t* error);

//Display transaction information
const char* grin_tx_get(const char* path, const char* chain_type, const char* account,const char* password, const char* check_node_api_http_addr, const bool refresh_from_node, const uint32_t tx_id, const uint8_t* error);

//Builds a transaction to send coins and creat transaction file
const char* grin_tx_create(const char* path, const char* chain_type, const char* account,const char* password, const char* check_node_api_http_addr, const uint64_t amount, const bool selection_strategy_is_use_all, const char* message, const uint8_t* error);

const char* grin_tx_strategies(const char* path, const char* chain_type, const char* account,const char* password, const char* check_node_api_http_addr, const uint64_t amount, const uint8_t* error);

//Cancels an previously created transaction, freeing previously locked outputs for use again
const char* grin_tx_cancel(const char* path, const char* chain_type, const char* account,const char* password, const char* check_node_api_http_addr, const uint32_t id, const uint8_t* error);

//Processes a transaction file to accept a transfer from a sender
const char* grin_tx_receive(const char* path, const char* chain_type, const char* account,const char* password, const char* check_node_api_http_addr, const char* slate_path, const char* message, const uint8_t* error);

// Processes a receiver's transaction file to finalize a transfer.
const char* grin_tx_finalize(const char* path,const char* chain_type,  const char* account,const char* password, const char* check_node_api_http_addr, const char* slate_path, const uint8_t* error);

//Builds a transaction to send coins and sends to the specified listener directly
const char* grin_tx_send(const char* path, const char* chain_type, const char* account,const char* password, const char* check_node_api_http_addr, const uint64_t amount, const bool selection_strategy_is_use_all, const char* message,  const char* dest, const uint8_t* error);

//Reposts a stored, completed but unconfirmed transaction to the chain,
const char* grin_tx_repost(const char* path, const char* chain_type, const char* account,const char* password, const char* check_node_api_http_addr, const uint32_t tx_id, const uint8_t* error);

//Initialize a new wallet seed file and database
const char* grin_wallet_init(const char* path, const char* chain_type, const char* password, const char* check_node_api_http_addr, const uint8_t* error);

//Wallet phrase
const char* grin_wallet_phrase(const char* path, const char* chain_type, const char* password, const char* check_node_api_http_addr, const uint8_t* error);

//Recovery the wallet from phrase
const char* grin_wallet_recovery(const char* path, const char* chain_type, const char* phrase,const char* password, const char* check_node_api_http_addr, const uint8_t* error);

//Checks a wallet's outputs against a live node, repairing and restoring missing outputs if required
const char* grin_wallet_check(const char* path, const char* chain_type, const char* account,const char* password, const char* check_node_api_http_addr, const uint8_t* error);

// Restores a wallet contents from a seed file
const char* grin_wallet_restore(const char* path, const char* chain_type, const char* account,const char* password, const char* check_node_api_http_addr, const uint8_t* error);



