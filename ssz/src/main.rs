use ssz::{Encode, Decode};
use ssz_derive::{Encode, Decode};
use alloy_primitives::{B256};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


// #[serde_as]
// #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
// struct BeaconBlock {
//     slot: u64,
//     proposer_index: u64,
//     parent_root: Option<B256>,
//     state_root: Option<B256>,
//     body_root: Option<B256>,
// }
#[serde_as]
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssz", derive(ssz_derive::Encode, ssz_derive::Decode))]
pub struct BidTrace {
    /// The slot associated with the block.
    #[serde_as(as = "DisplayFromStr")]
    pub slot: u64,
    /// The parent hash of the block.
    pub parent_hash: B256,
    /// The hash of the block.
    pub block_hash: B256,
    /// The public key of the builder.
    pub builder_pubkey: BlsPublicKey,
    /// The public key of the proposer.
    pub proposer_pubkey: BlsPublicKey,
    /// The recipient of the proposer's fee.
    pub proposer_fee_recipient: Address,
    /// The gas limit associated with the block.
    #[serde_as(as = "DisplayFromStr")]
    pub gas_limit: u64,
    /// The gas used within the block.
    #[serde_as(as = "DisplayFromStr")]
    pub gas_used: u64,
    /// The value associated with the block.
    #[serde_as(as = "DisplayFromStr")]
    pub value: U256,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "ssz", derive(ssz_derive::Decode, ssz_derive::Encode))]
pub struct SignedBidSubmissionV2 {
    /// The BidTrace message associated with the submission.
    pub message: BidTrace,
    /// The execution payload for the submission.
    #[serde(with = "alloy_rpc_types_beacon::payload::beacon_payload_v2")]
    pub execution_payload: ExecutionPayloadV2,
    /// The signature associated with the submission.
    pub signature: BlsSignature,
}
fn main() {
    // Create a BlockHeader object
    // let header = BeaconBlock {
    //     slot: 1,
    //     proposer_index: 1,
    //     parent_root: vec![1; 32],
    //     state_root: vec![2; 32],
    //     body_root: vec![3; 32],
    // };
    let bytes =
            include_bytes!("./signed_bid_submission_capella.ssz").to_vec();
        println!("Original Header: {:?}", bytes);
        let bid = SignedBidSubmissionV2::from_ssz_bytes(&bytes).unwrap();
    
    
    // // Serialize the BeaconBlock object into SSZ format
    // let encoded_header = header.encode(); //as_ssz_bytes();

    // // Deserialize the SSZ bytes back into a BeaconBlock object
    // let decoded_header = BeaconBlock::from_ssz_bytes(&encoded_header).unwrap();

    // // Verify that the original and decoded objects are equal
    // assert_eq!(header, decoded_header);

    // println!("Original Header: {:?}", header);
    // println!("Serialized Header: {:?}", encoded_header);
    // println!("Decoded Header: {:?}", decoded_header);
}
