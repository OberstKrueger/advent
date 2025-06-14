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
