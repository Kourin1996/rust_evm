use uint::construct_uint;

construct_uint! {
	pub struct U256(4);
}

pub type Address = [u8; 20];