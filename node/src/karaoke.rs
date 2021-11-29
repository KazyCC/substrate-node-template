//use codec::{Decode, Encode};
use futures::{
	channel::oneshot,
	future,
	future::{Future, FutureExt},
	select,
};
use log::{debug, error, info, trace, warn};
use sc_block_builder::{BlockBuilderApi, BlockBuilderProvider};
use sc_client_api::backend;
use sc_telemetry::{telemetry, TelemetryHandle, CONSENSUS_INFO};
use sc_transaction_pool_api::{InPoolTransaction, TransactionPool};
use sp_api::{ApiExt, ProvideRuntimeApi};
use sp_blockchain::{ApplyExtrinsicFailed::Validity, Error::ApplyExtrinsicFailed, HeaderBackend};
use sp_consensus::{
	evaluation, DisableProofRecording, EnableProofRecording, ProofRecording, Proposal,
};
use sp_core::traits::SpawnNamed;
use sp_inherents::InherentData;
use sp_runtime::{
	generic::BlockId,
	traits::{BlakeTwo256, Block as BlockT, Hash as HashT, Header as HeaderT},
	Digest, Percent, SaturatedConversion,
};
use std::{marker::PhantomData, pin::Pin, sync::Arc, time};

// use prometheus_endpoint::Registry as PrometheusRegistry;
// use sc_proposer_metrics::MetricsLink as PrometheusMetrics;


use sc_basic_authorship::{Proposer, ProposerFactory};

// Distributed Karaoke
pub struct KaraokeProposerFactory<A, B, C, PR> {
	real_proposer_factory : ProposerFactory<A, B, C, PR>,
	song: String,
	_phantom: PhantomData<(A, B, C, PR)>,
}

impl<A, B, C, PR> KaraokeProposerFactory<A, B, C, PR> {
	/// Create a new proposer factory.
	///
	/// Proof recording will be disabled when using proposers built by this instance to build
	/// blocks.
	pub fn new(
		real_proposer_factory : ProposerFactory<A, B, C, PR>,
		song: String,
	) -> Self {
		KaraokeProposerFactory {
			real_proposer_factory,
			song,
			_phantom: PhantomData,
		}
	}
}

// impl<B, Block, C, A, PR> KaraokeProposerFactory<A, B, C, PR>
// where
// 	A: TransactionPool<Block = Block> + 'static,
// 	B: backend::Backend<Block> + Send + Sync + 'static,
// 	Block: BlockT,
// 	C: BlockBuilderProvider<B, Block, C>
// 		+ HeaderBackend<Block>
// 		+ ProvideRuntimeApi<Block>
// 		+ Send
// 		+ Sync
// 		+ 'static,
// 	C::Api:
// 		ApiExt<Block, StateBackend = backend::StateBackendFor<B, Block>> + BlockBuilderApi<Block>,
// {
// 	fn init_with_now(
// 		&mut self,
// 		parent_header: &<Block as BlockT>::Header,
// 		now: Box<dyn Fn() -> time::Instant + Send + Sync>,
// 	) -> Proposer<B, Block, C, A, PR> {
// 		self.real_proposer_factory.init_with_now(parent_header,now)
// 	}
// }

impl<A, B, Block, C, PR> sp_consensus::Environment<Block> for KaraokeProposerFactory<A, B, C, PR>
where
	A: TransactionPool<Block = Block> + 'static,
	B: backend::Backend<Block> + Send + Sync + 'static,
	Block: BlockT,
	C: BlockBuilderProvider<B, Block, C>
		+ HeaderBackend<Block>
		+ ProvideRuntimeApi<Block>
		+ Send
		+ Sync
		+ 'static,
	C::Api:
		ApiExt<Block, StateBackend = backend::StateBackendFor<B, Block>> + BlockBuilderApi<Block>,
	PR: ProofRecording,
{
	// type CreateProposer = future::Ready<Result<Self::Proposer, Self::Error>>;
	// type Proposer = Proposer<B, Block, C, A, PR>;
	// type Error = sp_blockchain::Error;

	fn init(&mut self, parent_header: &<Block as BlockT>::Header) -> Self::CreateProposer {
		future::ready(Ok(self.real_proposer_factory.init_with_now(parent_header, Box::new(time::Instant::now))))
	}
}


// pub struct KaraokeProposer<B, Block, C, A, PR> {
// 	real_proposer: Proposer<B, Block, C, A, PR>,
// 	song_line: String,
// }

// impl<A, B, Block, C, PR> sp_consensus::Proposer<Block> for KaraokeProposer<B, Block, C, A, PR>
// {
// 	fn propose(
// 		self,
// 		inherent_data: InherentData,
// 		inherent_digests: Digest,
// 		max_duration: time::Duration,
// 		block_size_limit: Option<usize>,
// 	) -> Self::Proposal {
// 		// modify inherent data
//         self.real_proposer.propose(inherent_data,inherent_digests,max_duration,block_size_limit)
// 	}	
// }


// !Distributed Karaoke