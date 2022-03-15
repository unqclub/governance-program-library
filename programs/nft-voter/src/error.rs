use anchor_lang::prelude::*;

#[error_code]
pub enum NftVoterError {
    #[msg("Invalid Realm Authority")]
    InvalidRealmAuthority,

    #[msg("Invalid Registrar Realm")]
    InvalidRegistrarRealm,

    #[msg("Given NFT is not part of a collection or metadata format is not V2")]
    NotPartOfCollection,

    #[msg("There is no NFT in the account")]
    InsufficientAmountOnNFTAccount,

    #[msg("Invalid Collection Size")]
    InvalidCollectionSize,

    #[msg("Invalid MaxVoterWeightRecord Realm")]
    InvalidMaxVoterWeightRecordRealm,

    #[msg("Invalid MaxVoterWeightRecord Mint")]
    InvalidMaxVoterWeightRecordMint,

    #[msg("CastVote Is Not Allowed")]
    CastVoteIsNotAllowed,

    #[msg("Invalid VoterWeightRecord Realm")]
    InvalidVoterWeightRecordRealm,

    #[msg("Invalid VoterWeightRecord Mint")]
    InvalidVoterWeightRecordMint,

    #[msg("Invalid VoterWeightRecord Owner")]
    InvalidVoterWeightRecordOwner,

    #[msg("Collection must be verified")]
    CollectionMustBeVerified,

    #[msg("Voter does not own NFT")]
    VoterDoesNotOwnNft,

    #[msg("Collection not found")]
    CollectionNotFound,

    #[msg("Missing Metadata collection")]
    MissingMetadataCollection,

    #[msg("Token Metadata doesn't match")]
    TokenMetadataDoesNotMatch,

    #[msg("Invalid account owner")]
    InvalidAccountOwner,

    #[msg("Invalid token metadata account")]
    InvalidTokenMetadataAccount,

    #[msg("Duplicated NFT detected")]
    DuplicatedNftDetected,

    #[msg("NFT already voted")]
    NftAlreadyVoted,
}
