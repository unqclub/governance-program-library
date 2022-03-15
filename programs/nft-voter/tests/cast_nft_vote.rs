use crate::program_test::nft_voter_test::ConfigureCollectionArgs;
use gpl_nft_voter::{error::NftVoterError, tools::governance::VoterWeightAction};
use program_test::{nft_voter_test::NftVoterTest, tools::assert_nft_voter_err};
use solana_program_test::*;
use solana_sdk::transport::TransportError;

mod program_test;

#[tokio::test]
async fn test_cast_nft_vote() -> Result<(), TransportError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    let registrar_cookie = nft_voter_test.with_registrar(&realm_cookie).await?;

    let nft_collection_cookie = nft_voter_test.token_metadata.with_nft_collection().await?;

    let max_voter_weight_record_cookie = nft_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    nft_voter_test
        .with_collection(
            &registrar_cookie,
            &nft_collection_cookie,
            &max_voter_weight_record_cookie,
            Some(ConfigureCollectionArgs {
                weight: 10,
                size: 20,
            }),
        )
        .await?;

    let voter_cookie = nft_voter_test.bench.with_wallet().await;

    let voter_weight_record_cookie = nft_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let proposal_cookie = nft_voter_test
        .governance
        .with_proposal(&realm_cookie)
        .await?;

    let nft_cookie1 = nft_voter_test
        .token_metadata
        .with_nft_v2(&nft_collection_cookie, &voter_cookie, true)
        .await?;

    nft_voter_test.bench.advance_clock().await;
    let clock = nft_voter_test.bench.get_clock().await;

    // Act
    let nft_vote_record_cookies = nft_voter_test
        .cast_nft_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &[&nft_cookie1],
        )
        .await?;

    // Assert
    let nft_vote_record = nft_voter_test
        .get_nf_vote_record_account(&nft_vote_record_cookies[0].address)
        .await;

    assert_eq!(nft_vote_record_cookies[0].account, nft_vote_record);

    let voter_weight_record = nft_voter_test
        .get_voter_weight_record(&voter_weight_record_cookie.address)
        .await;

    assert_eq!(voter_weight_record.voter_weight, 10);
    assert_eq!(voter_weight_record.voter_weight_expiry, Some(clock.slot));
    assert_eq!(
        voter_weight_record.weight_action,
        Some(VoterWeightAction::CastVote.into())
    );
    assert_eq!(
        voter_weight_record.weight_action_target,
        Some(proposal_cookie.address)
    );

    Ok(())
}

#[tokio::test]
async fn test_cast_nft_vote_with_multiple_nfts() -> Result<(), TransportError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    let registrar_cookie = nft_voter_test.with_registrar(&realm_cookie).await?;

    let nft_collection_cookie = nft_voter_test.token_metadata.with_nft_collection().await?;

    let max_voter_weight_record_cookie = nft_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    nft_voter_test
        .with_collection(
            &registrar_cookie,
            &nft_collection_cookie,
            &max_voter_weight_record_cookie,
            Some(ConfigureCollectionArgs {
                weight: 10,
                size: 20,
            }),
        )
        .await?;

    let voter_cookie = nft_voter_test.bench.with_wallet().await;

    let voter_weight_record_cookie = nft_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let proposal_cookie = nft_voter_test
        .governance
        .with_proposal(&realm_cookie)
        .await?;

    let nft_cookie1 = nft_voter_test
        .token_metadata
        .with_nft_v2(&nft_collection_cookie, &voter_cookie, true)
        .await?;

    let nft_cookie2 = nft_voter_test
        .token_metadata
        .with_nft_v2(&nft_collection_cookie, &voter_cookie, true)
        .await?;

    nft_voter_test.bench.advance_clock().await;
    let clock = nft_voter_test.bench.get_clock().await;

    // Act
    let nft_vote_record_cookies = nft_voter_test
        .cast_nft_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &[&nft_cookie1, &nft_cookie2],
        )
        .await?;

    // Assert
    let nft_vote_record1 = nft_voter_test
        .get_nf_vote_record_account(&nft_vote_record_cookies[0].address)
        .await;

    assert_eq!(nft_vote_record_cookies[0].account, nft_vote_record1);

    let nft_vote_record2 = nft_voter_test
        .get_nf_vote_record_account(&nft_vote_record_cookies[1].address)
        .await;

    assert_eq!(nft_vote_record_cookies[1].account, nft_vote_record2);

    let voter_weight_record = nft_voter_test
        .get_voter_weight_record(&voter_weight_record_cookie.address)
        .await;

    assert_eq!(voter_weight_record.voter_weight, 20);
    assert_eq!(voter_weight_record.voter_weight_expiry, Some(clock.slot));
    assert_eq!(
        voter_weight_record.weight_action,
        Some(VoterWeightAction::CastVote.into())
    );
    assert_eq!(
        voter_weight_record.weight_action_target,
        Some(proposal_cookie.address)
    );

    Ok(())
}

