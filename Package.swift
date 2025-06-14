// swift-tools-version: 6.2

import PackageDescription

let package = Package(
    name: "advent",
    targets: [
        .executableTarget(
            name: "advent",
            resources: [.copy("Inputs")]
        ),
        .testTarget(
            name: "adventTests",
            dependencies: ["advent"]
        ),
    ]
)
