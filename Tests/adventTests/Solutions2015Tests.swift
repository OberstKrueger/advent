import XCTest
@testable import advent

final class Solutions2015Tests: XCTestCase {
    func test_2015_01() {
        XCTAssertEqual(solution_2015_01("(())").first, 0)
        XCTAssertEqual(solution_2015_01("()()").first, 0)
        XCTAssertEqual(solution_2015_01("(((").first, 3)
        XCTAssertEqual(solution_2015_01("(()(()(").first, 3)
        XCTAssertEqual(solution_2015_01("))(((((").first, 3)
        XCTAssertEqual(solution_2015_01("())").first, -1)
        XCTAssertEqual(solution_2015_01("))(").first, -1)
        XCTAssertEqual(solution_2015_01(")))").first, -3)
        XCTAssertEqual(solution_2015_01(")())())").first, -3)
        XCTAssertEqual(solution_2015_01(")").second, 1)
        XCTAssertEqual(solution_2015_01("()())").second, 5)
    }
    
    func test_2015_02() {
        XCTAssertEqual(solution_2015_02("2x3x4").first, 58)
        XCTAssertEqual(solution_2015_02("1x1x10").first, 43)
        XCTAssertEqual(solution_2015_02("2x3x4").second, 34)
        XCTAssertEqual(solution_2015_02("1x1x10").second, 14)
    }
    
    func test_2015_03() {
        XCTAssertEqual(solution_2015_03(">").first, 2)
        XCTAssertEqual(solution_2015_03("^>v<").first, 4)
        XCTAssertEqual(solution_2015_03("^v^v^v^v^v").first, 2)
        XCTAssertEqual(solution_2015_03("^v").second, 3)
        XCTAssertEqual(solution_2015_03("^>v<").second, 3)
        XCTAssertEqual(solution_2015_03("^v^v^v^v^v").second, 11)
    }
}
