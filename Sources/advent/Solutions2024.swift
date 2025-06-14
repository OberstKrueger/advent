import Foundation

func solution_2024_01(_ input: String) -> Answer {
    var answer = Answer()
    var list1: [Int] = []
    var list2: [Int] = []
    var hash: [Int: Int] = [:]

    for line in input.components(separatedBy: .newlines) where line.isEmpty == false {
        let values: [Int] = line.split(separator: " ")
            .compactMap { Int($0.trimmingCharacters(in: .whitespaces)) }
        
        if values.count == 2 {
            list1.append(values[0])
            list2.append(values[1])
            hash[values[1], default: 0] += 1
        }
    }
    
    list1.sort()
    list2.sort()
    
    for (index, number) in list1.enumerated() {
        let check = hash[number] ?? 0
        
        answer.first += abs(number - list2[index])
        answer.second += number * check
    }
    
    return answer
}

func solution_2024_02(_ input: String) -> Answer {
    var answer = Answer()
    
    for line in input.components(separatedBy: .newlines) where line.isEmpty == false {
        var check = false
        var reports: [[Int]] = []
        
        reports.append(
            line.split(separator: " ")
                .compactMap { Int($0.trimmingCharacters(in: .whitespaces)) }
        )
        
        for index in 0..<reports[0].count {
            var copy = reports[0]
            copy.remove(at: index)
            reports.append(copy)
        }
        
        for (index, report) in reports.enumerated() {
            var errors = 0
            
            let ascending = report.sorted()
            let descending = report.sorted(by: >)
            
            if report == ascending || report == descending {
                for i in 0..<report.count - 1 {
                    let difference = abs(report[i + 1] - report[i])
                    
                    if difference < 1 || difference > 3 {
                        errors += 1
                    }
                }
            } else {
                errors += 1
            }
            
            switch (index, errors) {
            case (0, 0):
                answer.first += 1
            case (1..., 0):
                check = true
            default:
                break
            }
        }
        
        if check {
            answer.second += 1
        }
    }
    
    return answer
}

