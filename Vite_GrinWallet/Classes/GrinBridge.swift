//
//  GrinBridge.swift
//  Pods-Vite_GrinWallet_Example
//
//  Created by haoshenyang on 2019/2/19.
//

import Foundation
import Result
import ObjectMapper
import SwiftyJSON

func handleCResult(error: UInt8, cResult: UnsafePointer<Int8>) -> Result<String, GrinWalletError> {
    let result = String(cString: cResult)
    cstr_free(cResult)
    if error != 0 {
        return .failure(GrinWalletError(code: Int(error), message: result))
    } else {
        return .success(result)
    }
}


open class GrinBridge {

    public init(chainType: GrinChainType, walletUrl: URL,  password: String) {
        self.walletUrl = walletUrl
        self.chainType = chainType.rawValue
        self.password = password
        checkDirectories()
        checkApiSecret()
    }

    open var chainType: String
    open var walletUrl: URL
    open var password: String
    open var checkNodeApiHttpAddr = "https://grin.vite.net/fullnode"
    open var apiSecret = "Pbwnf9nJDEVcVPR8B42u"
    private let account = "default"
    lazy var paresDataError = GrinWalletError(code: -1, message: "paresDataError")

    public func walletExists() -> Bool {
        let path = walletUrl.path + "/wallet_data/wallet.seed"
        return FileManager.default.fileExists(atPath:path)
    }

    public func walletInfo(refreshFromNode: Bool) -> Result<WalletInfo, GrinWalletError> {
        var error: UInt8 = 0
        let cResult = grin_balance(walletUrl.path, chainType, account, password, checkNodeApiHttpAddr, refreshFromNode, &error)
        return handleCResult(error:error, cResult:cResult!)
            .flatMap {
                if let walletInfo = WalletInfo(JSONString: $0) {
                    return .success(walletInfo)
                } else {
                    return .failure(paresDataError)
                }
            }
    }

    public func txsGet(refreshFromNode: Bool) -> Result<(refreshed:Bool, txLogEntries:[TxLogEntry]), GrinWalletError> {
        var error: UInt8 = 0
        let cResult = grin_txs_get(walletUrl.path, chainType, account, password, checkNodeApiHttpAddr, refreshFromNode, &error)
        return handleCResult(error:error, cResult:cResult!)
            .flatMap {
                guard let jsonArray = JSON(parseJSON: $0).array,
                    let refreshed = jsonArray.first?.bool,
                    let txLogEntries =  Mapper<TxLogEntry>().mapArray(JSONObject: jsonArray.last?.arrayObject) else {
                    return .failure(paresDataError)
                }
                return .success((refreshed, txLogEntries))
        }
    }

    public func txGet(refreshFromNode: Bool, txId: UInt32) -> Result<(refreshed:Bool, txLogEntry:TxLogEntry), GrinWalletError> {
        var error: UInt8 = 0
        let cResult = grin_tx_get(walletUrl.path, chainType, account, password, checkNodeApiHttpAddr, refreshFromNode, txId, &error)
        return handleCResult(error:error, cResult:cResult!)
            .flatMap {
                if let jsonArray = JSON(parseJSON: $0).array,
                    let refreshed = jsonArray.first?.bool,
                    let dictionaryObject = jsonArray.last?.array?.first?.dictionaryObject,
                    let txLogEntry = TxLogEntry(JSON: dictionaryObject) {
                    return .success((refreshed, txLogEntry))
                } else {
                    return .failure(paresDataError)
                }
        }
    }

    public func txCreate(amount: UInt64, selectionStrategyIsUseAll: Bool, message: String) -> Result<Slate, GrinWalletError> {
        var error: UInt8 = 0
        let cResult = grin_tx_create(walletUrl.path, chainType, account, password, checkNodeApiHttpAddr, amount, selectionStrategyIsUseAll, message, &error)
        return handleCResult(error:error, cResult:cResult!)
            .flatMap {
                if let slate = Slate(JSONString:$0) {
                    return .success(slate)
                } else {
                    return .failure(paresDataError)
                }
        }
    }

