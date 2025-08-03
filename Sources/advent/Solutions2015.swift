import CommonCrypto
import Foundation

func solution_2015_01(_ input: String) -> Answer {
    var answer = Answer()
    var found: Bool = false

    for (index, character) in input.enumerated() {
        switch character {
        case "(":
            answer.first += 1
        case ")":
            answer.first -= 1
        default:
            break
        }

        if found == false && answer.first == -1 {
            answer.second = index + 1
            found = true
        }
    }

    return answer
}

func solution_2015_02(_ input: String) -> Answer {
    var answer = Answer()

    for line in input.components(separatedBy: .newlines) where line.isEmpty == false {
        var dimensions: [Int] = line.split(separator: "x")
            .compactMap { Int($0) }

        if dimensions.count == 3 {
            dimensions.sort()

            let side1 = dimensions[0] * dimensions[1]
            let side2 = dimensions[0] * dimensions[2]
            let side3 = dimensions[1] * dimensions[2]

            let total = 2 * (side1 + side2 + side3)

            let ribbon = 2 * dimensions[0] + 2 * dimensions[1] + dimensions.reduce(1, *)

            answer.first += side1 + total
            answer.second += ribbon
        }
    }

    return answer
}

func solution_2015_03(_ input: String) -> Answer {
    struct Coordinate: Hashable {
        var x: Int = 0
        var y: Int = 0
    }

    var answer = Answer()
    var houses_1: Set<Coordinate> = []
    var houses_2: Set<Coordinate> = []
    var robot: Coordinate = Coordinate()
    var santa_1: Coordinate = Coordinate()
    var santa_2: Coordinate = Coordinate()

    houses_1.insert(santa_1)
    houses_2.insert(santa_2)

    // Single loop that handles both parts 1 and 2.
    for (index, direction) in input.enumerated() {
        switch (direction, index.isMultiple(of: 2)) {
        case ("^", true):
            santa_1.y += 1
            santa_2.y += 1
        case ("^", false):
            robot.y += 1
            santa_1.y += 1
        case (">", true):
            santa_1.x += 1
            santa_2.x += 1
        case (">", false):
            robot.x += 1
            santa_1.x += 1
        case ("v", true):
            santa_1.y -= 1
            santa_2.y -= 1
        case ("v", false):
            robot.y -= 1
            santa_1.y -= 1
        case ("<", true):
            santa_1.x -= 1
            santa_2.x -= 1
        case ("<", false):
            robot.x -= 1
            santa_1.x -= 1
        default:
            break   // This should never trigger given proper input. But not going to error check.
        }
        houses_1.insert(santa_1)
        houses_2.insert(index.isMultiple(of: 2) ? santa_2 : robot)
    }

    answer.first = houses_1.count
    answer.second = houses_2.count

    return answer
}

func solution_2015_04(_ input: String) -> Answer {
    // Using CommonCrypto as CryptoKit is slow.
    // Sssslllloooowwww....
    // Probably some more that could be done here...

    let key = input.trimmingCharacters(in: .whitespacesAndNewlines)
    var answer = Answer()
    var hash = [UInt8](repeating: 0, count: Int(CC_MD5_DIGEST_LENGTH))

    for number: Int in 1...Int.max {
        let combined = key + String(number)
        let inputData = Data(combined.utf8)
        
        _ = inputData.withUnsafeBytes { bytes in
            CC_MD5(bytes.baseAddress, CC_LONG(inputData.count), &hash)
        }

        if answer.first == 0 && hash[0] == 0 && hash[1] == 0 && hash[2] < 0x10 {
            answer.first = number
        }
        if answer.second == 0 && hash[0] == 0 && hash[1] == 0 && hash[2] == 0 {
            answer.second = number
        }

        if answer.first != 0 && answer.second != 0 {
            break
        }
    }

    return answer
}

