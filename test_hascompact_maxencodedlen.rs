use parity_scale_codec::{Encode, MaxEncodedLen, HasCompact};

// Custom type that implements HasCompact
#[derive(Clone, Copy)]
struct Balance(u128);

impl From<Balance> for parity_scale_codec::Compact<Balance> {
    fn from(b: Balance) -> Self {
        parity_scale_codec::Compact(b)
    }
}

impl From<parity_scale_codec::Compact<Balance>> for Balance {
    fn from(c: parity_scale_codec::Compact<Balance>) -> Self {
        c.0
    }
}

// Make Balance implement HasCompact by implementing the required traits
impl parity_scale_codec::CompactAs for Balance {
    type As = u128;

    fn encode_as(&self) -> &Self::As {
        &self.0
    }

    fn decode_from(v: Self::As) -> Result<Self, parity_scale_codec::Error> {
        Ok(Balance(v))
    }
}

impl parity_scale_codec::Encode for Balance {
    fn encode_to<T: parity_scale_codec::Output + ?Sized>(&self, dest: &mut T) {
        self.0.encode_to(dest)
    }
}

// This should work but doesn't
#[derive(Encode, MaxEncodedLen)]
struct Account {
    #[codec(compact)]
    balance: Balance,
}

fn main() {
    println!("Testing HasCompact with MaxEncodedLen");
}