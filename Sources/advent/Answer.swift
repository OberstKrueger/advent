import Foundation

struct Answer {
    var first: Int = 0
    var second: Int = 0
}

extension Answer: CustomStringConvertible {
    var description: String {
        "\(first), \(second)"
    }
}
