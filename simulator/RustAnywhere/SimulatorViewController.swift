//
//  ViewController.swift
//  RustAnywhere
//

import Cocoa

class SimulatorViewController: NSViewController {
    private let simulator = Simulator.sharedInstance

    @IBOutlet private var greenLEDView: LEDView?
    @IBOutlet private var redLEDView: LEDView?
    @IBOutlet private var buttonView: ButtonView?

    override func viewDidLoad() {
        super.viewDidLoad()
        simulator.delegate = self
    }
}

extension SimulatorViewController: SimulatorDelegate {
    func onSetLED(which: LEDs, on: Bool) {
        switch (which, on) {
        case (GREEN, true):
            greenLEDView?.color = .systemGreen
        case (GREEN, false):
            greenLEDView?.color = .black
        case (RED, true):
            redLEDView?.color = .systemRed
        case (RED, false):
            redLEDView?.color = .black
        default:
            // Unreachable.
            print("Unexpected LED: \(which)")
        }
    }
}