#[tokio::test]
async fn test_cast_nft_vote_with_nft_already_voted_error() -> Result<(), TransportError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    let registrar_cookie = nft_voter_test.with_registrar(&realm_cookie).await?;

    let nft_collection_cookie = nft_voter_test.token_metadata.with_nft_collection().await?;

    let max_voter_weight_record_cookie = nft_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    nft_voter_test
        .with_collection(
            &registrar_cookie,
            &nft_collection_cookie,
            &max_voter_weight_record_cookie,
            None,
        )
        .await?;

    let voter_cookie = nft_voter_test.bench.with_wallet().await;

    let voter_weight_record_cookie = nft_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let proposal_cookie = nft_voter_test
        .governance
        .with_proposal(&realm_cookie)
        .await?;

    let nft_cookie1 = nft_voter_test
        .token_metadata
        .with_nft_v2(&nft_collection_cookie, &voter_cookie, true)
        .await?;

    nft_voter_test
        .cast_nft_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &[&nft_cookie1],
        )
        .await?;

    nft_voter_test.bench.advance_clock().await;

    // Act

    let err = nft_voter_test
        .cast_nft_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &[&nft_cookie1],
        )
        .await
        .err()
        .unwrap();

    // Assert
    assert_nft_voter_err(err, NftVoterError::NftAlreadyVoted);

    Ok(())
}

#[tokio::test]
async fn test_cast_nft_vote_invalid_voter_error() -> Result<(), TransportError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    let registrar_cookie = nft_voter_test.with_registrar(&realm_cookie).await?;

    let nft_collection_cookie = nft_voter_test.token_metadata.with_nft_collection().await?;

    let max_voter_weight_record_cookie = nft_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    nft_voter_test
        .with_collection(
            &registrar_cookie,
            &nft_collection_cookie,
            &max_voter_weight_record_cookie,
            None,
        )
        .await?;

    let voter_cookie = nft_voter_test.bench.with_wallet().await;

    let voter_weight_record_cookie = nft_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let proposal_cookie = nft_voter_test
        .governance
        .with_proposal(&realm_cookie)
        .await?;

    let nft_cookie1 = nft_voter_test
        .token_metadata
        .with_nft_v2(&nft_collection_cookie, &voter_cookie, true)
        .await?;

    let voter_cookie2 = nft_voter_test.bench.with_wallet().await;

    // Act

    let err = nft_voter_test
        .cast_nft_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie2,
            &[&nft_cookie1],
        )
        .await
        .err()
        .unwrap();

    // Assert
    assert_nft_voter_err(err, NftVoterError::InvalidVoterWeightRecordOwner);

    Ok(())
}

#[tokio::test]
async fn test_cast_nft_vote_with_unverified_collection_error() -> Result<(), TransportError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    let registrar_cookie = nft_voter_test.with_registrar(&realm_cookie).await?;

    let nft_collection_cookie = nft_voter_test.token_metadata.with_nft_collection().await?;

    let max_voter_weight_record_cookie = nft_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    nft_voter_test
        .with_collection(
            &registrar_cookie,
            &nft_collection_cookie,
            &max_voter_weight_record_cookie,
            Some(ConfigureCollectionArgs {
                weight: 10,
                size: 20,
            }),
        )
        .await?;

    let voter_cookie = nft_voter_test.bench.with_wallet().await;

    let voter_weight_record_cookie = nft_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let proposal_cookie = nft_voter_test
        .governance
        .with_proposal(&realm_cookie)
        .await?;

    // Create NFT without verified collection
    let nft_cookie1 = nft_voter_test
        .token_metadata
        .with_nft_v2(&nft_collection_cookie, &voter_cookie, false)
        .await?;

    // Act
    let err = nft_voter_test
        .cast_nft_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &[&nft_cookie1],
        )
        .await
        .err()
        .unwrap();

    // Assert
    assert_nft_voter_err(err, NftVoterError::CollectionMustBeVerified);

    Ok(())
}

#[tokio::test]
async fn test_cast_nft_vote_with_invalid_owner_error() -> Result<(), TransportError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    let registrar_cookie = nft_voter_test.with_registrar(&realm_cookie).await?;

    let nft_collection_cookie = nft_voter_test.token_metadata.with_nft_collection().await?;

    let max_voter_weight_record_cookie = nft_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    nft_voter_test
        .with_collection(
            &registrar_cookie,
            &nft_collection_cookie,
            &max_voter_weight_record_cookie,
            Some(ConfigureCollectionArgs {
                weight: 10,
                size: 20,
            }),
        )
        .await?;

    let voter_cookie = nft_voter_test.bench.with_wallet().await;

    let voter_weight_record_cookie = nft_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let voter_cookie2 = nft_voter_test.bench.with_wallet().await;

    let proposal_cookie = nft_voter_test
        .governance
        .with_proposal(&realm_cookie)
        .await?;

    let nft_cookie = nft_voter_test
        .token_metadata
        .with_nft_v2(&nft_collection_cookie, &voter_cookie2, true)
        .await?;

    // Act
    let err = nft_voter_test
        .cast_nft_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &[&nft_cookie],
        )
        .await
        .err()
        .unwrap();

    // Assert
    assert_nft_voter_err(err, NftVoterError::VoterDoesNotOwnNft);

    Ok(())
}

