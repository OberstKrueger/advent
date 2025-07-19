import Foundation

func readFile(_ filename: String) -> String {
    guard let url = Bundle.module.url(forResource: filename, withExtension: "txt", subdirectory: "Inputs") else {
        fatalError("Could not find input file: \(filename).txt")
    }
    
    do {
        return try String(contentsOf: url, encoding: .utf8)
    } catch {
        fatalError("Could not read input: \(error)")
    }
}

print("2015.01 => \(solution_2015_01(readFile("2015_01")))")
print("2015.02 => \(solution_2015_02(readFile("2015_02")))")
print("2015.03 => \(solution_2015_03(readFile("2015_03")))")

print("\n2024.01 => \(solution_2024_01(readFile("2024_01")))")
print("2024.02 => \(solution_2024_02(readFile("2024_02")))")
