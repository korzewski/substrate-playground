#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{codec::{Encode, Decode}, decl_error, decl_event, decl_module, decl_storage, traits::Randomness, ensure};
use frame_system::{self as system, ensure_signed};
use sp_std::{vec::Vec};
use sp_core::{H256};

pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

	type RandomnessSource: Randomness<H256>;
}

type KittyIdType = u128;
type KittyPriceType = u128;

decl_storage! {
	trait Store for Module<T: Trait> as KittiesModule {
		NextKittyId get(fn next_kitty_id): KittyIdType;
		Kitties get(fn kitties): map hasher(blake2_128_concat) KittyIdType => Kitty<T::AccountId>;

		KittiesForSale get(fn kitties_for_sale): Vec<KittyIdType>;

		UserData get(fn user_data): map hasher(blake2_128_concat) T::AccountId => User<T::AccountId>;

		Nonce get(fn nonce): u32;
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

			let kitty_id = Self::generate_kitty_id();
			let kitty_dna = Self::generate_random();
			let kitty = Kitty::new(kitty_id, account_id.clone(), kitty_dna);

			Kitties::<T>::insert(&kitty_id, &kitty);

			let mut user = Self::user_data(&account_id);
			user.add_kitty(kitty.clone());

			UserData::<T>::insert(&account_id, &user);

			Self::deposit_event(RawEvent::KittyCreated(account_id.clone(), kitty));
			Self::deposit_event(RawEvent::UserUpdated(account_id, user));
		}

		#[weight = 10_000]
		fn kitty_for_sale(origin, kitty_id: KittyIdType, price: KittyPriceType) {
			let account_id = ensure_signed(origin)?;

			let mut kitties_for_sale = Self::kitties_for_sale();

			match kitties_for_sale.binary_search(&kitty_id) {
				Ok(_) => {
					// Err(Error::<T>::KittyAlreadyForSale); ??? it's not working
					ensure!(false, Error::<T>::KittyAlreadyForSale);
				},
				Err(index) => {
					kitties_for_sale.insert(index, kitty_id);
				},
			}
			
			let mut kitty = Kitties::<T>::get(kitty_id);
			
			ensure!(kitty.owner_id == account_id, Error::<T>::NotKittyOwner);

			kitty.set_price(price);

			Kitties::<T>::insert(&kitty_id, &kitty);
			KittiesForSale::put(kitties_for_sale);

			Self::deposit_event(RawEvent::KittyForSale(account_id, kitty));
		}
	}
}

impl<T: Trait> Module<T> {
	fn encode_and_update_seed() -> Vec<u8> {
		let nonce = Self::nonce();
		Nonce::put(nonce.wrapping_add(1));
		nonce.encode()
	}

	fn generate_kitty_id() -> KittyIdType {
		let next_kitty_id = Self::next_kitty_id();
		// TODO - ERROR HANDLING
		let next_kitty_id = next_kitty_id.checked_add(1).expect("next_kitty_id is out of scope");
		NextKittyId::put(next_kitty_id);

		next_kitty_id
	}

	fn generate_random() -> H256 {
		let subject = Self::encode_and_update_seed();
		T::RandomnessSource::random(&subject)
	}
}

decl_event! {
	pub enum Event<T>
	where
		<T as system::Trait>::AccountId,
	{
		KittyCreated(AccountId, Kitty<AccountId>),
		KittyForSale(AccountId, Kitty<AccountId>),
		UserUpdated(AccountId, User<AccountId>),
	}
}

decl_error! {
	pub enum Error for Module<T: Trait> {
		KittyAlreadyForSale,
		NotKittyOwner,
	}
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Default, Debug)]
pub struct Kitty<AccountId> {
	id: KittyIdType,
	owner_id: AccountId,
	dna: H256,
	price: KittyPriceType,
}

impl<AccountId> Kitty<AccountId> {
	pub fn new(id: KittyIdType, owner_id: AccountId, dna: H256) -> Kitty<AccountId> {
		Kitty {
			id,
			owner_id,
			dna,
			price: 0,
		}
	}

	pub fn set_price(&mut self, price: KittyPriceType) {
		self.price = price;
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