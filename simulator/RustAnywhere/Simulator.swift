//
//  Simulator.swift
//  RustAnywhere
//

import CoreVideo
import Foundation
import Metal
import MetalKit
import QuartzCore

protocol SimulatorDelegate {
    func onSetLED(which: LEDs, on: Bool)
}

class Simulator: NSObject {
    static let sharedInstance = Simulator()

    var delegate: SimulatorDelegate?

    var mtkView: MTKView? {
        didSet {
            guard let mtkView = mtkView else { return }
            mtkView.delegate = self
            mtkView.device = metalDevice
            mtkView.colorPixelFormat = .bgra8Unorm_srgb
        }
    }

    private let metalDevice: MTLDevice
    private let commandQueue: MTLCommandQueue
    private let pipelineState: MTLRenderPipelineState
    private let vertexBuffer: MTLBuffer
    private let texCoordBuffer: MTLBuffer

    private let textureCache: [MTLTexture]
    private var currentTexture: Int = 0

    private var frameBuffer = [UInt32](repeating: 0, count: Int(LV_HOR_RES * LV_VER_RES))
    private var frameDirty = false

    private var displayDriver = lv_disp_drv_t()
    private var mouseDriver = lv_indev_drv_t()

    private var lastMouseX: lv_coord_t = 0
    private var lastMouseY: lv_coord_t = 0
    private var mouseDown: Bool = false

    private var buttonPressed: Bool = false

    override init() {
        metalDevice = createSystemDevice()
        commandQueue = createCommandQueue(device: metalDevice)
        pipelineState = createPipelineState(device: metalDevice)
        vertexBuffer = createVertexBuffer(device: metalDevice)
        texCoordBuffer = createTexCoordBuffer(device: metalDevice)
        textureCache = createTextureCache(device: metalDevice)
        super.init()

        // Initialize the lvgl library.
        lv_init()

        // Register the display callbacks.
        lv_disp_drv_init(&displayDriver)
        displayDriver.disp_flush = displayFlush
        displayDriver.disp_fill = displayFill
        displayDriver.disp_map = displayMap
        lv_disp_drv_register(&displayDriver)

        // Register the mouse callbacks.
        lv_indev_drv_init(&mouseDriver)
        mouseDriver.type = lv_hal_indev_type_t(LV_INDEV_TYPE_POINTER)
        mouseDriver.read = mouseRead
        lv_indev_drv_register(&mouseDriver)

        // Create a memory monitor task which prints the memory usage periodically.
        lv_task_create(memoryMonitor, 30000, UInt8(LV_TASK_PRIO_LOWEST), nil)

        // Initialize the API components.
        hmi_init()
    }

    func run() {
        var lastTime = CACurrentMediaTime()
        let timer = Timer(timeInterval: 0.005, repeats: true) { [weak self] _ in
            guard let self = self else { return }

            // Update the API components.
            hmi_update();

            // Run the lvgl event loop.
            let currentTime = CACurrentMediaTime()
            let elapsedTime = UInt32(1000 * (currentTime - lastTime))
            lastTime = currentTime
            lv_tick_inc(elapsedTime)
            lv_task_handler()
            self.updateTextures()
        }
        RunLoop.main.add(timer, forMode: .common)
    }

    private func updateTextures() {
        guard frameDirty else { return }
        frameDirty = false

        currentTexture = (currentTexture + 1) % textureCache.count
        let texture = textureCache[currentTexture]

        let origin = MTLOrigin(x: 0, y: 0, z: 0)
        let size = MTLSize(width: Int(LV_HOR_RES), height: Int(LV_VER_RES), depth: 1)
        let region = MTLRegion(origin: origin, size: size)
        let bytesPerRow = Int(LV_HOR_RES) * MemoryLayout<Float>.stride
        texture.replace(region: region, mipmapLevel: 0, withBytes: frameBuffer, bytesPerRow: bytesPerRow)
    }
}

extension Simulator {
    func onMouseMoved(x: CGFloat, y: CGFloat) {
        lastMouseX = lv_coord_t(x)
        lastMouseY = lv_coord_t(y)
    }

    func onMouseDown(x: CGFloat, y: CGFloat) {
        lastMouseX = lv_coord_t(x)
        lastMouseY = lv_coord_t(y)
        mouseDown = true
    }

    func onMouseUp(x: CGFloat, y: CGFloat) {
        lastMouseX = lv_coord_t(x)
        lastMouseY = lv_coord_t(y)
        mouseDown = false
    }

    func onButtonPressed(pressed: Bool) {
        buttonPressed = pressed
    }
}