    public func txStrategies(amount: UInt64) -> Result<(all:TxStrategy,smallest:TxStrategy), GrinWalletError> {
        var error: UInt8 = 0
        let cResult = grin_tx_strategies(walletUrl.path, chainType, account, password, checkNodeApiHttpAddr, amount, &error)
        return handleCResult(error:error, cResult:cResult!)
            .flatMap {
                if let arrayObject = JSON(parseJSON: $0).arrayObject,
                    let txStrategies = Mapper<TxStrategy>().mapArray(JSONObject:arrayObject) {
                    return .success((txStrategies.first!, txStrategies.last!))
                } else {
                    return .failure(paresDataError)
                }
        }
    }

    public func txCancel(id: UInt32) -> Result<Void, GrinWalletError> {
        var error: UInt8 = 0
        let cResult = grin_tx_cancel(walletUrl.path, chainType, account, password, checkNodeApiHttpAddr, id, &error)
        return handleCResult(error:error, cResult:cResult!).map { _ in ()}
    }

    public func txReceive(slatePath: String, message: String) -> Result<Slate, GrinWalletError> {
        var error: UInt8 = 0
        let cResult = grin_tx_receive(walletUrl.path, chainType, account, password, checkNodeApiHttpAddr, slatePath,message, &error)
        return handleCResult(error:error, cResult:cResult!)
            .flatMap {
                if let slate = Slate(JSONString:$0) {
                    return .success(slate)
                } else {
                    return .failure(paresDataError)
                }
        }
    }

    public func txFinalize(slatePath: String) -> Result<String, GrinWalletError> {
        var error: UInt8 = 0
        let cResult = grin_tx_finalize(walletUrl.path, chainType, account, password, checkNodeApiHttpAddr, slatePath, &error)
        return handleCResult(error:error, cResult:cResult!)
    }

    public func txSend(amount: UInt64, selectionStrategyIsUseAll: Bool, message: String, dest:String) -> Result<Slate, GrinWalletError> {
        var error: UInt8 = 0
        let cResult = grin_tx_send(walletUrl.path, chainType, account, password, checkNodeApiHttpAddr, amount, selectionStrategyIsUseAll, message, dest, &error)
        return handleCResult(error:error, cResult:cResult!).flatMap {
            if let slate = Slate(JSONString:$0) {
                return .success(slate)
            } else {
                return .failure(paresDataError)
            }
        }
    }

    public func txRepost(txId: UInt32) -> Result<String, GrinWalletError> {
        var error: UInt8 = 0
        let cResult = grin_tx_repost(walletUrl.path, chainType, account, password, checkNodeApiHttpAddr,  txId, &error)
        return handleCResult(error:error, cResult:cResult!)
    }

    public func walletInit() -> Result<String, GrinWalletError> {
        var error: UInt8 = 0
        let cResult = grin_wallet_init(walletUrl.path, chainType, password, checkNodeApiHttpAddr, &error)
        return handleCResult(error:error, cResult:cResult!)
    }

    public func walletPhrase() -> Result<String, GrinWalletError> {
        var error: UInt8 = 0
        let cResult = grin_wallet_phrase(walletUrl.path, chainType, password, checkNodeApiHttpAddr, &error)
        return handleCResult(error:error, cResult:cResult!)
    }

    public func walletRecovery(_ phrase: String) -> Result<String, GrinWalletError> {
        var error: UInt8 = 0
        let cResult = grin_wallet_recovery(walletUrl.path, chainType, phrase, password, checkNodeApiHttpAddr, &error)
        return handleCResult(error:error, cResult:cResult!)
    }

    public func walletCheck() -> Result<Void, GrinWalletError> {
        var error: UInt8 = 0
        let cResult = grin_wallet_check(walletUrl.path, chainType, account, password, checkNodeApiHttpAddr, &error)
        return handleCResult(error:error, cResult:cResult!).map { _ in ()}
    }

