use prost::{DecodeError, Message};
use proton::test_jigs::test_jig_base::TestJigType;
use proton::test_jigs::TestJigBase;

// Create a can test
fn can_test(mac_address: String, current: f32) -> TestJigType {
    // Instantiate a serialisable instance of a CAN WiFi test result. The return type
    // is actually an enum (TestJigType). The protobuf union types are just enums in Rust.
    use proton::test_jigs::AppTestCanWifi;
    let mut ct = AppTestCanWifi::default();
    ct.mac_address = mac_address;
    ct.current_5v_on = current;
    TestJigType::AppTestCanWifi(ct)
}

fn cell_position_test(serial: String, cell_serial: String, position: i32) -> TestJigType {
    // Instantiate a serialisable instance of a cell position test
    use proton::test_jigs::AppTestCellPositions;
    let mut cp = AppTestCellPositions::default();
    cp.battery_serial_number = serial;
    cp.cell_serial_number = cell_serial;
    cp.position = position;
    TestJigType::AppTestCellPositions(cp)
}

fn create_test_result(jig_id: String, test_result: bool, rep: TestJigType) -> TestJigBase {
    // Build a final test jig output, adding the individual test result as a union type
    let mut t = TestJigBase::default();
    t.jig_id = jig_id;
    t.test_result = test_result;
    t.test_jig_type = Some(rep);
    t
}

fn serialise(t: TestJigBase) -> Vec<u8> {
    // Turn the rust structure into a protobuf encoded byte array
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
    // Fake analyser for a test result mostly to demonstrate how pattern matching works on
    // the enum. You can make magic here.
    println!("Test jig {} test state: {}", test.jig_id, test.test_result);
    // Behold the power of pattern matching.
    match test.test_jig_type {
        Some(TestJigType::AppTestCanWifi(_)) => println!("This is a can wifi test test"),
        Some(TestJigType::AppTestCellPositions(_)) => println!("This is a cell position test"),
        None => println!("No test information present"),
    }
}

fn main() {
    println!("Silly protobuf test v{}", env!("CARGO_PKG_VERSION"));
    let can = can_test("12/23/34".to_string(), 12.1);
    let _cell = cell_position_test("P027-000126".to_string(), "123456".to_string(), 12);
    let test = create_test_result("jig12".to_string(), true, can);
    let buf = serialise(test);
    let res = match deserialise(buf) {
        Ok(r) => r,
        Err(e) => {
            println!("Encountered error while decoding: {}", e);
            panic!()
        }
    };
    analyse(res )
}

// Example test case
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serdes() {
        let can = can_test("12/23/34".to_string(), 12.1);
        let test = create_test_result("jig12".to_string(), true, can);
        let buf = serialise(test);
        let res = deserialise(buf).unwrap();
        assert_eq!(res.jig_id, "jig12".to_string());
        assert_eq!(res.test_result, true);
        match res.test_jig_type.unwrap() {
            TestJigType::AppTestCanWifi(m) => assert_eq!(m.mac_address, "12/23/34".to_string()),
            _ => panic!("Failed to decode the correct type")
        }
    }
}