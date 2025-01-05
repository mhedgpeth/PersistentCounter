import SwiftUI

@main
struct SimpleCounterApp: App {
    var core: Core
    
    init() {
        core = Core()
    }
    
    var body: some Scene {
        WindowGroup {
            ContentView(core: core)
        }
    }
}
