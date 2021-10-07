#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>
pub use pallet::*;

mod proof;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
    use frame_system::pallet_prelude::*;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    // The pallet's runtime storage items.
    // https://substrate.dev/docs/en/knowledgebase/runtime/storage
    #[pallet::storage]
    #[pallet::getter(fn something)]
    // Learn more about declaring storage items:
    // https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
    pub type Something<T> = StorageValue<_, u128>;

    /// Mock type alias while I can't get the pallet-balances
    type Balance = u128;
    type ChainID = u16;

    #[pallet::storage]
    #[pallet::getter(fn get_locked_amount)]
    pub(super) type Locks<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, (ChainID, Balance), ValueQuery>;

    // Pallets use events to inform users when important changes are made.
    // https://substrate.dev/docs/en/knowledgebase/runtime/events
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        LockedFunds(T::AccountId, Balance, ChainID),
        UnlockedFunds(T::AccountId, Balance, ChainID),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Error names should be descriptive.
        NoneValue,
        /// Errors should have helpful documentation associated with them.
        StorageOverflow,

        /// The provided proof failed validation
        InvalidProof,
    }

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// An example dispatchable that takes a singles value as a parameter, writes the value to
        /// storage and emits an event. This function must be dispatched by a signed extrinsic.
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn lock(
            origin: OriginFor<T>,
            amount: Balance,
            destination_chain_id: ChainID,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            <Locks<T>>::insert(who.clone(), (destination_chain_id, amount));

            Self::deposit_event(Event::LockedFunds(who, amount, destination_chain_id));

            Ok(())
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn unlock(
            origin: OriginFor<T>,
            proof: super::stub::Proof,
            chain_id: ChainID,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            if proof {
                // Emit an event.
                Self::deposit_event(Event::UnlockedFunds(who, 123, chain_id));
                // Actually unlock the funds in the dest chain
                Ok(())
            } else {
                Err(Error::<T>::InvalidProof.into())
            }
        }
    }
}

pub mod stub {
    pub type Proof = bool;
}
