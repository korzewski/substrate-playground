#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	codec::{Encode, Decode},
	decl_error,
	decl_event,
	decl_module,
	decl_storage,
	traits::{Randomness, Currency, ExistenceRequirement},
	ensure
};
use frame_system::{self as system, ensure_signed};
use sp_std::{vec::Vec};

pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

	type RandomnessSource: Randomness<Self::Hash>;

	type Currency: Currency<Self::AccountId>;
}

type KittyIdType = u128;
type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;

decl_storage! {
	trait Store for Module<T: Trait> as KittiesModule {
		NextKittyId get(fn next_kitty_id): KittyIdType;
		Kitties get(fn kitties): map hasher(blake2_128_concat) KittyIdType => Kitty<T::AccountId, T::Hash>;

		KittiesForSale get(fn kitties_for_sale): map hasher(blake2_128_concat) KittyIdType => BalanceOf<T>;

		Users get(fn user_data): map hasher(blake2_128_concat) T::AccountId => User;

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

			let mut user = Users::<T>::get(&account_id);
			user.add_kitty(kitty_id);

			Users::<T>::insert(&account_id, &user);

			Self::deposit_event(RawEvent::KittyCreated(account_id, kitty));
		}

		#[weight = 10_000]
		fn sell_kitty(origin, kitty_id: KittyIdType, price: BalanceOf<T>) {
			let account_id = ensure_signed(origin)?;

			ensure!(!KittiesForSale::<T>::contains_key(&kitty_id), Error::<T>::KittyAlreadyForSale);

			let kitty = Kitties::<T>::get(&kitty_id);
			ensure!(kitty.owner_id == account_id, Error::<T>::NotKittyOwner);

			KittiesForSale::<T>::insert(&kitty_id, &price);

			Self::deposit_event(RawEvent::KittyForSale(account_id, kitty, price));
		}

		#[weight = 10_000]
		fn cancel_sell_kitty(origin, kitty_id: KittyIdType) {
			let account_id = ensure_signed(origin)?;

			ensure!(KittiesForSale::<T>::contains_key(&kitty_id), Error::<T>::KittyIsNotForSale);
			
			let kitty = Kitties::<T>::get(&kitty_id);
			ensure!(kitty.owner_id == account_id, Error::<T>::NotKittyOwner);

			KittiesForSale::<T>::remove(&kitty_id);

			Self::deposit_event(RawEvent::CancelKittyForSale(account_id, kitty));
		}
		
		#[weight = 10_000]
		fn buy_kitty(origin, kitty_id: KittyIdType) {
			let account_id = ensure_signed(origin)?;
			
			ensure!(KittiesForSale::<T>::contains_key(&kitty_id), Error::<T>::KittyIsNotForSale);
			
			let mut kitty = Kitties::<T>::get(&kitty_id);
			ensure!(kitty.owner_id != account_id, Error::<T>::OwnerCanNotBuyKitty);
			
			let price = KittiesForSale::<T>::get(&kitty_id);
			T::Currency::transfer(&account_id, &kitty.owner_id, price, ExistenceRequirement::KeepAlive)?;
			
			KittiesForSale::<T>::remove(&kitty_id);
			kitty.set_owner(account_id.clone());
			
			Kitties::<T>::insert(&kitty_id, &kitty);

			Self::deposit_event(RawEvent::KittyWasBought(account_id, kitty, price));
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
		let next_kitty_id = NextKittyId::get();
		let next_kitty_id = next_kitty_id.checked_add(1).expect("next_kitty_id is out of scope");
		NextKittyId::put(next_kitty_id);

		next_kitty_id
	}

	fn generate_random() -> T::Hash {
		let subject = Self::encode_and_update_seed();
		T::RandomnessSource::random(&subject)
	}
}

decl_event! {
	pub enum Event<T>
	where
		<T as system::Trait>::AccountId,
		<T as system::Trait>::Hash,
		Balance = BalanceOf<T>,
	{
		KittyCreated(AccountId, Kitty<AccountId, Hash>),
		KittyForSale(AccountId, Kitty<AccountId, Hash>, Balance),
		CancelKittyForSale(AccountId, Kitty<AccountId, Hash>),
		Transfer(AccountId, AccountId),
		KittyWasBought(AccountId, Kitty<AccountId, Hash>, Balance),
	}
}

decl_error! {
	pub enum Error for Module<T: Trait> {
		KittyAlreadyForSale,
		KittyIsNotForSale,
		NotKittyOwner,
		OwnerCanNotBuyKitty,
	}
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Default, Debug)]
pub struct Kitty<AccountId, Hash> {
	id: KittyIdType,
	owner_id: AccountId,
	dna: Hash,
}

impl<AccountId, Hash> Kitty<AccountId, Hash> {
	pub fn new(id: KittyIdType, owner_id: AccountId, dna: Hash) -> Kitty<AccountId, Hash> {
		Kitty {
			id,
			owner_id,
			dna,
		}
	}

	pub fn set_owner(&mut self, owner_id: AccountId) {
		self.owner_id = owner_id;
	}
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Default, Debug)]
pub struct User {
	kitties: Vec<KittyIdType>,
}

impl User {
	pub fn add_kitty(&mut self, kitty_id: KittyIdType) {
		self.kitties.push(kitty_id);
	}
}