use prost::{DecodeError, Message};
use proton::test_jigs::test_jig_base::TestJigType;
use proton::test_jigs::TestJigBase;

// Create a can test
fn can_test(mac_address: String, current: f32) -> TestJigType {
    use proton::test_jigs::AppTestCanWifi;
    let mut ct = AppTestCanWifi::default();
    ct.mac_address = mac_address;
    ct.current_5v_on = current;
    TestJigType::AppTestCanWifi(ct)
}

fn cell_position_test(serial: String, cell_serial: String, position: i32) -> TestJigType {
    use proton::test_jigs::AppTestCellPositions;
    let mut cp = AppTestCellPositions::default();
    cp.battery_serial_number = serial;
    cp.cell_serial_number = cell_serial;
    cp.position = position;
    TestJigType::AppTestCellPositions(cp)
}

fn create_test_result(jig_id: String, test_result: bool, rep: TestJigType) -> TestJigBase {
    let mut t = TestJigBase::default();
    t.jig_id = jig_id;
    t.test_result = test_result;
    t.test_jig_type = Some(rep);
    t
}

fn serialise(t: TestJigBase) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(t.encoded_len());
    t.encode(&mut buf).unwrap();
    buf
}

fn deserialise(buf: Vec<u8>) -> Result<TestJigBase, DecodeError> {
    use std::io::Cursor;
    TestJigBase::decode(&mut Cursor::new(buf))
}

fn analyse(test: TestJigBase) {
    println!("Test jig {} test state: {}", test.jig_id, test.test_result);
    // Behold the power of pattern matching. Having to scope these types is a bit of a pain but
    // proto has chosen to make the enumeration names the same as the struct names so instant
    // namespace clash
    match test.test_jig_type {
        Some(TestJigType::AppTestCanWifi(_)) => println!("This is a can wifi test test"),
        Some(TestJigType::AppTestCellPositions(_)) => println!("This is a cell position test"),
        None => println!("No test information present"),
        _ => println!("Unknown test jig type"),
    }
}

fn main() {
    let can = can_test("12/23/34".to_string(), 12.1);
    let _cell = cell_position_test("P027-000126".to_string(), "123456".to_string(), 12);
    let test = create_test_result("jig12".to_string(), true, can);
    let buf = serialise(test);
    let res = deserialise(buf);
    analyse(res.unwrap())
}


// Example test case
#[test]
fn test_serdes() {
    let can = can_test("12/23/34".to_string(), 12.1);
    let _cell = cell_position_test("P027-000126".to_string(), "123456".to_string(), 12);
    let test = create_test_result("jig12".to_string(), true, can);
    let buf = serialise(test);
    let res = deserialise(buf).unwrap();
    assert_eq!(res.jig_id, "jig12".to_string());
    assert_eq!(res.test_result, true)
}
