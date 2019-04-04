import XCTest
import Vite_GrinWallet

class Tests: XCTestCase {

    var firstBridge: GrinBridge!
    var secondBridge: GrinBridge!

    override func setUp() {
        super.setUp()
        // Put setup code here. This method is called before the invocation of each test method in the class.
        let libraryDirectory = FileManager.default.urls(for: .libraryDirectory, in: .userDomainMask).first!
        print(libraryDirectory.path)
        let firstURL = libraryDirectory.appendingPathComponent("grin/fisetWallet")
        let secondURL = libraryDirectory.appendingPathComponent("grin/secondWallet")
        firstBridge = GrinBridge.init(chainType: .usernet, walletUrl: firstURL, password: "")
        secondBridge = GrinBridge.init(chainType: .usernet, walletUrl: secondURL, password: "")
        if !firstBridge.walletExists() {
            let result = firstBridge.walletRecovery("whip swim spike cousin dinosaur vacuum save few boring monster crush ocean brown suspect swamp zone bounce hard sadness bulk reform crack crack accuse")
            switch result {
            case .success(_):
                break
            case .failure(let error):
                XCTAssert(false, error.message)
            }
        }
        if !secondBridge.walletExists() {
            let result = secondBridge.walletInit()
            switch result {
            case .success(_):
                break
            case .failure(let error):
                XCTAssert(false, error.message)
            }
        }
    }
    
    override func tearDown() {
        // Put teardown code here. This method is called after the invocation of each test method in the class.
        super.tearDown()
    }
    
    func testGetBalance() {
        let fromLocal = firstBridge.walletInfo(refreshFromNode: false)
        switch fromLocal {
        case .success(let info):
            print(info)
            break
        case .failure(let error):
            XCTAssert(false, error.message)
        }
       let fromNode = firstBridge.walletInfo(refreshFromNode: true)
        switch fromNode {
        case .success(let info):
            print(info)
        case .failure(let error):
            XCTAssert(false, error.message)
        }

        XCTAssert(true, "Pass")
    }

    func testGetTxs() {
        let fromLocal = firstBridge.txsGet(refreshFromNode: false)
        switch fromLocal {
        case .success(_):
            break
        case .failure(let error):
            XCTAssert(false, error.message)
        }

        let fromNode = firstBridge.txsGet(refreshFromNode: true)
        switch fromNode {
        case .success(let txs):
            print(txs)
        case .failure(let error):
            XCTAssert(false, error.message)
        }

        XCTAssert(true, "Pass")
    }

    func testWalletRestore() {
        for bridge in [firstBridge, secondBridge] {
            let result = bridge!.walletRestore()
            switch result {
            case .success(_):
                break
            case .failure(let error):
                XCTAssert(false, error.message)
            }
        }
        XCTAssert(true, "Pass")
    }

    func testWalletCheck() {
        let result = firstBridge.walletCheck()
        switch result {
        case .success(_):
            XCTAssert(true, "Pass")
        case .failure(let error):
            XCTAssert(false, error.message)
        }
    }

    func testTxStrategies() {
        let result = firstBridge.txStrategies(amount: 10)
        switch result {
        case .success(let strategies):
            print(strategies)
            XCTAssert(true, "Pass")
        case .failure(let error):
            XCTAssert(false, error.message)
        }
    }

    func testTxViaFile() {
        let send = firstBridge.txCreate(amount: 1, selectionStrategyIsUseAll: false, message: "test send")
        switch send {
        case .success(let sendSlate):
            print(sendSlate.id)
            let sendSlateUrl = firstBridge.getSlateUrl(slateId: sendSlate.id, isResponse: false)
            do {
                try sendSlate.toJSONString()?.write(to: sendSlateUrl, atomically: true, encoding: .utf8)
            } catch {
                XCTAssert(false, error.localizedDescription)
            }
            let receive = secondBridge.txReceive(slatePath: sendSlateUrl.path, message: "test receive")
            switch receive {
            case .success(let receiveSlate):
                let receiveSlateUrl = secondBridge.getSlateUrl(slateId: receiveSlate.id, isResponse: true)
                do {
                    try receiveSlate.toJSONString()?.write(to: receiveSlateUrl, atomically: true, encoding: .utf8)
                } catch {
                    XCTAssert(false, error.localizedDescription)
                }
                let finalize = firstBridge.txFinalize(slatePath: receiveSlateUrl.path)
                switch finalize {
                case .success(_):
                    XCTAssert(true, "Pass")
                case .failure(let error):
                    XCTAssert(false, error.message)
                }
            case .failure(let error):
                XCTAssert(false, error.message)
            }
        case .failure(let error):
            XCTAssert(false, error.message)
        }
    }

    func testTxViaHttp() {
        let result = firstBridge.txSend(amount: 1, selectionStrategyIsUseAll: false, message: "test tx", dest: "http://192.168.31.47:23415")
        switch result {
        case .success(_):
            XCTAssert(true, "Pass")
        case .failure(let error):
            XCTAssert(false, error.message)
        }
    }

