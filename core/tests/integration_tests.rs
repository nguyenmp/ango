extern crate ango;

#[test]
fn create_payload() {
	let payload_1 = ango::Payload::new();
	let payload_2 = ango::Payload::new();
	assert_eq!(payload_1.version(), ango::Version(0));
	assert_eq!(payload_1.version(), payload_2.version());
}
