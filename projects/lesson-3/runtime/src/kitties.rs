use support::{decl_module, decl_storage, StorageValue, StorageMap, ensure};
use codec::{Encode, Decode};
use runtime_io::blake2_128;
use system::ensure_signed;

pub trait Trait: system::Trait {
}

#[derive(Encode, Decode, Default)]
pub struct Kitty(pub [u8; 16]);

decl_storage! {
	trait Store for Module<T: Trait> as Kitties {
		/// Stores all the kitties, key is the kitty id / index
		pub Kitties get(kitty): map u32 => Kitty;
		/// Stores the total number of kitties. i.e. the next kitty index
		pub KittiesCount get(kitties_count): u32;
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		/// Create a new kitty
		pub fn create(origin) {
			let sender = ensure_signed(origin)?;
			let count = Self::kitties_count();
			if count == u32::max_value() {
				return Err("Kitties count overflow");
			}
			let payload = (<system::Module<T>>::random_seed(), sender, <system::Module<T>>::extrinsic_index(), <system::Module<T>>::block_number());
			let dna = payload.using_encoded(blake2_128);
			let kitty = Kitty(dna);
			Kitties::insert(count, kitty);
			KittiesCount::put(count + 1);
		}

		/// Get a baby kitty from parent1 and parent2
        pub fn baby_kitty(origin, kittyIndex1:u32, kittyIndex2:u32) {
             let sender = ensure_signed(origin)?;
             let count = Self::kitties_count();
             if count == u32::max_value() {
                return Err("Can not make more kitty");
             }
             let newCount = count.checked_add(1).ok_or("kitty count is overflow");
             ensure!(<Kitties>::exists(kittyIndex1), "dna1 kitty don't exist");
             ensure!(<Kitties>::exists(kittyIndex2), "dna2 kitty don't exist");

             let parentKitty1 = Self::Kitty(kittyIndex1);
             let parentKitty2 = Self::Kitty(kittyIndex1);
             let rand = (<system::Module<T>>::random_seed(), sender, <system::Module<T>>::extrinsic_index(), <system::Module<T>>::block_number());
             let newDna = (rand, kittyIndex2, kittyIndex2).using_encoded(blake2_128);
             let babyKitty = Kitty(newDna);
             Kitties::insert(count, babyKitty);
             KittiesCount::put(count + 1);

        }
	}
}
