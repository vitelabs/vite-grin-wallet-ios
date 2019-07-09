//
//  GrinTypes.swift
//  Pods-Vite_GrinWallet_Example
//
//  Created by haoshenyang on 2019/2/21.
//

import Foundation
import ObjectMapper

public struct WalletInfo: Mappable {
    /// height from which info was taken
    public var lastConfirmedHeight: Int = 0
    /// Minimum number of confirmations for an output to be treated as "spendable".
    public var minimumConfirmations: Int = 0
    /// total amount in the wallet
    public var total: Int = 0
    /// amount awaiting confirmation
    public var amountAwaitingConfirmation: Int = 0
    public var amountAwaitingFinalization: Int = 0
    /// coinbases waiting for lock height
    public var amountImmature: Int = 0
    /// amount currently spendable
    public var amountCurrentlySpendable: Int = 0
    /// amount locked via previous transactions
    public var amountLocked: Int = 0

    public init?(map: Map) { }

    public mutating func mapping(map: Map) {
        lastConfirmedHeight <- (map["last_confirmed_height"], JSONTransformer.int)
        minimumConfirmations <- (map["minimum_confirmations"], JSONTransformer.int)
        total <- (map["total"], JSONTransformer.int)
        amountAwaitingConfirmation <- (map["amount_awaiting_confirmation"], JSONTransformer.int)
        amountAwaitingFinalization <- (map["amount_awaiting_finalization"], JSONTransformer.int)
        amountImmature <- (map["amount_immature"], JSONTransformer.int)
        amountCurrentlySpendable <- (map["amount_currently_spendable"], JSONTransformer.int)
        amountLocked <- (map["amount_locked"], JSONTransformer.int)
    }
}

/// Types of transactions that can be contained within a TXLog entry
public enum TxLogEntryType: String {

    /// A coinbase transaction becomes confirmed
    case confirmedCoinbase = "ConfirmedCoinbase"
    /// Outputs created when a transaction is received
    case txReceived = "TxReceived"
    /// Inputs locked + change outputs when a transaction is created
    case txSent = "TxSent"
    /// Received transaction that was rolled back by user
    case txReceivedCancelled = "TxReceivedCancelled"
    /// Sent transaction that was rolled back by user
    case txSentCancelled = "TxSentCancelled"
}


public struct TxLogEntry: Mappable {
    /// BIP32 account path used for creating this tx
    public var parent_key_id: String = ""
    /// Local id for this transaction (distinct from a slate transaction id)
    public var id: UInt32 = 0
    /// Slate transaction this entry is associated with, if any
    public var txSlateId: String?
    /// Transaction type (as above)
    public var txType: TxLogEntryType!
    /// Time this tx entry was created
    public var creationTs: String  = ""
    /// Time this tx was confirmed (by this wallet)
    public var confirmationTs: String?
    /// Whether the inputs+outputs involved in this transaction have been
    /// confirmed (In all cases either all outputs involved in a tx should be
    /// confirmed, or none should be; otherwise there's a deeper problem)
    public var confirmed: Bool!
    /// number of inputs involved in TX
    public var numInputs: Int = 0
    /// number of outputs involved in TX
    public var numOutputs: Int  = 0
    /// Amount credited via this transaction
    public var amountCredited: Int?
    /// Amount debited via this transaction
    public var amountDebited: Int?
    /// Fee
    public var fee: Int?
    /// Message data, stored as json
    public var messages: [String: [ParticipantMessageData]]?
    /// Location of the store transaction, (reference or resending)
    public var storedTx: String?

    public init?(map: Map) { }

    public mutating func mapping(map: Map) {
        parent_key_id <- map["parent_key_id"]
        id <- map["id"]
        txSlateId <- map["tx_slate_id"]
        txType <- map["tx_type"]
        creationTs <- map["creation_ts"]
        confirmationTs <- map["confirmation_ts"]
        confirmed <- map["confirmed"]
        numInputs <- map["num_inputs"]
        numOutputs <- map["num_outputs"]
        amountCredited <- (map["amount_credited"], JSONTransformer.int)
        amountDebited <- (map["amount_debited"], JSONTransformer.int)
        fee <- (map["fee"], JSONTransformer.int)
        messages <- map["messages"]
        storedTx <- map["stored_tx"]
    }
}

public struct ParticipantMessageData {
    /// id of the particpant in the tx
    public var id: Int
    /// Public key
    public var public_key: String
    /// Message,
    public var message: String?
    /// Signature
    public var message_sig: String?
}

