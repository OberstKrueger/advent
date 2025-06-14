import XCTest
@testable import advent

final class Solutions2024Tests: XCTestCase {
    func test_2024_01() {
        let input = """
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
        """
        
        XCTAssertEqual(solution_2024_01(input).first, 11)
        XCTAssertEqual(solution_2024_01(input).second, 31)
    }
    
    func test_2024_02() {
        XCTAssertEqual(solution_2024_02("7 6 4 2 1").first, 1)
        XCTAssertEqual(solution_2024_02("1 2 7 8 9").first, 0)
        XCTAssertEqual(solution_2024_02("9 7 6 2 1").first, 0)
        XCTAssertEqual(solution_2024_02("1 3 2 4 5").first, 0)
        XCTAssertEqual(solution_2024_02("1 3 6 7 9").first, 1)
        
        XCTAssertEqual(solution_2024_02("7 6 4 2 1").second, 1)
        XCTAssertEqual(solution_2024_02("1 2 7 8 9").second, 0)
        XCTAssertEqual(solution_2024_02("9 7 6 2 1").second, 0)
        XCTAssertEqual(solution_2024_02("1 3 2 4 5").second, 1)
        XCTAssertEqual(solution_2024_02("1 3 6 7 9").second, 1)
    }
}
