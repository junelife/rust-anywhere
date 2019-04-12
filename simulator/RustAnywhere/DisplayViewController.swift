//
//  DisplayViewController.swift
//  RustAnywhere
//

import Cocoa
import MetalKit

class DisplayViewController: NSViewController {
    private let simulator = Simulator.sharedInstance

    override func viewDidLoad() {
        super.viewDidLoad()
        simulator.mtkView = view as? MTKView
        simulator.run()
    }

    override func viewDidAppear() {
        super.viewDidAppear()

        // NOTE: By default the view controller won't receive mouse move events,
        // so set up a tracking area to force them to be sent.
        let trackingOptions: NSTrackingArea.Options = [
            .activeAlways,
            .inVisibleRect,
            .mouseMoved
        ]
        let trackingArea = NSTrackingArea(rect: NSZeroRect, options: trackingOptions, owner: self, userInfo: nil)
        view.addTrackingArea(trackingArea)
    }

    override func mouseDown(with event: NSEvent) {
        let loc = event.locationInWindow
        let x = loc.x * CGFloat(LV_HOR_RES) / view.bounds.size.width
        let y = (view.bounds.size.height - loc.y) * CGFloat(LV_VER_RES) / view.bounds.size.height
        simulator.onMouseDown(x: x, y: y)
    }

    override func mouseUp(with event: NSEvent) {
        let loc = event.locationInWindow
        let x = loc.x * CGFloat(LV_HOR_RES) / view.bounds.size.width
        let y = (view.bounds.size.height - loc.y) * CGFloat(LV_VER_RES) / view.bounds.size.height
        simulator.onMouseUp(x: x, y: y)
    }

    override func mouseMoved(with event: NSEvent) {
        let loc = event.locationInWindow
        let x = loc.x * CGFloat(LV_HOR_RES) / view.bounds.size.width
        let y = (view.bounds.size.height - loc.y) * CGFloat(LV_VER_RES) / view.bounds.size.height
        simulator.onMouseMoved(x: x, y: y)
    }

    override func mouseDragged(with event: NSEvent) {
        let loc = event.locationInWindow
        let x = loc.x * CGFloat(LV_HOR_RES) / view.bounds.size.width
        let y = (view.bounds.size.height - loc.y) * CGFloat(LV_VER_RES) / view.bounds.size.height
        simulator.onMouseMoved(x: x, y: y)
    }
}
