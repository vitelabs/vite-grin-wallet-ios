# Vite_GrinWallet iOS

A CocoaPods wrapper for official [Grin](https://github.com/mimblewimble/grin/) wallet and integrated in [Vite iOS App](https://github.com/vitelabs/vite-app-ios)

Rust-C bridge library and API definition are based on the existing work of [IronBelly](https://github.com/cyclefortytwo/ironbelly). All original copyrights have been retained.

## Installation

Add the following line to your Podfile:
```
pod 'Vite_GrinWallet', :git => 'https://github.com/vitelabs/Vite_GrinWallet.git'
```

## Bridge API Specification

```c
/** 
 * @brief                           Basic wallet contents summary.
 *
 * @param path                      The directory in which wallet files are stored.
 * @param chain_type                Chain parameters.
 * @param account                   Account name.
 * @param password                  Wallet password.
 * @param check_node_api_http_addr  The api address of a running server node against which transaction inputs will be checked during send.
 * @param refresh_from_node         Whether refresh from node.
 * @param error                     Error code pointer.
 *
 * @return                          Wallet contents summary.
 *
 */
const char* grin_balance(const char* path, const char* chain_type, const char* account, const char* password, const char* check_node_api_http_addr, const bool refresh_from_node, const uint8_t* error);


/** 
 * @brief                           Display transaction informations.
 *
 * @param path                      The directory in which wallet files are stored.
 * @param chain_type                Chain parameters.
 * @param account                   Account name.
 * @param password                  Wallet password.
 * @param check_node_api_http_addr  The api address of a running server node against which transaction inputs will be checked during send.
 * @param refresh_from_node         Whether refresh from node.
 * @param error                     Error code pointer.
 *
 * @return                          Transaction information.
 *
 */
const char* grin_txs_get(const char* path, const char* chain_type, const char* account,const char* password, const char* check_node_api_http_addr, const bool refresh_from_node, const uint8_t* error);


/** 
 * @brief                           Display one transaction information.
 *
 * @param path                      The directory in which wallet files are stored.
 * @param chain_type                Chain parameters.
 * @param account                   Account name.
 * @param password                  Wallet password.
 * @param check_node_api_http_addr  The api address of a running server node against which transaction inputs will be checked during send.
 * @param refresh_from_node         Whether refresh from node.
 * @param tx_id                     Transaction id.
 * @param error                     Error code pointer.
 *
 * @return                          Transaction information.
 *
 */
const char* grin_tx_get(const char* path, const char* chain_type, const char* account, const char* password, const char* check_node_api_http_addr, const bool refresh_from_node, const uint32_t tx_id, const uint8_t* error);


/** 
 * @brief Builds a transaction to send coins and creat transaction file
 *
 * @param path                          The directory in which wallet files are stored.
 * @param chain_type                    Chain parameters.
 * @param account                       Account name.
 * @param password                      Wallet password.
 * @param check_node_api_http_addr      The api address of a running server node against which transaction inputs will be checked during send.
 * @param amount                        The amount to send, in nanogrins. (`1 G = 1_000_000_000nG`).
 * @param selection_strategy_is_use_all If `true`, attempt to use up as many outputs as possible to create the transaction. 
 * @param message                       An optional participant message to include alongside the sender's public ParticipantData within the slate.
 * @param error                         Error code pointer.
 *
 * @return                              Transaction slate.
 *
 */
const char* grin_tx_create(const char* path, const char* chain_type, const char* account, const char* password, const char* check_node_api_http_addr, const uint64_t amount, const bool selection_strategy_is_use_all, const char* message, const uint8_t* error);


/** 
 * @brief                               Estimates the amount to be locked and fee for the transaction with two strategies.
 * @param path                          The directory in which wallet files are stored.
 * @param chain_type                    Chain parameters.
 * @param account                       Account name.
 * @param password                      Wallet password.
 * @param check_node_api_http_addr      The api address of a running server node against which transaction inputs will be checked during send.
 * @param amount                        The amount to send, in nanogrins. (`1 G = 1_000_000_000nG`).
 * @param error                         Error code pointer.
 *
 * @return                              A result containing (total, fee) with two strategies
 *
 */
const char* grin_tx_strategies(const char* path, const char* chain_type, const char* account,const char* password, const char* check_node_api_http_addr, const uint64_t amount, const uint8_t* error);


/** 
 * @brief                               Cancels an previously created transaction, freeing previously locked outputs for use again
 *
 * @param path                          The directory in which wallet files are stored.
 * @param chain_type                    Chain parameters.
 * @param account                       Account name.
 * @param password                      Wallet password.
 * @param check_node_api_http_addr      The api address of a running server node against which transaction inputs will be checked during send.
 * @param id                            Transaction id.
 * @param error                         Error code pointer.
 *
 * @return                              A result containing (total, fee) with two strategies
 *
 */
const char* grin_tx_cancel(const char* path, const char* chain_type, const char* account,const char* password, const char* check_node_api_http_addr, const uint32_t id, const uint8_t* error);


/** 
 * @brief                               Processes a transaction file to accept a transfer from a sender
 * 
 * @param path                          The directory in which wallet files are stored.
 * @param chain_type                    Chain parameters.
 * @param account                       Account name.
 * @param password                      Wallet password.
 * @param check_node_api_http_addr      The api address of a running server node against which transaction inputs will be checked during send.
 * @param slate_path                    The directory in which slate files are stored.
 * @param message                       An optional participant message to include alongside the sender's public ParticipantData within the slate.
 * @param error                         Error code pointer.
 *
 * @return                              Void String.
 *
 */
const char* grin_tx_receive(const char* path, const char* chain_type, const char* account,const char* password, const char* check_node_api_http_addr, const char* slate_path, const char* message, const uint8_t* error);


/** 
 * @brief                               Processes a receiver's transaction file to finalize a transfer.
 * 
 * @param path                          The directory in which wallet files are stored.
 * @param chain_type                    Chain parameters.
 * @param account                       Account name.
 * @param password                      Wallet password.
 * @param check_node_api_http_addr      The api address of a running server node against which transaction inputs will be checked during send.
 * @param slate_path                    The directory in which slate files are stored.
 * @param error                         Error code pointer.
 *
 * @return                              Void String.
 *
 */
const char* grin_tx_finalize(const char* path,const char* chain_type,  const char* account,const char* password, const char* check_node_api_http_addr, const char* slate_path, const uint8_t* error);


/** 
 * @brief                               Builds a transaction to send coins and sends to the specified listener directly
 * 
 * @param path                          The directory in which wallet files are stored.
 * @param chain_type                    Chain parameters.
 * @param account                       Account name.
 * @param password                      Wallet password.
 * @param check_node_api_http_addr      The api address of a running server node against which transaction inputs will be checked during send.
 * @param amount                        The amount to send, in nanogrins. (`1 G = 1_000_000_000nG`).
 * @param selection_strategy_is_use_all If `true`, attempt to use up as many outputs as possible to create the transaction. 
 * @param message                       An optional participant message to include alongside the sender's public ParticipantData within the slate.
 * @param dest                          Http address.
 * @param error                         Error code pointer.
 *
 * @return                              Void String.
 *
 */
const char* grin_tx_send(const char* path, const char* chain_type, const char* account,const char* password, const char* check_node_api_http_addr, const uint64_t amount, const bool selection_strategy_is_use_all, const char* message,  const char* dest, const uint8_t* error);


/** 
 * @brief                               Reposts a stored, completed but unconfirmed transaction to the chain.
 * 
 * @param path                          The directory in which wallet files are stored.
 * @param chain_type                    Chain parameters.
 * @param account                       Account name.
 * @param password                      Wallet password.
 * @param check_node_api_http_addr      The api address of a running server node against which transaction inputs will be checked during send.
 * @param tx_id                         Transaction id.
 * @param error                         Error code pointer.
 *
 * @return                              Void String.
 *
 */
const char* grin_tx_repost(const char* path, const char* chain_type, const char* account,const char* password, const char* check_node_api_http_addr, const uint32_t tx_id, const uint8_t* error);


/** 
 * @brief                               Initialize a new wallet seed file and database.
 * 
 * @param path                          The directory in which wallet files are stored.
 * @param chain_type                    Chain parameters.
 * @param password                      Wallet password.
 * @param check_node_api_http_addr      The api address of a running server node against which transaction inputs will be checked during send.
 * @param error                         Error code pointer.
 *
 * @return                              The mnemonic word.
 *
 */
const char* grin_wallet_init(const char* path, const char* chain_type, const char* password, const char* check_node_api_http_addr, const uint8_t* error);


/** 
 * @brief                               Get wallet mnemonic word.
 *
 * @param path                          The directory in which wallet files are stored.
 * @param chain_type                    Chain parameters.
 * @param password                      Wallet password.
 * @param check_node_api_http_addr      The api address of a running server node against which transaction inputs will be checked during send.
 * @param error                         Error code pointer.
 *
 * @return                              The mnemonic phrase.
 *
 */
const char* grin_wallet_phrase(const char* path, const char* chain_type, const char* password, const char* check_node_api_http_addr, const uint8_t* error);


/** 
 * @brief                               Recovery the wallet from mnemonic phrase.
 *
 * @param path                          The directory in which wallet files are stored.
 * @param chain_type                    Chain parameters.
 * @param phrase                        The mnemonic phrase.
 * @param password                      Wallet password.
 * @param check_node_api_http_addr      The api address of a running server node against which transaction inputs will be checked during send.
 * @param error                         Error code pointer.
 *
 * @return                              The mnemonic phrase.
 *
 */
const char* grin_wallet_recovery(const char* path, const char* chain_type, const char* phrase, const char* password, const char* check_node_api_http_addr, const uint8_t* error);



/** 
 * @brief                               Checks a wallet's outputs against a live node, repairing and restoring missing outputs if required
 *
 * @param path                          The directory in which wallet files are stored.
 * @param chain_type                    Chain parameters.
 * @param account                       Account name.
 * @param password                      Wallet password.
 * @param check_node_api_http_addr      The api address of a running server node against which transaction inputs will be checked during send.
 * @param error                         Error code pointer.
 *
 * @return                              The mnemonic phrase.
 *
 */
const char* grin_wallet_check(const char* path, const char* chain_type, const char* account,const char* password, const char* check_node_api_http_addr, const uint8_t* error);


/** 
 * @brief                               Restores a wallet contents from a seed file
 *
 * @param path                          The directory in which wallet files are stored.
 * @param chain_type                    Chain parameters.
 * @param account                       Account name.
 * @param password                      Wallet password.
 * @param check_node_api_http_addr      The api address of a running server node against which transaction inputs will be checked during send.
 * @param error                         Error code pointer.
 *
 * @return                              The mnemonic phrase.
 *
 */
const char* grin_wallet_restore(const char* path, const char* chain_type, const char* account,const char* password, const char* check_node_api_http_addr, const uint8_t* error);
```