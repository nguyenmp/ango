extern crate uuid;

pub struct Key {
	name: String,
	value: String,
}

pub struct Namespace {
	pub name: String,
	pub keys: Vec<Key>,
}

pub struct Vault {
	pub namespaces: Vec<Namespace>,
}

impl Vault {
	fn new() -> Vault {
		Vault {
			namespaces: vec![],
		}
	}
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub struct Version(pub u64);

pub struct Payload {
	version: Version,
}

impl Payload {
	pub fn new() -> Payload {
		Payload{version: Version(0)}
	}

	pub fn version(self: &Payload) -> Version {
		self.version
	}
}