extension Simulator {
    fileprivate func onIsButtonPressed() -> Bool {
        return buttonPressed
    }

    fileprivate func onSetLED(which: LEDs, on: Bool) {
        delegate?.onSetLED(which: which, on: on)
    }
}

extension Simulator {
    fileprivate func onDisplayFlush(x1: Int32, y1: Int32, x2: Int32, y2: Int32, color: UnsafePointer<lv_color_t>) {
        defer {
            // Let lvgl know we have finished flushing the buffer.
            lv_flush_ready()
        }
        guard x1 >= 0 && x1 < LV_HOR_RES else { return }
        guard x2 >= 0 && x2 < LV_HOR_RES else { return }
        guard y1 >= 0 && y1 < LV_VER_RES else { return }
        guard y2 >= 0 && y2 < LV_VER_RES else { return }

        // Update the internal framebuffer with the new pixel values.
        var c = color
        for y in y1...y2 {
            for x in x1...x2 {
                frameBuffer[Int(LV_HOR_RES * y + x)] = lv_color_to32(c.pointee)
                c = c.successor()
            }
        }
        frameDirty = true
    }

    fileprivate func onDisplayFill(x1: Int32, y1: Int32, x2: Int32, y2: Int32, color: lv_color_t) {
        guard x1 >= 0 && x1 < LV_HOR_RES else { return }
        guard x2 >= 0 && x2 < LV_HOR_RES else { return }
        guard y1 >= 0 && y1 < LV_VER_RES else { return }
        guard y2 >= 0 && y2 <= LV_VER_RES else { return }

        // Update the internal framebuffer with the new pixel values.
        let c = lv_color_to32(color)
        for y in y1...y2 {
            for x in x1...x2 {
                frameBuffer[Int(LV_HOR_RES * y + x)] = c
            }
        }
        frameDirty = true
    }

    fileprivate func onDisplayMap(x1: Int32, y1: Int32, x2: Int32, y2: Int32, color: UnsafePointer<lv_color_t>) {
        guard x1 >= 0 && x1 < LV_HOR_RES else { return }
        guard x2 >= 0 && x2 < LV_HOR_RES else { return }
        guard y1 >= 0 && y1 < LV_VER_RES else { return }
        guard y2 >= 0 && y2 < LV_VER_RES else { return }

        // Update the internal framebuffer with the new pixel values.
        var c = color
        for y in y1...y2 {
            for x in x1...x2 {
                frameBuffer[Int(LV_HOR_RES * y + x)] = lv_color_to32(c.pointee)
                c = c.successor()
            }
        }
        frameDirty = true
    }

    fileprivate func onMouseRead(data: UnsafeMutablePointer<lv_indev_data_t>) -> Bool {
        data.pointee.point.x = lastMouseX
        data.pointee.point.y = lastMouseY
        data.pointee.state = mouseDown ? lv_indev_state_t(LV_INDEV_STATE_PR) : lv_indev_state_t(LV_INDEV_STATE_REL)
        return false
    }

    fileprivate func onMemoryMonitor() {
        var mon = lv_mem_monitor_t()
        lv_mem_monitor(&mon)
        NSLog("used: %6d (%3d%%), frag: %3d%%, biggest free: %6d", mon.total_size - mon.free_size, mon.used_pct, mon.frag_pct, mon.free_biggest_size)
    }
}

extension Simulator: MTKViewDelegate {
    func mtkView(_ view: MTKView, drawableSizeWillChange size: CGSize) {
    }

    func draw(in view: MTKView) {
        guard let drawable = view.currentDrawable else { return }
        guard let descriptor = view.currentRenderPassDescriptor else { return }
        guard let commandBuffer = commandQueue.makeCommandBuffer() else { return }
        guard let commandEncoder = commandBuffer.makeRenderCommandEncoder(descriptor: descriptor) else { return }
        let texture = textureCache[currentTexture]
        commandEncoder.setRenderPipelineState(pipelineState)
        commandEncoder.setVertexBuffer(vertexBuffer, offset: 0, index: 0)
        commandEncoder.setVertexBuffer(texCoordBuffer, offset: 0, index: 1)
        commandEncoder.setFragmentTexture(texture, index: 0)
        commandEncoder.drawPrimitives(type: .triangleStrip, vertexStart: 0, vertexCount: 4)
        commandEncoder.endEncoding()
        commandBuffer.present(drawable)
        commandBuffer.commit()
    }
}

// Metal configuration.

private func createSystemDevice() -> MTLDevice {
    guard let device = MTLCreateSystemDefaultDevice() else {
        preconditionFailure("Metal is not supported on this device")
    }
    return device
}

