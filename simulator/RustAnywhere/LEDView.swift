//
//  LEDView.swift
//  RustAnywhere
//

import Cocoa

@IBDesignable class LEDView: NSView {
    @IBInspectable var color: NSColor = .black {
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
}