public struct Slate: Mappable {
    public var versionInfo: VersionInfo = VersionInfo()
    /// The number of participants intended to take part in this transaction
    public var numParticipants: Int = 0
    /// Unique transaction ID, selected by sender
    public var id: String = ""
    /// The core transaction data:
    /// inputs, outputs, kernels, kernel offset
    public var tx: [String: Any] = [:]
    /// base amount (excluding fee)
    public var amount: Int = 0
    /// fee amount
    public var fee: Int = 0
    /// Block height for the transaction
    public var height: Int = 0
    /// Lock height
    public var lockHeight: Int = 0
    /// Participant data, each participant in the transaction will
    /// insert their public data here. For now, 0 is sender and 1
    /// is receiver, though this will change for multi-party
    public var participantData: [Any] = []
    /// Slate format version

    public init?(map: Map) { }

    public mutating func mapping(map: Map) {
        versionInfo <- map["version_info"]
        numParticipants <- map["num_participants"]
        id <- map["id"]
        tx <- map["tx"]
        amount <- (map["amount"] ,JSONTransformer.int)
        fee <- (map["fee"] ,JSONTransformer.int)
        height <- (map["height"] ,JSONTransformer.int)
        lockHeight <- (map["lock_height"] ,JSONTransformer.int)
        participantData <- map["participant_data"]
    }
}

public class VersionInfo: Mappable {

    init() {

    }
    public var version: Int = 0
    public var orig_version: Int = 0
    public var block_header_version: Int = 0

    required public init?(map: Map) { }

    public func mapping(map: Map) {
        version <- map["version"]
        orig_version <- map["orig_version"]
        block_header_version <- map["block_header_version"]
    }

}


public struct ParticipantData {
    /// Id of participant in the transaction. (For now, 0=sender, 1=rec)
    public var id: Int
    /// Public key corresponding to private blinding factor
    public var public_blind_excess: [Int]
    /// Public key corresponding to private nonce
    public var public_nonce: [Int]
    /// Public partial signature
    public var part_sig: [Int]?
    /// A message for other participants
    public var message: String?
    /// Signature, created with private key corresponding to 'public_blind_excess'
    public var message_sig: [Int]?
}

public enum OutputStatus: String {
    /// Unconfirmed
    case unconfirmed = "Unconfirmed"
    /// Unspent
    case unspent = "Unspent"
    /// Locked
    case locked = "Locked"
    /// Spent
    case spent = "Spent"
}

/// Information about an output that's being tracked by the wallet. Must be
/// enough to reconstruct the commitment associated with the ouput when the
/// root private key is known.

public struct OutputData: Mappable {
    /// Root key_id that the key for this output is derived from
    public var root_key_id: String = ""
    /// Derived key for this output
    public var key_id: String = ""
    /// How many derivations down from the root key
    public var n_child: UInt32 = 0
    /// The actual commit, optionally stored
    public var commit: String?
    /// PMMR Index, used on restore in case of duplicate wallets using the same
    /// key_id (2 wallets using same seed, for instance
    public var mmr_index: UInt64?
    /// Value of the output, necessary to rebuild the commitment
    public var value: Int  = 0
    /// Current status of the output
    public var status: OutputStatus = .unconfirmed
    /// Height of the output
    public var height: Int  = 0
    /// Height we are locked until
    public var lock_height: Int = 0
    /// Is this a coinbase output? Is it subject to coinbase locktime?
    public var is_coinbase: Bool = false
    /// Optional corresponding internal entry in tx entry log
    public var tx_log_entry: UInt32?

    public init?(map: Map) { }

    public mutating func mapping(map: Map) {
        root_key_id <- map["root_key_id"]
        key_id <- map["key_id"]
        n_child <- map["n_child"]
        commit <- map["commit"]
        mmr_index <- map["mmr_index"]
        value <- (map["value"], JSONTransformer.int)
        status <- map["status"]
        height <- (map["height"], JSONTransformer.int)
        lock_height <- (map["lock_height"], JSONTransformer.int)
        is_coinbase <- map["is_coinbase"]
        tx_log_entry <- map["tx_log_entry"]
    }

}

public struct TxStrategy: Mappable  {
    public var selectionStrategyIsUseAll: Bool = false
    public var total: Int = 0
    public var fee: Int = 0

    public init?(map: Map) { }

    public mutating func mapping(map: Map) {
        selectionStrategyIsUseAll <- map["selection_strategy_is_use_all"]
        total <- map["total"]
        fee <- map["fee"]
    }
}

public struct GrinWalletError: Error {
    public let code: Int
    public let message: String
}

public enum GrinChainType: String {
    case usernet
    case floonet
    case mainnet
}


public struct JSONTransformer {


    public static let int = TransformOf<Int, String>(fromJSON: { (string) -> Int? in
        guard let string = string, let num = Int(string) else { return nil }
        return num
    }, toJSON: { (num) -> String? in
        guard let num = num else { return nil }
        return String(num)
    })

}
