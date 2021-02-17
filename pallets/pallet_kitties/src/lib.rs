#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{codec::{Encode, Decode}, decl_error, decl_event, decl_module, decl_storage};
use frame_system::{self as system, ensure_signed};
use sp_std::{vec::Vec};
use sp_runtime::{RuntimeDebug};

pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_event! {
	pub enum Event<T>
	where
		<T as system::Trait>::AccountId,
	{
		KittyCreated(AccountId, Kitty<AccountId>),
	}
}

decl_error! {
	pub enum Error for Module<T: Trait> {
		
	}
}

decl_storage! {
	trait Store for Module<T: Trait> as KittiesModule {
		Kitties get(fn kitties): map hasher(blake2_128_concat) T::AccountId => Vec<Kitty<T::AccountId>>;
	}
}

decl_module! {

	pub struct Module<T: Trait> for enum Call
	where
		origin: T::Origin 
	{
		type Error = Error<T>;

		fn deposit_event() = default;

		#[weight = 10_000]
		fn create_kitty(origin) {
			let account = ensure_signed(origin)?;

			let kitty = Kitty::new(account.clone());

			// let mut kitties = Kitties::<T>::get(&account);
			// kitties.push(kitty.clone());
			
			// Kitties::<T>::insert(&account, kitties);

			Self::deposit_event(RawEvent::KittyCreated(account, kitty));
		}
	}
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Default, RuntimeDebug)]
pub struct Kitty<AccountId> {
	id: Vec<u8>,
	owner_id: AccountId,
	dna: i128,
}

impl<AccountId> Kitty<AccountId> {
	pub fn new(owner_id: AccountId) -> Kitty<AccountId> {
		let id = Vec::from("kitty_id_example");
		let dna = 128;

		Kitty {
			id,
			owner_id,
			dna,
		}
	}
}

// struct User {
// 	id: Vec<u8>,
// 	kitties: Vec<Kitty>,
// }

// impl User {
// 	pub fn new() -> User {
// 		let id = Vec::from("user_id_example");

// 		User {
// 			id,
// 			kitties: Vec::new(),
// 		}
// 	}

// 	pub fn create_kitty(&mut self) {
// 		let kitty = Kitty::new(&self.id);

// 		self.kitties.push(kitty);
// 	}
// }