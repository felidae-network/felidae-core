use crate::{types::*, Config, Error};
use frame_support::{dispatch::DispatchResult, inherent::Vec};

/// Traits of verification process
pub trait VerificationProcess<C: Config> {
	/// Creates a DID verification request
	fn create_verification_request(
		who: &C::AccountId,
		list_of_documents: Vec<u8>,
	) -> DispatchResult;

	/// alot the new tasks to eligible verifiers
	/// in round-robin for now
	/// and allow ack & vp-submit
	fn allot_verification_task(
		current_block: C::BlockNumber,
		verifiers: Vec<C::AccountId>,
		verification_reuests: Vec<(&C::AccountId, u16)>,
	) -> Result<(), Error<C>>;

	/// Acknowledge the acceptence with confidence score
	fn ack_verification_task(
		_who: &C::AccountId,
		consumer_account_id: &C::AccountId,
		confidence_score: u8,
	) -> DispatchResult;

	/// Check if the verifier has been allotted the task
	fn is_verifier_allowed_ack(
		_who: &C::AccountId,
		consumer_account_id: &C::AccountId,
	) -> DispatchResult;

	/// Verifier submits the verification parameter
	fn submit_verification_parameter(
		_who: &C::AccountId,
		consumer_account_id: &C::AccountId,
		verification_parameters: sp_core::H256,
	) -> DispatchResult;

	/// check if verifier accepted the task and can submit verification parameter
	fn is_verifier_allowed_vp(
		_who: &C::AccountId,
		consumer_account_id: &C::AccountId,
	) -> DispatchResult;

	/// Reveal the verificatoin parameter
	fn reveal_verification_parameter(
		_who: &C::AccountId,
		consumer_account_id: &C::AccountId,
		clear_parameters: Vec<u8>,
		secret: Vec<u8>,
	) -> DispatchResult;

	/// Check if verifier submitted verification parameter and can reveal now
	fn is_verifier_allowed_reveal(
		_who: &C::AccountId,
		consumer_account_id: &C::AccountId,
	) -> DispatchResult;

	/// Check if wait time for submit_vp is over. re-allot to
	/// more verifiers if wait is over and not completely fulfilled
	/// This takes list of verification request ids to act on
	fn act_on_wait_over_for_submit_vp(
		parameters: ProtocolParameterValues,
		list_verification_req: Vec<&C::AccountId>,
	) -> Result<(), Error<C>>;

	/// Start the reveal stage
	fn start_reveal(
		current_block: C::BlockNumber,
		list_verification_req: Vec<&C::AccountId>,
	) -> Result<(), Error<C>>;

	/// eval the submissions to get the result: accept/reject/can't decide
	fn eval(
		current_block: C::BlockNumber,
		list_verification_req: Vec<&C::AccountId>,
	) -> Result<Vec<(C::AccountId, VerifierUpdateData)>, Error<C>>;
}