    public func walletRestore() -> Result<(Void), GrinWalletError> {
        var error: UInt8 = 0
        let cResult = grin_wallet_restore(walletUrl.path, chainType, account, password, checkNodeApiHttpAddr, &error)
        return handleCResult(error:error, cResult:cResult!).map { _ in ()}
    }

    public func height() -> Result<(Bool, Int), GrinWalletError> {
        var error: UInt8 = 0
        let cResult = grin_height(walletUrl.path, chainType, account, password, checkNodeApiHttpAddr, &error)
        return handleCResult(error:error, cResult:cResult!)
            .flatMap {
                guard let jsonArray = JSON(parseJSON: $0).array,
                    let refreshed = jsonArray.last?.bool,
                    let height =  jsonArray.first?.int else {
                        return .failure(paresDataError)
                }
                return .success((refreshed, height))
        }
    }

    public func outputsGet(refreshFromNode: Bool) -> Result<(refreshed:Bool, outputs:[(OutputData,[Int])]), GrinWalletError> {
        var error: UInt8 = 0
        let cResult = grin_outputs_get(walletUrl.path, chainType, account, password, checkNodeApiHttpAddr, refreshFromNode, &error)
        return handleCResult(error:error, cResult:cResult!)
            .flatMap {
                guard let jsonArray = JSON(parseJSON: $0).array,
                    let refreshed = jsonArray.first?.bool,
                    let infoArray = jsonArray.last?.arrayObject as? [[Any]]else {
                        return .failure(paresDataError)
                }
                let infos = infoArray.flatMap { (data: [Any]) -> (OutputData,[Int])? in
                    if let outputDict = data.first as? [String: Any],
                    let commitment = data.last as? [Int],
                    let output = OutputData(JSON: outputDict) {
                        return (output, commitment)
                    } else {
                        return nil
                    }
                }
                return .success((refreshed,infos))
        }
    }

    public func outputGet(refreshFromNode: Bool, txId: UInt32) -> Result<(refreshed:Bool, outputs:[(OutputData,[Int])]), GrinWalletError> {
        var error: UInt8 = 0
        let cResult = grin_output_get(walletUrl.path, chainType, account, password, checkNodeApiHttpAddr, refreshFromNode, txId, &error)
        return handleCResult(error:error, cResult:cResult!)
            .flatMap {
                guard let jsonArray = JSON(parseJSON: $0).array,
                    let refreshed = jsonArray.first?.bool,
                    let infoArray = jsonArray.last?.arrayObject as? [[Any]]else {
                        return .failure(paresDataError)
                }
                let infos = infoArray.flatMap { (data: [Any]) -> (OutputData,[Int])? in
                    if let outputDict = data.first as? [String: Any],
                        let commitment = data.last as? [Int],
                        let output = OutputData(JSON: outputDict) {
                        return (output, commitment)
                    } else {
                        return nil
                    }
                }
                return .success((refreshed,infos))
        }
    }


    public func isResponseSlate(slatePath: String) -> Bool {
        return slatePath.components(separatedBy: ".").last == "response" || slatePath.contains("response")
    }

    public func getSlateUrl(slateId: String, isResponse: Bool) -> URL {
        let path = "\(walletUrl.path)/slates/\(slateId).grinslate\(isResponse ? ".response" : "")"
        return URL(fileURLWithPath: path)
    }

    public func checkApiSecret() {
        let url =  walletUrl.appendingPathComponent(".api_secret")
        let exists = FileManager.default.fileExists(atPath: url.path)
        if !exists {
            do {
                try apiSecret.write(to: url, atomically: true, encoding: .utf8)
            } catch {
                print(error)
            }
        }
    }

    public func checkDirectories() {
        let walletDataUrl = walletUrl.appendingPathComponent("wallet_data")
        let slatesUrl = walletUrl.appendingPathComponent("slates")
        for url in [walletUrl, walletDataUrl, slatesUrl] {
            if !FileManager.default.fileExists(atPath: url.path) {
                do {
                    try FileManager.default.createDirectory(at: url, withIntermediateDirectories: true, attributes: nil)
                } catch {
                    print(error)
                }
            }
        }
    }

}
