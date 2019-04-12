//
//  ButtonView.swift
//  RustAnywhere
//

import Cocoa

@IBDesignable class ButtonView: NSView {
    private let simulator = Simulator.sharedInstance

    @IBInspectable var color: NSColor = .systemGray {
        didSet {
            layer?.backgroundColor = color.cgColor
        }
    }

    override init(frame frameRect: NSRect) {
        super.init(frame: frameRect)
        wantsLayer = true
    }

    required init?(coder decoder: NSCoder) {
        super.init(coder: decoder)
        wantsLayer = true
    }

    override func prepareForInterfaceBuilder() {
        layer?.backgroundColor = color.cgColor
    }

    override func awakeFromNib() {
        layer?.backgroundColor = color.cgColor
    }

    override func mouseDown(with event: NSEvent) {
        simulator.onButtonPressed(pressed: true)
        color = .systemBlue
    }

    override func mouseUp(with event: NSEvent) {
        simulator.onButtonPressed(pressed: false)
        color = .systemGray
    }
}
