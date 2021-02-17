#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{codec::{Encode, Decode}, decl_error, decl_event, decl_module, decl_storage};
use frame_system::{self as system, ensure_signed};
use sp_std::{vec::Vec};

pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_event! {
	pub enum Event<T>
	where
		<T as system::Trait>::AccountId,
	{
		KittyCreated(AccountId, User<AccountId>),
	}
}

decl_error! {
	pub enum Error for Module<T: Trait> {
		
	}
}

decl_storage! {
	trait Store for Module<T: Trait> as KittiesModule {
		UserData get(fn user_data): map hasher(blake2_128_concat) T::AccountId => User<T::AccountId>;
		NextKittyId get(fn next_kitty_id): u128;
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
			let account_id = ensure_signed(origin)?;

			let next_kitty_id = Self::next_kitty_id();
			let kitty = Kitty::new(next_kitty_id, account_id.clone());

			let next_kitty_id = next_kitty_id.checked_add(1).expect("next_kitty_id is out of scope");
			NextKittyId::put(next_kitty_id);

			let mut user = Self::user_data(account_id.clone());

			user.add_kitty(kitty);

			UserData::<T>::insert(&account_id, &user);

			Self::deposit_event(RawEvent::KittyCreated(account_id, user));
		}
	}
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Default, Debug)]
pub struct Kitty<AccountId> {
	id: u128,
	owner_id: AccountId,
	dna: i128,
}

impl<AccountId> Kitty<AccountId> {
	pub fn new(id: u128, owner_id: AccountId) -> Kitty<AccountId> {
		let dna = 128;

		Kitty {
			id,
			owner_id,
			dna,
		}
	}
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Default, Debug)]
pub struct User<AccountId> {
	kitties: Vec<Kitty<AccountId>>,
}

impl<AccountId> User<AccountId> {
	pub fn add_kitty(&mut self, kitty: Kitty<AccountId>) {
		self.kitties.push(kitty);
	}
}