#[tokio::test]
async fn test_cast_nft_vote_with_invalid_collection_error() -> Result<(), TransportError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    let registrar_cookie = nft_voter_test.with_registrar(&realm_cookie).await?;

    let nft_collection_cookie = nft_voter_test.token_metadata.with_nft_collection().await?;

    let max_voter_weight_record_cookie = nft_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    nft_voter_test
        .with_collection(
            &registrar_cookie,
            &nft_collection_cookie,
            &max_voter_weight_record_cookie,
            Some(ConfigureCollectionArgs {
                weight: 10,
                size: 20,
            }),
        )
        .await?;

    let proposal_cookie = nft_voter_test
        .governance
        .with_proposal(&realm_cookie)
        .await?;

    let voter_cookie = nft_voter_test.bench.with_wallet().await;

    let voter_weight_record_cookie = nft_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let nft_collection_cookie2 = nft_voter_test.token_metadata.with_nft_collection().await?;

    let nft_cookie = nft_voter_test
        .token_metadata
        .with_nft_v2(&nft_collection_cookie2, &voter_cookie, true)
        .await?;

    // Act
    let err = nft_voter_test
        .cast_nft_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &[&nft_cookie],
        )
        .await
        .err()
        .unwrap();

    // Assert
    assert_nft_voter_err(err, NftVoterError::CollectionNotFound);

    Ok(())
}

#[tokio::test]
async fn test_cast_nft_vote_with_invalid_metadata_error() -> Result<(), TransportError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    let registrar_cookie = nft_voter_test.with_registrar(&realm_cookie).await?;

    let nft_collection_cookie = nft_voter_test.token_metadata.with_nft_collection().await?;

    let max_voter_weight_record_cookie = nft_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    nft_voter_test
        .with_collection(
            &registrar_cookie,
            &nft_collection_cookie,
            &max_voter_weight_record_cookie,
            Some(ConfigureCollectionArgs {
                weight: 10,
                size: 20,
            }),
        )
        .await?;

    let proposal_cookie = nft_voter_test
        .governance
        .with_proposal(&realm_cookie)
        .await?;

    let voter_cookie = nft_voter_test.bench.with_wallet().await;

    let voter_weight_record_cookie = nft_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let mut nft1_cookie = nft_voter_test
        .token_metadata
        .with_nft_v2(&nft_collection_cookie, &voter_cookie, false)
        .await?;

    let nft2_cookie = nft_voter_test
        .token_metadata
        .with_nft_v2(&nft_collection_cookie, &voter_cookie, true)
        .await?;

    // Try to use verified NFT Metadata
    nft1_cookie.metadata = nft2_cookie.metadata;

    // Act
    let err = nft_voter_test
        .cast_nft_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &[&nft1_cookie],
        )
        .await
        .err()
        .unwrap();

    // Assert
    assert_nft_voter_err(err, NftVoterError::TokenMetadataDoesNotMatch);

    Ok(())
}

#[tokio::test]
async fn test_cast_nft_vote_with_same_nft_error() -> Result<(), TransportError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    let registrar_cookie = nft_voter_test.with_registrar(&realm_cookie).await?;

    let nft_collection_cookie = nft_voter_test.token_metadata.with_nft_collection().await?;

    let max_voter_weight_record_cookie = nft_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    nft_voter_test
        .with_collection(
            &registrar_cookie,
            &nft_collection_cookie,
            &max_voter_weight_record_cookie,
            None,
        )
        .await?;

    let proposal_cookie = nft_voter_test
        .governance
        .with_proposal(&realm_cookie)
        .await?;

    let voter_cookie = nft_voter_test.bench.with_wallet().await;

    let voter_weight_record_cookie = nft_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let nft_cookie = nft_voter_test
        .token_metadata
        .with_nft_v2(&nft_collection_cookie, &voter_cookie, true)
        .await?;

    // Act
    let err = nft_voter_test
        .cast_nft_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &[&nft_cookie, &nft_cookie],
        )
        .await
        .err()
        .unwrap();

    // Assert

    assert_nft_voter_err(err, NftVoterError::DuplicatedNftDetected);

    Ok(())
}