private func createCommandQueue(device: MTLDevice) -> MTLCommandQueue {
    guard let commandQueue = device.makeCommandQueue() else {
        preconditionFailure("Failed to create command queue")
    }
    return commandQueue
}

private func createPipelineState(device: MTLDevice) -> MTLRenderPipelineState {
    // Load the vertex and shader functions.
    guard let library = device.makeDefaultLibrary() else {
        preconditionFailure("Failed to create default library")
    }
    guard let vertexFunction = library.makeFunction(name: "passthrough_vertex") else {
        preconditionFailure("Failed to create vertex function")
    }
    guard let fragmentFunction = library.makeFunction(name: "passthrough_fragment") else {
        preconditionFailure("Failed to create fragment function")
    }

    // Prepare the render pipeline descriptor.
    let pipelineDescriptor = MTLRenderPipelineDescriptor()
    pipelineDescriptor.label = "RenderPipeline"
    pipelineDescriptor.vertexFunction = vertexFunction
    pipelineDescriptor.fragmentFunction = fragmentFunction
    pipelineDescriptor.colorAttachments[0].pixelFormat = .bgra8Unorm_srgb

    // Return the render pipeline state.
    do {
        let pipelineState = try device.makeRenderPipelineState(descriptor: pipelineDescriptor)
        return pipelineState
    } catch {
        preconditionFailure("Failed to create pipeline state: \(error)")
    }
}

private func createVertexBuffer(device: MTLDevice) -> MTLBuffer {
    let vertices: [Float] = [
        -1.0, -1.0, 0.0, 1.0,
         1.0, -1.0, 0.0, 1.0,
        -1.0,  1.0, 0.0, 1.0,
         1.0,  1.0, 0.0, 1.0
    ]
    guard let vertexBuffer = device.makeBuffer(bytes: vertices, length: vertices.count * MemoryLayout<Float>.stride, options: []) else {
        preconditionFailure("Failed to create vertex buffer")
    }
    return vertexBuffer
}

private func createTexCoordBuffer(device: MTLDevice) -> MTLBuffer {
    let coords: [Float] = [
        0.0, 1.0,
        1.0, 1.0,
        0.0, 0.0,
        1.0, 0.0
    ]
    guard let texCoordBuffer = device.makeBuffer(bytes: coords, length: coords.count * MemoryLayout<Float>.stride, options: []) else {
        preconditionFailure("Failed to create texture coordinate buffer")
    }
    return texCoordBuffer
}

private func createTextureCache(device: MTLDevice) -> [MTLTexture] {
    let textureDescriptor = MTLTextureDescriptor()
    textureDescriptor.pixelFormat = .bgra8Unorm_srgb
    textureDescriptor.width = Int(LV_HOR_RES)
    textureDescriptor.height = Int(LV_VER_RES)

    var textureCache = [MTLTexture]()
    for _ in 0..<3 {
        guard let texture = device.makeTexture(descriptor: textureDescriptor) else {
            preconditionFailure("Failed to create texture")
        }
        textureCache.append(texture)
    }
    return textureCache
}

// Embedded callbacks.

@_cdecl("drivers_is_button_pressed")
func drivers_is_button_pressed() -> Bool {
    return Simulator.sharedInstance.onIsButtonPressed()
}

@_cdecl("drivers_set_led")
func drivers_set_led(which: LEDs, on: Bool) {
    Simulator.sharedInstance.onSetLED(which: which, on: on)
}

// LittlevGL callbacks.

private func displayFlush(x1: Int32, y1: Int32, x2: Int32, y2: Int32, color: UnsafePointer<lv_color_t>?) {
    guard let color = color else { return }
    Simulator.sharedInstance.onDisplayFlush(x1: x1, y1: y1, x2: x2, y2: y2, color: color)
}

private func displayFill(x1: Int32, y1: Int32, x2: Int32, y2: Int32, color: lv_color_t) {
    Simulator.sharedInstance.onDisplayFill(x1: x1, y1: y1, x2: x2, y2: y2, color: color)
}

private func displayMap(x1: Int32, y1: Int32, x2: Int32, y2: Int32, color: UnsafePointer<lv_color_t>?) {
    guard let color = color else { return }
    Simulator.sharedInstance.onDisplayMap(x1: x1, y1: y1, x2: x2, y2: y2, color: color)
}

private func mouseRead(data: UnsafeMutablePointer<lv_indev_data_t>?) -> Bool {
    guard let data = data else { return false }
    return Simulator.sharedInstance.onMouseRead(data: data)
}

private func memoryMonitor(param: UnsafeMutableRawPointer?) -> Void {
    Simulator.sharedInstance.onMemoryMonitor()
}