    func testTxReceive() {
          let receiveSlateUrl = firstBridge.getSlateUrl(slateId: "19b9c0bd-2e7f-47e6-88c7-1c6b76bbe725", isResponse: true)
        print(receiveSlateUrl.path)
        let receive = secondBridge.txReceive(slatePath: receiveSlateUrl.path, message: "")
        switch receive {
        case .success(let receiveSlate):
            print(receiveSlate)
        case .failure(let error):
            XCTAssert(false, error.message)
        }
    }


    func testTxFinalize() {
        let receiveSlateUrl = firstBridge.getSlateUrl(slateId: "19b9c0bd-2e7f-47e6-88c7-1c6b76bbe725", isResponse: true)
        print(receiveSlateUrl.path)
        let receive = firstBridge.txFinalize(slatePath:  receiveSlateUrl.path)
        switch receive {
        case .success(let receiveSlate):
            print(receiveSlate)
        case .failure(let error):
            XCTAssert(false, error.message)
        }
    }

    func testTxCancleAll() {
        let fromNode = firstBridge.txsGet(refreshFromNode: true)
        switch fromNode {
        case .success((_, let txs)):
            let sents = txs.filter { $0.txType  == .txSent }
            if sents.isEmpty {
                _ = firstBridge.txCreate(amount: 1, selectionStrategyIsUseAll: false, message: "")
                testTxCancleAll()
                return
            }
            for sent in sents {
                let cancel = firstBridge.txCancel(id: UInt32(sent.id))
                switch cancel {
                case .success(_):
                    break
                case .failure(let error):
                    XCTAssert(false, error.message)
                }
            }
            XCTAssert(true)
        case .failure(let error):
            XCTAssert(false, error.message)
        }
    }

    func testCancelSendTx() {
        let result = firstBridge.txCreate(amount: 2, selectionStrategyIsUseAll: false, message: "testCanclesSendTx")
        switch result {
        case .success(let slate):
            print(slate.id)
            let txsResult = firstBridge.txsGet(refreshFromNode: false)
            switch txsResult {
            case .success((_, let txs)):
                guard let tx = txs.filter ({
                    return $0.txSlateId == slate.id && $0.txType == .txSent
                }).first else {
                    XCTAssert(false); return
                }
                let cancleResult = firstBridge.txCancel(id: UInt32(tx.id))
                switch cancleResult {
                case .success(_):
                    let cancledTxResult = firstBridge.txGet(refreshFromNode: true, txId: UInt32(tx.id))
                    switch cancledTxResult {
                    case .success((_,let cancledTx)):
                        if cancledTx.txType == .txSentCancelled {
                            XCTAssert(true)
                        } else {
                            XCTAssert(false)
                        }
                    case .failure(let error):
                        XCTAssert(false, error.message)
                    }
                case .failure(let error):
                    XCTAssert(false, error.message)
                }
            case .failure(let error):
                XCTAssert(false, error.message)
            }
        case .failure(let error):
            XCTAssert(false, error.message)
        }
    }

    func testCancelReceiveTx() {
        let result = firstBridge.txCreate(amount: 2, selectionStrategyIsUseAll: false, message: "testCancelReceiveTx-send")
        switch result {
        case .success(let sendSlate):
            let sendSlateUrl = firstBridge.getSlateUrl(slateId: sendSlate.id, isResponse: false)
            do {
                try sendSlate.toJSONString()?.write(to: sendSlateUrl, atomically: true, encoding: .utf8)
            } catch {
                XCTAssert(false, error.localizedDescription)
            }
            let receive = secondBridge.txReceive(slatePath: sendSlateUrl.path, message: "testCancelReceiveTx-receive")
            switch receive {
            case .success(let slate):
                print(slate.id)
                let txsResult = secondBridge.txsGet(refreshFromNode: false)
                switch txsResult {
                case .success((_, let txs)):
                    guard let tx = txs.filter ({
                        return $0.txSlateId == slate.id && $0.txType == .txReceived
                    }).first else {
                        XCTAssert(false); return
                    }
                    let cancleResult = secondBridge.txCancel(id: UInt32(tx.id))
                    switch cancleResult {
                    case .success(_):
                        let cancledTxResult = secondBridge.txGet(refreshFromNode: true, txId: UInt32(tx.id))
                        switch cancledTxResult {
                        case .success((_,let cancledTx)):
                            if cancledTx.txType == .txReceivedCancelled {
                                XCTAssert(true)
                            } else {
                                XCTAssert(false)
                            }
                        case .failure(let error):
                            XCTAssert(false, error.message)
                        }
                    case .failure(let error):
                        XCTAssert(false, error.message)
                    }
                case .failure(let error):
                    XCTAssert(false, error.message)
                }
            case .failure(let error):
                XCTAssert(false, error.message)
            }
        case .failure(let error):
            XCTAssert(false, error.message)
        }
    }

}
